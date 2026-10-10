//! Sammeln der Ausgabedateien und Schreiben ins Ausgabeverzeichnis.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

/// Inhalt einer Ausgabedatei.
#[derive(Debug, Clone)]
pub enum Content {
    Bytes(Vec<u8>),
    /// Kopie einer vorhandenen Datei (Original, Cache, statische Datei).
    File(PathBuf),
}

/// Alle Ausgaben eines Builds, geordnet nach URL-Pfad (unkodiert, mit `/` am Anfang).
#[derive(Default)]
pub struct Outputs {
    files: Mutex<BTreeMap<String, Content>>,
}

impl Outputs {
    /// Fügt eine Ausgabe hinzu. Ein Pfad, der auf `/` endet, wird zu `index.html`.
    pub fn insert(&self, url_path: impl Into<String>, content: Content) {
        let mut path = url_path.into();
        if path.ends_with('/') {
            path.push_str("index.html");
        }
        self.files.lock().unwrap().insert(path, content);
    }

    pub fn contains(&self, url_path: &str) -> bool {
        let files = self.files.lock().unwrap();
        if url_path.ends_with('/') {
            files.contains_key(&format!("{url_path}index.html"))
        } else {
            files.contains_key(url_path)
        }
    }

    /// Kopie aller HTML-Ausgaben, die als Bytes vorliegen (für die Linkprüfung).
    pub fn snapshot_html(&self) -> Vec<(String, String)> {
        self.files
            .lock()
            .unwrap()
            .iter()
            .filter(|(p, _)| p.ends_with(".html"))
            .filter_map(|(p, c)| match c {
                Content::Bytes(b) => Some((p.clone(), String::from_utf8_lossy(b).into_owned())),
                Content::File(_) => None,
            })
            .collect()
    }

    pub fn into_map(self) -> BTreeMap<String, Content> {
        self.files.into_inner().unwrap()
    }

    pub fn len(&self) -> usize {
        self.files.lock().unwrap().len()
    }
}

/// Name der Markierungsdatei im Ausgabeverzeichnis.
pub const MANIFEST: &str = ".flauschiversum-manifest";

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
struct Entry {
    hash: String,
    len: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WriteStats {
    pub new: usize,
    pub changed: usize,
    pub unchanged: usize,
    pub deleted: usize,
}

fn target(out: &std::path::Path, url_path: &str) -> std::path::PathBuf {
    out.join(url_path.trim_start_matches('/'))
}

fn write_atomic(dest: &std::path::Path, content: &Content) -> anyhow::Result<()> {
    use anyhow::Context;
    let dir = dest.parent().expect("Datei liegt in einem Verzeichnis");
    std::fs::create_dir_all(dir).with_context(|| format!("{} anlegen", dir.display()))?;
    let name = dest.file_name().unwrap().to_string_lossy();
    let tmp = dir.join(format!(".{name}.tmp-{}", std::process::id()));
    match content {
        Content::Bytes(b) => std::fs::write(&tmp, b),
        Content::File(src) => std::fs::copy(src, &tmp).map(|_| ()),
    }
    .with_context(|| format!("{} schreiben", dest.display()))?;
    std::fs::rename(&tmp, dest).with_context(|| format!("{} schreiben", dest.display()))?;
    Ok(())
}

/// Schreibt alle Ausgaben nach `out`: nur Geändertes, Verwaistes wird gelöscht.
pub fn write(out: &std::path::Path, files: BTreeMap<String, Content>) -> anyhow::Result<WriteStats> {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let manifest_path = out.join(MANIFEST);
    let old: BTreeMap<String, Entry> = if manifest_path.is_file() {
        serde_json::from_slice(&std::fs::read(&manifest_path)?).unwrap_or_default()
    } else if out.is_dir() && std::fs::read_dir(out)?.next().is_some() {
        anyhow::bail!(
            "{} ist nicht leer und enthält keine Markierung `{MANIFEST}` eines früheren Builds – es wird nichts gelöscht oder überschrieben",
            out.display()
        );
    } else {
        BTreeMap::new()
    };
    std::fs::create_dir_all(out)?;

    let (new, changed, unchanged) = (AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0));
    let entries: Vec<(String, Entry)> = files
        .par_iter()
        .map(|(path, content)| -> anyhow::Result<(String, Entry)> {
            let entry = match content {
                Content::Bytes(b) => Entry { hash: blake3::hash(b).to_hex().to_string(), len: b.len() as u64 },
                Content::File(src) => {
                    let mut h = blake3::Hasher::new();
                    h.update_mmap(src).map_err(|e| anyhow::anyhow!("{}: {e}", src.display()))?;
                    Entry { hash: h.finalize().to_hex().to_string(), len: std::fs::metadata(src)?.len() }
                }
            };
            let dest = target(out, path);
            let on_disk = std::fs::metadata(&dest).ok().map(|m| m.len());
            match old.get(path) {
                Some(prev) if *prev == entry && on_disk == Some(entry.len) => {
                    unchanged.fetch_add(1, Ordering::Relaxed);
                }
                _ => {
                    write_atomic(&dest, content)?;
                    if on_disk.is_some() { &changed } else { &new }.fetch_add(1, Ordering::Relaxed);
                }
            }
            Ok((path.clone(), entry))
        })
        .collect::<anyhow::Result<_>>()?;
    let manifest: BTreeMap<String, Entry> = entries.into_iter().collect();

    // Verwaiste Dateien und leere Verzeichnisse entfernen
    let mut deleted = 0;
    let mut dirs = Vec::new();
    for e in walkdir::WalkDir::new(out).min_depth(1).into_iter().filter_map(Result::ok) {
        let rel = format!("/{}", e.path().strip_prefix(out).unwrap().to_string_lossy().replace('\\', "/"));
        if e.file_type().is_dir() {
            dirs.push(e.path().to_path_buf());
        } else if rel != format!("/{MANIFEST}") && !manifest.contains_key(&rel) {
            std::fs::remove_file(e.path())?;
            deleted += 1;
        }
    }
    dirs.sort_by_key(|d| std::cmp::Reverse(d.components().count()));
    for d in dirs {
        let _ = std::fs::remove_dir(&d); // schlägt fehl, wenn nicht leer – gewollt
    }

    if manifest != old {
        write_atomic(&manifest_path, &Content::Bytes(serde_json::to_vec_pretty(&manifest)?))?;
    }
    Ok(WriteStats {
        new: new.into_inner(),
        changed: changed.into_inner(),
        unchanged: unchanged.into_inner(),
        deleted,
    })
}
