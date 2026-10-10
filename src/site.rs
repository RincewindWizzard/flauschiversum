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
                // Auch Textlinks und eingebettete Pfade absolut machen (Feedreader kennen die Basis-URL nicht)
                let content_html = abs
                    .html
                    .replace("href=\"/", &format!("href=\"{BLOG_URL}/"))
                    .replace("src=\"/", &format!("src=\"{BLOG_URL}/"));
                feed::FeedEntry { content_html, enclosure }
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
        write(root, "content/pages/about.md", "---\ntitle: \"About\"\n---\n\n# Das Flauschiversum\n\nEin Blog über Wolle & Filz.\n\n[Kürbis](/2020/10/kurbis/)\n");
        write(
            root,
            "content/posts/alraune/index.md",
            "---\ntitle: \"Alraune\"\npermalink: /2014/01/alraune/\nauthor: Ermeline\ndate: 2014-01-04 19:10:28\nimage: \"DSCF0527.jpg\"\nexcerpt: \"Mal wieder etwas für das Hobby LARP.\"\n---\n\n![Alraune oben](DSCF0527.jpg)\n![Alraune nah oben](DSCF0528.jpg)\n![Clip](clip.mp4)\n\nMal wieder etwas für das Hobby LARP.\n\n![Ohrenschützer](DSCF0531.jpg)\n\nSiehe [Kürbis](/2020/10/kurbis/) und [kaputt](/2014/01/gibts-nicht/).\n",
        );
        write(root, "content/posts/alraune/DSCF0527.jpg", jpeg(800, 600, 40));
        write(root, "content/posts/alraune/DSCF0528.jpg", jpeg(600, 800, 90));
        write(root, "content/posts/alraune/DSCF0531.jpg", jpeg(1200, 900, 150));
        write(root, "content/posts/alraune/clip.mp4", b"kein echtes Video");
        write(root, "content/posts/alraune/Stifttextur.xcf", b"ignoriert");
        write(
            root,
            "content/posts/kuerbis/index.md",
            "---\ntitle: \"Kürbis\"\npermalink: /2020/10/kurbis/\nauthor: Ermeline\ndate: 2020-10-31\nimage: \"kürbis thumb.png\"\nexcerpt: \"Es ist wieder die gruselige Jahreszeit und da muss man einen Kürbis haben.\"\n---\n\n![Kürbis](kürbis thumb.png)\n",
        );
        write(root, "content/posts/kuerbis/kürbis thumb.png", png(400, 300));
        write(root, "content/posts/grüne-mütze/index.md", "---\ntitle: \"Grüne Mütze\"\ndate: 2026-01-05\nexcerpt: \"Ohne Bild.\"\n---\n\nText.\n");
        write(root, "content/posts/morgen/index.md", "---\ntitle: \"Morgen\"\ndate: 2099-01-01\nimage: \"bild.jpg\"\n---\n\n![Bild](bild.jpg)\n");
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
        insta::assert_snapshot!("index", stable(&read(root, "index.html")));
        insta::assert_snapshot!("page_about", stable(&read(root, "about/index.html")));
        let rss = read(root, "rss.xml");
        assert!(rss.contains("<a href=\"https://flauschiversum.de/2020/10/kurbis/\">Kürbis</a>"));
        insta::assert_snapshot!("rss", rss);
        insta::assert_snapshot!("sitemap", read(root, "sitemap.xml"));
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

        fs::remove_dir_all(root.join("content/posts/alraune")).unwrap();
        let (_, _, ws) = build(root);
        assert!(ws.deleted >= 5, "{ws:?}");
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

        let index = read(root, "index.html");
        assert!(index.contains("<title>Flauschiversum – Mehr als nur eine Masche</title>"));
        assert_eq!(index.matches("<h1").count(), 1);
        let png_src = "/2020/10/kurbis/200x/k%C3%BCrbis%20thumb.png";
        assert!(index.contains(png_src), "kodierte Thumbnail-URL");
        assert!(read(root, "robots.txt").contains("Sitemap: https://flauschiversum.de/sitemap.xml"));
    }

    /// Baut die echte Seite aus `content/` (langsam beim ersten Mal, daher `--ignored`).
    #[test]
    #[ignore]
    fn full_site_keeps_all_live_urls() {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
        let opts = BuildOptions { root: repo.to_path_buf(), cache: repo.join(".cache"), now: BuildOptions::now().unwrap(), livereload: false };
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
        // Alle Medien-Referenzen lösen auf: Warnungen betreffen nur Seitenlinks in den Inhalten
        for d in diag.all() {
            let is_media = [".jpg", ".jpeg", ".png", ".webp", ".gif", ".mp4", ".webm", ".css", ".js"]
                .iter()
                .any(|ext| d.message.to_lowercase().contains(ext));
            assert!(!is_media, "Medium fehlt: {}: {}", d.file, d.message);
        }
    }
}
