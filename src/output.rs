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

    pub fn into_map(self) -> BTreeMap<String, Content> {
        self.files.into_inner().unwrap()
    }

    pub fn len(&self) -> usize {
        self.files.lock().unwrap().len()
    }
}
