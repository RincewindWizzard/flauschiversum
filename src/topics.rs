//! Themen: Zuordnung der Posts zu Schlagworten (es gibt keine Kategorien) und ähnliche Beiträge.

use crate::content::{Post, Site};
use crate::diag::Diagnostics;
use crate::urls;
use std::collections::HashMap;

/// Anzahl ähnlicher Beiträge unter einem Beitrag.
pub const RELATED_MAX: usize = 6;

/// Ein ähnlicher Beitrag mit den gemeinsamen Schlagworten 
pub struct Related<'a> {
    pub post: &'a Post,
    /// Gemeinsame Schlagworte (die Anzeige nutzt sie nicht, die Tests prüfen damit die Auswahl).
    #[cfg_attr(not(test), allow(dead_code))]
    pub shared: Vec<&'a str>,
}

/// Anzahl der Posts je Schlagwort.
pub fn tag_counts(posts: &[Post]) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for t in posts.iter().flat_map(|p| &p.tags) {
        *counts.entry(t.as_str()).or_insert(0) += 1;
    }
    counts
}

/// Ähnliche Beiträge zu `posts[index]` (`posts` = veröffentlichte Posts, neueste zuerst): alle Beiträge mit mindestens einem
/// gemeinsamen Schlagwort, höchstens `RELATED_MAX`. Wert = Summe von 1/Anzahl der Beiträge mit dem Schlagwort über die
/// gemeinsamen Schlagworte (seltene zählen mehr). Reihenfolge: Wert absteigend, Datum absteigend, URL aufsteigend.
pub fn related<'a>(posts: &'a [Post], index: usize, counts: &HashMap<&str, usize>) -> Vec<Related<'a>> {
    let me = &posts[index];
    let mut scored: Vec<(f64, Related<'a>)> = posts
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .filter_map(|(_, q)| {
            let shared: Vec<&str> = me.tags.iter().filter(|t| q.tags.contains(t)).map(String::as_str).collect();
            if shared.is_empty() {
                return None;
            }
            let score: f64 = shared.iter().map(|t| 1.0 / counts.get(t).copied().unwrap_or(1) as f64).sum();
            Some((score, Related { post: q, shared }))
        })
        .collect();
    scored.sort_by(|a, b| {
        b.0.total_cmp(&a.0).then_with(|| b.1.post.date.cmp(&a.1.post.date)).then_with(|| a.1.post.url.cmp(&b.1.post.url))
    });
    scored.into_iter().map(|(_, r)| r).take(RELATED_MAX).collect()
}

/// Ein Schlagwort mit seinen veröffentlichten Posts (neueste zuerst).
pub struct TagTopic<'a> {
    pub name: &'a str,
    pub slug: String,
    pub posts: Vec<&'a Post>,
}

pub struct Topics<'a> {
    /// Alphabetisch (deutsche Sortierung); Schlagworte ohne Post entfallen.
    pub tags: Vec<TagTopic<'a>>,
}

pub fn tag_url(slug: &str) -> String {
    format!("/tag/{slug}/")
}

/// Sortierschlüssel nach deutscher Alltagsregel: Umlaute wie ihre Grundbuchstaben, ohne Groß-/Kleinschreibung.
pub fn sort_key(text: &str) -> String {
    text.chars()
        .flat_map(char::to_lowercase)
        .flat_map(|c| match c {
            'ä' => vec!['a'],
            'ö' => vec!['o'],
            'ü' => vec!['u'],
            'ß' => vec!['s', 's'],
            c => vec![c],
        })
        .collect()
}

/// Größenstufe 1–5 der Tag-Cloud: logarithmisch zwischen dem seltensten (1) und häufigsten (5) Schlagwort,
/// bei gleicher Anzahl überall die mittlere Stufe 3.
pub fn cloud_level(count: usize, min: usize, max: usize) -> u8 {
    if max <= min {
        return 3;
    }
    let ln = |n: usize| (n.max(1) as f64).ln();
    let level = 1.0 + 4.0 * (ln(count) - ln(min)) / (ln(max) - ln(min));
    level.round().clamp(1.0, 5.0) as u8
}

impl<'a> Topics<'a> {
    /// Schlagworte der Tag-Cloud (alphabetisch) mit Größenstufe.
    pub fn cloud(&self) -> Vec<(&TagTopic<'a>, u8)> {
        let counts = self.tags.iter().map(|t| t.posts.len());
        let (min, max) = (counts.clone().min().unwrap_or(0), counts.max().unwrap_or(0));
        self.tags.iter().map(|t| (t, cloud_level(t.posts.len(), min, max))).collect()
    }

    pub fn build(site: &'a Site, diag: &Diagnostics) -> Self {
        let mut names: Vec<&String> = site.tags.iter().collect();
        names.sort_by_key(|n| (sort_key(n), n.to_string()));
        let mut slugs: HashMap<String, &str> = HashMap::new();
        let mut tags = Vec::new();
        for name in names {
            let slug = urls::slugify(name);
            if slug.is_empty() {
                diag.error("content/tags.yaml", format!("Schlagwort `{name}` ergibt keine gültige Adresse"));
                continue;
            }
            if let Some(other) = slugs.insert(slug.clone(), name) {
                diag.error("content/tags.yaml", format!("Schlagworte `{other}` und `{name}` ergeben dieselbe Adresse `{}`", tag_url(&slug)));
                continue;
            }
            let posts: Vec<&Post> = site.posts.iter().filter(|p| p.tags.contains(name)).collect();
            if !posts.is_empty() {
                tags.push(TagTopic { name, slug, posts });
            }
        }
        Topics { tags }
    }

    #[cfg(test)]
    pub fn tag(&self, name: &str) -> Option<&TagTopic<'a>> {
        self.tags.iter().find(|t| t.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{self, Site};
    use std::fs;
    use std::path::Path;

    fn site(root: &Path, posts: &[(&str, &str)], tags: &str) -> Site {
        let c = root.join("content");
        fs::create_dir_all(&c).unwrap();
        fs::write(c.join("tags.yaml"), tags).unwrap();
        for (dir, fm) in posts {
            let d = c.join("posts").join(dir);
            fs::create_dir_all(&d).unwrap();
            fs::write(d.join("index.md"), format!("---\ntitle: {dir}\n{fm}---\nText\n")).unwrap();
        }
        let now = "2026-10-10T12:00[Europe/Berlin]".parse().unwrap();
        content::load(root, &now, &Diagnostics::default())
    }

    #[test]
    fn groups_counts_and_sorts() {
        let root = tempfile::tempdir().unwrap();
        let s = site(
            root.path(),
            &[
                ("a", "date: 2020-01-01\ntags: [Katzen, Hüte]\n"),
                ("b", "date: 2021-01-01\ntags: [Katzen]\n"),
                ("c", "date: 2022-01-01\ntags: [Taschen, Katzen]\n"),
                ("geplant", "date: 2099-01-01\ntags: [Taschen]\n"),
                ("ohne", "date: 2023-01-01\n"),
            ],
            "- name: Taschen\n- name: Katzen\n- name: Hüte\n- name: Leer\n",
        );
        let diag = Diagnostics::default();
        let t = Topics::build(&s, &diag);
        assert!(!diag.has_errors());
        // Schlagworte alphabetisch mit deutscher Sortierung, leere entfallen, geplanter Post zählt nicht
        let names: Vec<_> = t.tags.iter().map(|x| (x.name, x.posts.len())).collect();
        assert_eq!(names, [("Hüte", 1), ("Katzen", 3), ("Taschen", 1)]);
        // Posts neueste zuerst
        let titles: Vec<_> = t.tag("Katzen").unwrap().posts.iter().map(|p| p.title.as_str()).collect();
        assert_eq!(titles, ["c", "b", "a"]);
    }

    #[test]
    fn tag_with_umlaut_gets_ascii_address() {
        let root = tempfile::tempdir().unwrap();
        let s = site(root.path(), &[("a", "date: 2020-01-01\ntags: [Nähen]\n")], "- name: Nähen\n");
        let t = Topics::build(&s, &Diagnostics::default());
        assert_eq!(t.tags[0].slug, "naehen");
        assert_eq!(tag_url(&t.tags[0].slug), "/tag/naehen/");
    }

    #[test]
    fn colliding_tag_slugs_are_an_error() {
        let root = tempfile::tempdir().unwrap();
        let s = site(root.path(), &[("a", "date: 2020-01-01\ntags: [Hüte]\n")], "- name: Hüte\n- name: Huete\n");
        let diag = Diagnostics::default();
        Topics::build(&s, &diag);
        let m = diag.all();
        assert_eq!(m.len(), 1);
        assert!(m[0].message.contains("`Huete`") && m[0].message.contains("`Hüte`") && m[0].message.contains("/tag/huete/"), "{}", m[0].message);
    }

    #[test]
    fn cloud_levels_are_logarithmic_between_min_and_max() {
        assert_eq!(cloud_level(1, 1, 21), 1);
        assert_eq!(cloud_level(21, 1, 21), 5);
        assert_eq!(cloud_level(9, 1, 21), 4);
        assert_eq!(cloud_level(3, 1, 21), 2);
        assert!(cloud_level(21, 1, 21) > cloud_level(9, 1, 21));
        // alle gleich häufig oder nur ein Schlagwort: mittlere Stufe, keine Division durch null
        assert_eq!(cloud_level(5, 5, 5), 3);
        assert_eq!(cloud_level(0, 0, 0), 3);
    }

    #[test]
    fn cloud_is_alphabetical_with_levels_and_ignores_scheduled() {
        let root = tempfile::tempdir().unwrap();
        let s = site(
            root.path(),
            &[
                ("a", "date: 2020-01-01\ntags: [Taschen, Katzen, Hüte]\n"),
                ("b", "date: 2021-01-01\ntags: [Katzen, Hüte]\n"),
                ("c", "date: 2022-01-01\ntags: [Katzen]\n"),
                ("geplant", "date: 2099-01-01\ntags: [Leer]\n"),
            ],
            "- name: Taschen\n- name: Katzen\n- name: Hüte\n- name: Leer\n",
        );
        let t = Topics::build(&s, &Diagnostics::default());
        let cloud: Vec<_> = t.cloud().iter().map(|(t, l)| (t.name, *l)).collect();
        assert_eq!(cloud, [("Hüte", 4), ("Katzen", 5), ("Taschen", 1)]);
    }

    #[test]
    fn umlauts_sort_like_their_base_letters() {
        let mut v = vec!["Taschen", "Äpfel", "Hüte", "Zebra", "Handyhüllen", "Häkeln"];
        v.sort_by_key(|x| sort_key(x));
        assert_eq!(v, ["Äpfel", "Häkeln", "Handyhüllen", "Hüte", "Taschen", "Zebra"]);
    }

    fn titles<'a>(r: &[Related<'a>]) -> Vec<&'a str> {
        r.iter().map(|x| x.post.title.as_str()).collect()
    }

    fn related_of(s: &Site, title: &str) -> Vec<(String, Vec<String>)> {
        let counts = tag_counts(&s.posts);
        let i = s.posts.iter().position(|p| p.title == title).unwrap();
        related(&s.posts, i, &counts).into_iter().map(|r| (r.post.title.clone(), r.shared.iter().map(|t| t.to_string()).collect())).collect()
    }

    #[test]
    fn rare_shared_tag_counts_more_than_common_one() {
        let root = tempfile::tempdir().unwrap();
        let mut posts = vec![
            ("A", "date: 2020-01-01\ntags: [Katzen, Hüte]\n".to_string()),
            ("B", "date: 2020-01-02\ntags: [Katzen]\n".to_string()),
            ("C", "date: 2020-01-03\ntags: [Hüte]\n".to_string()),
        ];
        for i in 0..5 {
            posts.push((["D", "E", "F", "G", "H"][i], format!("date: 2020-02-0{}\ntags: [Katzen]\n", i + 1)));
        }
        let refs: Vec<(&str, &str)> = posts.iter().map(|(a, b)| (*a, b.as_str())).collect();
        let s = site(root.path(), &refs, "- name: Katzen\n- name: Hüte\n");
        let r = related_of(&s, "A");
        // „Hüte“ (2 Beiträge) wiegt mehr als „Katzen“ (7 Beiträge): C vor allen Katzen-Beiträgen, die danach nach Datum folgen
        assert_eq!(r[0].0, "C");
        assert_eq!(r[0].1, ["Hüte"]);
        assert_eq!(r[1].0, "H", "gleicher Wert: neuester zuerst");
        assert_eq!(r[1].1, ["Katzen"]);
        assert_eq!(r.len(), 6, "höchstens sechs: C und die fünf neuesten Katzen-Beiträge");
        assert!(r.iter().all(|(t, _)| t != "B"), "B ist der älteste und fällt heraus");
        assert!(r.iter().all(|(t, _)| t != "A"), "nicht der Beitrag selbst");
    }

    #[test]
    fn ties_are_broken_by_date_then_url() {
        let root = tempfile::tempdir().unwrap();
        let s = site(
            root.path(),
            &[
                ("a", "date: 2020-01-01\ntags: [Katzen]\n"),
                ("alt", "date: 2019-01-01\ntags: [Katzen]\n"),
                ("neu-b", "date: 2021-05-05\ntags: [Katzen]\n"),
                ("neu-a", "date: 2021-05-05\ntags: [Katzen]\n"),
            ],
            "- name: Katzen\n",
        );
        let counts = tag_counts(&s.posts);
        let i = s.posts.iter().position(|p| p.title == "a").unwrap();
        assert_eq!(titles(&related(&s.posts, i, &counts)), ["neu-a", "neu-b", "alt"]);
        // zweimal berechnet: gleiche Reihenfolge
        assert_eq!(titles(&related(&s.posts, i, &counts)), ["neu-a", "neu-b", "alt"]);
    }

    #[test]
    fn only_shared_tags_count_and_scheduled_posts_are_excluded() {
        let root = tempfile::tempdir().unwrap();
        let s = site(
            root.path(),
            &[
                ("ohne", "date: 2020-01-01\n"),
                ("w1", "date: 2020-03-01\ntags: [Katzen]\n"),
                ("w2", "date: 2020-02-01\ntags: [Hüte]\n"),
                ("fremd", "date: 2020-04-01\ntags: [Katzen]\n"),
                ("geplant", "date: 2099-01-01\ntags: [Katzen]\n"),
            ],
            "- name: Katzen\n- name: Hüte\n",
        );
        // ohne Schlagworte keine ähnlichen Beiträge, auch wenn es andere Beiträge gibt
        assert!(related_of(&s, "ohne").is_empty());
        // gemeinsames Schlagwort: nur w1 und fremd, der geplante Beitrag fehlt, w2 teilt nichts
        let r = related_of(&s, "w1");
        assert_eq!(r.iter().map(|(t, _)| t.as_str()).collect::<Vec<_>>(), ["fremd"]);
        assert_eq!(r[0].1, ["Katzen"]);
    }

    #[test]
    fn lonely_post_has_no_related_posts() {
        let root = tempfile::tempdir().unwrap();
        let s = site(root.path(), &[("einzig", "date: 2020-01-01\ntags: [Hüte]\n"), ("anders", "date: 2020-01-02\ntags: [Katzen]\n")], "- name: Katzen\n- name: Hüte\n");
        assert!(related_of(&s, "einzig").is_empty());
    }
}
