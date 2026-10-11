# Proposal

## Why

Das Layout hat zuletzt drei Spalten bekommen (Navigation links, Inhalt, Sidebar rechts), und trotzdem finden Besucher Beiträge schlecht: Die Navigation ist ein Mix aus Kopfleiste, linker Spalte, Sidebar und Footer, die Startseite ist ein Katalog statt einer Begrüßung, und die Beitragsliste blättert in 4er-Schritten durch 49 Seiten. Gewünscht ist ein klares Blog-Muster: eine einzige Topbar mit Logo und wenigen Menüs, eine Startseite mit Begrüßung und den neuesten Beiträgen, ein Beiträge-Bereich mit Archiv nach Monat und Jahr und Kategorieseiten mit eigenem Einleitungstext.

## What Changes

- **Topbar statt Sidebars:** Oben steht eine Leiste. Links der Titel „Flauschiversum“ (Link auf die Startseite), rechtsbündig die Menüs **About me**, **Beiträge** und **Kategorien** und das Suchfeld. Die linke Navigationsspalte und die rechte Sidebar entfallen; Kopfnavigation und Kopfbereich gehen in der Topbar auf.
- **About me:** Ein einfacher Link auf die Seite über die Autorin (`/about/`), ohne Untermenü.
- **Beiträge:** Das Menü klappt die zehn neuesten Beiträge als Auswahl auf (plus „Alle Beiträge“). Ein Klick auf „Beiträge“ selbst öffnet `/beitraege/`.
- **Seite Beiträge (`/beitraege/`):** Zeigt die zehn neuesten Beiträge und daneben ein **Archiv**: alle Beiträge nach Jahr und Monat, jede Gruppe ein- und ausklappbar, jeder Beitrag ein Link. Die bisherige Blätterliste (`/page/<N>/`) bleibt unter ihren Adressen erreichbar, wird aber nicht mehr verlinkt.
- **Kategorien:** Das Menü klappt die sechs Kategorien auf (Filzen, Basteln, Malen, Wolle, Brandings, Nähen). Ein Klick führt auf die Kategorieseite: oben ein kurzer Text zur Kategorie, darunter die Beiträge, daneben dasselbe Archiv nach Monat und Jahr, aber nur mit den Beiträgen dieser Kategorie.
- **Startseite (Klick auf den Titel):** Zuerst ein kurzer Begrüßungstext (in `content/start.md` pflegbar), direkt danach die neuesten Beiträge. Die Kategoriekacheln entfallen, weil die Kategorien im Menü stehen.
- **Tag-Cloud nur auf der Startseite:** Die Schlagwort-Cloud steht nur noch dort (Karte neben bzw. unter den Beiträgen). Auf allen anderen Seiten entfallen Cloud und Kategorienliste.
- **Beibehalten:** Footer (Tipps und Tricks, About, Impressum, Abonnieren, E-Mail, Socials, RSS, Copyright), Farbschema, Typografie und Karten, alle Beitrags-, Schlagwort-, Themen- und Suchseiten samt URLs.

Nicht Teil dieses Changes: neue Inhalte außer dem Begrüßungstext und den sechs Kategorietexten (Entwürfe zur Prüfung), Änderungen an Suche, Feed oder Deployment.

## Capabilities

### New Capabilities

- `topbar`: Die Topbar mit Titel, Menüs About me, Beiträge, Kategorien und Suche, einschließlich Dropdowns, Tastaturbedienung und Handy-Menü.
- `post-archive`: Das einklappbare Archiv aller (oder einer Kategorie von) Beiträgen nach Jahr und Monat.

### Modified Capabilities

- `site-layout`: Seitenaufbau mit Topbar; die Spalten-Anforderungen (Navigationsspalte, Dreispaltig, Zweispaltig, Sidebar, Karten für Seitenspalten, Suchfeld im Kopf/links) entfallen oder werden ersetzt.
- `sidebar`: Die Tag-Cloud steht nur noch auf der Startseite; die Kategorienliste in der Seitenspalte entfällt.
- `home-page`: Begrüßung und letzte Beiträge statt Suchfeld, Kacheln und „Neu im Flauschiversum“.
- `topic-pages`: Kategorieseite mit Einleitungstext, Beitragsliste und Archiv; die Schlagwortliste in der Kategorie entfällt.
- `post-index`: `/beitraege/` zeigt die zehn neuesten Beiträge; die Blätterseiten bleiben unverändert erreichbar.
- `content-format`: Neue Datei `content/start.md` für den Begrüßungstext; optionales Feld `intro` je Kategorie.

## Impact

- **Code:** `templates/layout.html` (Topbar, Wegfall von Spalten und Sidebar), `home.html`, `topic.html`, `index.html` (Beiträge-Seite), neue Partials für Topbar und Archiv, `src/render.rs`, `src/site.rs`, `src/topics.rs` (Archivdaten), `src/content.rs` (`start.md`, `intro`), `src/config.rs`, `content/style/` (neue `topbar.css`, `archive.css`; Wegfall von `sidebar.css`-Teilen), ein kleines Skript `assets/js/topbar.js` für Dropdowns, Snapshots und Browser-Tests.
- **Inhalte:** neu `content/start.md` und das Feld `intro` in `content/categories.yaml`; Beiträge und URLs bleiben unverändert.
- **URLs:** Alle bestehenden URLs bleiben erreichbar (auch `/page/<N>/`, `/themen/`, `/tag/<name>/`, `/kategorie/<name>/`, `/beitraege/`).
- **Abnahme:** Der Staging-Lauf veröffentlicht die Änderung; Vergleichsbilder für Desktop, Tablet und Handy.
