//! RSS-2.0-Feed mit den neuesten veröffentlichten Posts.

use crate::config::{BLOG_DESCRIPTION, BLOG_NAME, BLOG_URL, FEED_ENTRIES};
use crate::content::Site;
use crate::urls;
use rss::extension::atom::{AtomExtensionBuilder, Link};
use rss::extension::dublincore::DublinCoreExtensionBuilder;
use rss::{ChannelBuilder, EnclosureBuilder, GuidBuilder, ItemBuilder};
use std::collections::BTreeMap;

/// Pro Post: vollständiger HTML-Inhalt mit absoluten URLs und optionales Enclosure (URL, Länge, MIME-Typ).
pub struct FeedEntry {
    pub content_html: String,
    pub enclosure: Option<(String, u64, &'static str)>,
}

pub fn mime_for(file_name: &str) -> &'static str {
    match file_name.rsplit('.').next().map(str::to_ascii_lowercase).as_deref() {
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "image/jpeg",
    }
}

/// Erzeugt den Feed. `entries` enthält Inhalt und Enclosure der neuesten Posts in derselben Reihenfolge.
pub fn render(site: &Site, entries: Vec<FeedEntry>) -> String {
    let items: Vec<_> = site
        .posts
        .iter()
        .take(FEED_ENTRIES)
        .zip(entries)
        .map(|(post, entry)| {
            let url = urls::absolute(&post.url);
            let mut item = ItemBuilder::default();
            item.title(Some(post.title.clone()))
                .link(Some(url.clone()))
                .description(post.excerpt.clone())
                .content(Some(entry.content_html))
                .pub_date(Some(post.date.timestamp().strftime_with_offset(post.date.offset())))
                .guid(Some(GuidBuilder::default().value(url).permalink(false).build()));
            if let Some(author) = &post.author {
                item.dublin_core_ext(Some(DublinCoreExtensionBuilder::default().creators(vec![author.clone()]).build()));
            }
            if let Some((url, len, mime)) = entry.enclosure {
                item.enclosure(Some(EnclosureBuilder::default().url(url).length(len.to_string()).mime_type(mime).build()));
            }
            item.build()
        })
        .collect();

    let last_build = site.posts.first().map(|p| p.date.timestamp().strftime_with_offset(p.date.offset()));
    let mut namespaces = BTreeMap::new();
    namespaces.insert("atom".to_string(), "http://www.w3.org/2005/Atom".to_string());
    namespaces.insert("content".to_string(), "http://purl.org/rss/1.0/modules/content/".to_string());
    namespaces.insert("dc".to_string(), "http://purl.org/dc/elements/1.1/".to_string());
    let channel = ChannelBuilder::default()
        .title(BLOG_NAME)
        .link(format!("{BLOG_URL}/"))
        .description(BLOG_DESCRIPTION)
        .language(Some("de-DE".to_string()))
        .last_build_date(last_build)
        .namespaces(namespaces)
        .atom_ext(Some(
            AtomExtensionBuilder::default()
                .links(vec![Link {
                    href: format!("{BLOG_URL}/rss.xml"),
                    rel: "self".into(),
                    mime_type: Some("application/rss+xml".into()),
                    ..Default::default()
                }])
                .build(),
        ))
        .items(items)
        .build();
    let mut out = channel.pretty_write_to(Vec::new(), b' ', 2).expect("Feed in Speicher schreiben");
    out.push(b'\n');
    String::from_utf8(out).expect("Feed ist UTF-8")
}

/// Formatiert einen Zeitpunkt nach RFC 2822 (für `pubDate`).
trait Rfc2822 {
    fn strftime_with_offset(&self, offset: jiff::tz::Offset) -> String;
}

impl Rfc2822 for jiff::Timestamp {
    fn strftime_with_offset(&self, offset: jiff::tz::Offset) -> String {
        jiff::fmt::rfc2822::to_string(&self.to_zoned(jiff::tz::TimeZone::fixed(offset))).expect("gültiges Datum")
    }
}
