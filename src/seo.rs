//! Suchmaschinen und Vorschauen: Meta-Beschreibung, JSON-LD, Sitemap, robots.txt.

use crate::config::{BLOG_NAME, BLOG_URL, META_DESCRIPTION_MAX};
use crate::content::{Post, Site};
use crate::topics::{self, Topics};
use crate::urls;
use serde_json::json;
use std::fmt::Write;

/// Kürzt einen Text an einer Wortgrenze auf höchstens `META_DESCRIPTION_MAX` Zeichen (inkl. „…“).
pub fn meta_description(text: &str) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() <= META_DESCRIPTION_MAX {
        return text;
    }
    let mut out = String::new();
    for word in text.split(' ') {
        let candidate_len = out.chars().count() + usize::from(!out.is_empty()) + word.chars().count();
        if candidate_len + 1 > META_DESCRIPTION_MAX {
            break;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    let out = out.trim_end_matches([',', ';', ':', '-', '–', '.']).to_string();
    format!("{out}…")
}

/// ISO-8601-Zeitstempel mit Zeitzonen-Offset.
pub fn iso(post: &Post) -> String {
    post.date.timestamp().display_with_offset(post.date.offset()).to_string()
}

/// JSON-LD `BlogPosting` für eine Post-Seite.
pub fn blog_posting(post: &Post, image: Option<&str>, section: Option<&str>) -> String {
    let url = urls::absolute(&post.url);
    let mut v = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": post.title,
        "datePublished": iso(post),
        "mainEntityOfPage": url,
        "url": url,
        "inLanguage": "de-DE",
        "publisher": { "@type": "Organization", "name": BLOG_NAME, "url": format!("{BLOG_URL}/") },
    });
    if let Some(a) = &post.author {
        v["author"] = json!({ "@type": "Person", "name": a });
    }
    if let Some(i) = image {
        v["image"] = json!(i);
    }
    if let Some(e) = &post.excerpt {
        v["description"] = json!(e);
    }
    if let Some(name) = section {
        v["articleSection"] = json!(name);
    }
    if !post.tags.is_empty() {
        v["keywords"] = json!(post.tags.join(", "));
    }
    // `</` darf in einem Script-Block nicht vorkommen
    v.to_string().replace("</", "<\\/")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Sitemap mit Startseite, Themenseiten, allen veröffentlichten Posts und Einzelseiten.
pub fn sitemap(site: &Site, topics: &Topics) -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );
    let _ = writeln!(out, "  <url><loc>{BLOG_URL}/</loc></url>");
    let _ = writeln!(out, "  <url><loc>{BLOG_URL}/themen/</loc></url>");
    let topic_urls = topics
        .categories
        .iter()
        .map(|c| topics::category_url(&c.slug))
        .chain(topics.tags.iter().map(|t| topics::tag_url(&t.slug)));
    for url in topic_urls {
        let _ = writeln!(out, "  <url><loc>{}</loc></url>", xml_escape(&urls::absolute(&url)));
    }
    for post in &site.posts {
        let _ = writeln!(
            out,
            "  <url><loc>{}</loc><lastmod>{}</lastmod></url>",
            xml_escape(&urls::absolute(&post.url)),
            post.date.date()
        );
    }
    for page in &site.pages {
        let _ = writeln!(out, "  <url><loc>{}</loc></url>", xml_escape(&urls::absolute(&page.url)));
    }
    out.push_str("</urlset>\n");
    out
}

pub fn robots() -> String {
    format!("User-agent: *\nAllow: /\n\nSitemap: {BLOG_URL}/sitemap.xml\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_is_kept() {
        let t = "Es ist wieder die gruselige Jahreszeit und da muss man einen Kürbis haben.";
        assert_eq!(meta_description(t), t);
    }

    #[test]
    fn long_text_is_cut_at_word_boundary() {
        let t = "Wort ".repeat(60);
        let d = meta_description(&t);
        assert!(d.chars().count() <= META_DESCRIPTION_MAX, "{}", d.chars().count());
        assert!(d.ends_with("Wort…"));
        let t = format!("{} Langeswortamende", "a".repeat(150));
        assert_eq!(meta_description(&t), format!("{}…", "a".repeat(150)));
    }

    #[test]
    fn robots_points_to_sitemap() {
        assert!(robots().contains("Sitemap: https://flauschiversum.de/sitemap.xml"));
    }
}
