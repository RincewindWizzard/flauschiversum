//! Ablauf eines Builds: Laden → Medien → Rendern → Ausgaben (ohne zu schreiben).

use crate::config::{BLOG_URL, POSTS_PER_PAGE, TIME_ZONE};
use crate::content::{self, Site};
use crate::diag::Diagnostics;
use crate::feed;
use crate::markdown::{self, PostContext};
use crate::media::{self, MediaIndex, MediaInfo, Stats};
use crate::output::{Content, Outputs};
use crate::render::{self, Common, PostCard, PostPageInput, Thumb};
use crate::seo;
use crate::urls;
use anyhow::{Context, Result, bail};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct BuildOptions {
    pub root: PathBuf,
    pub cache: PathBuf,
    pub now: jiff::Zoned,
    pub livereload: bool,
}

impl BuildOptions {
    pub fn now() -> Result<jiff::Zoned> {
        Ok(jiff::Zoned::now().with_time_zone(jiff::tz::TimeZone::get(TIME_ZONE)?))
    }
}

pub struct Generated {
    pub outputs: Outputs,
    pub site: Site,
}

/// Verzeichnis der vom Generator ausgelieferten Dateien (Skripte, Badge-Vorlage).
fn assets_dir(root: &Path) -> PathBuf {
    root.join("assets")
}

fn short_hash(data: &[u8]) -> String {
    blake3::hash(data).to_hex()[..10].to_string()
}

/// Bündelt `content/style/main.css` (inkl. `@import`) und minimiert das Ergebnis.
pub fn bundle_css(style_dir: &Path) -> Result<String> {
    use lightningcss::bundler::{Bundler, FileProvider};
    use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions};
    use lightningcss::targets::{Browsers, Targets};
    let entry = style_dir.join("main.css");
    let provider = FileProvider::new();
    let mut bundler = Bundler::new(&provider, None, ParserOptions::default());
    let mut sheet = bundler.bundle(&entry).map_err(|e| anyhow::anyhow!("{}: {e}", entry.display()))?;
    // Nesting für ältere Browser auflösen
    let targets = Targets::from(Browsers { chrome: Some(100 << 16), firefox: Some(100 << 16), safari: Some(15 << 16), ..Default::default() });
    sheet.minify(MinifyOptions { targets, ..Default::default() }).map_err(|e| anyhow::anyhow!("CSS: {e}"))?;
    let css = sheet
        .to_css(PrinterOptions { minify: true, targets, ..Default::default() })
        .map_err(|e| anyhow::anyhow!("CSS: {e}"))?;
    Ok(css.code)
}

/// Erzeugt alle Ausgaben. Bricht ab, wenn die Inhalte Fehler enthalten.
pub fn generate(opts: &BuildOptions, diag: &Diagnostics, stats: &Stats) -> Result<Generated> {
    let site = content::load(&opts.root, &opts.now, diag);
    if diag.has_errors() {
        bail!("Inhalte enthalten Fehler");
    }
    let outputs = Outputs::default();

    // Statische Dateien
    for f in &site.static_files {
        outputs.insert(f.url.clone(), Content::File(f.path.clone()));
    }

    // Generator-Assets (ohne Badge-Vorlage) unter /assets/
    let assets = assets_dir(&opts.root);
    let mut script_href = String::from("/assets/js/media.js");
    for entry in walkdir::WalkDir::new(&assets).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
        let rel = entry.path().strip_prefix(&assets).unwrap().to_string_lossy().replace('\\', "/");
        if rel == "badge.png" {
            continue;
        }
        if rel == "js/media.js" {
            script_href = format!("/assets/js/media.js?v={}", short_hash(&fs::read(entry.path())?));
        }
        outputs.insert(format!("/assets/{rel}"), Content::File(entry.path().to_path_buf()));
    }

    // Stylesheet
    let css = bundle_css(&site.style_dir).context("Stylesheet")?;
    let common = Common {
        style_href: format!("/style.css?v={}", short_hash(css.as_bytes())),
        script_href,
        year: opts.now.year(),
        livereload: opts.livereload,
    };
    outputs.insert("/style.css", Content::Bytes(css.into_bytes()));

    // Medien
    let badge = fs::read(assets.join("badge.png")).context("assets/badge.png fehlt")?;
    let index: MediaIndex = media::build(&site, &badge, &opts.cache, &outputs, diag, stats)?;
    let mut by_post: HashMap<&str, HashMap<String, MediaInfo>> = HashMap::new();
    for ((url, name), info) in &index {
        by_post.entry(url.as_str()).or_default().insert(name.clone(), info.clone());
    }
    let empty = HashMap::new();
    let media_of = |url: &str| by_post.get(url).unwrap_or(&empty);

    let thumb_of = |post: &content::Post| -> Option<Thumb> {
        let name = post.image.as_ref()?;
        let size = media_of(&post.url).get(name)?.thumb?;
        Some(Thumb { src: urls::encode_path(&urls::variant_path(&post.url, 200, name)), width: size.width, height: size.height })
    };

    // Post-Seiten
    let posts = &site.posts;
    let feed_parts: Vec<Option<feed::FeedEntry>> = posts
        .par_iter()
        .enumerate()
        .map(|(i, post)| {
            let media = media_of(&post.url);
            let ctx = PostContext { url: &post.url, media, absolute: false };
            let rendered = markdown::render_post(&post.markdown, &ctx);
            for m in &rendered.missing {
                diag.warning(&post.source, format!("verweist auf `{m}`, das nicht als Medium im Post-Verzeichnis existiert"));
            }
            let og_image = post.image.as_ref().and_then(|name| {
                let size = media.get(name)?.full?;
                Some((urls::absolute(&urls::variant_path(&post.url, 1000, name)), size))
            });
            let has_media = rendered.html.contains("<figure class=\"media");
            let html = render::render_post(
                PostPageInput {
                    post,
                    content: rendered.html,
                    thumb: thumb_of(post),
                    og_image,
                    newer: i.checked_sub(1).map(|j| &posts[j]),
                    older: posts.get(i + 1),
                    has_media,
                },
                &common,
            );
            outputs.insert(post.url.clone(), Content::Bytes(html.into_bytes()));

            (i < crate::config::FEED_ENTRIES).then(|| {
                let abs = markdown::render_post(&post.markdown, &PostContext { url: &post.url, media, absolute: true });
                let enclosure = post.image.as_ref().and_then(|name| {
                    let info = media.get(name)?;
                    Some((urls::absolute(&urls::variant_path(&post.url, 700, name)), info.preview_len, feed::mime_for(name)))
                });
                feed::FeedEntry { content_html: abs.html, enclosure }
            })
        })
        .collect();

    // Indexseiten
    let count = render::page_count(posts.len());
    (0..count).into_par_iter().for_each(|index| {
        let cards = posts
            .iter()
            .skip(index * POSTS_PER_PAGE)
            .take(POSTS_PER_PAGE)
            .map(|p| PostCard {
                url: urls::encode_path(&p.url),
                title: p.title.clone(),
                date_iso: seo::iso(p),
                date_human: render::date_human(p),
                thumb: thumb_of(p),
                excerpt: p.excerpt.clone().unwrap_or_default(),
            })
            .collect();
        let html = render::render_index(index, count, cards, &common);
        outputs.insert(render::index_url(index), Content::Bytes(html.into_bytes()));
    });

    // Einzelseiten
    for page in &site.pages {
        let html = markdown::render_page(&page.markdown);
        let desc = markdown::first_paragraph_text(&page.markdown);
        outputs.insert(page.url.clone(), Content::Bytes(render::render_page(page, html, desc, &common).into_bytes()));
    }

    // Feed, Sitemap, robots.txt
    let feed_xml = feed::render(&site, feed_parts.into_iter().flatten().collect());
    outputs.insert("/rss.xml", Content::Bytes(feed_xml.into_bytes()));
    outputs.insert("/sitemap.xml", Content::Bytes(seo::sitemap(&site).into_bytes()));
    outputs.insert("/robots.txt", Content::Bytes(seo::robots().into_bytes()));

    let mut sources: HashMap<String, String> = HashMap::new();
    for p in &site.posts {
        sources.insert(format!("{}index.html", p.url), p.source.clone());
    }
    for p in &site.pages {
        sources.insert(format!("{}index.html", p.url), p.source.clone());
    }
    check_links(&outputs, &sources, diag);
    Ok(Generated { outputs, site })
}

/// Prüft `href`- und `src`-Verweise aller HTML-Seiten auf interne Ziele.
fn check_links(outputs: &Outputs, sources: &HashMap<String, String>, diag: &Diagnostics) {
    let map = outputs.snapshot_html();
    let exists = |path: &str| outputs.contains(path) || (!path.ends_with('/') && outputs.contains(&format!("{path}/")));
    map.par_iter().for_each(|(page_path, html)| {
        let base = page_path.trim_end_matches("index.html");
        let mut reported = Vec::new();
        for attr in ["href=\"", "src=\""] {
            for (pos, _) in html.match_indices(attr) {
                let rest = &html[pos + attr.len()..];
                let Some(end) = rest.find('"') else { continue };
                let raw = rest[..end].replace("&amp;", "&");
                let target = if let Some(t) = raw.strip_prefix(BLOG_URL) {
                    t.to_string()
                } else if raw.starts_with("http://flauschiversum.de") || raw.starts_with("https://www.flauschiversum.de") || raw.starts_with("http://www.flauschiversum.de") {
                    raw.split_once(".de").map(|x| x.1.to_string()).unwrap_or_default()
                } else if raw.contains(':') || raw.starts_with('#') || raw.starts_with("//") || raw.is_empty() {
                    continue;
                } else if raw.starts_with('/') {
                    raw.clone()
                } else {
                    format!("{base}{raw}")
                };
                let target = target.split(['#', '?']).next().unwrap_or("").to_string();
                if target.is_empty() || target == "/__livereload" {
                    continue;
                }
                let decoded = urls::decode_path(&target);
                if !exists(&decoded) && !reported.contains(&decoded) {
                    reported.push(decoded);
                }
            }
        }
        let file = sources.get(page_path).cloned().unwrap_or_else(|| format!("templates (Seite {page_path})"));
        for target in reported {
            diag.warning(&file, format!("interner Link auf `{target}` führt ins Leere"));
        }
    });
}

/// Hilfsfunktion für Tests: baut eine Site in einem Verzeichnis zu einer festen Zeit.
#[cfg(test)]
pub fn test_options(root: &Path) -> BuildOptions {
    BuildOptions {
        root: root.to_path_buf(),
        cache: root.join(".cache"),
        now: "2026-10-10T12:00[Europe/Berlin]".parse().unwrap(),
        livereload: false,
    }
}

