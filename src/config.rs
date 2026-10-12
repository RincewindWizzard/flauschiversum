//! Feste Einstellungen des Flauschiversums.

pub const BLOG_URL: &str = "https://flauschiversum.de";
pub const BLOG_NAME: &str = "Flauschiversum";
pub const BLOG_DESCRIPTION: &str = "Mehr als nur eine Masche";
pub const RIGHTS_CREATOR: &str = "Flauschiversum / Ermeline Wollknoll";
pub const RIGHTS_WEB_STATEMENT: &str = "https://flauschiversum.de/impressum/";
pub const TIME_ZONE: &str = "Europe/Berlin";

/// Anzahl Posts pro Indexseite.
pub const POSTS_PER_PAGE: usize = 4;
/// Seitenlinks vor und nach der aktuellen Seite in der Paginierung.
pub const PAGINATION_WINDOW: usize = 2;
/// Anzahl Einträge im RSS-Feed.
pub const FEED_ENTRIES: usize = 20;
/// Maximale Länge der Meta-Beschreibung.
pub const META_DESCRIPTION_MAX: usize = 160;

/// Bildbreiten: Thumbnail, Preview, Full.
pub const THUMB_WIDTH: u32 = 200;
pub const PREVIEW_WIDTH: u32 = 700;
pub const FULL_WIDTH: u32 = 1000;
/// Obergrenze für die Dateigröße der Full-Variante.
pub const FULL_MAX_BYTES: usize = 1024 * 1024;
/// Ab dieser Größe warnt der Build bei Animationen und Videos.
pub const MEDIA_WARN_BYTES: u64 = 20 * 1024 * 1024;

/// Erste Pfadsegmente, die der Generator selbst belegt (Posts und Einzelseiten dürfen sie nicht verwenden).
pub const RESERVED_PATHS: [&str; 6] = ["themen", "suche", "beitraege", "kategorie", "tag", "pagefind"];

/// Anzahl der neuesten Beiträge auf der Startseite und auf `/beitraege/`.
pub const HOME_NEWS: usize = 10;
pub const POSTS_LATEST: usize = 10;

/// Kontaktwege (Fußzeile). Jeder `mailto:`-Betreff enthält „Flauschiversum“ (Spamfilter).
pub const CONTACT_EMAIL: &str = "ermeline@flauschiversum.de";
pub const INSTAGRAM_URL: &str = "https://www.instagram.com/ermelinewollknoll/";
pub const MASTODON_URL: &str = "https://chaos.social/@ErmelineWollknoll";
