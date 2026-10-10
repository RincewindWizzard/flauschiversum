//! HTML-Seiten über Askama-Templates (beim Kompilieren geprüft).

use crate::config::{BLOG_DESCRIPTION, BLOG_NAME, BLOG_URL, PAGINATION_WINDOW, POSTS_PER_PAGE};
use crate::content::{Page, Post};
use crate::media::Size;
use crate::seo;
use crate::urls;
use askama::Template;

const MONTHS: [&str; 12] = [
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
    pub logo_is_h1: bool,
    pub year: i16,
    pub livereload: bool,
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
    pub year: i16,
    pub livereload: bool,
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

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub head: Head,
    pub posts: Vec<PostCard>,
    pub pagination: Pagination,
}

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate {
    pub head: Head,
    pub title: String,
    pub thumb: Option<Thumb>,
    pub author: Option<String>,
    pub mail_subject: String,
    pub content: String,
    pub prev: Option<NavLink>,
    pub next: Option<NavLink>,
    pub has_media: bool,
    pub script_href: String,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate {
    pub head: Head,
    pub title: String,
    pub content: String,
}

/// URL der Indexseite `index` (0 = Startseite).
pub fn index_url(index: usize) -> String {
    if index == 0 { "/".to_string() } else { format!("/page/{index}/") }
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

fn mail_subject(title: &str) -> String {
    let s = format!("[{BLOG_NAME}] {title}");
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

pub fn render_index(index: usize, count: usize, cards: Vec<PostCard>, common: &Common) -> String {
    let title = if index == 0 {
        format!("{BLOG_NAME} – {BLOG_DESCRIPTION}")
    } else {
        format!("{BLOG_NAME} – Seite {}", index + 1)
    };
    let head = Head {
        og_title: title.clone(),
        title,
        description: BLOG_DESCRIPTION.to_string(),
        canonical: format!("{BLOG_URL}{}", index_url(index)),
        og_type: "website",
        og_image: None,
        published_time: None,
        json_ld: None,
        style_href: common.style_href.clone(),
        logo_is_h1: true,
        year: common.year,
        livereload: common.livereload,
    };
    IndexTemplate { head, posts: cards, pagination: pagination(index, count) }.render().expect("Index-Template")
}

pub struct PostPageInput<'a> {
    pub post: &'a Post,
    pub content: String,
    pub thumb: Option<Thumb>,
    pub og_image: Option<(String, Size)>,
    pub newer: Option<&'a Post>,
    pub older: Option<&'a Post>,
    pub has_media: bool,
}

pub fn render_post(input: PostPageInput, common: &Common) -> String {
    let post = input.post;
    let image_url = input.og_image.as_ref().map(|(u, _)| u.as_str());
    let head = Head {
        title: format!("{} – {BLOG_NAME}", post.title),
        og_title: post.title.clone(),
        description: seo::meta_description(post.excerpt.as_deref().unwrap_or(BLOG_DESCRIPTION)),
        canonical: urls::absolute(&post.url),
        og_type: "article",
        og_image: input.og_image.as_ref().map(|(url, size)| OgImage {
            url: url.clone(),
            width: size.width,
            height: size.height,
            alt: post.title.clone(),
        }),
        published_time: Some(seo::iso(post)),
        json_ld: Some(seo::blog_posting(post, image_url)),
        style_href: common.style_href.clone(),
        logo_is_h1: false,
        year: common.year,
        livereload: common.livereload,
    };
    let link = |p: &Post| NavLink { url: urls::encode_path(&p.url), title: p.title.clone() };
    PostTemplate {
        head,
        title: post.title.clone(),
        thumb: input.thumb,
        author: post.author.clone(),
        mail_subject: mail_subject(&post.title),
        content: input.content,
        prev: input.older.map(link),
        next: input.newer.map(link),
        has_media: input.has_media,
        script_href: common.script_href.clone(),
    }
    .render()
    .expect("Post-Template")
}

pub fn render_page(page: &Page, content: String, description: String, common: &Common) -> String {
    let head = Head {
        title: format!("{} – {BLOG_NAME}", page.title),
        og_title: page.title.clone(),
        description: seo::meta_description(&description),
        canonical: urls::absolute(&page.url),
        og_type: "website",
        og_image: None,
        published_time: None,
        json_ld: None,
        style_href: common.style_href.clone(),
        logo_is_h1: false,
        year: common.year,
        livereload: common.livereload,
    };
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
        assert_eq!(mail_subject("Alraune"), "%5BFlauschiversum%5D%20Alraune");
    }
}
