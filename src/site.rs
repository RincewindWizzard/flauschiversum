//! Ablauf eines Builds: Laden → Medien → Rendern → Ausgaben (ohne zu schreiben).

use crate::archive;
use crate::config::{BLOG_NAME, HOME_NEWS, POSTS_LATEST, POSTS_PER_PAGE, TIME_ZONE};
use crate::content::{self, Site};
use crate::diag::Diagnostics;
use crate::feed;
use crate::markdown::{self, PostContext};
use crate::media::{self, MediaIndex, MediaInfo, Stats};
use crate::output::{Content, Outputs};
use crate::render::{self, CloudTag, Common, Entry, PostCard, PostPageInput, Thumb, TopicLink, TopicPageInput};
use std::sync::Arc;
use crate::search::{self, SearchDoc};
use crate::seo;
use crate::topics::{self, Topics};
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
    /// Basis-URL für Absolut-URLs (Standard `https://flauschiversum.de`).
    pub base: urls::Base,
    /// Staging-Modus mit Branchname.
    pub staging: Option<String>,
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
    crate::progress::step("Lese Inhalte (Beiträge, Seiten, Schlagworte) …");
    let site = content::load(&opts.root, &opts.now, diag);
    if diag.has_errors() {
        bail!("Inhalte enthalten Fehler");
    }
    crate::progress::note(&format!("{} Beiträge, {} Seiten, {} Medien", site.posts.len(), site.pages.len(), site.posts.iter().map(|p| p.media.len()).sum::<usize>()));
    let outputs = Outputs::default();

    crate::progress::step("Kopiere statische Dateien und Skripte …");
    // Statische Dateien
    for f in &site.static_files {
        outputs.insert(f.url.clone(), Content::File(f.path.clone()));
    }

    // Generator-Assets (ohne Badge-Vorlage) unter /assets/
    let assets = assets_dir(&opts.root);
    let mut script_href = String::from("/assets/js/media.js");
    let mut search_script_href = String::from("/assets/js/search.js");
    let mut topbar_script_href = String::from("/assets/js/topbar.js");
    for entry in walkdir::WalkDir::new(&assets).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
        let rel = entry.path().strip_prefix(&assets).unwrap().to_string_lossy().replace('\\', "/");
        if rel == "badge.png" {
            continue;
        }
        if rel == "js/media.js" {
            script_href = format!("/assets/js/media.js?v={}", short_hash(&fs::read(entry.path())?));
        }
        if rel == "js/topbar.js" {
            topbar_script_href = format!("/assets/js/topbar.js?v={}", short_hash(&fs::read(entry.path())?));
        }
        if rel == "js/search.js" {
            search_script_href = format!("/assets/js/search.js?v={}", short_hash(&fs::read(entry.path())?));
        }
        outputs.insert(format!("/assets/{rel}"), Content::File(entry.path().to_path_buf()));
    }

    // Stylesheet
    crate::progress::step("Baue Stylesheet und Themenlisten …");
    let css = bundle_css(&site.style_dir).context("Stylesheet")?;
    let topics = Topics::build(&site, diag);
    if diag.has_errors() {
        bail!("Inhalte enthalten Fehler");
    }
    let cloud = Arc::new(
        topics
            .cloud()
            .into_iter()
            .map(|(t, level)| CloudTag { url: topics::tag_url(&t.slug), name: t.name.to_string(), count: t.posts.len(), level })
            .collect::<Vec<_>>(),
    );
    let common = Common {
        style_href: format!("/style.css?v={}", short_hash(css.as_bytes())),
        script_href,
        topbar_script_href,
        year: opts.now.year(),
        livereload: opts.livereload,
        base: opts.base.clone(),
        staging: opts.staging.clone(),
        cloud,
    };
    outputs.insert("/style.css", Content::Bytes(css.into_bytes()));

    // Medien
    crate::progress::step("Verarbeite Bilder und Medien (Größen, Wasserzeichen; beim ersten Lauf lang) …");
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

    let tag_links = |post: &content::Post| -> Vec<TopicLink> {
        post.tags
            .iter()
            .map(|t| TopicLink { url: topics::tag_url(&urls::slugify(t)), name: t.clone(), count: None })
            .collect()
    };
    let entry_of = |p: &content::Post| Entry {
        url: urls::encode_path(&p.url),
        title: p.title.clone(),
        date_iso: seo::iso(p),
        date_human: render::date_human(p),
        thumb: thumb_of(p),
        excerpt: p.excerpt.clone().unwrap_or_default(),
    };

    crate::progress::step("Rendere Beiträge (Markdown nach HTML) …");
    // Post-Seiten
    let posts = &site.posts;
    let counts = topics::tag_counts(posts);
    let related_links = |i: usize| -> Vec<Entry> { topics::related(posts, i, &counts).into_iter().map(|r| entry_of(r.post)).collect() };
    let post_parts: Vec<(Option<feed::FeedEntry>, SearchDoc)> = posts
        .par_iter()
        .enumerate()
        .map(|(i, post)| {
            let media = media_of(&post.url);
            let ctx = PostContext { url: &post.url, media, base: None };
            let rendered = markdown::render_post(&post.markdown, &ctx);
            for m in &rendered.missing {
                diag.warning(&post.source, format!("verweist auf `{m}`, das nicht als Medium im Post-Verzeichnis existiert"));
            }
            let og_image = post.image.as_ref().and_then(|name| {
                let size = media.get(name)?.full?;
                Some((common.base.absolute(&urls::variant_path(&post.url, 1000, name)), size))
            });
            let has_media = rendered.html.contains("<figure class=\"media");
            let search_doc = SearchDoc {
                url: post.url.clone(),
                title: post.title.clone(),
                image: thumb_of(post).map(|t| t.src),
                tags: post.tags.clone(),
                body_html: rendered.html.clone(),
            };
            let html = render::render_post(
                PostPageInput {
                    post,
                    content: rendered.html,
                    thumb: thumb_of(post),
                    og_image,
                    newer: i.checked_sub(1).map(|j| &posts[j]),
                    older: posts.get(i + 1),
                    has_media,
                    related: related_links(i),
                    tags: tag_links(post),
                },
                &common,
            );
            outputs.insert(post.url.clone(), Content::Bytes(html.into_bytes()));

            let feed_entry = (i < crate::config::FEED_ENTRIES).then(|| {
                let abs = markdown::render_post(&post.markdown, &PostContext { url: &post.url, media, base: Some(&common.base) });
                let enclosure = post.image.as_ref().and_then(|name| {
                    let info = media.get(name)?;
                    Some((common.base.absolute(&urls::variant_path(&post.url, 700, name)), info.preview_len, feed::mime_for(name)))
                });
                // Auch Textlinks und eingebettete Pfade absolut machen (Feedreader kennen die Basis-URL nicht)
                let content_html = abs
                    .html
                    .replace("href=\"/", &format!("href=\"{}/", common.base.as_str()))
                    .replace("src=\"/", &format!("src=\"{}/", common.base.as_str()));
                feed::FeedEntry { content_html, enclosure }
            });
            (feed_entry, search_doc)
        })
        .collect();
    let (feed_parts, mut search_docs): (Vec<_>, Vec<_>) = post_parts.into_iter().unzip();

    crate::progress::step("Erzeuge Listen, Archiv und Themenseiten …");
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
        if index > 0 {
            let html = render::render_index(index, count, cards, &common);
            outputs.insert(render::index_url(index), Content::Bytes(html.into_bytes()));
        }
    });
    {
        let all: Vec<&content::Post> = posts.iter().collect();
        let latest = posts.iter().take(POSTS_LATEST).map(entry_of).collect();
        let html = render::render_posts(latest, archive::build(&all), &common);
        outputs.insert(render::index_url(0), Content::Bytes(html.into_bytes()));
    }

    // Startseite, Themenübersicht und Themenseiten
    let news = posts.iter().take(HOME_NEWS).map(entry_of).collect();
    let welcome = site.start.as_deref().map(markdown::render_page);
    outputs.insert("/", Content::Bytes(render::render_home(welcome, news, &common).into_bytes()));

    let link_of = |url: String, name: &str, n: usize| TopicLink { url, name: name.to_string(), count: Some(n) };
    let overview_tags = topics.tags.iter().map(|t| link_of(topics::tag_url(&t.slug), t.name, t.posts.len())).collect();
    outputs.insert("/themen/", Content::Bytes(render::render_topics(overview_tags, &common).into_bytes()));

    for t in &topics.tags {
        let url = topics::tag_url(&t.slug);
        let html = render::render_topic(
            TopicPageInput {
                url: url.clone(),
                name: t.name.to_string(),
                count: t.posts.len(),
                archive: archive::build(&t.posts),
                entries: t.posts.iter().map(|p| entry_of(p)).collect(),
            },
            &common,
        );
        outputs.insert(url, Content::Bytes(html.into_bytes()));
    }

    // Weiterleitungen alter Adressen (Cool URIs): kleine Seite mit Meta-Refresh und kanonischem Verweis
    for (from, to) in &site.redirects {
        if outputs.contains(from) {
            diag.error("content/redirects.yaml", format!("Weiterleitung `{from}`: unter dieser Adresse gibt es schon eine Seite"));
            continue;
        }
        if !outputs.contains(to) {
            diag.error("content/redirects.yaml", format!("Weiterleitung `{from}` → `{to}`: Ziel existiert nicht"));
            continue;
        }
        let target = urls::encode_path(to);
        let canonical = common.base.absolute(to);
        let html = format!(
            "<!DOCTYPE html>\n<html lang=\"de\">\n<head>\n<meta charset=\"utf-8\">\n<title>Weiterleitung – {BLOG_NAME}</title>\n<meta name=\"robots\" content=\"noindex\">\n<link rel=\"canonical\" href=\"{canonical}\">\n<meta http-equiv=\"refresh\" content=\"0; url={target}\">\n</head>\n<body>\n<p>Diese Seite ist umgezogen: <a href=\"{target}\">{target}</a></p>\n</body>\n</html>\n"
        );
        outputs.insert(from.clone(), Content::Bytes(html.into_bytes()));
    }
    if diag.has_errors() {
        bail!("Inhalte enthalten Fehler");
    }

    // Einzelseiten
    for page in &site.pages {
        let html = markdown::render_page(&page.markdown);
        let desc = markdown::first_paragraph_text(&page.markdown);
        search_docs.push(SearchDoc {
            url: page.url.clone(),
            title: page.title.clone(),
            image: None,
            tags: Vec::new(),
            body_html: html.clone(),
        });
        outputs.insert(page.url.clone(), Content::Bytes(render::render_page(page, html, desc, &common).into_bytes()));
    }

    crate::progress::step("Baue den Suchindex …");
    // Suche: Seite und Index
    outputs.insert("/suche/", Content::Bytes(render::render_search(search_script_href, &common).into_bytes()));
    for (path, data) in search::build(search_docs, &opts.cache).context("Suchindex")? {
        outputs.insert(path, Content::Bytes(data));
    }

    crate::progress::step("Erzeuge Feed, Sitemap und robots.txt …");
    // Feed, Sitemap, robots.txt
    let feed_xml = feed::render(&site, feed_parts.into_iter().flatten().collect(), &common.base);
    outputs.insert("/rss.xml", Content::Bytes(feed_xml.into_bytes()));
    if opts.staging.is_none() {
        outputs.insert("/sitemap.xml", Content::Bytes(seo::sitemap(&site, &topics, &common.base).into_bytes()));
    }
    outputs.insert("/robots.txt", Content::Bytes(seo::robots(&common.base, opts.staging.is_some()).into_bytes()));

    let mut sources: HashMap<String, String> = HashMap::new();
    for p in &site.posts {
        sources.insert(format!("{}index.html", p.url), p.source.clone());
    }
    for p in &site.pages {
        sources.insert(format!("{}index.html", p.url), p.source.clone());
    }
    crate::progress::step("Prüfe interne Links …");
    check_links(&outputs, &sources, &common.base, diag);
    Ok(Generated { outputs, site })
}

/// Prüft `href`- und `src`-Verweise aller HTML-Seiten auf interne Ziele.
fn check_links(outputs: &Outputs, sources: &HashMap<String, String>, site_base: &urls::Base, diag: &Diagnostics) {
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
                let target = if let Some(t) = raw.strip_prefix(site_base.as_str()) {
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
        base: urls::Base::default(),
        staging: None,
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::output;
    use std::sync::atomic::Ordering;

    fn jpeg(width: u32, height: u32, shade: u8) -> Vec<u8> {
        let mut out = Vec::new();
        jpeg_encoder::Encoder::new(&mut out, 90)
            .encode(&vec![shade; (width * height * 3) as usize], width as u16, height as u16, jpeg_encoder::ColorType::Rgb)
            .unwrap();
        out
    }

    fn png(width: u32, height: u32) -> Vec<u8> {
        let mut out = Vec::new();
        image::DynamicImage::new_rgba8(width, height)
            .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
            .unwrap();
        out
    }

    fn write(root: &Path, rel: &str, data: impl AsRef<[u8]>) {
        let p = root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, data).unwrap();
    }

    /// Kleine Test-Site: drei veröffentlichte Posts, ein geplanter, eine Seite.
    pub fn fixture_site(root: &Path) {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
        for e in walkdir::WalkDir::new(repo.join("assets")).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            write(root, &format!("assets/{}", e.path().strip_prefix(repo.join("assets")).unwrap().display()), fs::read(e.path()).unwrap());
        }
        for e in fs::read_dir(repo.join("content/style")).unwrap().filter_map(Result::ok) {
            write(root, &format!("content/style/{}", e.file_name().to_string_lossy()), fs::read(e.path()).unwrap());
        }
        write(root, "content/static/favicon.ico", b"ico");
        for mascot in ["flauschi_normal.svg", "flauschi_surprised.svg"] {
            write(root, &format!("content/static/images/{mascot}"), fs::read(repo.join("content/static/images").join(mascot)).unwrap());
        }
        write(root, "content/start.md", "Willkommen im **Flauschiversum**!\n");
        write(root, "content/tags.yaml", "- name: LARP\n- name: Hüte\n- name: Halloween\n");
        write(root, "content/redirects.yaml", "/kategorie/basteln/: /tag/larp/\n");
        write(root, "content/pages/about.md", "---\ntitle: \"About\"\n---\n\n# Das Flauschiversum\n\nEin Blog über Wolle & Filz.\n\n[Kürbis](/2020/10/kurbis/)\n");
        write(
            root,
            "content/posts/alraune/index.md",
            "---\ntitle: \"Alraune\"\npermalink: /2014/01/alraune/\nauthor: Ermeline\ntags: [LARP, Hüte]\ndate: 2014-01-04 19:10:28\nimage: \"DSCF0527.jpg\"\nexcerpt: \"Mal wieder etwas für das Hobby LARP.\"\n---\n\n![Alraune oben](DSCF0527.jpg)\n![Alraune nah oben](DSCF0528.jpg)\n![Clip](clip.mp4)\n\nMal wieder etwas für das Hobby LARP.\n\n![Ohrenschützer](DSCF0531.jpg)\n\nSiehe [Kürbis](/2020/10/kurbis/) und [kaputt](/2014/01/gibts-nicht/).\n",
        );
        write(root, "content/posts/alraune/DSCF0527.jpg", jpeg(800, 600, 40));
        write(root, "content/posts/alraune/DSCF0528.jpg", jpeg(600, 800, 90));
        write(root, "content/posts/alraune/DSCF0531.jpg", jpeg(1200, 900, 150));
        write(root, "content/posts/alraune/clip.mp4", b"kein echtes Video");
        write(root, "content/posts/alraune/Stifttextur.xcf", b"ignoriert");
        write(
            root,
            "content/posts/kuerbis/index.md",
            "---\ntitle: \"Kürbis\"\npermalink: /2020/10/kurbis/\nauthor: Ermeline\ntags: [Halloween, Hüte]\ndate: 2020-10-31\nimage: \"kürbis thumb.png\"\nexcerpt: \"Es ist wieder die gruselige Jahreszeit und da muss man einen Kürbis haben.\"\n---\n\n![Kürbis](kürbis thumb.png)\n",
        );
        write(root, "content/posts/kuerbis/kürbis thumb.png", png(400, 300));
        write(root, "content/posts/grüne-mütze/index.md", "---\ntitle: \"Grüne Mütze\"\ntags: [Hüte, LARP]\ndate: 2026-01-05\nexcerpt: \"Ohne Bild.\"\n---\n\nText.\n");
        write(root, "content/posts/morgen/index.md", "---\ntitle: \"Morgen\"\ntags: [Halloween]\ndate: 2099-01-01\nimage: \"bild.jpg\"\n---\n\n![Bild](bild.jpg)\n");
        write(root, "content/posts/morgen/bild.jpg", jpeg(300, 200, 10));
    }

    fn build(root: &Path) -> (Diagnostics, Stats, output::WriteStats) {
        let diag = Diagnostics::default();
        let stats = Stats::default();
        let g = generate(&test_options(root), &diag, &stats).unwrap();
        let ws = output::write(&root.join("dist"), g.outputs.into_map()).unwrap();
        (diag, stats, ws)
    }

    fn read(root: &Path, rel: &str) -> String {
        fs::read_to_string(root.join("dist").join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
    }

    /// Entfernt build-abhängige Werte (Hashes) für Snapshots.
    fn stable(html: &str) -> String {
        let mut out = String::new();
        let mut rest = html;
        while let Some(i) = rest.find("?v=") {
            out.push_str(&rest[..i + 3]);
            out.push_str("HASH");
            rest = &rest[i + 3..];
            rest = &rest[rest.find('"').unwrap_or(0)..];
        }
        out.push_str(rest);
        out
    }

    /// `clean` erfasst jede Datei, die ein Build im Cache anlegt; der nächste Build liefert dasselbe Ergebnis.
    #[test]
    fn clean_covers_everything_a_build_caches() {
        let t = tempfile::tempdir().unwrap();
        fixture_site(t.path());
        let opts = test_options(t.path());
        let build = |opts: &BuildOptions| {
            let g = generate(opts, &Diagnostics::default(), &Stats::default()).unwrap();
            let dist = t.path().join("dist");
            output::write(&dist, g.outputs.into_map()).unwrap();
            let mut files: Vec<(String, Vec<u8>)> = walkdir::WalkDir::new(&dist)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file() && !e.file_name().to_string_lossy().starts_with(".flauschiversum"))
                // Pagefind ist nicht bitgenau reproduzierbar (Hash-Namen); sein Verhalten prüft `search.rs`
                .filter(|e| !e.path().starts_with(dist.join("pagefind")))
                .map(|e| (e.path().strip_prefix(&dist).unwrap().display().to_string(), fs::read(e.path()).unwrap()))
                .collect();
            files.sort();
            files
        };
        let first = build(&opts);
        assert!(fs::read_dir(&opts.cache).unwrap().next().is_some(), "der Build legt einen Cache an");
        // Vorschau-Ausgabe wie bei `serve`
        let serve_out = opts.cache.join("serve");
        output::write(&serve_out, std::collections::BTreeMap::from([("/index.html".to_string(), output::Content::Bytes(b"x".to_vec()))])).unwrap();
        let removed = crate::clean::clean_cache(&opts.cache, t.path()).unwrap();
        assert!(removed.files > 0);
        assert_eq!(fs::read_dir(&opts.cache).unwrap().count(), 0, "nichts bleibt im Cache zurück");
        let second = build(&opts);
        let names = |v: &[(String, Vec<u8>)]| v.iter().map(|f| f.0.clone()).collect::<Vec<_>>();
        let (n1, n2) = (names(&first), names(&second));
        let diff: Vec<_> = n1.iter().filter(|n| !n2.contains(n)).chain(n2.iter().filter(|n| !n1.contains(n))).collect();
        assert!(diff.is_empty(), "dieselben Dateien nach dem Leeren, abweichend: {diff:?}");
        let differing: Vec<_> = first.iter().zip(&second).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0.clone()).collect();
        assert!(differing.is_empty(), "gleicher Inhalt nach dem Leeren, abweichend: {differing:?}");
    }

    #[test]
    fn builds_fixture_site() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (diag, stats, ws) = build(root);

        // Geplanter Post: weder Seite noch Medien
        assert!(!root.join("dist/2099").exists());
        assert!(!read(root, "rss.xml").contains("Morgen"));
        assert!(!read(root, "sitemap.xml").contains("2099"));
        // Neuer Post ohne Permalink: URL aus dem Verzeichnisnamen
        assert!(root.join("dist/2026/01/gruene-muetze/index.html").is_file());
        // Medien
        for f in ["200x/DSCF0527.jpg", "700x/DSCF0527.jpg", "1000x/DSCF0527.jpg", "clip.mp4"] {
            assert!(root.join("dist/2014/01/alraune").join(f).is_file(), "{f}");
        }
        assert_eq!(fs::read(root.join("dist/2014/01/alraune/clip.mp4")).unwrap(), b"kein echtes Video");
        assert!(!root.join("dist/2014/01/alraune/Stifttextur.xcf").exists());
        assert!(fs::read(root.join("dist/2020/10/kurbis/700x/kürbis thumb.png")).unwrap().starts_with(b"\x89PNG"));
        assert_eq!(stats.processed.load(Ordering::Relaxed), 4);

        // Kaputter interner Link wird mit Quelldatei gemeldet
        let warnings: Vec<_> = diag.all().into_iter().map(|d| format!("{}: {}", d.file, d.message)).collect();
        assert!(
            warnings.iter().any(|w| w.starts_with("content/posts/alraune/index.md") && w.contains("/2014/01/gibts-nicht/")),
            "{warnings:?}"
        );
        assert_eq!(ws.deleted, 0);

        insta::assert_snapshot!("post_alraune", stable(&read(root, "2014/01/alraune/index.html")));
        insta::assert_snapshot!("index", stable(&read(root, "beitraege/index.html")));
        insta::assert_snapshot!("home", stable(&read(root, "index.html")));
        insta::assert_snapshot!("tag_larp", stable(&read(root, "tag/larp/index.html")));
        insta::assert_snapshot!("redirect_kategorie", stable(&read(root, "kategorie/basteln/index.html")));
        insta::assert_snapshot!("tag_huete", stable(&read(root, "tag/huete/index.html")));
        insta::assert_snapshot!("topics", stable(&read(root, "themen/index.html")));
        insta::assert_snapshot!("search", stable(&read(root, "suche/index.html")));
        insta::assert_snapshot!("page_about", stable(&read(root, "about/index.html")));
        let rss = read(root, "rss.xml");
        assert!(rss.contains("<a href=\"https://flauschiversum.de/2020/10/kurbis/\">Kürbis</a>"));
        insta::assert_snapshot!("rss", rss);
        insta::assert_snapshot!("sitemap", read(root, "sitemap.xml"));
    }

    fn generate_with(root: &Path, base: &str, staging: Option<&str>) -> (Generated, Diagnostics) {
        let mut opts = test_options(root);
        opts.base = urls::Base::parse(base).unwrap();
        opts.staging = staging.map(String::from);
        let diag = Diagnostics::default();
        let g = generate(&opts, &diag, &Stats::default()).unwrap();
        (g, diag)
    }

    #[test]
    fn staging_build_is_marked_and_uses_staging_base() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let branch = "claude/beautiful-darwin-66rwab";
        let (g, diag) = generate_with(root, "https://staging.flauschiversum.de", Some(branch));
        let html = g.outputs.snapshot_html();
        assert!(html.len() > 10);
        for (path, page) in &html {
            assert!(page.contains("<meta name=\"robots\" content=\"noindex\">"), "noindex fehlt: {path}");
            if page.contains("http-equiv=\"refresh\"") {
                continue; // Weiterleitungsseiten sind ohne Seitenrahmen
            }
            let bar = format!("<div class=\"staging-bar\" role=\"note\">Staging: {branch}</div>");
            assert!(page.contains(&bar), "Leiste fehlt: {path}");
            assert!(page.find("staging-bar").unwrap() < page.find("<header").unwrap(), "Leiste nicht oben: {path}");
            assert_eq!(page.matches("class=\"staging-bar\"").count(), 1, "{path}");
            assert!(page.contains("canonical\" href=\"https://staging.flauschiversum.de/"), "{path}");
            // Absolute Verweise auf die Produktivseite gibt es nur als E-Mail-Adresse
            assert!(!page.replace("ermeline@flauschiversum.de", "").contains("https://flauschiversum.de"), "{path}");
        }
        assert!(!g.outputs.contains("/sitemap.xml"));
        let robots = std::str::from_utf8(g.outputs.snapshot_bytes("/robots.txt").as_slice()).unwrap().to_string();
        assert_eq!(robots, "User-agent: *\nDisallow: /\n");
        let post = html.iter().find(|(p, _)| p == "/2014/01/alraune/index.html").unwrap().1.clone();
        assert!(post.contains("href=\"https://staging.flauschiversum.de/rss.xml\""));
        assert!(post.contains("\"mainEntityOfPage\":\"https://staging.flauschiversum.de/2014/01/alraune/\""));
        assert!(post.contains("href=\"/themen/\""), "interne Links bleiben wurzelbezogen");
        let feed = String::from_utf8(g.outputs.snapshot_bytes("/rss.xml")).unwrap();
        assert!(feed.contains("https://staging.flauschiversum.de/2020/10/kurbis/") && !feed.contains("https://flauschiversum.de"));
        // Keine zusätzlichen Warnungen durch die andere Basis
        let (_, prod_diag) = generate_with(root, "https://flauschiversum.de", None);
        assert_eq!(diag.all().len(), prod_diag.all().len());
    }

    #[test]
    fn staging_branch_name_is_escaped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (g, _) = generate_with(root, "https://flauschiversum.de", Some("<b>&x"));
        let home = g.outputs.snapshot_html().into_iter().find(|(p, _)| p == "/index.html").unwrap().1;
        assert!(home.contains("Staging: &#60;b&#62;&#38;x</div>"), "{}", &home[home.find("staging-bar").unwrap()..][..120]);
    }

    #[test]
    fn production_build_has_no_staging_markers() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (g, _) = generate_with(root, "https://flauschiversum.de", None);
        for (path, page) in g.outputs.snapshot_html() {
            assert!(!page.contains("staging-bar"), "{path}");
            // noindex nur auf der Suchseite und auf Weiterleitungen alter Adressen
            assert_eq!(page.contains("noindex"), path == "/suche/index.html" || page.contains("http-equiv=\"refresh\""), "{path}");
        }
        assert!(g.outputs.contains("/sitemap.xml"));
        let robots = String::from_utf8(g.outputs.snapshot_bytes("/robots.txt")).unwrap();
        assert!(robots.contains("Sitemap: https://flauschiversum.de/sitemap.xml"));
    }

    fn between<'a>(html: &'a str, start: &str, end: &str) -> &'a str {
        let i = html.find(start).unwrap_or_else(|| panic!("`{start}` fehlt"));
        let rest = &html[i..];
        &rest[..rest.find(end).unwrap_or_else(|| panic!("`{end}` fehlt")) + end.len()]
    }

    #[test]
    fn footer_navigation_and_contact_on_every_page() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (g, _) = generate_with(root, "https://flauschiversum.de", None);
        let pages = g.outputs.snapshot_html();
        assert!(pages.len() > 10);
        for (path, html) in &pages {
            if html.contains("http-equiv=\"refresh\"") {
                continue; // Weiterleitungsseiten haben keinen Seitenrahmen
            }
            // Topbar: Marke mit Maskottchen, drei Menüs, Suchfeld; keine Seitenspalten mehr
            let bar = between(html, "<header class=\"topbar\"", "</header>");
            assert!(bar.contains("<a class=\"brand\" href=\"/\">"), "{path}");
            assert!(bar.find("mascot-normal").unwrap() < bar.find("brand-name").unwrap(), "{path}: Maskottchen vor dem Titel");
            assert!(bar.contains("src=\"/images/flauschi_normal.svg\"") && bar.contains("src=\"/images/flauschi_surprised.svg\""), "{path}");
            let top = between(html, "<nav class=\"topnav", "</nav>");
            for (href, text) in [("/beitraege/", "Beiträge"), ("/themen/", "Themen"), ("/tipps-und-tricks/", "Tipps und Tricks")] {
                assert!(top.contains(&format!("<a href=\"{href}\">{text}</a>")), "{path}: {text}");
            }
            assert!(top.find(">Beiträge</a>").unwrap() < top.find(">Themen</a>").unwrap() && top.find(">Themen</a>").unwrap() < top.find(">Tipps und Tricks</a>").unwrap());
            assert!(!top.contains("About"), "{path}: About me steht in der Fußzeile");
            assert!(!top.contains("aria-expanded") && !top.contains("submenu") && !top.contains("Kategorie"), "keine Dropdowns, keine Kategorien: {path}");
            assert!(bar.contains("<summary aria-label=\"Menü\">"), "{path}: Hamburger-Knopf");
            assert!(bar.contains("class=\"theme-toggle\" data-state=\"auto\""), "{path}: Umschalter für das Farbschema");
            // Farbschema: Meta-Angabe und Skript gegen das Aufblitzen stehen im head, vor dem Stylesheet
            let head_end = html.find("</head>").unwrap();
            assert!(html.contains("<meta name=\"color-scheme\" content=\"light dark\">"), "{path}");
            assert!(html.find("localStorage.getItem(\"theme\")").unwrap() < html.find("<link rel=\"stylesheet\"").unwrap() && html.find("localStorage.getItem(\"theme\")").unwrap() < head_end, "{path}");
            assert!(bar.find("</nav>").unwrap() < bar.find("class=\"search topsearch\"").unwrap(), "{path}: Suche nach den Menüs");
            assert!(!html.contains("sitenav") && !html.contains("<aside class=\"sidebar\"") && !html.contains("blogheader"), "{path}");
            // Fußzeile: vier Seitenlinks, Kontakt, Socials, Copyright
            let footer = between(html, "<footer id=\"blogfooter\"", "</footer>");
            for (href, text) in [("/about/", "About me"), ("/impressum/", "Impressum")] {
                assert!(footer.contains(&format!("<a href=\"{href}\">{text}</a>")), "{path}: {text}");
            }
            assert!(footer.contains("href=\"mailto:ermeline@flauschiversum.de?subject=Flauschiversum\""), "{path}");
            assert!(footer.contains("aria-label=\"E-Mail schreiben\""), "{path}");
            // Die Adresse steht nicht als sichtbarer Text im Footer, nur im Link und im Tooltip
            let visible = footer.replace("mailto:ermeline@flauschiversum.de", "").replace("title=\"ermeline@flauschiversum.de\"", "");
            assert!(!visible.contains("ermeline@"), "{path}: Adresse als Text");
            assert!(footer.contains("href=\"https://www.instagram.com/ermelinewollknoll/\" rel=\"noopener\" aria-label=\"Instagram\""), "{path}");
            assert!(footer.contains("href=\"https://chaos.social/@ErmelineWollknoll\" rel=\"me noopener\" aria-label=\"Mastodon\""), "{path}");
            assert_eq!(footer.matches("<svg").count(), 4, "vier Icons als inline SVG: {path}");
            // RSS-Icon: einmal, nach Mastodon, mit Namen und Feed-Attributen; der Textlink „Abonnieren“ bleibt
            let rss = "<a href=\"/rss.xml\" rel=\"alternate\" type=\"application/rss+xml\" aria-label=\"RSS-Feed abonnieren\" title=\"RSS-Feed abonnieren\">";
            assert_eq!(footer.matches(rss).count(), 1, "{path}");
            assert!(footer.find(rss).unwrap() > footer.find("aria-label=\"Mastodon\"").unwrap(), "{path}: RSS nach Mastodon");
            assert!(footer.find("aria-label=\"Instagram\"").unwrap() > footer.find("aria-label=\"E-Mail schreiben\"").unwrap(), "{path}: Reihenfolge E-Mail, Instagram");
            assert_eq!(footer.matches("href=\"/rss.xml\"").count(), 1, "nur das RSS-Icon, kein Textlink: {path}");
            assert!(!footer.contains("Abonnieren") && !footer.contains("Tipps und Tricks"), "{path}: Tipps und Tricks steht in der Topbar");
            // Links Icons mit dem Copyright darunter, rechts die Seitenlinks untereinander
            assert!(footer.find("class=\"socials\"").unwrap() < footer.find("class=\"copyright\"").unwrap() && footer.find("class=\"copyright\"").unwrap() < footer.find("class=\"footer-links\"").unwrap(), "{path}");
            assert!(footer.contains("&copy; 2026 Ermeline Wollknoll"), "{path}");
            assert!(!footer.contains("http-equiv") && !html.contains("fonts.googleapis"), "{path}");
            // Jeder mailto-Link hat einen Betreff mit „Flauschiversum“
            for (i, _) in html.match_indices("mailto:") {
                let link = &html[i..i + html[i..].find('"').unwrap()];
                let subject = urls::decode_path(link.split("subject=").nth(1).unwrap_or_else(|| panic!("kein Betreff: {link}")));
                assert!(subject.contains("Flauschiversum"), "{path}: {subject}");
            }
        }
    }

    #[test]
    fn topbar_menus_cloud_home_and_author_placement() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (g, _) = generate_with(root, "https://flauschiversum.de", None);
        let pages: std::collections::BTreeMap<String, String> = g.outputs.snapshot_html().into_iter().collect();

        // Menü auf jeder Seite gleich (Beiträge, Themen, Tipps und Tricks als einfache Links)
        let mut menus = std::collections::BTreeSet::new();
        for (path, html) in &pages {
            if html.contains("http-equiv=\"refresh\"") {
                assert!(["/kategorie/basteln/index.html"].contains(&path.as_str()), "unerwartete Weiterleitung {path}");
                continue;
            }
            menus.insert(between(html, "<nav class=\"topnav", "</nav>").to_string());
            // Cloud nur auf der Startseite
            assert_eq!(html.contains("class=\"cloud\""), path == "/index.html", "{path}");
            // Archiv nur auf der Beiträge-Seite und auf Themenseiten
            let has_archive = path == "/beitraege/index.html" || path.starts_with("/tag/");
            assert_eq!(html.contains("class=\"archive card\""), has_archive, "{path}");
            assert_eq!(html.contains("class=\"has-aside"), path == "/index.html" || has_archive, "{path}");
        }
        assert_eq!(menus.len(), 1, "Menüs auf allen Seiten gleich");
        let menu = menus.into_iter().next().unwrap();
        assert!(menu.contains("<a href=\"/themen/\">Themen</a>") && !menu.contains("Kategorien"));

        // Startseite: h1, Begrüßung, Beiträge, Cloud rechts; kein Suchfeld im Inhalt, keine Kacheln
        let home = &pages["/index.html"];
        assert_eq!(home.matches("<h1").count(), 0, "Titel steht nur in der Topbar");
        let (welcome, news) = (home.find("Willkommen im <strong>Flauschiversum</strong>!").unwrap(), home.find("<ul class=\"entries\"").unwrap());
        assert!(welcome < news);
        assert!(!home.contains("class=\"tiles\"") && !home.contains("Neu im Flauschiversum") && !home.contains("big"));
        assert_eq!(home.matches("type=\"search\"").count(), 1, "nur das Suchfeld der Topbar");
        let cloud = between(home, "<ul class=\"cloud\"", "</ul>");
        let pos = |n: &str| cloud.find(n).unwrap_or_else(|| panic!("{n}"));
        assert!(pos(">Halloween ") < pos(">Hüte ") && pos(">Hüte ") < pos(">LARP "));
        assert!(cloud.contains("<a class=\"cloud-5\" href=\"/tag/huete/\" title=\"Hüte: 3 Beiträge\">Hüte <span class=\"count\">3</span></a>"), "{cloud}");
        assert!(cloud.contains("title=\"Halloween: 1 Beitrag\""), "{cloud}");
        assert!(home.contains("<a href=\"/themen/\">Alle Themen</a>") && home.contains("<a href=\"/beitraege/\">Alle Beiträge</a></p>"));
        assert!(!home.contains("aside-early"));

        // Beiträge: Einträge und Archiv (neuestes Jahr und Monat offen)
        let index = &pages["/beitraege/index.html"];
        assert!(index.contains("<h1>Beiträge</h1>") && !index.contains("class=\"pagination\""));
        assert_eq!(between(index, "<main", "</main>").matches("<li class=\"entry\"").count(), 3);
        let archive = between(index, "<aside class=\"archive card\"", "</aside>");
        assert!(archive.contains("<details class=\"year\" open>\n    <summary>2026 <span class=\"count\">(1)</span>"), "{archive}");
        assert!(archive.contains("<details class=\"year\">\n    <summary>2020") && archive.contains("<details class=\"year\">\n    <summary>2014"));
        assert_eq!(archive.matches("<details class=\"month\" open>").count(), 1);
        assert!(archive.contains("<a href=\"/2020/10/kurbis/\">Kürbis</a>") && archive.contains("<span class=\"day\">31.</span>"));
        assert!(archive.contains("Oktober <span class=\"count\">(1)</span>") && archive.contains("Januar <span class=\"count\">(1)</span>"));
        // Themenseite: Beiträge und Archiv nur dieses Themas, auf schmalen Fenstern früh
        let topic = &pages["/tag/huete/index.html"];
        assert!(topic.contains("class=\"has-aside aside-early\"") && topic.contains("3 Beiträge"));
        assert!(!topic.contains("class=\"motto\"") && !topic.contains("class=\"intro\""));
        let topic_archive = between(topic, "<aside class=\"archive card\"", "</aside>");
        assert!(topic_archive.contains("Alraune") && topic_archive.contains("Kürbis") && topic_archive.contains("Mütze") && !topic_archive.contains("Morgen"));
        let halloween = &pages["/tag/halloween/index.html"];
        assert!(between(halloween, "<aside class=\"archive card\"", "</aside>").contains("Kürbis") && !between(halloween, "<aside class=\"archive card\"", "</aside>").contains("Alraune"));

        // Alte Kategorie-Adresse leitet auf das Thema weiter (Cool URIs)
        let redirect = &pages["/kategorie/basteln/index.html"];
        assert!(redirect.contains("<meta http-equiv=\"refresh\" content=\"0; url=/tag/larp/\">") && redirect.contains("<link rel=\"canonical\" href=\"https://flauschiversum.de/tag/larp/\">") && redirect.contains("noindex"));

        // Suchfeld in der Topbar und (Suchseite) groß im Inhalt, verschiedene ids
        let post = &pages["/2014/01/alraune/index.html"];
        assert_eq!(post.matches("type=\"search\"").count(), 1);
        assert!(post.contains("<button type=\"submit\" aria-label=\"Suchen\" title=\"Suchen\"><svg"), "Lupe als Icon");
        assert!(!post.contains(">Suchen</button>") && post.contains("id=\"q-top\""));
        let search = &pages["/suche/index.html"];
        assert_eq!(search.matches("type=\"search\"").count(), 2);
        assert!(search.contains("id=\"q\"") && search.contains("id=\"q-top\""));

        // Autor: nicht im Kopf, sondern nach dem Text und vor „älterer Beitrag“
        assert!(!between(post, "<div class=\"post-header\"", "<main").contains("Von "));
        let author = post.find("<p class=\"author\">Von <a href=\"/about/\" class=\"author\">Ermeline</a></p>").expect("Autorenzeile");
        assert!(author > post.find("</figure>").unwrap(), "nach dem Text");
        assert!(author < post.find("class=\"post-nav\"").unwrap_or(usize::MAX));
        assert!(author > post.rfind("Siehe ").unwrap(), "nach dem letzten Absatz");
        // Ohne Autor keine Zeile
        let muetze = &pages["/2026/01/gruene-muetze/index.html"];
        assert!(!muetze.contains("Von <a"));
    }

    /// Farben stehen nur in `theme.css` (Tokens für Hell und Dunkel); alle anderen Stylesheets nutzen Variablen.
    #[test]
    fn colors_are_defined_only_in_theme_css() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut checked = 0;
        for entry in fs::read_dir(repo.join("content/style")).unwrap().filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name == "theme.css" || !name.ends_with(".css") {
                continue;
            }
            let css = fs::read_to_string(entry.path()).unwrap();
            // Kommentare entfernen
            let mut code = String::new();
            let mut rest = css.as_str();
            while let Some(i) = rest.find("/*") {
                code.push_str(&rest[..i]);
                rest = &rest[i + 2..];
                rest = rest.find("*/").map_or("", |j| &rest[j + 2..]);
            }
            code.push_str(rest);
            for (n, line) in code.lines().enumerate() {
                let bytes = line.as_bytes();
                let hex = line.match_indices('#').any(|(i, _)| {
                    let digits = bytes[i + 1..].iter().take_while(|b| b.is_ascii_hexdigit()).count();
                    (3..=8).contains(&digits) && !bytes.get(i + 1 + digits).is_some_and(|b| b.is_ascii_alphanumeric())
                });
                let func = ["rgb(", "rgba(", "hsl(", "hsla("].iter().any(|f| line.contains(f));
                let named = line.split(|c: char| !c.is_ascii_alphabetic() && c != '-').any(|w| matches!(w, "white" | "black" | "red" | "green" | "blue" | "gray" | "grey"));
                assert!(!(hex || func || named), "{name}:{}: feste Farbe `{}` (Farben gehören nach theme.css)", n + 1, line.trim());
            }
            checked += 1;
        }
        assert!(checked >= 10, "Stylesheets gefunden: {checked}");
        let theme = fs::read_to_string(repo.join("content/style/theme.css")).unwrap();
        assert!(theme.contains("prefers-color-scheme: dark") && theme.contains("[data-theme=\"dark\"]") && theme.contains("[data-theme=\"light\"]"));
        for token in ["--bgcolor", "--card-bg", "--acolor", "--muted", "--link", "--accent", "--accent-ink", "--accent-soft", "--border-strong", "--footer-bg"] {
            assert_eq!(theme.matches(&format!("{token}:")).count(), 3, "{token} in Hell, System-Dunkel und Dunkel gesetzt");
        }
    }

    #[test]
    fn related_posts_card_on_post_pages() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        let (g, _) = generate_with(root, "https://flauschiversum.de", None);
        let pages: std::collections::BTreeMap<String, String> = g.outputs.snapshot_html().into_iter().collect();
        let post = &pages["/2014/01/alraune/index.html"];
        let card = between(post, "<aside class=\"related card\"", "</aside>");
        assert!(card.contains("<h2 class=\"side-title\" id=\"aehnlich-titel\">Ähnliche Beiträge</h2>"));
        // Grüne Mütze teilt LARP und Hüte, Kürbis nur Hüte: Mütze zuerst; Alraune selbst und der geplante Beitrag fehlen
        assert!(card.find("Grüne Mütze").unwrap() < card.find("Kürbis").unwrap());
        assert!(!card.contains("Alraune") && !card.contains("Morgen"));
        // Einträge wie in der Beitragsliste: Liste `entries`, Titel-Link, Datum; keine Schlagwort-Chips
        assert!(card.contains("<ul class=\"entries\">") && card.contains("class=\"entry-meta\"") && card.contains("<time datetime="), "{card}");
        assert!(!card.contains("class=\"tag\"") && !card.contains("class=\"shared\""));
        assert!(card.contains("<a href=\"/2020/10/kurbis/\">Kürbis</a>"));
        // Datum im Kopf, über dem Beitragstext; Kontakt-Hinweis am Ende nach dem Autor und vor den ähnlichen Beiträgen
        assert!(post.find("class=\"post-date\"").unwrap() < post.find("<main class=\"page\">").unwrap());
        assert!(post.contains("<p class=\"post-date\"><time datetime=\"2014-01-04"));
        let header = between(post, "<div class=\"post-header\">", "<main class=\"page\">");
        assert!(!header.contains("Nachricht"), "Hinweis steht nicht im Kopf");
        assert!(post.find("class=\"author\">Von").unwrap() < post.find("class=\"contact\"").unwrap());
        assert!(post.find("class=\"contact\"").unwrap() < post.find("class=\"related card\"").unwrap());
        // Karte vor der Navigation zum älteren und neueren Beitrag
        assert!(post.find("class=\"author\">Von").unwrap() < post.find("class=\"related card\"").unwrap());
        assert!(post.find("class=\"related card\"").unwrap() < post.find("class=\"post-nav\"").unwrap());
        // Einordnung nur über Schlagworte, keine Kategorien
        for (path, html) in pages.iter().filter(|(p, _)| p.starts_with("/20")) {
            assert!(!html.contains("class=\"category\"") && !html.contains("/kategorie/"), "{path}");
        }
        assert!(post.contains("<p class=\"classification\"><a class=\"tag\" href=\"/tag/larp/\">LARP</a>") || post.contains("class=\"classification\""));
    }

    #[test]
    fn menu_and_lists_with_more_than_ten_posts() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        for i in 1..=10 {
            write(root, &format!("content/posts/p{i:02}/index.md"), format!("---\ntitle: \"Beitrag {i:02}\"\ntags: [Halloween]\ndate: 2021-{:02}-15\n---\nText\n", i.min(12)));
        }
        let (g, _) = generate_with(root, "https://flauschiversum.de", None);
        let pages: std::collections::BTreeMap<String, String> = g.outputs.snapshot_html().into_iter().collect();
        let home = &pages["/index.html"];
        // 13 veröffentlichte Beiträge: zehn neueste auf der Startseite und auf /beitraege/, kein Beiträge-Dropdown
        assert!(!home.contains("menu-beitraege"));
        assert_eq!(between(home, "<main", "</main>").matches("<li class=\"entry\"").count(), 10);
        let index = &pages["/beitraege/index.html"];
        assert_eq!(between(index, "<main", "</main>").matches("<li class=\"entry\"").count(), 10);
        // Archiv enthält alle 13, nach Jahr gruppiert
        let archive = between(index, "<aside class=\"archive card\"", "</aside>");
        assert_eq!(archive.matches("<li><span class=\"day\">").count(), 13);
        assert!(archive.contains("<summary>2021 <span class=\"count\">(10)</span>"));
        assert!(archive.find("2026").unwrap() < archive.find("2021").unwrap() && archive.find("2021").unwrap() < archive.find("2014").unwrap());
        // Bisherige Blätterseiten bleiben: 13 Beiträge, 4 je Seite
        for n in 1..=3 {
            assert!(g.outputs.contains(&format!("/page/{n}/")), "/page/{n}/");
        }
        assert!(!g.outputs.contains("/page/4/"));
        assert!(pages["/page/1/index.html"].contains("class=\"pagination\""));
        assert!(!pages["/page/1/index.html"].contains("class=\"archive"));
        let topic = &pages["/tag/halloween/index.html"];
        assert_eq!(between(topic, "<main", "</main>").matches("<li class=\"entry\"").count(), 11, "alle Beiträge des Themas auf einer Seite");
    }

    #[test]
    fn second_build_writes_nothing_and_deleted_post_disappears() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        build(root);
        let (_, stats, ws) = build(root);
        assert_eq!((ws.new, ws.changed, ws.deleted), (0, 0, 0));
        assert_eq!(stats.processed.load(Ordering::Relaxed), 0);

        assert!(root.join("dist/pagefind/pagefind.js").is_file());
        let fragments = |root: &Path| -> Vec<String> {
            let mut v: Vec<_> = fs::read_dir(root.join("dist/pagefind/fragment")).unwrap().filter_map(Result::ok).map(|e| e.file_name().to_string_lossy().into_owned()).collect();
            v.sort();
            v
        };
        let before = fragments(root);
        assert_eq!(before.len(), 4, "drei Posts und eine Seite: {before:?}");

        fs::remove_dir_all(root.join("content/posts/alraune")).unwrap();
        let (_, _, ws) = build(root);
        assert!(ws.deleted >= 5, "{ws:?}");
        let after = fragments(root);
        assert_eq!(after.len(), 3, "Fragment des gelöschten Beitrags entfernt: {after:?}");
        assert!(before.iter().filter(|f| !after.contains(f)).count() == 1);
        assert!(!root.join("dist/2014").exists(), "leere Verzeichnisse entfernt");
    }

    #[test]
    fn foreign_output_directory_is_not_touched() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        write(root, "dist/wichtig.txt", "nicht löschen");
        let g = generate(&test_options(root), &Diagnostics::default(), &Stats::default()).unwrap();
        assert!(output::write(&root.join("dist"), g.outputs.into_map()).is_err());
        assert_eq!(fs::read_to_string(root.join("dist/wichtig.txt")).unwrap(), "nicht löschen");
    }

    #[test]
    fn page_structure_and_seo() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fixture_site(root);
        build(root);
        let post = read(root, "2014/01/alraune/index.html");
        assert_eq!(post.matches("<h1").count(), 1);
        assert!(post.contains("<h1>Alraune</h1>"));
        assert!(post.contains("<title>Alraune – Flauschiversum</title>"));
        assert!(post.contains("<link rel=\"canonical\" href=\"https://flauschiversum.de/2014/01/alraune/\">"));
        assert!(post.contains("<meta property=\"og:image\" content=\"https://flauschiversum.de/2014/01/alraune/1000x/DSCF0527.jpg\">"));
        assert!(post.contains("/style.css?v="));
        let ld_start = post.find("application/ld+json\">").unwrap() + 21;
        let ld_end = post[ld_start..].find("</script>").unwrap() + ld_start;
        let ld: serde_json::Value = serde_json::from_str(&post[ld_start..ld_end]).unwrap();
        assert_eq!(ld["@type"], "BlogPosting");
        assert_eq!(ld["datePublished"], "2014-01-04T19:10:28+01:00");

        let home = read(root, "index.html");
        assert!(home.contains("<title>Flauschiversum – Mehr als nur eine Masche</title>"));
        assert_eq!(home.matches("<h1").count(), 0);
        assert!(!home.contains("weiterlesen"));
        let index = read(root, "beitraege/index.html");
        assert!(index.contains("<title>Alle Beiträge – Flauschiversum</title>"));
        assert_eq!(index.matches("<h1").count(), 1);
        let png_src = "/2020/10/kurbis/200x/k%C3%BCrbis%20thumb.png";
        assert!(index.contains(png_src), "kodierte Thumbnail-URL");
        // Themenseiten: je eine h1, Zählung ohne geplanten Beitrag
        for page in ["tag/huete/", "tag/larp/", "themen/", "beitraege/"] {
            assert_eq!(read(root, &format!("{page}index.html")).matches("<h1").count(), 1, "{page}");
        }
        let tag = read(root, "tag/huete/index.html");
        assert!(tag.contains("3 Beiträge"));
        assert!(tag.find("Grüne Mütze").unwrap() < tag.find("Alraune").unwrap(), "neueste zuerst");
        assert!(tag.contains("<title>Hüte – Flauschiversum</title>") && tag.contains("content=\"3 Beiträge zu Hüte im Flauschiversum\""));
        assert!(!tag.contains("Kategorie"));
        assert!(read(root, "tag/halloween/index.html").contains("1 Beitrag"));
        // Suchseite: noindex, kein Suchfeld im Kopf, eine h1, Index ausgeliefert
        let search = read(root, "suche/index.html");
        assert!(search.contains("<meta name=\"robots\" content=\"noindex\">"));
        assert!(search.contains("<title>Suche – Flauschiversum</title>"));
        assert_eq!(search.matches("<h1").count(), 1);
        assert_eq!(search.matches("name=\"q\"").count(), 2, "Topbar und großes Feld");
        assert!(!read(root, "robots.txt").contains("suche"));
        assert!(!post.contains("noindex") && !home.contains("noindex"));
        assert!(!read(root, "sitemap.xml").contains("/suche/"));
        for f in ["pagefind/pagefind.js", "pagefind/pagefind-ui.js", "pagefind/pagefind-ui.css", "assets/js/search.js"] {
            assert!(root.join("dist").join(f).is_file(), "{f}");
        }
        // Themenübersicht alphabetisch
        let themen = read(root, "themen/index.html");
        assert!(themen.find("Halloween").unwrap() < themen.find("Hüte").unwrap() && themen.find("Hüte").unwrap() < themen.find("LARP").unwrap());
        // Beitrag trägt Schlagworte auch im Datensatz, keine Kategorie
        assert!(post.contains("<a class=\"tag\" href=\"/tag/larp/\">LARP</a>"));
        assert!(ld.get("articleSection").is_none());
        assert_eq!(ld["keywords"], "LARP, Hüte");
        assert!(read(root, "robots.txt").contains("Sitemap: https://flauschiversum.de/sitemap.xml"));
    }

    /// Baut die echte Seite aus `content/` (langsam beim ersten Mal, daher `--ignored`).
    #[test]
    #[ignore]
    fn full_site_keeps_all_live_urls() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
        let opts = BuildOptions { root: repo.to_path_buf(), cache: repo.join(".cache"), now: BuildOptions::now().unwrap(), livereload: false, base: urls::Base::default(), staging: None };
        let diag = Diagnostics::default();
        let g = generate(&opts, &diag, &Stats::default()).unwrap();
        assert!(!diag.has_errors());
        let live = fs::read_to_string(repo.join("tests/fixtures/live-urls.txt")).unwrap();
        let mut count = 0;
        for url in live.lines().filter(|l| !l.is_empty()) {
            assert!(g.outputs.contains(url), "Live-URL fehlt: {url}");
            count += 1;
        }
        assert_eq!(count, 194);
        for page in ["/", "/about/", "/impressum/", "/tipps-und-tricks/", "/rss.xml", "/sitemap.xml", "/robots.txt", "/style.css", "/favicon.ico"] {
            assert!(g.outputs.contains(page), "{page}");
        }
        // Themen: jeder Beitrag steht auf den Seiten seiner Schlagworte; es gibt keine Kategorien mehr
        let pages: HashMap<String, String> = g.outputs.snapshot_html().into_iter().collect();
        let page = |url: &str| -> String {
            pages.get(&format!("{url}index.html")).unwrap_or_else(|| panic!("Seite fehlt: {url}")).clone()
        };
        let site = &g.site;
        assert!(site.posts.iter().all(|p| !p.tags.is_empty()), "jeder Beitrag hat mindestens ein Schlagwort");
        for p in &site.posts {
            for t in &p.tags {
                let html = page(&format!("/tag/{}/", urls::slugify(t)));
                assert!(html.contains(&format!("href=\"{}\"", urls::encode_path(&p.url))), "{} fehlt bei Schlagwort {t}", p.url);
            }
        }
        // Chronologische Liste: alle bisherigen /page/<N>/ bleiben, erste Seite unter /beitraege/
        assert!(g.outputs.contains("/beitraege/"));
        for n in 1..49 {
            assert!(g.outputs.contains(&format!("/page/{n}/")), "/page/{n}/");
        }
        for page in ["/themen/", "/suche/", "/pagefind/pagefind.js", "/pagefind/pagefind-entry.json"] {
            assert!(g.outputs.contains(page), "{page}");
        }
        // Jeder mailto-Link der ganzen Seite hat „Flauschiversum“ im Betreff; Menüs überall gleich, Cloud nur auf der Startseite
        let mut menus = std::collections::BTreeSet::new();
        for (path, html) in pages.iter().filter(|(_, h)| !h.contains("http-equiv=\"refresh\"")) {
            for (i, _) in html.match_indices("mailto:") {
                let link = &html[i..i + html[i..].find(|c| c == '"' || c == ')').unwrap()];
                let subject = urls::decode_path(link.split("subject=").nth(1).unwrap_or_else(|| panic!("{path}: kein Betreff in {link}")));
                assert!(subject.contains("Flauschiversum"), "{path}: {link}");
            }
            menus.insert(html[html.find("<nav class=\"topnav").unwrap()..html.find("</nav>").unwrap()].to_string());
            assert_eq!(html.contains("class=\"cloud\""), path == "/index.html", "{path}");
        }
        assert_eq!(menus.len(), 1);
        // Archiv: jedes Thema und /beitraege/ führen alle ihre Beiträge auf (Zahl der Archiv-Einträge = Zahl der Beiträge)
        let entries = |html: &str| {
            let a = &html[html.find("<aside class=\"archive card\"").unwrap()..];
            a[..a.find("</aside>").unwrap()].matches("<li><span class=\"day\">").count()
        };
        assert_eq!(entries(&page("/beitraege/")), site.posts.len());
        for t in &site.tags {
            let n = site.posts.iter().filter(|p| p.tags.contains(t)).count();
            if n > 0 {
                assert_eq!(entries(&page(&format!("/tag/{}/", urls::slugify(t)))), n, "{t}");
            }
        }
        // Alte Kategorie-Adressen leiten weiter (Cool URIs)
        for (from, to) in &site.redirects {
            assert!(g.outputs.contains(from) && g.outputs.contains(to), "{from} → {to}");
            assert!(page(from).contains(&format!("url={to}")), "{from}");
        }
        assert!(site.redirects.iter().any(|(f, _)| f == "/kategorie/basteln/") && site.redirects.iter().any(|(f, _)| f == "/kategorie/brandings/"));
        // Alle Medien-Referenzen lösen auf: Warnungen betreffen nur Seitenlinks in den Inhalten
        for d in diag.all() {
            let is_media = [".jpg", ".jpeg", ".png", ".webp", ".gif", ".mp4", ".webm", ".css", ".js"]
                .iter()
                .any(|ext| d.message.to_lowercase().contains(ext));
            assert!(!is_media, "Medium fehlt: {}: {}", d.file, d.message);
        }
    }
}
