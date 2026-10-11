# Proposal

## Why

Das Flauschiversum ist inzwischen ein Handbuch mit 194 Anleitungen und Ideen, wird aber wie eine Nachrichtenseite präsentiert: Die Startseite ist eine rein chronologische Liste (49 Seiten zu je 4 Beiträgen), und einen Beitrag von 2014 findet nur, wer fast bis zum Ende blättert. Es gibt keine Kategorie- oder Themenseiten und keine Suche, obwohl jeder Beitrag schon eine Kategorie trägt (basteln 54, wolle 49, filzen 37, nähen 27, brandings 16, malen 6) und im Code früher eine Kategorie-Navigation mit Bildern und Reimen angelegt, aber auskommentiert war. Besucher sollen Dinge finden, die sie interessieren, egal wie alt der Beitrag ist: mehr Wiki, weniger Zeitung.

## What Changes

- **Kategorieseiten:** Jede Kategorie bekommt eine Seite `/kategorie/<name>/` mit allen zugehörigen Beiträgen in einer kompakten, gut überfliegbaren Übersicht (ohne Blättern). Die Kategorie ist künftig Pflicht und wird mit einer kleinen festen Liste in `content/categories.yaml` verwaltet (Anzeigename, Bild, Reim).
- **Schlagworte (Tags):** Posts erhalten ein Frontmatter-Feld `tags` mit feineren Themen wie Katzen, Hüte, Taschen, LARP oder Geschenke. Jedes Schlagwort hat eine Seite `/tag/<name>/`. Erlaubt sind nur Schlagworte aus einer festen Liste in `content/tags.yaml`, damit Tippfehler und Doppelungen auffallen. Eine Übersichtsseite `/themen/` zeigt alle Kategorien und Schlagworte mit Anzahl.
- **Suche:** Ein Suchfeld auf jeder Seite durchsucht Titel, Text, Kategorie und Schlagworte aller Beiträge direkt im Browser, ohne Server. Der Suchindex entsteht beim Build. Ergebnisse stehen auf `/suche/`.
- **Handbuch-Startseite:** `/` zeigt statt der chronologischen Liste einen Einstieg: Suchfeld, Kacheln für die Kategorien (mit den vorhandenen Bildern und Reimen), häufige Themen und einen kleinen Abschnitt „Neu im Flauschiversum“. Die chronologische Liste bleibt unter `/beitraege/` erreichbar. Die Folgeseiten `/page/<N>/` bleiben bestehen.
- **Beiträge zeigen ihre Einordnung:** Auf jeder Beitragsseite stehen Kategorie und Schlagworte als Links. Das hilft beim Weiterstöbern, und Suchmaschinen erfahren die Einordnung (`articleSection`, `keywords`).
- **Navigation und SEO:** Die Hauptnavigation führt zu Start, Themen und Beiträgen. Kategorie- und Schlagwortseiten kommen in die Sitemap und haben eigene Titel und Beschreibungen.
- **Zuordnung der 194 Beiträge:** Kategorie (5 Beiträge haben noch keine) und Schlagworte aller Beiträge werden von Claude vorgeschlagen und von dir in einer Übersicht geprüft und korrigiert, bevor sie ins Frontmatter geschrieben werden. Das ist die einzige Änderung an den Inhalten; die Texte und alle Beitrags-URLs bleiben unverändert.

Nicht Teil dieses Changes (bewusst nicht gewählt, später möglich): Verwandte Beiträge unter jedem Beitrag, eine Kennzeichnung „Anleitung“ gegenüber „Werkstück“, ein weniger prominentes Datum, ein Alphabet- oder Jahresarchiv sowie das neue Deployment.

## Capabilities

### New Capabilities

- `home-page`: Die Handbuch-Startseite unter `/` mit Suche, Kategoriekacheln, häufigen Themen und den neuesten Beiträgen.
- `topic-pages`: Kategorieseiten, Schlagwortseiten und die Themenübersicht, samt der festen Listen für Kategorien und Schlagworte.
- `site-search`: Die Volltextsuche im Browser mit beim Build erzeugtem Suchindex und der Ergebnisseite.

### Modified Capabilities

- `content-format`: Neues Feld `tags`, Pflichtfeld `category` mit Wertevorrat, Listen `categories.yaml` und `tags.yaml`.
- `url-scheme`: Chronologische Liste unter `/beitraege/` statt `/`; neue URLs für Themen, Kategorien, Schlagworte und Suche.
- `post-index`: Die chronologische Liste zieht von `/` nach `/beitraege/` um.
- `post-page`: Kategorie und Schlagworte werden auf der Beitragsseite angezeigt.
- `site-layout`: Neue Hauptnavigation und Suchfeld im Kopf; Seitentitel der neuen Seiten.
- `seo`: Sitemap, Titel und Beschreibungen der Themenseiten; `articleSection` und `keywords` im strukturierten Datensatz.
- `build`: Prüfung von Kategorien und Schlagworten; Suchindex wird mit ausgeliefert.

## Impact

- **Code:** `src/content.rs` (Schlagworte, Listen, Prüfung), neue Module für Themenseiten und Suche, `src/site.rs`, `src/render.rs`, neue Templates (`home.html`, `topic.html`, `topics.html`, `search.html`), angepasste `layout.html`, `index.html` und `post.html`, neues Skript für die Suche unter `assets/`.
- **Inhalte:** 194 `index.md` erhalten `tags:` (und 5 fehlende `category:`); neu sind `content/categories.yaml` und `content/tags.yaml`. Die Texte, `permalink`s und Medien ändern sich nicht.
- **Abhängigkeiten:** Das Rust-Crate `pagefind` (MIT, in Rust geschrieben, deutsche Wortstämme) als Bibliothek für den Suchindex; dadurch dauert das Kompilieren länger. Die Suche läuft ohne JavaScript-Bibliothek von Dritten, alles wird von der eigenen Domain ausgeliefert.
- **URLs:** Alle bestehenden URLs bleiben erreichbar. `/` und `/index.html` zeigen künftig die Handbuch-Startseite; die chronologische Liste liegt unter `/beitraege/`, die Folgeseiten bleiben unter `/page/<N>/`. Neu sind `/themen/`, `/kategorie/<name>/`, `/tag/<name>/`, `/suche/` und die Suchdateien unter `/pagefind/`.
