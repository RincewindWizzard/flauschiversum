//! `clean`: leert den Cache (und auf Wunsch das Ausgabeverzeichnis), ohne Fremdes anzufassen.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

/// Verzeichnisse im Cache, die der Generator anlegt (Bildvarianten, Vorschau-Ausgabe).
const CACHE_DIRS: &[&str] = &["v1", "serve"];
/// Dateien im Cache, die der Generator anlegt (Suchindex und seine Zwischendatei).
const CACHE_FILE_PREFIX: &str = "search-";
const CACHE_FILE_SUFFIXES: &[&str] = &[".bin", ".tmp"];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Removed {
    pub files: usize,
    pub bytes: u64,
}

impl Removed {
    fn add(&mut self, other: Removed) {
        self.files += other.files;
        self.bytes += other.bytes;
    }
}

/// Entfernt `path` (Datei, Link oder Verzeichnis) und zählt Dateien und Bytes. Links werden nicht verfolgt.
fn remove_counting(path: &Path) -> Result<Removed> {
    let meta = std::fs::symlink_metadata(path).with_context(|| path.display().to_string())?;
    let mut removed = Removed::default();
    if meta.is_dir() {
        for entry in std::fs::read_dir(path).with_context(|| path.display().to_string())? {
            removed.add(remove_counting(&entry?.path())?);
        }
        std::fs::remove_dir(path).with_context(|| format!("{} konnte nicht entfernt werden", path.display()))?;
    } else {
        removed.files = 1;
        removed.bytes = if meta.file_type().is_symlink() { 0 } else { meta.len() };
        std::fs::remove_file(path).with_context(|| format!("{} konnte nicht entfernt werden", path.display()))?;
    }
    Ok(removed)
}

/// Bricht ab, wenn der Pfad nicht sicher geleert werden darf.
fn guard(cache: &Path, root: &Path) -> Result<()> {
    let Ok(cache_abs) = cache.canonicalize() else { return Ok(()) }; // fehlt: nichts zu löschen
    let root_abs = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let reason = if cache_abs.parent().is_none() {
        Some("das Wurzelverzeichnis")
    } else if cache_abs == root_abs || root_abs.starts_with(&cache_abs) {
        Some("das Projektverzeichnis oder ein Elternverzeichnis davon")
    } else if cache_abs.join("content").exists() || cache_abs.join("Cargo.toml").exists() {
        Some("ein Verzeichnis mit `content/` oder `Cargo.toml`")
    } else {
        None
    };
    match reason {
        Some(r) => bail!("{} ist als Cache nicht erlaubt: {r}. Es wird nichts gelöscht.", cache.display()),
        None => Ok(()),
    }
}

/// Leert den Cache: nur bekannte Einträge, das Verzeichnis selbst bleibt.
pub fn clean_cache(cache: &Path, root: &Path) -> Result<Removed> {
    guard(cache, root)?;
    let mut removed = Removed::default();
    let Ok(entries) = std::fs::read_dir(cache) else { return Ok(removed) };
    let mut errors = Vec::new();
    for entry in entries.filter_map(Result::ok) {
        let name = entry.file_name().to_string_lossy().into_owned();
        let known = CACHE_DIRS.contains(&name.as_str())
            || (name.starts_with(CACHE_FILE_PREFIX) && CACHE_FILE_SUFFIXES.iter().any(|s| name.ends_with(s)));
        if !known {
            continue;
        }
        match remove_counting(&entry.path()) {
            Ok(r) => removed.add(r),
            Err(e) => errors.push(format!("{e:#}")),
        }
    }
    if !errors.is_empty() {
        bail!("{}", errors.join("; "));
    }
    Ok(removed)
}

/// Leert das Ausgabeverzeichnis, aber nur, wenn es vom Generator stammt (Markierung wie in `output::write`).
pub fn clean_dist(out: &Path) -> Result<Removed> {
    let mut removed = Removed::default();
    if !out.is_dir() {
        return Ok(removed);
    }
    let manifest_path = out.join(crate::output::MANIFEST);
    if !manifest_path.is_file() {
        if std::fs::read_dir(out)?.next().is_none() {
            return Ok(removed);
        }
        bail!(
            "{} ist nicht leer und enthält keine Markierung `{}` eines früheren Builds – es wird nichts gelöscht",
            out.display(),
            crate::output::MANIFEST
        );
    }
    let manifest: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_slice(&std::fs::read(&manifest_path)?).unwrap_or_default();
    let mut dirs: Vec<PathBuf> = Vec::new();
    for rel in manifest.keys() {
        let path = out.join(rel.trim_start_matches('/'));
        if std::fs::symlink_metadata(&path).is_ok() {
            removed.add(remove_counting(&path)?);
        }
        let mut parent = path.parent();
        while let Some(p) = parent.filter(|p| *p != out && p.starts_with(out)) {
            dirs.push(p.to_path_buf());
            parent = p.parent();
        }
    }
    removed.add(remove_counting(&manifest_path)?);
    dirs.sort_by_key(|d| std::cmp::Reverse(d.components().count()));
    dirs.dedup();
    for d in dirs {
        let _ = std::fs::remove_dir(&d); // nur leere Verzeichnisse
    }
    Ok(removed)
}

pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 { format!("{bytes} B") } else { format!("{value:.1} {}", UNITS[unit]).replace('.', ",") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fill(cache: &Path) {
        fs::create_dir_all(cache.join("v1/ab")).unwrap();
        fs::write(cache.join("v1/ab/x.jpg"), vec![0u8; 100]).unwrap();
        fs::write(cache.join("v1/ab/x.json"), "{}").unwrap();
        fs::create_dir_all(cache.join("serve")).unwrap();
        fs::write(cache.join("serve/index.html"), "hi").unwrap();
        fs::write(cache.join("search-abc.bin"), "idx").unwrap();
        fs::write(cache.join("search-abc.tmp"), "t").unwrap();
    }

    #[test]
    fn removes_known_entries_and_counts() {
        let t = tempfile::tempdir().unwrap();
        let (root, cache) = (t.path().join("proj"), t.path().join("proj/.cache"));
        fs::create_dir_all(&root).unwrap();
        fill(&cache);
        fs::write(cache.join("notizen.txt"), "meins").unwrap();
        let r = clean_cache(&cache, &root).unwrap();
        assert_eq!(r.files, 5);
        assert_eq!(r.bytes, 100 + 2 + 2 + 3 + 1);
        assert!(!cache.join("v1").exists() && !cache.join("serve").exists() && !cache.join("search-abc.bin").exists());
        assert!(cache.join("notizen.txt").exists(), "fremde Datei bleibt");
        assert!(cache.is_dir(), "Verzeichnis bleibt");
        assert_eq!(clean_cache(&cache, &root).unwrap(), Removed::default(), "wiederholt: leer");
    }

    #[test]
    fn missing_cache_is_fine() {
        let t = tempfile::tempdir().unwrap();
        assert_eq!(clean_cache(&t.path().join("gibtsnicht"), t.path()).unwrap(), Removed::default());
    }

    #[test]
    fn guard_refuses_project_parents_and_content_dirs() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path().join("proj");
        fs::create_dir_all(root.join("content")).unwrap();
        fs::write(root.join("Cargo.toml"), "").unwrap();
        fs::create_dir_all(root.join("v1")).unwrap();
        assert!(clean_cache(&root, &root).is_err(), "Projektverzeichnis");
        assert!(clean_cache(t.path(), &root).is_err(), "Elternverzeichnis");
        assert!(clean_cache(Path::new("/"), &root).is_err(), "Wurzel");
        let other = t.path().join("fremd");
        fs::create_dir_all(other.join("content")).unwrap();
        fs::create_dir_all(other.join("v1")).unwrap();
        assert!(clean_cache(&other, &root).is_err(), "Verzeichnis mit content/");
        assert!(root.join("v1").exists() && other.join("v1").exists(), "nichts gelöscht");
    }

    #[cfg(unix)]
    #[test]
    fn link_is_removed_but_not_followed() {
        let t = tempfile::tempdir().unwrap();
        let (root, cache) = (t.path().join("proj"), t.path().join("proj/.cache"));
        let target = t.path().join("ziel");
        fs::create_dir_all(&cache).unwrap();
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("wichtig.txt"), "x").unwrap();
        std::os::unix::fs::symlink(&target, cache.join("v1")).unwrap();
        clean_cache(&cache, &root).unwrap();
        assert!(fs::symlink_metadata(cache.join("v1")).is_err());
        assert!(target.join("wichtig.txt").exists());
    }

    #[test]
    fn dist_only_when_generated() {
        let t = tempfile::tempdir().unwrap();
        let out = t.path().join("dist");
        let mut files = std::collections::BTreeMap::new();
        files.insert("/index.html".to_string(), crate::output::Content::Bytes(b"hi".to_vec()));
        files.insert("/a/b/c.txt".to_string(), crate::output::Content::Bytes(b"c".to_vec()));
        crate::output::write(&out, files).unwrap();
        fs::write(out.join("eigen.txt"), "x").unwrap();
        let r = clean_dist(&out).unwrap();
        assert_eq!(r.files, 3);
        assert!(!out.join("index.html").exists() && !out.join("a").exists());
        assert!(out.join("eigen.txt").exists(), "Fremdes bleibt");

        let foreign = t.path().join("fremd");
        fs::create_dir_all(&foreign).unwrap();
        fs::write(foreign.join("x.txt"), "x").unwrap();
        assert!(clean_dist(&foreign).is_err());
        assert!(foreign.join("x.txt").exists());
        assert_eq!(clean_dist(&t.path().join("nix")).unwrap(), Removed::default());
    }

    #[test]
    fn sizes_are_readable() {
        assert_eq!(human_size(10), "10 B");
        assert_eq!(human_size(1536), "1,5 KB");
        assert_eq!(human_size(1_181_116_006), "1,1 GB");
    }
}
