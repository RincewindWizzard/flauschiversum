//! URL-Bildung: Slugs, Medien-URLs und Prozentkodierung.

use crate::config::BLOG_URL;
use jiff::Zoned;

/// Bildet einen Slug: kleingeschrieben, deutsche Umlaute als ae/oe/ue, ß als ss,
/// alles außer a-z und 0-9 wird zu einem einzelnen `-`.
pub fn slugify(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars().flat_map(char::to_lowercase) {
        let mapped: &str = match c {
            'ä' => "ae",
            'ö' => "oe",
            'ü' => "ue",
            'ß' => "ss",
            'à' | 'á' | 'â' | 'ã' | 'å' => "a",
            'è' | 'é' | 'ê' | 'ë' => "e",
            'ì' | 'í' | 'î' | 'ï' => "i",
            'ò' | 'ó' | 'ô' | 'õ' | 'ø' => "o",
            'ù' | 'ú' | 'û' => "u",
            'ç' => "c",
            'ñ' => "n",
            c if c.is_ascii_alphanumeric() => {
                out.push(c);
                continue;
            }
            _ => "-",
        };
        out.push_str(mapped);
    }
    let mut collapsed = String::with_capacity(out.len());
    for c in out.chars() {
        if c == '-' && (collapsed.is_empty() || collapsed.ends_with('-')) {
            continue;
        }
        collapsed.push(c);
    }
    collapsed.trim_end_matches('-').to_string()
}

/// URL eines Posts ohne Permalink: `/<JJJJ>/<MM>/<slug>/`.
pub fn default_post_url(date: &Zoned, dir_name: &str) -> String {
    format!("/{:04}/{:02}/{}/", date.year(), date.month(), slugify(dir_name))
}

/// URL einer Einzelseite: `/<slug(titel)>/`.
pub fn page_url(title: &str) -> String {
    format!("/{}/", slugify(title))
}

/// URL-Pfad einer Bildvariante (unkodiert).
pub fn variant_path(post_url: &str, width: u32, file_name: &str) -> String {
    format!("{post_url}{width}x/{file_name}")
}

/// URL-Pfad eines unveränderten Mediums (unkodiert).
pub fn original_path(post_url: &str, file_name: &str) -> String {
    format!("{post_url}{file_name}")
}

/// Prozentkodiert einen URL-Pfad nach RFC 3986. `/` bleibt erhalten.
pub fn encode_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    for b in path.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Absolute, kodierte URL für einen URL-Pfad.
pub fn absolute(path: &str) -> String {
    format!("{BLOG_URL}{}", encode_path(path))
}

/// Dekodiert Prozentkodierung (für die Prüfung interner Links).
pub fn decode_path(path: &str) -> String {
    percent_encoding::percent_decode_str(path).decode_utf8_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_transliterates_german() {
        assert_eq!(slugify("grüne-mütze"), "gruene-muetze");
        assert_eq!(slugify("Ein weißer Magierhut"), "ein-weisser-magierhut");
        assert_eq!(slugify("Tipps und Tricks"), "tipps-und-tricks");
        assert_eq!(slugify("Stricklies'ln & Häkelpetra's"), "stricklies-ln-haekelpetra-s");
        assert_eq!(slugify("  --Hallo!!  "), "hallo");
    }

    #[test]
    fn page_urls_match_live_site() {
        assert_eq!(page_url("About"), "/about/");
        assert_eq!(page_url("Impressum"), "/impressum/");
        assert_eq!(page_url("Tipps und Tricks"), "/tipps-und-tricks/");
    }

    #[test]
    fn default_url_uses_directory_name() {
        let date: Zoned = "2026-11-01T00:00[Europe/Berlin]".parse().unwrap();
        assert_eq!(default_post_url(&date, "grüne-mütze"), "/2026/11/gruene-muetze/");
    }

    #[test]
    fn encodes_spaces_and_umlauts() {
        assert_eq!(encode_path("700x/Schwacher Moment.jpg"), "700x/Schwacher%20Moment.jpg");
        assert_eq!(
            encode_path("/2020/10/kurbis/200x/kürbis_thumb.jpg"),
            "/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg"
        );
        assert_eq!(decode_path("k%C3%BCrbis%20a.jpg"), "kürbis a.jpg");
    }
}
