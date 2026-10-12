//! Suchindex: erzeugt mit Pagefind (als Bibliothek) im Speicher, ausgeliefert unter `/pagefind/`.

use anyhow::{Context, Result};
use pagefind::api::PagefindIndex;
use pagefind::options::PagefindServiceConfig;
use pulldown_cmark_escape::{escape_html, escape_html_body_text};

/// Ein Dokument des Suchindex: ein veröffentlichter Post oder eine Einzelseite.
pub struct SearchDoc {
    /// URL-Pfad (unkodiert), z.B. `/2020/10/kurbis/`.
    pub url: String,
    pub title: String,
    /// Kodierte URL des Vorschaubilds, falls vorhanden.
    pub image: Option<String>,
    pub tags: Vec<String>,
    /// Gerenderter Inhalt (HTML) ohne Seitenrahmen.
    pub body_html: String,
}

/// Name des Filters nach Themen (Schlagworten) im Index (so heißt er auch in der Oberfläche).
pub const TOPIC_FILTER: &str = "Thema";

fn attr(value: &str) -> String {
    let mut out = String::new();
    escape_html(&mut out, value).expect("String");
    out
}

fn text(value: &str) -> String {
    let mut out = String::new();
    escape_html_body_text(&mut out, value).expect("String");
    out
}

/// Der Text, den Pagefind liest: nur das, was durchsucht und angezeigt werden soll.
fn document(doc: &SearchDoc) -> String {
    let mut html = String::from("<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"></head><body data-pagefind-body>");
    html.push_str(&format!("<h1>{}</h1>", text(&doc.title)));
    if let Some(image) = &doc.image {
        html.push_str(&format!("<span data-pagefind-meta=\"image:{}\"></span>", attr(image)));
    }
    for tag in &doc.tags {
        html.push_str(&format!("<span data-pagefind-filter=\"{TOPIC_FILTER}:{}\"></span>", attr(tag)));
    }
    html.push_str(&doc.body_html);
    // Schlagworte sind durchsuchbar; am Ende, damit Ausschnitte bevorzugt den Text zeigen
    if !doc.tags.is_empty() {
        html.push_str(&format!("<p data-pagefind-weight=\"3\">{}</p>", text(&doc.tags.join(" "))));
    }
    html.push_str("</body></html>");
    html
}

/// Dateien, die die Suchseite braucht (der Rest des Pagefind-Pakets, z.B. Playground und Komponenten, entfällt).
fn is_needed(path: &str) -> bool {
    let name = path.trim_start_matches('/').trim_start_matches("pagefind/");
    name.starts_with("index/")
        || name.starts_with("fragment/")
        || name.starts_with("filter/")
        || name.starts_with("pagefind.de_")
        || matches!(
            name,
            "pagefind.js" | "pagefind-worker.js" | "pagefind-ui.js" | "pagefind-ui.css" | "pagefind-entry.json" | "wasm.de.pagefind"
        )
}

fn input_key(docs: &[SearchDoc]) -> String {
    let mut h = blake3::Hasher::new();
    h.update(b"pagefind-1.5.2/v1\0");
    for d in docs {
        let doc = document(d);
        h.update(&(d.url.len() as u64).to_le_bytes());
        h.update(d.url.as_bytes());
        h.update(&(doc.len() as u64).to_le_bytes());
        h.update(doc.as_bytes());
    }
    h.finalize().to_hex()[..16].to_string()
}

fn encode(files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (path, data) in files {
        out.extend((path.len() as u32).to_le_bytes());
        out.extend(path.as_bytes());
        out.extend((data.len() as u64).to_le_bytes());
        out.extend(data);
    }
    out
}

fn decode(mut bytes: &[u8]) -> Option<Vec<(String, Vec<u8>)>> {
    let mut files = Vec::new();
    while !bytes.is_empty() {
        let n = u32::from_le_bytes(bytes.get(..4)?.try_into().ok()?) as usize;
        let path = String::from_utf8(bytes.get(4..4 + n)?.to_vec()).ok()?;
        bytes = &bytes[4 + n..];
        let m = u64::from_le_bytes(bytes.get(..8)?.try_into().ok()?) as usize;
        files.push((path, bytes.get(8..8 + m)?.to_vec()));
        bytes = &bytes[8 + m..];
    }
    Some(files)
}

fn generate(docs: &[SearchDoc]) -> Result<Vec<(String, Vec<u8>)>> {
    let runtime = tokio::runtime::Builder::new_current_thread().build().context("Laufzeit für die Suche")?;
    runtime.block_on(async {
        let config = PagefindServiceConfig::builder().force_language("de".to_string()).build();
        let mut index = PagefindIndex::new(Some(config)).context("Suchindex")?;
        for doc in docs {
            index
                .add_html_file(None, Some(doc.url.clone()), document(doc))
                .await
                .with_context(|| format!("Suchindex: {}", doc.url))?;
        }
        let files = index.get_files().await.context("Suchindex erzeugen")?;
        Ok(files
            .into_iter()
            .map(|f| (format!("/pagefind/{}", f.filename.to_string_lossy().replace('\\', "/")), f.contents))
            .filter(|(path, _)| is_needed(path))
            .collect())
    })
}

/// Erzeugt den Index (Liefert `(URL-Pfad, Inhalt)` aller Dateien unter `/pagefind/`).
///
/// Pagefind selbst ist nicht ganz reproduzierbar (die Filterdatei hängt von der Reihenfolge einer
/// internen Hash-Tabelle ab). Deshalb merkt sich `cache` das Ergebnis zu den Eingaben: Sind alle
/// Dokumente unverändert, kommen byte-gleiche Dateien heraus, und der Build schreibt nichts.
pub fn build(mut docs: Vec<SearchDoc>, cache: &std::path::Path) -> Result<Vec<(String, Vec<u8>)>> {
    docs.sort_by(|a, b| a.url.cmp(&b.url));
    let key = input_key(&docs);
    let cached = cache.join(format!("search-{key}.bin"));
    if let Some(files) = std::fs::read(&cached).ok().and_then(|b| decode(&b)) {
        crate::progress::note("Suchindex aus dem Cache übernommen");
        return Ok(files);
    }
    crate::progress::note("Suchindex ist neu zu berechnen (das kann bei vielen Seiten einige Minuten dauern) …");
    let files = generate(&docs)?;
    if let Ok(entries) = std::fs::read_dir(cache) {
        for e in entries.filter_map(Result::ok) {
            if e.file_name().to_string_lossy().starts_with("search-") {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    // Der Cache ist nur eine Abkürzung: Schreibfehler sind kein Grund abzubrechen
    if std::fs::create_dir_all(cache).is_ok() {
        let tmp = cache.join(format!("search-{key}.tmp"));
        if std::fs::write(&tmp, encode(&files)).is_ok() {
            let _ = std::fs::rename(&tmp, &cached);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docs() -> Vec<SearchDoc> {
        let doc = |url: &str, title: &str, tags: &[&str], body: &str| SearchDoc {
            url: url.into(),
            title: title.into(),
            image: None,
            tags: tags.iter().map(|t| t.to_string()).collect(),
            body_html: body.into(),
        };
        vec![
            doc("/2020/10/kurbis/", "Kürbis", &["Basteln", "Halloween"], "<p>Ein gefilzter Kürbis für den Herbst.</p>"),
            doc("/2014/01/katzenmuetze/", "Katzenmütze", &["Wolle", "Katzen", "Mützen und Stirnbänder"], "<p>Eine Mütze mit Ohren.</p>"),
            doc("/2016/05/tasche/", "Tasche", &["Taschen"], "<p>Aus altem Filz genäht.</p>"),
        ]
    }

    fn sizes(files: &[(String, Vec<u8>)]) -> usize {
        files.iter().map(|(_, c)| c.len()).sum()
    }

    #[test]
    fn index_has_needed_files_and_is_cached() {
        let cache = tempfile::tempdir().unwrap();
        let files = build(docs(), cache.path()).unwrap();
        for need in ["/pagefind/pagefind.js", "/pagefind/pagefind-ui.js", "/pagefind/wasm.de.pagefind", "/pagefind/pagefind-entry.json"] {
            assert!(files.iter().any(|(p, _)| p == need), "{need}");
        }
        assert!(files.iter().all(|(p, _)| !p.contains("component") && !p.contains("modular")));
        assert!(sizes(&files) > 100_000);
        // Zweiter Lauf mit gleichen Eingaben: byte-gleich, auch die Filterdatei
        let again = build(docs(), cache.path()).unwrap();
        assert_eq!(files, again);
        // Reihenfolge der Eingaben spielt keine Rolle
        let mut reversed = docs();
        reversed.reverse();
        assert_eq!(files, build(reversed, cache.path()).unwrap());
        // Geänderter Text: neuer Index
        let mut changed = docs();
        changed[0].body_html.push_str("<p>Neu</p>");
        assert_ne!(files, build(changed, cache.path()).unwrap());
        // Es bleibt genau ein Cache-Eintrag
        let n = std::fs::read_dir(cache.path()).unwrap().filter_map(Result::ok).filter(|e| e.file_name().to_string_lossy().starts_with("search-")).count();
        assert_eq!(n, 1);
    }

    #[test]
    fn cache_encoding_roundtrips() {
        let files = vec![("/pagefind/a".to_string(), vec![1, 2, 3]), ("/pagefind/ü".to_string(), vec![])];
        assert_eq!(decode(&encode(&files)).unwrap(), files);
        assert!(decode(&encode(&files)[..5]).is_none());
    }
}
