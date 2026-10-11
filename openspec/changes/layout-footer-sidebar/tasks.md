# Tasks

## 1. Footer und Navigation

- [ ] 1.1 Konstanten `CONTACT_EMAIL`, `INSTAGRAM_URL`, `MASTODON_URL` in `src/config.rs` und eine Funktion `mailto(subject)` in `render.rs`, die den Betreff kodiert und ohne „Flauschiversum“ im Betreff nicht zulässt; `post.html` nutzt sie für den bestehenden Betreff; prüfen: Unit-Tests für Kodierung (Umlaute, Leerzeichen, Klammern) und für die Ablehnung eines Betreffs ohne das Wort
- [ ] 1.2 Partial `templates/partials/icons.html` mit den inline SVG für Instagram, Mastodon und Briefumschlag (`aria-hidden`, `focusable="false"`, `fill="currentColor"`); prüfen: Snapshot-Test der Footer-Ausgabe enthält die drei SVG
- [ ] 1.3 `layout.html`: Kopfnavigation auf Start, Themen, Beiträge kürzen; Fußzeile mit Seitenlinks (Tipps und Tricks, About, Impressum, Abonnieren), E-Mail-Link (Betreff „Flauschiversum“), Instagram- und Mastodon-Icon-Links (zugängliche Namen, `rel="me"` beim Mastodon-Link, `rel="noopener"`) und Copyright; prüfen: Test über alle Seiten der Fixture-Site: Navigation im Kopf hat genau drei Links, Footer enthält die vier Seitenlinks, beide Profil-URLs und den `mailto:`-Link, Copyright mit Jahr
- [ ] 1.4 `content/style/footer.css` (in `main.css` importiert): Flex-Aufbau, gestrichelter Rand, Hover und Fokus, Klickflächen mindestens 44 × 44 px, Umbruch am Handy; Kopfnavigation an drei Einträge anpassen; prüfen: Playwright-Test bei 1280 und 360 px: Footer ist das letzte sichtbare Element, alle Links sichtbar, kein waagerechtes Scrollen, Tab-Fokus ist auf den Icons sichtbar

## 2. Sidebar und Layout

- [ ] 2.1 Sidebar-Daten in `src/topics.rs`: Kategorien mit Anzahl (Reihenfolge der Datei), Tags alphabetisch (deutsche Sortierung) mit Anzahl und Größenstufe 1–5 (logarithmisch, bei gleicher Anzahl Stufe 3), nur veröffentlichte Beiträge, an `Common` übergeben; prüfen: Unit-Tests für Stufen (Häufigstes = 5, Seltenstes = 1, alle gleich = 3, nur ein Schlagwort), geplanten Beitrag und Sortierung „Hüte, Katzen, Taschen“
- [ ] 2.2 Partial `templates/partials/sidebar.html` (`aside` mit Beschriftung, Kategorienliste mit Anzahl, Tag-Cloud mit Klassen `cloud-1…5`, Titel „Hüte: 21 Beiträge“, Link „Alle Themen“) und `layout.html` mit `div.page-layout`, `head.sidebar` (aus auf Suche und `/themen/`); Abschnitt „Häufige Themen“ aus `home.html` und `HOME_TOPICS` entfernen; prüfen: Fixture-Test: Sidebar steht auf Start, Liste, Beitrag, Kategorie, Schlagwort und Einzelseite, nicht auf Suche und `/themen/`, überall identisch, auf der Startseite fehlt „Häufige Themen“
- [ ] 2.3 CSS: Seitenrahmen auf 1200 px, Zweispalter ab 64 em (`minmax(0, 1fr) 17em`), Sidebar `sticky`, `content/style/sidebar.css` (Cloud-Größen 0,85–1,6 em, Hover, Fokus), Beitragstext zentriert bis 40 em, Listen und Kacheln füllen die Inhaltsspalte; prüfen: Playwright-Test bei 1440 px (Sidebar rechts neben dem Text, Text höchstens etwa 40 em breit) und bei 360 px (Sidebar nach dem Inhalt, kein waagerechtes Scrollen)

## 3. Autor am Ende

- [ ] 3.1 `post.html`: „Von <Autor>“ aus dem Kopf entfernen und nach dem Beitragstext vor `nav.post-nav` einfügen (nur mit Autor); `autoren.css` und `header.css` anpassen; prüfen: Snapshot-Test der Beitragsseite und Test, dass „Von …“ nach dem letzten Absatz und vor „älterer Beitrag“ steht, im Kopf nicht, und ohne `author` fehlt

## 4. Absicherung

- [ ] 4.1 Test über die gesamte gebaute Seite: jeder `mailto:`-Link (Footer, Beiträge, Einzelseiten) hat einen Betreff mit „Flauschiversum“, und beim Laden einer Seite gehen alle Anfragen an die eigene Domain; prüfen: `cargo test --release -- --ignored` grün und Playwright-Test zu den Anfragen
- [ ] 4.2 Snapshots und bestehende Tests an Navigation, Sidebar und Autorenzeile anpassen (Seitenstruktur, Staging-Leiste, Suchseite, `navigation.mjs`); prüfen: `cargo test` und alle Playwright-Skripte in `tests/browser/` grün
- [ ] 4.3 Vergleichsbilder (Desktop 1440 px und Handy, Startseite, Beitrag, Kategorieseite, mit und ohne Staging-Leiste) zur Abnahme vorlegen; prüfen: Freigabe im Gespräch am veröffentlichten Staging

## 5. Dokumentation

- [ ] 5.1 `README.md` (Kontaktdaten und Social-Links ändern: nur `src/config.rs`; Sidebar entsteht automatisch aus Kategorien und Schlagworten) aktualisieren; prüfen: der beschriebene Weg funktioniert (Änderung der URL erscheint im Footer)

## Workflow follow-up

- Nach der Abnahme `/opsx:archive layout-footer-sidebar` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
