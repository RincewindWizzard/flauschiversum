//! Themen: Zuordnung der Posts zu Kategorien und Schlagworten.

use crate::content::{Category, Post, Site};
use crate::diag::Diagnostics;
use crate::urls;
use std::collections::HashMap;

/// Eine Kategorie mit ihren veröffentlichten Posts (neueste zuerst).
pub struct CategoryTopic<'a> {
    pub category: &'a Category,
    /// Adressteil der Seite (`/kategorie/<slug>/`).
    pub slug: String,
    pub posts: Vec<&'a Post>,
}

/// Ein Schlagwort mit seinen veröffentlichten Posts (neueste zuerst).
pub struct TagTopic<'a> {
    pub name: &'a str,
    pub slug: String,
    pub posts: Vec<&'a Post>,
}

pub struct Topics<'a> {
    /// In der Reihenfolge von `categories.yaml`; Kategorien ohne Post entfallen.
    pub categories: Vec<CategoryTopic<'a>>,
    /// Alphabetisch (deutsche Sortierung); Schlagworte ohne Post entfallen.
    pub tags: Vec<TagTopic<'a>>,
}

pub fn category_url(slug: &str) -> String {
    format!("/kategorie/{slug}/")
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
        let categories = site
            .categories
            .iter()
            .map(|category| CategoryTopic {
                category,
                slug: urls::slugify(&category.id),
                posts: site.posts.iter().filter(|p| p.category.as_deref() == Some(category.id.as_str())).collect(),
            })
            .filter(|c| !c.posts.is_empty())
            .collect();

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
        Topics { categories, tags }
    }

    #[cfg(test)]
    pub fn category(&self, id: &str) -> Option<&CategoryTopic<'a>> {
        self.categories.iter().find(|c| c.category.id == id)
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
        fs::create_dir_all(c.join("static/images")).unwrap();
        fs::write(c.join("static/images/f.jpg"), b"x").unwrap();
        fs::write(
            c.join("categories.yaml"),
            "- {id: filzen, name: Filzen, image: /images/f.jpg, motto: a}\n- {id: wolle, name: Wolle, image: /images/f.jpg, motto: b}\n- {id: nähen, name: Nähen, image: /images/f.jpg, motto: c}\n",
        )
        .unwrap();
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
                ("a", "date: 2020-01-01\ncategory: filzen\ntags: [Katzen, Hüte]\n"),
                ("b", "date: 2021-01-01\ncategory: filzen\ntags: [Katzen]\n"),
                ("c", "date: 2022-01-01\ntags: [Taschen, Katzen]\n"),
                ("geplant", "date: 2099-01-01\ncategory: nähen\ntags: [Taschen]\n"),
            ],
            "- name: Taschen\n- name: Katzen\n- name: Hüte\n- name: Leer\n",
        );
        let diag = Diagnostics::default();
        let t = Topics::build(&s, &diag);
        assert!(!diag.has_errors());
        // Kategorien: Reihenfolge der Liste, leere (malen: nur geplant) entfallen
        let ids: Vec<_> = t.categories.iter().map(|c| c.category.id.as_str()).collect();
        assert_eq!(ids, ["filzen"]);
        assert!(t.category("wolle").is_none());
        assert!(t.category("nähen").is_none(), "nur geplanter Beitrag");
        assert_eq!(t.category("filzen").unwrap().slug, "filzen");
        // Posts neueste zuerst
        let titles: Vec<_> = t.category("filzen").unwrap().posts.iter().map(|p| p.title.as_str()).collect();
        assert_eq!(titles, ["b", "a"]);
        // Schlagworte alphabetisch mit deutscher Sortierung, leere entfallen, geplanter Post zählt nicht
        let names: Vec<_> = t.tags.iter().map(|x| (x.name, x.posts.len())).collect();
        assert_eq!(names, [("Hüte", 1), ("Katzen", 3), ("Taschen", 1)]);
        // Post ohne Kategorie steht nur bei den Schlagworten
        assert!(t.tag("Taschen").unwrap().posts.iter().any(|p| p.title == "c"));
    }

    #[test]
    fn category_with_umlaut_gets_ascii_address() {
        let root = tempfile::tempdir().unwrap();
        let s = site(root.path(), &[("a", "date: 2020-01-01\ncategory: nähen\n")], "- name: Katzen\n");
        let t = Topics::build(&s, &Diagnostics::default());
        assert_eq!(t.categories[0].slug, "naehen");
        assert_eq!(category_url(&t.categories[0].slug), "/kategorie/naehen/");
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
}
