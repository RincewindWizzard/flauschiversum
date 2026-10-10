//! Markdown → HTML mit Erkennung von Mediengruppen (Einzelbild, Karussell, Video).

use crate::content::MediaKind;
use crate::media::MediaInfo;
use crate::urls;
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};
use std::collections::HashMap;
use std::fmt::Write;

/// Ein Medium, wie es im Markdown referenziert wird.
struct MediaRef {
    file_name: String,
    alt: String,
}

/// Kontext für das Rendern eines Posts.
pub struct PostContext<'a> {
    /// Post-URL (unkodiert), Basis für Medien-URLs.
    pub url: &'a str,
    /// Medien des Posts nach Dateiname.
    pub media: &'a HashMap<String, MediaInfo>,
    /// URLs absolut (für den Feed) statt relativ ausgeben.
    pub absolute: bool,
}

/// Ergebnis des Renderns.
pub struct Rendered {
    pub html: String,
    /// Referenzen auf nicht vorhandene Medien.
    pub missing: Vec<String>,
}

fn options() -> Options {
    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_FOOTNOTES
}

/// Python-Markdown akzeptierte Leerzeichen in Bildzielen (`![x](a b.jpg)`); CommonMark
/// verlangt dafür `<a b.jpg>`. Solche Ziele werden vor dem Parsen umgeschrieben.
fn normalize_image_destinations(md: &str) -> String {
    let mut out = String::with_capacity(md.len());
    let mut rest = md;
    while let Some(start) = rest.find("![") {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let Some(close) = tail.find("](") else {
            out.push_str(tail);
            return out;
        };
        let after = &tail[close + 2..];
        match after.find(')') {
            Some(end) if !tail[..close].contains('\n') => {
                let dest = &after[..end];
                if dest.contains(' ') && !dest.contains('"') && !dest.contains('<') && !dest.contains('\n') {
                    out.push_str(&tail[..close + 2]);
                    out.push('<');
                    out.push_str(dest);
                    out.push('>');
                } else {
                    out.push_str(&tail[..close + 2 + end]);
                }
                out.push(')');
                rest = &after[end + 1..];
            }
            _ => {
                out.push_str("![");
                rest = &tail[2..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// Python-Markdown erkannte `##Titel` (ohne Leerzeichen) als Überschrift, CommonMark nicht.
/// Außerhalb von Codeblöcken wird deshalb ein Leerzeichen ergänzt.
fn normalize_atx_headings(md: &str) -> String {
    let mut out = String::with_capacity(md.len() + 16);
    let mut in_fence = false;
    for line in md.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
        }
        let hashes = line.bytes().take_while(|b| *b == b'#').count();
        if !in_fence && (1..=6).contains(&hashes) {
            let rest = &line[hashes..];
            if rest.chars().next().is_some_and(|c| !c.is_whitespace() && c != '#') {
                out.push_str(&line[..hashes]);
                out.push(' ');
                out.push_str(rest);
                continue;
            }
        }
        out.push_str(line);
    }
    out
}

fn normalize(md: &str) -> String {
    normalize_atx_headings(&normalize_image_destinations(md))
}

/// Rendert Markdown einer Einzelseite (ohne Medienbehandlung).
pub fn render_page(md: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new_ext(&normalize(md), options()));
    out
}

fn is_local(dest: &str) -> bool {
    !dest.contains(':') && !dest.starts_with('/') && !dest.starts_with('#')
}

/// Rendert Markdown eines Posts. Bilder eines Absatzes werden zu einer Mediengruppe.
pub fn render_post(md: &str, ctx: &PostContext) -> Rendered {
    let normalized = normalize(md);
    let events: Vec<Event> = Parser::new_ext(&normalized, options()).collect();
    let mut out = String::new();
    let mut missing = Vec::new();
    let mut media_count = 0usize;
    let mut i = 0;
    while i < events.len() {
        if !matches!(events[i], Event::Start(Tag::Paragraph)) {
            // Außerhalb von Absätzen unverändert ausgeben (bis zum nächsten Absatz)
            let start = i;
            while i < events.len() && !matches!(events[i], Event::Start(Tag::Paragraph)) {
                i += 1;
            }
            html::push_html(&mut out, events[start..i].iter().cloned());
            continue;
        }
        // Absatz puffern
        let start = i;
        let mut depth = 0;
        loop {
            match &events[i] {
                Event::Start(Tag::Paragraph) => depth += 1,
                Event::End(TagEnd::Paragraph) => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        let paragraph = &events[start + 1..i];
        i += 1;

        let mut text: Vec<Event> = Vec::new();
        let mut group: Vec<MediaRef> = Vec::new();
        let mut j = 0;
        while j < paragraph.len() {
            if let Event::Start(Tag::Image { dest_url, .. }) = &paragraph[j] {
                let dest = urls::decode_path(dest_url);
                let mut alt = String::new();
                let mut k = j + 1;
                while k < paragraph.len() && !matches!(paragraph[k], Event::End(TagEnd::Image)) {
                    if let Event::Text(t) | Event::Code(t) = &paragraph[k] {
                        alt.push_str(t);
                    }
                    k += 1;
                }
                if is_local(&dest) && ctx.media.contains_key(&dest) {
                    group.push(MediaRef { file_name: dest, alt });
                    j = k + 1;
                    continue;
                }
                if is_local(&dest) {
                    missing.push(dest);
                }
            }
            text.push(paragraph[j].clone());
            j += 1;
        }

        let has_text = text.iter().any(|e| match e {
            Event::Text(t) => !t.trim().is_empty(),
            Event::SoftBreak | Event::HardBreak => false,
            Event::Html(h) | Event::InlineHtml(h) => !h.trim().is_empty() && !is_break(h),
            _ => true,
        });
        if has_text {
            let trimmed = trim_breaks(text);
            html::push_html(
                &mut out,
                std::iter::once(Event::Start(Tag::Paragraph))
                    .chain(trimmed)
                    .chain(std::iter::once(Event::End(TagEnd::Paragraph))),
            );
            out.push('\n');
        }
        if !group.is_empty() {
            write_group(&mut out, &group, ctx, &mut media_count);
        }
    }
    Rendered { html: out, missing }
}

fn is_break(h: &CowStr) -> bool {
    let h = h.trim().to_ascii_lowercase();
    h == "<br>" || h == "<br/>" || h == "<br />" || h == "</br>"
}

/// Entfernt Zeilenumbrüche am Anfang und Ende eines Textabsatzes.
fn trim_breaks(mut events: Vec<Event>) -> Vec<Event> {
    let is_ws = |e: &Event| match e {
        Event::SoftBreak | Event::HardBreak => true,
        Event::Text(t) => t.trim().is_empty(),
        _ => false,
    };
    while events.first().is_some_and(is_ws) {
        events.remove(0);
    }
    while events.last().is_some_and(is_ws) {
        events.pop();
    }
    events
}

fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    pulldown_cmark_escape::escape_html(&mut out, s).unwrap();
    out
}

fn media_url(ctx: &PostContext, path: &str) -> String {
    if ctx.absolute {
        urls::absolute(&format!("{}{}", ctx.url, path))
    } else {
        urls::encode_path(path)
    }
}

/// Maximale Anzeigehöhe eines Bildes im Post (Hochformate werden schmaler dargestellt).
const MAX_DISPLAY_HEIGHT: u32 = 560;

/// Anzeigegröße für `width`/`height`: höchstens `MAX_DISPLAY_HEIGHT` hoch, Seitenverhältnis bleibt.
fn display_size(width: u32, height: u32) -> (u32, u32) {
    if height <= MAX_DISPLAY_HEIGHT {
        (width, height)
    } else {
        (((width as u64 * MAX_DISPLAY_HEIGHT as u64 + height as u64 / 2) / height as u64) as u32, MAX_DISPLAY_HEIGHT)
    }
}

fn write_figure(out: &mut String, m: &MediaRef, info: &MediaInfo, ctx: &PostContext, count: &mut usize) {
    let alt = esc(&m.alt);
    let caption = if m.alt.trim().is_empty() { String::new() } else { format!("<figcaption>{alt}</figcaption>") };
    match info.kind {
        MediaKind::Image => {
            let (full, preview) = (info.full.unwrap(), info.preview.unwrap());
            let src = media_url(ctx, &format!("700x/{}", m.file_name));
            let src_full = media_url(ctx, &format!("1000x/{}", m.file_name));
            let srcset = if full.width > preview.width {
                format!(" srcset=\"{src} {}w, {src_full} {}w\" sizes=\"(min-width: 760px) 700px, 100vw\"", preview.width, full.width)
            } else {
                String::new()
            };
            let loading = if *count == 0 { "" } else { " loading=\"lazy\"" };
            let (w, h) = display_size(preview.width, preview.height);
            let _ = write!(
                out,
                "<figure class=\"media\"><a href=\"{src_full}\" data-pswp-width=\"{}\" data-pswp-height=\"{}\">\
                 <img src=\"{src}\"{srcset} width=\"{w}\" height=\"{h}\" alt=\"{alt}\"{loading} decoding=\"async\"></a>{caption}</figure>",
                full.width, full.height
            );
        }
        MediaKind::Animation => {
            let src = media_url(ctx, &m.file_name);
            let size = info.original.map(|s| format!(" width=\"{}\" height=\"{}\"", s.width, s.height)).unwrap_or_default();
            let loading = if *count == 0 { "" } else { " loading=\"lazy\"" };
            let _ = write!(out, "<figure class=\"media\"><img src=\"{src}\"{size} alt=\"{alt}\"{loading}>{caption}</figure>");
        }
        MediaKind::Video => {
            let src = media_url(ctx, &m.file_name);
            let label = if m.alt.is_empty() { String::new() } else { format!(" aria-label=\"{alt}\"") };
            let _ = write!(
                out,
                "<figure class=\"media media-video\"><video src=\"{src}\" autoplay muted loop playsinline preload=\"metadata\"{label}></video>{caption}</figure>"
            );
        }
    }
    *count += 1;
}

fn write_group(out: &mut String, group: &[MediaRef], ctx: &PostContext, count: &mut usize) {
    if group.len() == 1 {
        let m = &group[0];
        write_figure(out, m, &ctx.media[&m.file_name], ctx, count);
        out.push('\n');
        return;
    }
    let _ = write!(out, "<div class=\"carousel\" data-count=\"{}\"><ul class=\"carousel-track\">", group.len());
    for (n, m) in group.iter().enumerate() {
        let _ = write!(out, "<li class=\"carousel-slide\" aria-label=\"Bild {} von {}\">", n + 1, group.len());
        write_figure(out, m, &ctx.media[&m.file_name], ctx, count);
        out.push_str("</li>");
    }
    out.push_str("</ul></div>\n");
}

/// Reiner Text aus Markdown (für Meta-Beschreibungen von Einzelseiten).
pub fn first_paragraph_text(md: &str) -> String {
    let mut text = String::new();
    let mut in_para = false;
    for e in Parser::new_ext(md, options()) {
        match e {
            Event::Start(Tag::Paragraph) => in_para = true,
            Event::End(TagEnd::Paragraph) if in_para => {
                if !text.trim().is_empty() {
                    break;
                }
                in_para = false;
            }
            Event::Text(t) | Event::Code(t) if in_para => text.push_str(&t),
            Event::SoftBreak | Event::HardBreak if in_para => text.push(' '),
            _ => {}
        }
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::Size;

    fn image(w: u32, h: u32) -> MediaInfo {
        MediaInfo {
            kind: MediaKind::Image,
            full: Some(Size { width: w.min(1000), height: h }),
            preview: Some(Size { width: w.min(700), height: h }),
            thumb: Some(Size { width: 200, height: h }),
            original: None,
            preview_len: 1000,
        }
    }

    fn media() -> HashMap<String, MediaInfo> {
        let mut m = HashMap::new();
        for n in ["a.jpg", "b.jpg", "c.jpg", "Schwacher Moment.jpg"] {
            m.insert(n.to_string(), image(1000, 750));
        }
        m.insert(
            "clip.mp4".into(),
            MediaInfo { kind: MediaKind::Video, full: None, preview: None, thumb: None, original: None, preview_len: 0 },
        );
        m.insert(
            "anim.gif".into(),
            MediaInfo {
                kind: MediaKind::Animation,
                full: None,
                preview: None,
                thumb: None,
                original: Some(Size { width: 320, height: 240 }),
                preview_len: 0,
            },
        );
        m
    }

    fn render(md: &str) -> Rendered {
        let media = media();
        render_post(md, &PostContext { url: "/2014/01/alraune/", media: &media, absolute: false })
    }

    #[test]
    fn single_image_becomes_figure() {
        let r = render("Text davor.\n\n![Alraune oben](a.jpg)\n\nText danach.");
        assert!(r.html.contains("<p>Text davor.</p>"));
        assert!(r.html.contains("<figure class=\"media\"><a href=\"1000x/a.jpg\""));
        assert!(r.html.contains("<img src=\"700x/a.jpg\" srcset=\"700x/a.jpg 700w, 1000x/a.jpg 1000w\""));
        assert!(r.html.contains("<figcaption>Alraune oben</figcaption>"));
        assert!(!r.html.contains("carousel"));
        assert!(!r.html.contains("loading=\"lazy\""), "erstes Bild lädt sofort");
    }

    #[test]
    fn consecutive_images_become_carousel() {
        let r = render("![Eins](a.jpg)\n![Zwei](b.jpg)\n![Drei](c.jpg)\n");
        assert!(r.html.contains("<div class=\"carousel\" data-count=\"3\">"));
        assert_eq!(r.html.matches("<li class=\"carousel-slide\"").count(), 3);
        assert_eq!(r.html.matches("loading=\"lazy\"").count(), 2);
        assert!(!r.html.contains("<p>"), "{}", r.html);
    }

    #[test]
    fn text_before_images_stays_paragraph() {
        let r = render("Hier ein Text\n![Eins](a.jpg)\n![Zwei](b.jpg)");
        let p = r.html.find("<p>Hier ein Text</p>").expect(&r.html);
        let c = r.html.find("<div class=\"carousel\"").unwrap();
        assert!(p < c);
    }

    #[test]
    fn blank_line_separates_groups() {
        let r = render("![Eins](a.jpg)\n\n![Zwei](b.jpg)");
        assert_eq!(r.html.matches("<figure").count(), 2);
        assert!(!r.html.contains("carousel"));
    }

    #[test]
    fn video_and_animation() {
        let r = render("![Karteikartenfresser in Aktion](clip.mp4)\n\n![Animiert](anim.gif)");
        assert!(r.html.contains(
            "<video src=\"clip.mp4\" autoplay muted loop playsinline preload=\"metadata\" aria-label=\"Karteikartenfresser in Aktion\"></video>"
        ));
        assert!(r.html.contains("<img src=\"anim.gif\" width=\"320\" height=\"240\" alt=\"Animiert\""));
    }

    #[test]
    fn video_inside_group_is_a_slide() {
        let r = render("![Eins](a.jpg)\n![Clip](clip.mp4)\n![Zwei](b.jpg)");
        assert!(r.html.contains("data-count=\"3\""));
        assert!(r.html.contains("<li class=\"carousel-slide\" aria-label=\"Bild 2 von 3\"><figure class=\"media media-video\">"));
    }

    #[test]
    fn spaces_in_destination_are_accepted_and_encoded() {
        let r = render("![Mein schwacher Moment](Schwacher Moment.jpg)");
        assert!(r.html.contains("<img src=\"700x/Schwacher%20Moment.jpg\""), "{}", r.html);
    }

    #[test]
    fn unknown_reference_is_reported() {
        let r = render("![Foto](DSCF0572.jpg)");
        assert_eq!(r.missing, ["DSCF0572.jpg"]);
    }

    #[test]
    fn empty_alt_has_no_caption() {
        let r = render("![](a.jpg)");
        assert!(!r.html.contains("figcaption"));
    }

    #[test]
    fn absolute_urls_for_feed() {
        let media = media();
        let r = render_post("![X](a.jpg)", &PostContext { url: "/2014/01/alraune/", media: &media, absolute: true });
        assert!(r.html.contains("src=\"https://flauschiversum.de/2014/01/alraune/700x/a.jpg\""));
    }

    #[test]
    fn raw_html_is_kept() {
        assert!(render_page("Nadja Bender<br/>\nZeile").contains("<br/>"));
    }

    #[test]
    fn headings_without_space() {
        let html = render_page("##Kopf\nText\n\n```\n#nicht\n```\n");
        assert!(html.contains("<h2>Kopf</h2>"), "{html}");
        assert!(html.contains("#nicht"));
    }

    #[test]
    fn portrait_images_are_limited_in_height() {
        assert_eq!(display_size(700, 525), (700, 525));
        assert_eq!(display_size(562, 750), (420, 560));
    }

    #[test]
    fn first_paragraph() {
        assert_eq!(first_paragraph_text("# Titel\n\nErster *Absatz*\nmit Umbruch.\n\nZweiter."), "Erster Absatz mit Umbruch.");
    }
}
