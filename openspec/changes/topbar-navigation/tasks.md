# Tasks

## 1. Daten und Inhalte

- [x] 1.1 Menüdaten für die Topbar (zehn neueste veröffentlichte Beiträge mit Titel und URL, Kategorien in der Reihenfolge der Datei mit Anzahl, leere Kategorien entfallen) in `Common` bereitstellen; prüfen: Unit-Tests für zehn Einträge bei mehr als zehn Beiträgen, geplante Beiträge fehlen, Reihenfolge der Kategorien
- [x] 1.2 Archivdaten (`Year`/`Month`/Beiträge, neueste zuerst, leere Gruppen entfallen, Zählungen) für eine Beitragsliste berechnen (alle bzw. einer Kategorie); prüfen: Unit-Tests für Gruppierung über Jahre und Monate, Zählung, Sortierung, Kategorie-Filter und geplanten Beitrag
- [x] 1.3 `content/start.md` laden (Warnung bei fehlender Datei) und das optionale Feld `intro` in `content/categories.yaml` (Markdown) lesen und prüfen; prüfen: Unit-Tests für vorhandene und fehlende Datei (Warnung mit Pfad) und für Kategorie mit und ohne `intro`
- [x] 1.4 Inhalte als Entwurf anlegen: Begrüßungstext in `content/start.md` und je ein kurzer `intro` (zwei bis drei Sätze) für jede Kategorie aus `content/categories.yaml`, aus den vorhandenen Texten abgeleitet; prüfen: Entwürfe stehen zur Prüfung durch die Autorin im Gespräch, jede Kategorie hat einen Text, Build ohne Warnung

## 2. Topbar

- [x] 2.1 Topbar in `layout.html` (Titel links, Menüs About me, Beiträge, Kategorien rechtsbündig, Suchfeld mit Lupe), Untermenüs mit Knopf (`aria-expanded`, `aria-controls`), Handy-Menü mit `details`; Kopf, Kopfnavigation, `sitenav` und allgemeine Sidebar entfernen; prüfen: Fixture-Test über alle Seiten: Topbar mit den drei Menüs und dem Suchfeld, Untermenü Beiträge mit zehn Links und „Alle Beiträge“, Untermenü Kategorien mit den Kategorien, keine `sitenav` und kein `aside.sidebar` außer der Cloud-Karte der Startseite
- [x] 2.1a Maskottchen: SVGs aus `openspec/changes/topbar-navigation/assets/` bereinigt nach `content/static/images/flauschi_normal.svg` und `flauschi_surprised.svg` (gleiche `viewBox`), in `.brand` links vor dem Titel einbauen, beide Bilder im DOM und übereinander; prüfen: Playwright-Test: beide Bilder gleich groß (Bounding Box identisch), Position des Titels und der Menüs bei Mouse-over unverändert, `alt=""`, Wechsel bei Hover und Tastaturfokus, Dateien kleiner als 60 KB, Link-Prüfung ohne Warnung
- [x] 2.2 `content/style/topbar.css` (sticky, Karten für Untermenüs, rechtsbündig, Schatten, Typografie Lora halbfett, Titel Lobster; Handy unter 48 em) und `assets/js/topbar.js` (Knopf, Escape, Klick außerhalb, höchstens ein Menü offen, Pfeiltasten); Platzierung der Skripte mit Versionshash; prüfen: Playwright-Test für Maus-Hover, Klick, Tastatur (Tab, Enter, Pfeile, Escape), Touch am Handy, ohne JavaScript (Hover öffnet das Untermenü), kein waagerechtes Scrollen bei 360 px, Klickflächen mindestens 44 px

## 3. Seiten

- [x] 3.1 Startseite: Begrüßung aus `start.md`, zehn letzte Beiträge, „Alle Beiträge“, Karte mit Tag-Cloud rechts (nur hier); Kategoriekacheln, großes Suchfeld und „Neu im Flauschiversum“ entfernen; prüfen: Fixture-Test (Reihenfolge Überschrift, Begrüßung, Beiträge; Cloud nur auf `/`) und Snapshot
- [x] 3.2 Beiträge-Seite `/beitraege/` mit den zehn neuesten Beiträgen, ohne Paginierung, und Archiv-Karte (`partials/archive.html`, `details` für Jahr und Monat, neuestes Jahr und Monat offen, Knopf „Alle aufklappen/zuklappen“ mit JavaScript); `/page/<N>/` unverändert, „«“ auf `/page/1/` führt nach `/beitraege/`; prüfen: Tests für Aufbau, Zustand beim Laden, Anzahl der Einträge im Archiv gleich der Zahl der veröffentlichten Beiträge, Wegfall des Seitenzählers auf `/beitraege/`, Bestand aller `/page/<N>/`
- [x] 3.3 Kategorieseite: Einleitungstext (`intro`, Reim) oben, darunter alle Beiträge, rechts das Archiv der Kategorie; Schlagwortliste der Kategorie entfernt; prüfen: Test: jeder Beitrag mit Kategorie steht auf genau einer Kategorieseite und in deren Archiv, das Archiv enthält nur Beiträge der Kategorie, Zahl im Archiv gleich Zahl der Beiträge
- [x] 3.4 `archive.css` und Layout: Seitenrahmen bis 1200 px, Inhalt mit Karte ab 64 em rechts (Archiv bzw. Cloud, `sticky`), darunter auf schmalen Fenstern; Beitragsseiten, Suche und Themenübersicht einspaltig; prüfen: Playwright-Test bei 1440, 1100 und 360 px: Archiv rechts bzw. unter der Liste, Ein-/Ausklappen per Klick und Tastatur, Cloud nur auf der Startseite, kein waagerechtes Scrollen

## 4. Absicherung

- [ ] 4.1 Gesamtseitentest erweitern: alle Live-URLs inklusive `/page/<N>/`, `/themen/`, `/tag/<name>/` existieren weiter und `/kategorie/<name>/` leitet weiter; Menüs und Archiv-Links zeigen auf existierende Seiten; Sitemap und Link-Prüfung ohne neue Warnungen; prüfen: `cargo test --release -- --include-ignored` grün
- [x] 4.2 Bestehende Tests und Snapshots anpassen (Spalten-Tests in `tests/browser/layout.mjs` auf die Topbar umstellen, `navigation.mjs`, `staging.mjs`, `frontend.mjs`, Snapshots); prüfen: `cargo test` und alle Playwright-Skripte grün
- [ ] 4.3 Vergleichsbilder (Startseite, Beiträge, Kategorie, Beitrag, geöffnetes Menü, Handy mit offenem Menü, jeweils Desktop und Handy) zur Abnahme vorlegen; prüfen: Freigabe im Gespräch am veröffentlichten Staging

## 4a. Überarbeitung nach Rückmeldung

- [x] 4a.1 Topbar auf einer Linie zentrieren (Suchfeld-Außenabstand entfernt), „Beiträge“ ohne Dropdown, „About me“ in den Footer, Hamburger-Knopf am Handy; prüfen: Test auf Mittelpunkte, kein Beiträge-Untermenü, Hamburger-Knopf mindestens 44 px
- [x] 4a.2 Footer nach Vorbild der Segelgruppe Kiel (dunkles Band, links Icons mit Copyright darunter, rechts Links untereinander), „Abonnieren“ entfernt; prüfen: Test auf Anordnung und Höhe, `cargo test` und Browser-Tests grün

## 4b. Nur noch Themen

- [x] 4b.1 Kategorien aus Code, Daten und Templates entfernen (`Category`, `categories.yaml`, Kategorieseiten, Menü, Etiketten); alle 194 Beiträge: `category` weg, Wert als Schlagwort; prüfen: `cargo test` grün, `check` ohne Fehler, keine Beitrags-URL ändert sich
- [x] 4b.2 Topbar: Themen und Tipps und Tricks als einfache Links, kein Dropdown; Footer nur About me und Impressum; Tablet 770 px einzeilig; prüfen: `tests/browser/topbar.mjs` und `layout.mjs`
- [x] 4b.3 Themenseite mit Archiv, Themenübersicht nur Schlagworte, Suche mit Filter „Thema“; prüfen: `navigation.mjs`
- [x] 4b.4 `content/redirects.yaml` und Weiterleitungsseiten für `/kategorie/<name>/`; prüfen: Test auf alle früheren Kategorieadressen, Sitemap ohne Weiterleitungen
- [x] 4b.5 Konzentrische Lupe in Topbar und Suchseite; prüfen: Test auf gleiche Abstände oben, unten und rechts
- [x] 4b.6 Alle Playwright-Skripte (`topbar`, `layout`, `navigation`, `theme`, `frontend`, `staging`) und Snapshots anpassen; prüfen: alle grün

## 5. Dokumentation

- [x] 5.1 `README.md` (Topbar, Archiv, Themen, `content/start.md`, `content/redirects.yaml`) und `openspec/config.yaml` (Projektkontext) aktualisieren; prüfen: die beschriebenen Schritte funktionieren (Text ändern erscheint auf der Startseite)

## Workflow follow-up

- Nach der Abnahme `/opsx:archive topbar-navigation` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
