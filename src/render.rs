//! HTML-Seiten über Askama-Templates (beim Kompilieren geprüft).

use crate::config::{BLOG_DESCRIPTION, BLOG_NAME, CONTACT_EMAIL, INSTAGRAM_URL, MASTODON_URL, PAGINATION_WINDOW, POSTS_PER_PAGE};
use crate::archive::ArchiveYear;
use crate::content::{Page, Post};
use crate::media::Size;
use crate::seo;
use crate::urls::{self, Base};
use askama::Template;
use std::sync::Arc;

pub const MONTHS: [&str; 12] = [
    "Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember",
];

/// „31. Oktober 2020“
pub fn date_human(post: &Post) -> String {
    format!("{}. {} {}", post.date.day(), MONTHS[post.date.month() as usize - 1], post.date.year())
}

/// Gemeinsame Angaben für den Seitenkopf.
pub struct Head {
    pub title: String,
    pub og_title: String,
    pub description: String,
    pub canonical: String,
    pub og_type: &'static str,
    pub og_image: Option<OgImage>,
    pub published_time: Option<String>,
    pub json_ld: Option<String>,
    pub style_href: String,
    pub year: i16,
    pub livereload: bool,
    pub noindex: bool,
    /// Branchname im Staging-Modus (zeigt die Staging-Leiste).
    pub staging: Option<String>,
    pub feed_url: String,
    /// Menüs der Topbar (auf jeder Seite gleich).
    pub menu: Arc<Menu>,
    /// Klassen des `body`: `has-aside` (Karte neben dem Inhalt), `aside-early` (auf schmalen Fenstern vor der Liste).
    pub body_class: &'static str,
    pub topbar_script_href: String,
    /// Kontaktwege in der Fußzeile.
    pub mail_href: String,
    pub mail_address: &'static str,
    pub instagram_url: &'static str,
    pub mastodon_url: &'static str,
}

/// Ein Schlagwort der Tag-Cloud.
pub struct CloudTag {
    pub url: String,
    pub name: String,
    pub count: usize,
    /// Größenstufe 1–5.
    pub level: u8,
}

/// Menüdaten der Topbar: die neuesten Beiträge und alle Kategorien.
pub struct Menu {
    pub latest: Vec<NavLink>,
    pub categories: Vec<TopicLink>,
}

pub struct OgImage {
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub alt: String,
}

/// Seitenweite Angaben, die jede Seite braucht.
pub struct Common {
    pub style_href: String,
    pub script_href: String,
    pub topbar_script_href: String,
    pub year: i16,
    pub livereload: bool,
    pub base: Base,
    /// Staging-Modus mit Branchname: noindex und Staging-Leiste auf jeder Seite.
    pub staging: Option<String>,
    pub menu: Arc<Menu>,
    /// Tag-Cloud (nur auf der Startseite).
    pub cloud: Arc<Vec<CloudTag>>,
}

pub struct Thumb {
    pub src: String,
    pub width: u32,
    pub height: u32,
}

pub struct PostCard {
    pub url: String,
    pub title: String,
    pub date_iso: String,
    pub date_human: String,
    pub thumb: Option<Thumb>,
    pub excerpt: String,
}

/// Link mit Anzeigename, optional mit Anzahl der Beiträge.
pub struct TopicLink {
    pub url: String,
    pub name: String,
    pub count: Option<usize>,
}

/// Eintrag in einer Beitragsliste.
pub struct Entry {
    pub url: String,
    pub title: String,
    pub date_iso: String,
    pub date_human: String,
    pub thumb: Option<Thumb>,
    pub excerpt: String,
    pub category: Option<TopicLink>,
}

pub struct PageLink {
    pub number: usize,
    pub url: String,
    pub current: bool,
}

pub struct Pagination {
    pub prev: Option<String>,
    pub next: Option<String>,
    pub pages: Vec<PageLink>,
}

pub struct NavLink {
    pub url: String,
    pub title: String,
}

/// Ähnlicher Beitrag mit den gemeinsamen Schlagworten (leer bei aufgefüllten Einträgen).
pub struct RelatedLink {
    pub url: String,
    pub title: String,
    pub shared: Vec<TopicLink>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub head: Head,
    pub posts: Vec<PostCard>,
    pub pagination: Pagination,
}

#[derive(Template)]
#[template(path = "beitraege.html")]
pub struct PostsTemplate {
    pub head: Head,
    pub entries: Vec<Entry>,
    pub archive: Vec<ArchiveYear>,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub head: Head,
    pub welcome: Option<String>,
    pub news: Vec<Entry>,
    pub cloud: Arc<Vec<CloudTag>>,
}

#[derive(Template)]
#[template(path = "topic.html")]
pub struct TopicTemplate {
    pub head: Head,
    pub title: String,
    pub motto: Option<String>,
    pub count_text: String,
    pub intro: Option<String>,
    pub archive: Vec<ArchiveYear>,
    pub entries: Vec<Entry>,
}

#[derive(Template)]
#[template(path = "topics.html")]
pub struct TopicsTemplate {
    pub head: Head,
    pub categories: Vec<TopicLink>,
    pub tags: Vec<TopicLink>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub head: Head,
    pub script_href: String,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub head: Head,
    pub title: String,
    pub thumb: Option<Thumb>,
    pub author: Option<String>,
    pub mail_href: String,
    pub content: String,
    pub prev: Option<NavLink>,
    pub next: Option<NavLink>,
    pub has_media: bool,
    pub script_href: String,
    pub category: TopicLink,
    pub tags: Vec<TopicLink>,
    pub related: Vec<RelatedLink>,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub head: Head,
    pub title: String,
    pub content: String,
}

/// URL der Seite `index` der chronologischen Liste (0 = `/beitraege/`).
pub fn index_url(index: usize) -> String {
    if index == 0 { "/beitraege/".to_string() } else { format!("/page/{index}/") }
}

/// Paginierung für Seite `index` von `count` Seiten.
pub fn pagination(index: usize, count: usize) -> Pagination {
    let start = index.saturating_sub(PAGINATION_WINDOW);
    let end = (index + PAGINATION_WINDOW).min(count.saturating_sub(1));
    Pagination {
        prev: (index > 0).then(|| index_url(index - 1)),
        next: (index + 1 < count).then(|| index_url(index + 1)),
        pages: (start..=end).map(|i| PageLink { number: i + 1, url: index_url(i), current: i == index }).collect(),
    }
}

/// Anzahl Indexseiten für `posts` veröffentlichte Posts (mindestens eine).
pub fn page_count(posts: usize) -> usize {
    posts.div_ceil(POSTS_PER_PAGE).max(1)
}

/// `mailto:`-Adresse mit kodiertem Betreff. Der Betreff muss „Flauschiversum“ enthalten (Spamfilter).
pub fn mailto(subject: &str) -> String {
    assert!(subject.contains(BLOG_NAME), "Betreff `{subject}` enthält nicht „{BLOG_NAME}“");
    let mut out = format!("mailto:{CONTACT_EMAIL}?subject=");
    for b in subject.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// Gemeinsamer Seitenkopf; Aufrufer überschreiben, was abweicht.
fn base_head(common: &Common, title: String, og_title: String, description: String, canonical: String) -> Head {
    Head {
        title,
        og_title,
        description,
        canonical,
        og_type: "website",
        og_image: None,
        published_time: None,
        json_ld: None,
        style_href: common.style_href.clone(),
        year: common.year,
        livereload: common.livereload,
        noindex: common.staging.is_some(),
        staging: common.staging.clone(),
        feed_url: format!("{}/rss.xml", common.base.as_str()),
        menu: common.menu.clone(),
        body_class: "",
        topbar_script_href: common.topbar_script_href.clone(),
        mail_href: mailto(BLOG_NAME),
        mail_address: CONTACT_EMAIL,
        instagram_url: INSTAGRAM_URL,
        mastodon_url: MASTODON_URL,
    }
}

/// „1 Beitrag“ / „37 Beiträge“
pub fn count_text(n: usize) -> String {
    if n == 1 { "1 Beitrag".to_string() } else { format!("{n} Beiträge") }
}

pub fn render_posts(entries: Vec<Entry>, archive: Vec<ArchiveYear>, common: &Common) -> String {
    let title = format!("Alle Beiträge – {BLOG_NAME}");
    let mut head = base_head(
        common,
        title.clone(),
        title,
        "Alle Beiträge des Flauschiversums, die neuesten zuerst.".to_string(),
        format!("{}{}", common.base.as_str(), index_url(0)),
    );
    head.body_class = "has-aside";
    PostsTemplate { head, entries, archive }.render().expect("Beiträge-Template")
}

pub fn render_index(index: usize, count: usize, cards: Vec<PostCard>, common: &Common) -> String {
    let title = if index == 0 {
        format!("Alle Beiträge – {BLOG_NAME}")
    } else {
        format!("Alle Beiträge – Seite {} – {BLOG_NAME}", index + 1)
    };
    let head = base_head(
        common,
        title.clone(),
        title,
        "Alle Beiträge des Flauschiversums, die neuesten zuerst.".to_string(),
        format!("{}{}", common.base.as_str(), index_url(index)),
    );
    IndexTemplate { head, posts: cards, pagination: pagination(index, count) }.render().expect("Index-Template")
}

pub fn render_home(welcome: Option<String>, news: Vec<Entry>, common: &Common) -> String {
    let title = format!("{BLOG_NAME} – {BLOG_DESCRIPTION}");
    let mut head = base_head(
        common,
        title.clone(),
        title,
        "Anleitungen und Ideen rund um Wolle, Filz, Nähen und Basteln – zum Suchen, Stöbern und Nachmachen.".to_string(),
        format!("{}/", common.base.as_str()),
    );
    head.body_class = "has-aside";
    HomeTemplate { head, welcome, news, cloud: common.cloud.clone() }.render().expect("Start-Template")
}

/// Seite einer Kategorie oder eines Schlagworts.
pub struct TopicPageInput {
    pub url: String,
    pub name: String,
    pub motto: Option<String>,
    /// Einleitungstext als HTML (nur Kategorien).
    pub intro: Option<String>,
    pub count: usize,
    /// Archiv der Beiträge (nur Kategorien); leer bei Schlagworten.
    pub archive: Vec<ArchiveYear>,
    pub entries: Vec<Entry>,
}

pub fn render_topic(input: TopicPageInput, common: &Common) -> String {
    let mut description = format!("{} zu {} im {BLOG_NAME}", count_text(input.count), input.name);
    if let Some(m) = &input.motto {
        description = format!("{description}. {}", m.split_whitespace().collect::<Vec<_>>().join(" "));
    }
    let mut head = base_head(
        common,
        format!("{} – {BLOG_NAME}", input.name),
        input.name.clone(),
        seo::meta_description(&description),
        common.base.absolute(&input.url),
    );
    if !input.archive.is_empty() {
        head.body_class = "has-aside aside-early";
    }
    TopicTemplate {
        head,
        title: input.name,
        motto: input.motto,
        count_text: count_text(input.count),
        intro: input.intro,
        archive: input.archive,
        entries: input.entries,
    }
    .render()
    .expect("Themen-Template")
}

pub fn render_search(script_href: String, common: &Common) -> String {
    let mut head = base_head(
        common,
        format!("Suche – {BLOG_NAME}"),
        "Suche".to_string(),
        format!("Suche in den Anleitungen und Ideen des {BLOG_NAME}s."),
        common.base.absolute("/suche/"),
    );
    head.noindex = true;
    SearchTemplate { head, script_href }.render().expect("Such-Template")
}

pub fn render_topics(categories: Vec<TopicLink>, tags: Vec<TopicLink>, common: &Common) -> String {
    let description = format!("Alle Kategorien und Schlagworte im {BLOG_NAME}: {} Kategorien, {} Schlagworte.", categories.len(), tags.len());
    let head = base_head(common, format!("Themen – {BLOG_NAME}"), "Themen".to_string(), description, common.base.absolute("/themen/"));
    TopicsTemplate { head, categories, tags }.render().expect("Themenübersicht-Template")
}

pub struct PostPageInput<'a> {
    pub post: &'a Post,
    pub content: String,
    pub thumb: Option<Thumb>,
    pub og_image: Option<(String, Size)>,
    pub newer: Option<&'a Post>,
    pub older: Option<&'a Post>,
    pub has_media: bool,
    pub category: TopicLink,
    pub tags: Vec<TopicLink>,
    pub related: Vec<RelatedLink>,
}

pub fn render_post(input: PostPageInput, common: &Common) -> String {
    let post = input.post;
    let image_url = input.og_image.as_ref().map(|(u, _)| u.as_str());
    let mut head = base_head(
        common,
        format!("{} – {BLOG_NAME}", post.title),
        post.title.clone(),
        seo::meta_description(post.excerpt.as_deref().unwrap_or(BLOG_DESCRIPTION)),
        common.base.absolute(&post.url),
    );
    head.og_type = "article";
    head.og_image = input.og_image.as_ref().map(|(url, size)| OgImage {
        url: url.clone(),
        width: size.width,
        height: size.height,
        alt: post.title.clone(),
    });
    head.published_time = Some(seo::iso(post));
    head.json_ld = Some(seo::blog_posting(post, image_url, Some(input.category.name.as_str()), &common.base));
    let link = |p: &Post| NavLink { url: urls::encode_path(&p.url), title: p.title.clone() };
    PostTemplate {
        head,
        title: post.title.clone(),
        thumb: input.thumb,
        author: post.author.clone(),
        mail_href: mailto(&format!("[{BLOG_NAME}] {}", post.title)),
        content: input.content,
        prev: input.older.map(link),
        next: input.newer.map(link),
        has_media: input.has_media,
        script_href: common.script_href.clone(),
        category: input.category,
        tags: input.tags,
        related: input.related,
    }
    .render()
    .expect("Post-Template")
}

pub fn render_page(page: &Page, content: String, description: String, common: &Common) -> String {
    let head = base_head(
        common,
        format!("{} – {BLOG_NAME}", page.title),
        page.title.clone(),
        seo::meta_description(&description),
        common.base.absolute(&page.url),
    );
    PageTemplate { head, title: page.title.clone(), content }.render().expect("Seiten-Template")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn numbers(p: &Pagination) -> Vec<usize> {
        p.pages.iter().map(|l| l.number).collect()
    }

    #[test]
    fn first_page() {
        let p = pagination(0, 49);
        assert_eq!(numbers(&p), [1, 2, 3]);
        assert!(p.pages[0].current);
        assert_eq!(p.prev, None);
        assert_eq!(p.next.as_deref(), Some("/page/1/"));
    }

    #[test]
    fn middle_page() {
        let p = pagination(9, 49);
        assert_eq!(numbers(&p), [8, 9, 10, 11, 12]);
        assert!(p.pages[2].current);
        assert!(p.prev.is_some() && p.next.is_some());
    }

    #[test]
    fn last_page_has_no_phantom() {
        let p = pagination(48, 49);
        assert_eq!(numbers(&p), [47, 48, 49]);
        assert_eq!(p.next, None);
        assert_eq!(p.pages.last().unwrap().url, "/page/48/");
    }

    #[test]
    fn single_page() {
        let p = pagination(0, page_count(3));
        assert_eq!(numbers(&p), [1]);
        assert!(p.prev.is_none() && p.next.is_none());
    }

    #[test]
    fn page_counts() {
        assert_eq!(page_count(194), 49);
        assert_eq!(page_count(8), 2);
        assert_eq!(page_count(0), 1);
    }

    #[test]
    fn subject_is_encoded() {
        assert_eq!(mailto("[Flauschiversum] Alraune"), "mailto:ermeline@flauschiversum.de?subject=%5BFlauschiversum%5D%20Alraune");
        assert_eq!(mailto("Flauschiversum"), "mailto:ermeline@flauschiversum.de?subject=Flauschiversum");
        assert_eq!(mailto("[Flauschiversum] Mütze & Co"), "mailto:ermeline@flauschiversum.de?subject=%5BFlauschiversum%5D%20M%C3%BCtze%20%26%20Co");
    }

    #[test]
    #[should_panic(expected = "enthält nicht")]
    fn subject_without_blog_name_is_rejected() {
        mailto("Hallo");
    }
}
