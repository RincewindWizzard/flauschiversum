# Tasks

## 1. Footer und Navigation

- [x] 1.1 Konstanten `CONTACT_EMAIL`, `INSTAGRAM_URL`, `MASTODON_URL` in `src/config.rs` und eine Funktion `mailto(subject)` in `render.rs`, die den Betreff kodiert und ohne „Flauschiversum“ im Betreff nicht zulässt; `post.html` nutzt sie für den bestehenden Betreff; prüfen: Unit-Tests für Kodierung (Umlaute, Leerzeichen, Klammern) und für die Ablehnung eines Betreffs ohne das Wort
- [x] 1.2 Partial `templates/partials/icons.html` mit den inline SVG für Instagram, Mastodon und Briefumschlag (`aria-hidden`, `focusable="false"`, `fill="currentColor"`); prüfen: Snapshot-Test der Footer-Ausgabe enthält die drei SVG
- [x] 1.3 `layout.html`: Kopfnavigation auf Start, Themen, Beiträge kürzen; Fußzeile mit Seitenlinks (Tipps und Tricks, About, Impressum, Abonnieren), E-Mail-Link (Betreff „Flauschiversum“), Instagram- und Mastodon-Icon-Links (zugängliche Namen, `rel="me"` beim Mastodon-Link, `rel="noopener"`) und Copyright; prüfen: Test über alle Seiten der Fixture-Site: Navigation im Kopf hat genau drei Links, Footer enthält die vier Seitenlinks, beide Profil-URLs und den `mailto:`-Link, Copyright mit Jahr
- [x] 1.4 `content/style/footer.css` (in `main.css` importiert): Flex-Aufbau, gestrichelter Rand, Hover und Fokus, Klickflächen mindestens 44 × 44 px, Umbruch am Handy; Kopfnavigation an drei Einträge anpassen; prüfen: Playwright-Test bei 1280 und 360 px: Footer ist das letzte sichtbare Element, alle Links sichtbar, kein waagerechtes Scrollen, Tab-Fokus ist auf den Icons sichtbar

## 2. Spalten, Sidebar und Navigationsspalte

- [x] 2.1 Sidebar-Daten in `src/topics.rs`: Kategorien mit Anzahl (Reihenfolge der Datei), Tags alphabetisch (deutsche Sortierung) mit Anzahl und Größenstufe 1–5 (logarithmisch, bei gleicher Anzahl Stufe 3), nur veröffentlichte Beiträge, an `Common` übergeben; prüfen: Unit-Tests für Stufen (Häufigstes = 5, Seltenstes = 1, alle gleich = 3, nur ein Schlagwort), geplanten Beitrag und Sortierung „Hüte, Katzen, Taschen“
- [x] 2.2 Partials `sidebar.html` (`aside` mit Beschriftung, Kategorienliste für mittlere und schmale Fenster, Tag-Cloud mit Klassen `cloud-1…5`, Titel „Hüte: 21 Beiträge“, Link „Alle Themen“) und `sitenav.html` (`nav` mit Suchfeld mit eigener `id`, Links Start/Themen/Beiträge, Kategorienliste); `layout.html` mit `div.page-layout`, Quellreihenfolge Inhalt, Navigationsspalte, Sidebar, `head.sidebar` (aus auf Suche und `/themen/`); Abschnitt „Häufige Themen“ aus `home.html` und `HOME_TOPICS` entfernen; prüfen: Fixture-Test: Spalten stehen auf Start, Liste, Beitrag, Kategorie, Schlagwort und Einzelseite, nicht auf Suche und `/themen/`, überall identisch, Kategorien und Zahlen in Spalte und Sidebar stimmen überein, auf der Startseite fehlt „Häufige Themen“
- [x] 2.3 CSS: Rahmen 1400 px, `grid-template-areas` für drei Spalten ab 80 em, zwei Spalten ab 64 em, einspaltig darunter; Navigationsspalte `sticky`; Kopf ab 80 em nur mit Logo (Navigationsleiste und Suchfeld im Kopf ausgeblendet), darunter Leiste und Suchfeld im Kopf, Spalte und deren Suchfeld ausgeblendet; `content/style/sidebar.css` (Cloud-Größen 0,85–1,6 em, Hover, Fokus); Beitragstext mittig bis 40 em, Listen und Kacheln füllen die Mittelspalte; prüfen: Playwright-Test bei 1440 px (drei Spalten, Text höchstens etwa 40 em breit, Spalte bleibt beim Scrollen sichtbar), bei 1100 px (zwei Spalten, Navigation im Kopf, Kategorien in der Sidebar) und bei 360 px (Sidebar nach dem Inhalt, kein waagerechtes Scrollen), jeweils genau ein sichtbares Suchfeld und eine sichtbare Kategorienliste

## 3. Autor am Ende

- [x] 3.1 `post.html`: „Von <Autor>“ aus dem Kopf entfernen und nach dem Beitragstext vor `nav.post-nav` einfügen (nur mit Autor); `autoren.css` und `header.css` anpassen; prüfen: Snapshot-Test der Beitragsseite und Test, dass „Von …“ nach dem letzten Absatz und vor „älterer Beitrag“ steht, im Kopf nicht, und ohne `author` fehlt

## 4. Absicherung

- [x] 4.1 Test über die gesamte gebaute Seite: jeder `mailto:`-Link (Footer, Beiträge, Einzelseiten) hat einen Betreff mit „Flauschiversum“, und beim Laden einer Seite gehen alle Anfragen an die eigene Domain; prüfen: `cargo test --release -- --ignored` grün und Playwright-Test zu den Anfragen
- [x] 4.2 Snapshots und bestehende Tests an Navigation, Spalten und Autorenzeile anpassen (Seitenstruktur, Staging-Leiste, Suchseite, `navigation.mjs`: Locator für das Suchfeld und die Navigation nur für das sichtbare Element); prüfen: `cargo test` und alle Playwright-Skripte in `tests/browser/` grün
- [x] 4.3 Vergleichsbilder (nach der Überarbeitung 4.4–4.7 erneut) (Desktop 1440 px, Tablet 1100 px und Handy, Startseite, Beitrag, Kategorieseite, mit und ohne Staging-Leiste) zur Abnahme vorlegen; prüfen: Freigabe im Gespräch am veröffentlichten Staging

## 4b. Überarbeitung nach der Sichtprüfung

- [x] 4.4 Karten für Navigationsspalte und Sidebar-Blöcke (`.card`), angepasst an Kopf, Cloud und Kategorien; prüfen: Playwright-Test: die Spalten haben einen hellen Hintergrund (`background-color` mit Alpha > 0), `border-radius` > 8 px und Innenabstand > 8 px, bei 1440 und 360 px, ohne waagerechtes Scrollen
- [x] 4.5 Suchfeld mit eingebauter Lupe (`partials/icon-search.html`, `.searchbox`) für Kopf, Navigationsspalte, Startseite und Suchseite (ohne JavaScript); prüfen: Playwright-Test: Lupe liegt innerhalb des Feldes am rechten Rand, Feld und Knopf bleiben in der 13-em-Spalte und bei 360 px in einer Zeile, Enter und Klick auf die Lupe öffnen `/suche/?q=…`, der Knopf hat den Namen „Suchen“
- [x] 4.6 Footer: E-Mail nur als Icon mit `aria-label="E-Mail schreiben"` und Tooltip mit der Adresse; prüfen: Test über alle Seiten (kein Text mit der Adresse im Footer, `mailto:` mit Betreff „Flauschiversum“) und Playwright-Test der Klickfläche (mindestens 44 px)
- [x] 4.7 Typografie: Variablen und Skala einführen, Lobster nur für Logo und Titel, Navigation, Footer, Cloud, Kategorien, Knöpfe und Beschriftungen in Lora (halbfett für Navigationslinks, mindestens 0,9 em); prüfen: Playwright-Test: berechnete `font-family` der Links in Kopf, Spalte und Footer enthält nicht „Lobster“, Logo und Abschnittstitel schon; Vergleichsbilder zur erneuten Abnahme

- [x] 4.8 Neues Farbschema: Variablen in `theme.css` (Cremeweiß, Karten, Salbeigrün, dunkelgrüne Links), Hover, Tags, Pagination, Knöpfe, Karussell-Punkte und Leuchteffekte umstellen; prüfen: Playwright-Test: Kontrast von Text, Links, Knopf und Tag mit der berechneten Farbe mindestens 4,5 : 1, kein Neongrün (`#bdf239`, `#82e536`, `#eefc3c`) im ausgelieferten Stylesheet, Staging-Leiste bleibt rot

- [x] 4.9 Tag-Cloud beruhigen: Chips mit neutraler Textfarbe, Zahl der Beiträge im Chip, Größenstufen nur 0,9–1,1 em mit schwacher Tönung; prüfen: Playwright-Test (Größen 0,9–1,1 em, einheitliche Textfarbe, Zahl sichtbar, alphabetisch) und Vergleichsbild zur Abnahme

## 5. Dokumentation

- [x] 5.1 `README.md` (Kontaktdaten und Social-Links ändern: nur `src/config.rs`; Sidebar entsteht automatisch aus Kategorien und Schlagworten) aktualisieren; prüfen: der beschriebene Weg funktioniert (Änderung der URL erscheint im Footer)

## Workflow follow-up

- Nach der Abnahme `/opsx:archive layout-footer-sidebar` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
