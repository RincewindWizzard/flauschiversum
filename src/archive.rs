//! Archiv der Beiträge nach Jahr und Monat.

use crate::content::Post;
use crate::render::MONTHS;
use crate::urls;

pub struct ArchivePost {
    pub url: String,
    pub title: String,
    pub day: i8,
}

pub struct ArchiveMonth {
    pub name: &'static str,
    pub count: usize,
    pub open: bool,
    pub posts: Vec<ArchivePost>,
}

pub struct ArchiveYear {
    pub year: i16,
    pub count: usize,
    pub open: bool,
    pub months: Vec<ArchiveMonth>,
}

/// Gruppiert Posts nach Jahr und Monat, neueste zuerst. Leere Gruppen gibt es nicht; nur das neueste
/// Jahr und darin der neueste Monat sind aufgeklappt.
pub fn build(posts: &[&Post]) -> Vec<ArchiveYear> {
    let mut sorted: Vec<&Post> = posts.to_vec();
    sorted.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.url.cmp(&b.url)));
    let mut years: Vec<ArchiveYear> = Vec::new();
    let mut month_numbers: Vec<i8> = Vec::new();
    for p in sorted {
        let (year, month) = (p.date.year(), p.date.month());
        if years.last().is_none_or(|y| y.year != year) {
            years.push(ArchiveYear { year, count: 0, open: false, months: Vec::new() });
            month_numbers.clear();
        }
        let y = years.last_mut().unwrap();
        if month_numbers.last() != Some(&month) {
            month_numbers.push(month);
            y.months.push(ArchiveMonth { name: MONTHS[month as usize - 1], count: 0, open: false, posts: Vec::new() });
        }
        y.count += 1;
        let m = y.months.last_mut().unwrap();
        m.count += 1;
        m.posts.push(ArchivePost { url: urls::encode_path(&p.url), title: p.title.clone(), day: p.date.day() });
    }
    if let Some(y) = years.first_mut() {
        y.open = true;
        if let Some(m) = y.months.first_mut() {
            m.open = true;
        }
    }
    years
}

#[cfg(test)]
mod tests {
    use super::*;

    fn post(title: &str, date: &str) -> Post {
        Post {
            source: String::new(),
            title: title.to_string(),
            date: format!("{date}T00:00[Europe/Berlin]").parse().unwrap(),
            url: format!("/{}/", urls::slugify(title)),
            author: None,
            image: None,
            excerpt: None,
            markdown: String::new(),
            media: Vec::new(),
            tags: Vec::new(),
        }
    }

    #[test]
    fn groups_by_year_and_month_newest_first() {
        let posts = [post("A", "2019-03-02"), post("B", "2020-10-31"), post("C", "2020-10-01"), post("D", "2020-02-14"), post("E", "2019-03-30")];
        let refs: Vec<&Post> = posts.iter().collect();
        let years = build(&refs);
        let shape: Vec<_> = years
            .iter()
            .map(|y| (y.year, y.count, y.months.iter().map(|m| (m.name, m.count)).collect::<Vec<_>>()))
            .collect();
        assert_eq!(shape, [(2020, 3, vec![("Oktober", 2), ("Februar", 1)]), (2019, 2, vec![("März", 2)])]);
        let titles: Vec<_> = years[0].months[0].posts.iter().map(|p| (p.title.as_str(), p.day)).collect();
        assert_eq!(titles, [("B", 31), ("C", 1)]);
        assert_eq!(years[1].months[0].posts[0].title, "E");
    }

    #[test]
    fn only_newest_year_and_month_are_open() {
        let posts = [post("A", "2019-03-02"), post("B", "2020-10-31"), post("D", "2020-02-14")];
        let refs: Vec<&Post> = posts.iter().collect();
        let years = build(&refs);
        assert!(years[0].open && years[0].months[0].open);
        assert!(!years[0].months[1].open);
        assert!(!years[1].open && !years[1].months[0].open);
    }

    #[test]
    fn counts_add_up_and_empty_input_is_empty() {
        assert!(build(&[]).is_empty());
        let posts: Vec<Post> = (1..=12).map(|m| post(&format!("P{m}"), &format!("2018-{m:02}-05"))).collect();
        let refs: Vec<&Post> = posts.iter().collect();
        let years = build(&refs);
        assert_eq!(years.len(), 1);
        assert_eq!(years[0].count, 12);
        assert_eq!(years[0].months.len(), 12);
        assert_eq!(years[0].months.iter().map(|m| m.count).sum::<usize>(), 12);
        assert_eq!(years[0].months[0].name, "Dezember");
    }
}
