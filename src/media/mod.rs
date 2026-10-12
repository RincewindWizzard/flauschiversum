//! Medien eines Posts: Bildvarianten (mit Cache) und unverändert ausgelieferte Medien.

pub mod meta;
pub mod process;

use crate::config::{FULL_WIDTH, PREVIEW_WIDTH, RIGHTS_CREATOR, RIGHTS_WEB_STATEMENT, THUMB_WIDTH};
use crate::content::{MediaKind, Site};
use crate::diag::Diagnostics;
use crate::output::{Content, Outputs};
use crate::urls;
use anyhow::{Context, Result};
use process::{Badge, Format};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Version des Verfahrens. Erhöhen, wenn sich die Ausgabe bei gleichen Eingaben ändert.
const PIPELINE_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Was Templates und Markdown über ein Medium wissen müssen.
#[derive(Debug, Clone)]
pub struct MediaInfo {
    pub kind: MediaKind,
    /// Abmessungen von Full, Preview und Thumbnail (nur Bilder).
    pub full: Option<Size>,
    pub preview: Option<Size>,
    pub thumb: Option<Size>,
    /// Abmessungen des Originals (Animationen).
    pub original: Option<Size>,
    /// Dateigröße der Preview-Variante (für RSS-Enclosures).
    pub preview_len: u64,
}

/// Schlüssel: (Post-URL, Dateiname).
pub type MediaIndex = HashMap<(String, String), MediaInfo>;

#[derive(Default)]
pub struct Stats {
    pub processed: AtomicUsize,
    pub cached: AtomicUsize,
}

/// Metadaten einer Variante im Cache.
#[derive(Serialize, Deserialize)]
struct CacheMeta {
    size: Size,
    len: u64,
    oversized: bool,
}

fn key_for(orig: &blake3::Hash, badge: Option<&blake3::Hash>, width: u32, format: Format) -> String {
    let mut h = blake3::Hasher::new();
    h.update(orig.as_bytes());
    if let Some(b) = badge {
        h.update(b.as_bytes());
    }
    h.update(&width.to_le_bytes());
    h.update(&[process::QUALITY]);
    h.update(RIGHTS_CREATOR.as_bytes());
    h.update(RIGHTS_WEB_STATEMENT.as_bytes());
    h.update(&PIPELINE_VERSION.to_le_bytes());
    h.update(format!("{format:?}").as_bytes());
    h.finalize().to_hex()[..32].to_string()
}

fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    let tmp = path.with_extension(format!("tmp{}", std::process::id()));
    fs::write(&tmp, data)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Liest die Abmessungen aus dem Kopf eines GIF.
fn gif_size(data: &[u8]) -> Option<Size> {
    (data.len() >= 10).then(|| Size {
        width: u16::from_le_bytes([data[6], data[7]]) as u32,
        height: u16::from_le_bytes([data[8], data[9]]) as u32,
    })
}

/// Erzeugt alle Medien-Ausgaben der veröffentlichten Posts.
pub fn build(
    site: &Site,
    badge_png: &[u8],
    cache_dir: &Path,
    outputs: &Outputs,
    diag: &Diagnostics,
    stats: &Stats,
) -> Result<MediaIndex> {
    let badge = Badge::from_png(badge_png)?;
    let badge_hash = blake3::hash(badge_png);
    let dir = cache_dir.join(format!("v{PIPELINE_VERSION}"));
    fs::create_dir_all(&dir).with_context(|| format!("Cache-Verzeichnis {} kann nicht angelegt werden", dir.display()))?;

    let jobs: Vec<_> = site
        .posts
        .iter()
        .flat_map(|p| p.media.iter().map(move |m| (p, m)))
        .collect();

    let total = jobs.len();
    let finished = AtomicUsize::new(0);
    let results: Vec<_> = jobs
        .par_iter()
        .map(|(post, media)| {
            let source = format!("{}/{}", post.source.trim_end_matches("index.md").trim_end_matches('/'), media.file_name);
            let r = build_one(&post.url, &media.file_name, &media.path, media.kind, &badge, &badge_hash, &dir, outputs, stats);
            let done = finished.fetch_add(1, Ordering::Relaxed) + 1;
            crate::progress::count("Bilder und Medien", done, total, || {
                format!("· {} neu berechnet, {} aus dem Cache", stats.processed.load(Ordering::Relaxed), stats.cached.load(Ordering::Relaxed))
            });
            match r {
                Ok((info, oversized)) => {
                    if oversized {
                        diag.warning(&source, "Full-Variante ist größer als 1 MB (verlustfreies Format)");
                    }
                    Some(((post.url.clone(), media.file_name.clone()), info))
                }
                Err(e) => {
                    diag.error(&source, format!("Medium kann nicht verarbeitet werden: {e:#}"));
                    None
                }
            }
        })
        .collect();
    Ok(results.into_iter().flatten().collect())
}

#[allow(clippy::too_many_arguments)]
fn build_one(
    post_url: &str,
    file_name: &str,
    path: &Path,
    kind: MediaKind,
    badge: &Badge,
    badge_hash: &blake3::Hash,
    cache: &Path,
    outputs: &Outputs,
    stats: &Stats,
) -> Result<(MediaInfo, bool)> {
    let data = fs::read(path)?;
    if kind != MediaKind::Image {
        outputs.insert(urls::original_path(post_url, file_name), Content::File(path.to_path_buf()));
        let original = if kind == MediaKind::Animation { gif_size(&data) } else { None };
        let info = MediaInfo { kind, full: None, preview: None, thumb: None, original, preview_len: 0 };
        return Ok((info, false));
    }

    let format = Format::from_file_name(file_name).context("unbekanntes Bildformat")?;
    let ext = file_name.rsplit('.').next().unwrap_or("bin").to_ascii_lowercase();
    let orig_hash = blake3::hash(&data);
    let widths = [FULL_WIDTH, PREVIEW_WIDTH, THUMB_WIDTH];
    let keys = [
        key_for(&orig_hash, Some(badge_hash), FULL_WIDTH, format),
        key_for(&orig_hash, Some(badge_hash), PREVIEW_WIDTH, format),
        key_for(&orig_hash, None, THUMB_WIDTH, format),
    ];
    let files = keys.clone().map(|k| cache.join(format!("{k}.{ext}")));
    let metas = keys.clone().map(|k| cache.join(format!("{k}.json")));
    let mut cached: [Option<CacheMeta>; 3] = [0, 1, 2].map(|i| {
        files[i]
            .is_file()
            .then(|| fs::read(&metas[i]).ok().and_then(|m| serde_json::from_slice(&m).ok()))
            .flatten()
    });

    let need = [0, 1, 2].map(|i| cached[i].is_none());
    if need.iter().any(|n| *n) {
        let p = process::process(&data, format, badge, need)?;
        for (i, variant) in p.variants.into_iter().enumerate() {
            let Some(v) = variant else { continue };
            write_atomic(&files[i], &v.data)?;
            let m = CacheMeta { size: Size { width: v.width, height: v.height }, len: v.data.len() as u64, oversized: i == 0 && p.oversized };
            write_atomic(&metas[i], &serde_json::to_vec(&m)?)?;
            cached[i] = Some(m);
        }
        stats.processed.fetch_add(1, Ordering::Relaxed);
    } else {
        stats.cached.fetch_add(1, Ordering::Relaxed);
    }
    let [full, preview, thumb] = cached.map(|m| m.expect("Variante vorhanden"));

    for (file, width) in files.iter().zip(widths) {
        outputs.insert(urls::variant_path(post_url, width, file_name), Content::File(file.clone()));
    }
    let info = MediaInfo {
        kind,
        full: Some(full.size),
        preview: Some(preview.size),
        thumb: Some(thumb.size),
        original: None,
        preview_len: preview.len,
    };
    Ok((info, full.oversized))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_key_depends_on_all_inputs() {
        let a = blake3::hash(b"a");
        let b = blake3::hash(b"b");
        let base = key_for(&a, Some(&a), 700, Format::Jpeg);
        assert_ne!(base, key_for(&b, Some(&a), 700, Format::Jpeg));
        assert_ne!(base, key_for(&a, Some(&b), 700, Format::Jpeg));
        assert_ne!(base, key_for(&a, Some(&a), 1000, Format::Jpeg));
        assert_ne!(base, key_for(&a, Some(&a), 700, Format::Png));
        // Thumbnail hängt nicht vom Badge ab
        assert_eq!(key_for(&a, None, 200, Format::Jpeg), key_for(&a, None, 200, Format::Jpeg));
    }

    fn site_with_image(root: &Path) -> Site {
        let dir = root.join("content/posts/test");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("index.md"), "---\ntitle: Test\ndate: 2020-01-01\ncategory: wolle\n---\n").unwrap();
        fs::create_dir_all(root.join("content/static/images")).unwrap();
        fs::write(root.join("content/static/images/w.jpg"), b"x").unwrap();
        fs::write(root.join("content/categories.yaml"), "- id: wolle\n  name: Wolle\n  image: /images/w.jpg\n  motto: x\n").unwrap();
        let mut jpg = Vec::new();
        jpeg_encoder::Encoder::new(&mut jpg, 90)
            .encode(&vec![90u8; 800 * 600 * 3], 800, 600, jpeg_encoder::ColorType::Rgb)
            .unwrap();
        fs::write(dir.join("bild.jpg"), jpg).unwrap();
        let now: jiff::Zoned = "2026-01-01T00:00[Europe/Berlin]".parse().unwrap();
        crate::content::load(root, &now, &Diagnostics::default())
    }

    fn run(site: &Site, badge: &[u8], cache: &Path) -> (Stats, Vec<std::time::SystemTime>) {
        let stats = Stats::default();
        let outputs = Outputs::default();
        build(site, badge, cache, &outputs, &Diagnostics::default(), &stats).unwrap();
        let mut times = Vec::new();
        for (_, c) in outputs.into_map() {
            if let Content::File(p) = c {
                times.push(fs::metadata(p).unwrap().modified().unwrap());
            }
        }
        (stats, times)
    }

    #[test]
    fn second_run_uses_cache_only() {
        let root = tempfile::tempdir().unwrap();
        let site = site_with_image(root.path());
        let badge = include_bytes!("../../assets/badge.png");
        let cache = root.path().join(".cache");
        let (s1, _) = run(&site, badge, &cache);
        assert_eq!(s1.processed.load(Ordering::Relaxed), 1);
        let (s2, _) = run(&site, badge, &cache);
        assert_eq!(s2.processed.load(Ordering::Relaxed), 0);
        assert_eq!(s2.cached.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn new_badge_invalidates_only_preview_and_full() {
        let root = tempfile::tempdir().unwrap();
        let site = site_with_image(root.path());
        let cache = root.path().join(".cache");
        let badge = include_bytes!("../../assets/badge.png");
        run(&site, badge, &cache);
        let files = |dir: &Path| {
            let mut v: Vec<_> = fs::read_dir(dir.join("v1")).unwrap().map(|e| e.unwrap().file_name()).collect();
            v.sort();
            v
        };
        let before = files(&cache);
        // Andere Badge-Vorlage: gleiche Grafik, aber anders kodiert
        let img = image::load_from_memory(badge).unwrap();
        let mut other = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut other), image::ImageFormat::Png).unwrap();
        assert_ne!(&other[..], &badge[..]);
        run(&site, &other, &cache);
        let after = files(&cache);
        let new: Vec<_> = after.iter().filter(|f| !before.contains(f)).collect();
        // Full und Preview neu (je Bild + JSON), Thumbnail unverändert
        assert_eq!(new.len(), 4, "{new:?}");
    }
}
