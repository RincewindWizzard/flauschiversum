# Tasks

## 1. Zuordnung der Beiträge

- [x] 1.1 Alle 194 Beiträge lesen (Titel, Kurztext, Textanfang) und in `openspec/changes/handbuch-navigation/zuordnung.md` je Beitrag Kategorie und 1–4 Schlagworte vorschlagen, dazu die Liste der Schlagworte mit Anzahl; prüfen: die Tabelle hat 194 Zeilen, jede Kategorie stammt aus den sechs vorhandenen, jedes Schlagwort kommt mindestens zweimal vor
- [x] 1.2 Die Zuordnung von der Seitenbetreiberin prüfen und korrigieren lassen; prüfen: Freigabe im Gespräch, bevor Inhalte geändert werden
- [x] 1.3 `content/categories.yaml` (sechs Kategorien mit Bild und Reim aus der alten Navigation) und `content/tags.yaml` anlegen und per Skript `category:` und `tags:` als eingefügte Zeilen in die `index.md` aller Beiträge eintragen; prüfen: `git diff --numstat` zeigt für die Beiträge nur Einfügungen, und die Kategorie ist nur dort ergänzt, wo sie in der Zuordnung steht („Relaunch aus der Hackermatte“ bleibt ohne Kategorie)

## 2. Inhaltsmodell und Prüfung

- [x] 2.1 Das Frontmatter um `tags` (YAML-Liste oder kommagetrennter Text) erweitern und `category` als optionales Feld mit festem Wertevorrat prüfen; prüfen: Unit-Tests für Liste, Text, fehlende Kategorie (gültig) und unbekannte Kategorie (Meldung mit Pfad, Wert und erlaubten Werten)
- [x] 2.2 Die Listen laden und prüfen (unbekannte Kategorie oder Schlagwort mit „Meintest du …“, doppelte Schlagworte, fehlendes Kategoriebild, Schlagwort mit weniger als zwei Beiträgen als Warnung); prüfen: Unit-Tests für jeden Fall prüfen den Meldungstext
- [x] 2.3 `src/topics.rs`: Zuordnung, Zählung, Sortierung (neueste zuerst, Schlagworte alphabetisch mit deutscher Sortierung), nur veröffentlichte Beiträge, leere Themen entfallen; prüfen: Unit-Tests inkl. geplantem Beitrag und Slug-Kollision zweier Schlagworte
- [x] 2.4 Reservierte Pfade (`themen`, `suche`, `beitraege`, `kategorie`, `tag`, `pagefind`) gegen Posts und Einzelseiten prüfen; prüfen: Test mit einer Einzelseite „Themen“ bricht mit Datei und URL ab

## 3. Seiten und Navigation

- [x] 3.1 Die chronologische Liste nach `/beitraege/` verlegen (Paginierung, Titel „Alle Beiträge …“, Rücklink von `/page/1/`); prüfen: Unit-Tests der Paginierung und Titel, alle bisherigen `/page/<N>/` bleiben im Gesamtseitentest erhalten
- [x] 3.2 Templates `home.html`, `topic.html`, `topics.html` und Partial `entry.html` sowie `content/style/topics.css` (Kacheln, Schlagwortliste, kompakte Einträge) umsetzen; prüfen: Snapshot-Tests für Startseite, Kategorieseite, Schlagwortseite und Themenübersicht, jeweils mit einer einzigen `h1`
- [x] 3.3 Beitragskopf mit Kategorie und Schlagworten, neue Hauptnavigation und Suchfeld im Kopf umsetzen; prüfen: Snapshot-Test der Beitragsseite, Navigation enthält die sieben Einträge in der Reihenfolge der Spec
- [x] 3.4 SEO: Sitemap um Themenseiten erweitern, Titel und Meta-Beschreibungen der neuen Seiten, `articleSection` und `keywords` im JSON-LD, `noindex` auf der Suchseite; prüfen: Unit- und Snapshot-Tests, JSON-LD parst als gültiges JSON

## 4. Suche

- [x] 4.1 Probelauf: `pagefind` (ohne Server-Funktionen) als Bibliothek einbinden, aus der Test-Site einen Index erzeugen und im Browser durchsuchen; prüfen: Treffer für „katze“, „kuerbis“ und „filz“ im Chromium-Test, Build-Zeit und Größe der Dateien notiert; scheitert der Probelauf, wird stattdessen ein kleiner eigener Index gebaut (Design, Alternative b) und das hier dokumentiert
- [x] 4.2 `src/search.rs`: Posts und Einzelseiten in sortierter Reihenfolge indexieren, Dateien als Ausgaben unter `/pagefind/` einfügen, Templates mit `data-pagefind-body` und Kategoriefilter markieren; prüfen: Integrationstest „zweiter Build schreibt 0 Dateien“ und „gelöschter Beitrag entfernt seine Indexdateien“
- [x] 4.3 Suchseite `/suche/` mit `assets/js/search.js` (deutsche Texte, `?q=`, Adresse aktuell halten), Kategoriefilter und `<noscript>`-Hinweis umsetzen; prüfen: Snapshot-Test der Seite
- [x] 4.4 Browser-Tests (Playwright) für die Suche: von Startseite und Beitrag aus suchen, Wortform, Umlaut, Wortanfang, Kategoriefilter, ohne JavaScript; prüfen: alle Tests in `tests/browser/` grün

## 5. Abnahme und Dokumentation

- [x] 5.1 Gesamtseitentest erweitern: jeder veröffentlichte Beitrag mit Kategorie steht auf genau einer Kategorieseite, jeder Beitrag auf den Seiten seiner Schlagworte und in der Suche, alle Live-URLs und alle `/page/<N>/` existieren weiter; prüfen: `cargo test --release -- --ignored` grün
- [x] 5.2 Browser-Tests für Startseite (Kachel führt auf die Kategorieseite, Schlagwort führt auf die Schlagwortseite) und Handy-Ansicht ohne waagerechtes Scrollen; prüfen: Tests grün
- [x] 5.3 Vergleichsbilder (Startseite, Kategorieseite, Beitrag mit Einordnung, Suchergebnis, Handy) zur Abnahme vorlegen; prüfen: Freigabe im Gespräch
- [x] 5.4 `README.md` (Kategorien und Schlagworte pflegen, Suche) und `openspec/config.yaml` (Projektkontext) aktualisieren; prüfen: die beschriebenen Schritte funktionieren wie angegeben

## Workflow follow-up

- Nach der Abnahme `/opsx:archive handbuch-navigation` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
