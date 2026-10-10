//! Laden und Prüfen der Inhalte unter `content/`.

use crate::config::{MEDIA_WARN_BYTES, TIME_ZONE};
use crate::diag::Diagnostics;
use crate::urls;
use jiff::Zoned;
use jiff::civil::DateTime;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    /// Wird in drei Größen umgerechnet.
    Image,
    /// Animiertes GIF, wird unverändert ausgeliefert.
    Animation,
    /// Video, wird unverändert ausgeliefert.
    Video,
}

#[derive(Debug, Clone)]
pub struct Media {
    pub file_name: String,
    pub path: PathBuf,
    pub kind: MediaKind,
}

#[derive(Debug, Clone)]
pub struct Post {
    /// Relativer Pfad der `index.md` (für Meldungen).
    pub source: String,
    pub title: String,
    pub date: Zoned,
    /// URL-Pfad mit `/` am Anfang und Ende (unkodiert).
    pub url: String,
    pub author: Option<String>,
    pub image: Option<String>,
    pub excerpt: Option<String>,
    pub markdown: String,
    pub media: Vec<Media>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub source: String,
    pub title: String,
    pub url: String,
    pub markdown: String,
}

#[derive(Debug, Clone)]
pub struct StaticFile {
    /// URL-Pfad (unkodiert), z.B. `/fonts/Lobster.otf`.
    pub url: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Site {
    /// Veröffentlichte Posts, neueste zuerst.
    pub posts: Vec<Post>,
    /// Anzahl geplanter (noch nicht veröffentlichter) Posts.
    pub scheduled: usize,
    pub pages: Vec<Page>,
    pub static_files: Vec<StaticFile>,
    pub style_dir: PathBuf,
}

/// Ordnet eine Datei anhand ihrer Endung (und bei GIF ihres Inhalts) ein.
pub fn classify(path: &Path) -> Option<MediaKind> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "webp" => Some(MediaKind::Image),
        "gif" => {
            let data = fs::read(path).ok()?;
            Some(if gif_frame_count(&data) > 1 { MediaKind::Animation } else { MediaKind::Image })
        }
        "mp4" | "webm" => Some(MediaKind::Video),
        _ => None,
    }
}

/// Zählt die Einzelbilder eines GIF (bricht nach 2 ab).
pub fn gif_frame_count(data: &[u8]) -> usize {
    if data.len() < 13 || !data.starts_with(b"GIF") {
        return 0;
    }
    let mut i = 13;
    let flags = data[10];
    if flags & 0x80 != 0 {
        i += 3 * (1 << ((flags & 0x07) + 1));
    }
    let mut frames = 0;
    let skip_sub_blocks = |mut i: usize| -> usize {
        while i < data.len() {
            let len = data[i] as usize;
            i += 1;
            if len == 0 {
                break;
            }
            i += len;
        }
        i
    };
    while i < data.len() {
        match data[i] {
            0x2C => {
                frames += 1;
                if frames > 1 {
                    return frames;
                }
                if i + 10 > data.len() {
                    break;
                }
                let lflags = data[i + 9];
                i += 10;
                if lflags & 0x80 != 0 {
                    i += 3 * (1 << ((lflags & 0x07) + 1));
                }
                i += 1; // LZW-Mindestcodegröße
                i = skip_sub_blocks(i);
            }
            0x21 => i = skip_sub_blocks(i + 2),
            _ => break,
        }
    }
    frames
}

/// Parst `YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS` als Ortszeit in Europe/Berlin.
pub fn parse_date(value: &str) -> Option<Zoned> {
    let value = value.trim();
    let civil: DateTime = if value.len() == 10 {
        let date: jiff::civil::Date = value.parse().ok()?;
        date.to_datetime(jiff::civil::Time::midnight())
    } else if value.len() == 19 && value.as_bytes()[10] == b' ' {
        value.replacen(' ', "T", 1).parse().ok()?
    } else {
        return None;
    };
    civil.in_tz(TIME_ZONE).ok()
}

/// Wandelt einen YAML-Wert in Text (Zahlen und Wahrheitswerte eingeschlossen).
fn yaml_text(value: &serde_yaml_ng::Value) -> Option<String> {
    use serde_yaml_ng::Value;
    match value {
        Value::String(s) => Some(s.trim().to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Trennt Frontmatter und Markdown.
fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let rest = text.strip_prefix("---\n").or_else(|| text.strip_prefix("---\r\n"))?;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            return Some((&rest[..offset], &rest[offset + line.len()..]));
        }
        offset += line.len();
    }
    None
}

struct Frontmatter {
    fields: HashMap<String, String>,
    markdown: String,
}

fn read_frontmatter(path: &Path, source: &str, diag: &Diagnostics) -> Option<Frontmatter> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            diag.error(source, format!("Datei kann nicht gelesen werden: {e}"));
            return None;
        }
    };
    let Some((yaml, markdown)) = split_frontmatter(&text) else {
        diag.error(source, "Frontmatter fehlt (Datei muss mit `---` beginnen und den Block mit `---` schließen)");
        return None;
    };
    let mapping: serde_yaml_ng::Mapping = match serde_yaml_ng::from_str(yaml) {
        Ok(m) => m,
        Err(e) => {
            diag.error(source, format!("Frontmatter ist kein gültiges YAML: {e}"));
            return None;
        }
    };
    let mut fields = HashMap::new();
    for (key, value) in mapping {
        if let (Some(k), Some(v)) = (key.as_str(), yaml_text(&value)) {
            fields.insert(k.to_string(), v);
        }
    }
    Some(Frontmatter { fields, markdown: markdown.to_string() })
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn load_post(root: &Path, dir: &Path, diag: &Diagnostics) -> Option<Post> {
    let index = dir.join("index.md");
    let source = rel(root, &index);
    let fm = read_frontmatter(&index, &source, diag)?;
    let f = |k: &str| fm.fields.get(k).filter(|v| !v.is_empty()).cloned();

    let title = f("title");
    if title.is_none() {
        diag.error(&source, "Pflichtfeld `title` fehlt");
    }
    let date = match f("date") {
        None => {
            diag.error(&source, "Pflichtfeld `date` fehlt");
            None
        }
        Some(raw) => {
            let parsed = parse_date(&raw);
            if parsed.is_none() {
                diag.error(
                    &source,
                    format!("Feld `date` hat ein ungültiges Format: `{raw}` (erwartet `YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`)"),
                );
            }
            parsed
        }
    };
    let (title, date) = (title?, date?);

    let dir_name = dir.file_name()?.to_string_lossy().into_owned();
    let url = match f("permalink") {
        Some(p) if p.starts_with('/') && p.ends_with('/') && !p.contains("//") => p,
        Some(p) => {
            diag.error(&source, format!("Feld `permalink` muss mit `/` beginnen und enden: `{p}`"));
            return None;
        }
        None => urls::default_post_url(&date, &dir_name),
    };

    let mut media = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(dir).ok()?.filter_map(Result::ok).map(|e| e.path()).collect();
    entries.sort();
    for path in entries {
        if !path.is_file() || path.file_name().is_some_and(|n| n == "index.md") {
            continue;
        }
        let Some(kind) = classify(&path) else { continue };
        let file_name = path.file_name()?.to_string_lossy().into_owned();
        if kind != MediaKind::Image {
            let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            if size > MEDIA_WARN_BYTES {
                diag.warning(
                    rel(root, &path),
                    format!("{} ist {:.1} MB groß (Grenze für Warnungen: 20 MB)", file_name, size as f64 / 1_048_576.0),
                );
            }
        }
        media.push(Media { file_name, path, kind });
    }

    let image = f("image");
    if let Some(img) = &image {
        let ok = media.iter().any(|m| &m.file_name == img && m.kind == MediaKind::Image);
        if !ok {
            diag.error(&source, format!("Feld `image`: Bild `{img}` existiert nicht im Post-Verzeichnis"));
            return None;
        }
    }

    Some(Post {
        source,
        title,
        date,
        url,
        author: f("author"),
        image,
        excerpt: f("excerpt"),
        markdown: fm.markdown,
        media,
    })
}

fn load_page(root: &Path, path: &Path, diag: &Diagnostics) -> Option<Page> {
    let source = rel(root, path);
    let fm = read_frontmatter(path, &source, diag)?;
    let Some(title) = fm.fields.get("title").filter(|t| !t.is_empty()).cloned() else {
        diag.error(&source, "Pflichtfeld `title` fehlt");
        return None;
    };
    Some(Page { source, url: urls::page_url(&title), title, markdown: fm.markdown })
}

/// Lädt alle Inhalte aus `<root>/content`. `now` bestimmt, welche Posts veröffentlicht sind.
pub fn load(root: &Path, now: &Zoned, diag: &Diagnostics) -> Site {
    let content = root.join("content");
    let mut all_posts = Vec::new();
    if let Ok(dirs) = fs::read_dir(content.join("posts")) {
        let mut dirs: Vec<_> = dirs.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.is_dir()).collect();
        dirs.sort();
        for dir in dirs {
            if dir.join("index.md").is_file()
                && let Some(post) = load_post(root, &dir, diag) {
                    all_posts.push(post);
                }
        }
    } else {
        diag.error(rel(root, &content.join("posts")), "Verzeichnis fehlt");
    }

    let mut pages = Vec::new();
    if let Ok(files) = fs::read_dir(content.join("pages")) {
        let mut files: Vec<_> = files.filter_map(Result::ok).map(|e| e.path()).collect();
        files.sort();
        for path in files.into_iter().filter(|p| p.extension().is_some_and(|e| e == "md")) {
            if let Some(page) = load_page(root, &path, diag) {
                pages.push(page);
            }
        }
    }

    // Doppelte URLs
    let mut seen: HashMap<&str, &str> = HashMap::new();
    for (url, source) in all_posts.iter().map(|p| (p.url.as_str(), p.source.as_str()))
        .chain(pages.iter().map(|p| (p.url.as_str(), p.source.as_str())))
    {
        if let Some(other) = seen.insert(url, source) {
            diag.error(source, format!("URL `{url}` wird auch von `{other}` verwendet"));
        }
    }

    let total = all_posts.len();
    all_posts.retain(|p| p.date <= *now);
    let scheduled = total - all_posts.len();
    all_posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.url.cmp(&b.url)));

    let static_root = content.join("static");
    let mut static_files: Vec<StaticFile> = walkdir::WalkDir::new(&static_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| StaticFile { url: format!("/{}", rel(&static_root, e.path())), path: e.path().to_path_buf() })
        .collect();
    static_files.sort_by(|a, b| a.url.cmp(&b.url));

    Site { posts: all_posts, scheduled, pages, static_files, style_dir: content.join("style") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_date_formats_in_berlin_time() {
        let d = parse_date("2014-01-04 19:10:28").unwrap();
        assert_eq!(d.to_string(), "2014-01-04T19:10:28+01:00[Europe/Berlin]");
        let d = parse_date("2020-10-31").unwrap();
        assert_eq!(d.to_string(), "2020-10-31T00:00:00+01:00[Europe/Berlin]");
        let d = parse_date("2019-07-01 12:00:00 ").unwrap();
        assert_eq!(d.offset().seconds(), 7200);
        assert!(parse_date("31.10.2020").is_none());
        assert!(parse_date("2020-13-01").is_none());
    }

    #[test]
    fn splits_frontmatter() {
        let (y, m) = split_frontmatter("---\ntitle: x\n---\n\nText\n").unwrap();
        assert_eq!(y, "title: x\n");
        assert_eq!(m, "\nText\n");
        assert!(split_frontmatter("title: x\n").is_none());
    }

    #[test]
    fn classifies_extensions() {
        let dir = tempfile::tempdir().unwrap();
        let p = |n: &str| dir.path().join(n);
        for n in ["a.jpg", "b.JPG", "c.jpeg", "d.PNG", "e.webp", "f.mp4", "g.WebM", "h.xcf", "i.xmp", "j.html"] {
            fs::write(p(n), b"x").unwrap();
        }
        assert_eq!(classify(&p("a.jpg")), Some(MediaKind::Image));
        assert_eq!(classify(&p("b.JPG")), Some(MediaKind::Image));
        assert_eq!(classify(&p("c.jpeg")), Some(MediaKind::Image));
        assert_eq!(classify(&p("d.PNG")), Some(MediaKind::Image));
        assert_eq!(classify(&p("e.webp")), Some(MediaKind::Image));
        assert_eq!(classify(&p("f.mp4")), Some(MediaKind::Video));
        assert_eq!(classify(&p("g.WebM")), Some(MediaKind::Video));
        assert_eq!(classify(&p("h.xcf")), None);
        assert_eq!(classify(&p("i.xmp")), None);
        assert_eq!(classify(&p("j.html")), None);
    }

    fn write_post(root: &Path, dir: &str, frontmatter: &str) {
        let d = root.join("content/posts").join(dir);
        fs::create_dir_all(&d).unwrap();
        fs::write(d.join("index.md"), format!("---\n{frontmatter}---\nText\n")).unwrap();
    }

    fn now() -> Zoned {
        "2026-10-10T12:00[Europe/Berlin]".parse().unwrap()
    }

    #[test]
    fn scheduled_posts_are_not_published() {
        let root = tempfile::tempdir().unwrap();
        write_post(root.path(), "alt", "title: Alt\ndate: 2020-01-01\n");
        write_post(root.path(), "morgen", "title: Morgen\ndate: 2026-10-11\n");
        let diag = Diagnostics::default();
        let site = load(root.path(), &now(), &diag);
        assert!(!diag.has_errors());
        assert_eq!(site.posts.len(), 1);
        assert_eq!(site.posts[0].title, "Alt");
        assert_eq!(site.scheduled, 1);
    }

    #[test]
    fn reports_missing_title_with_path() {
        let root = tempfile::tempdir().unwrap();
        write_post(root.path(), "ohne-titel", "date: 2020-01-01\n");
        let diag = Diagnostics::default();
        load(root.path(), &now(), &diag);
        let all = diag.all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].file, "content/posts/ohne-titel/index.md");
        assert!(all[0].message.contains("`title`"));
    }

    #[test]
    fn reports_invalid_date_with_expected_format() {
        let root = tempfile::tempdir().unwrap();
        write_post(root.path(), "x", "title: X\ndate: 31.10.2020\n");
        let diag = Diagnostics::default();
        load(root.path(), &now(), &diag);
        let msg = &diag.all()[0].message;
        assert!(msg.contains("`date`") && msg.contains("YYYY-MM-DD"), "{msg}");
    }

    #[test]
    fn reports_missing_image_and_duplicate_urls() {
        let root = tempfile::tempdir().unwrap();
        write_post(root.path(), "a", "title: A\ndate: 2020-01-01\nimage: fehlt.jpg\n");
        write_post(root.path(), "b", "title: B\ndate: 2020-01-01\npermalink: /x/\n");
        write_post(root.path(), "c", "title: C\ndate: 2020-01-01\npermalink: /x/\n");
        let diag = Diagnostics::default();
        load(root.path(), &now(), &diag);
        let msgs: Vec<_> = diag.all().into_iter().map(|d| format!("{}: {}", d.file, d.message)).collect();
        assert!(msgs.iter().any(|m| m.contains("a/index.md") && m.contains("fehlt.jpg")), "{msgs:?}");
        assert!(msgs.iter().any(|m| m.contains("c/index.md") && m.contains("b/index.md")), "{msgs:?}");
    }

    #[test]
    fn url_from_permalink_or_directory() {
        let root = tempfile::tempdir().unwrap();
        write_post(root.path(), "kuerbis", "title: Kürbis\ndate: 2020-10-31\npermalink: /2020/10/kurbis/\n");
        write_post(root.path(), "grüne-mütze", "title: Grüne Mütze\ndate: 2026-01-05\n");
        let diag = Diagnostics::default();
        let site = load(root.path(), &now(), &diag);
        let urls: Vec<_> = site.posts.iter().map(|p| p.url.as_str()).collect();
        assert_eq!(urls, ["/2026/01/gruene-muetze/", "/2020/10/kurbis/"]);
    }

    #[test]
    fn counts_gif_frames() {
        // Minimales GIF mit einem bzw. zwei Einzelbildern
        let header = b"GIF89a\x01\x00\x01\x00\x00\x00\x00";
        let frame = b"\x2C\x00\x00\x00\x00\x01\x00\x01\x00\x00\x02\x02\x44\x01\x00";
        let mut one = header.to_vec();
        one.extend_from_slice(frame);
        one.push(0x3B);
        let mut two = header.to_vec();
        two.extend_from_slice(frame);
        two.extend_from_slice(frame);
        two.push(0x3B);
        assert_eq!(gif_frame_count(&one), 1);
        assert_eq!(gif_frame_count(&two), 2);
    }
}
