# build Specification

## Purpose

Beschreibt den Aufruf des Generators und sein Ergebnis: aus den Quellen in `src/` entsteht ein vollständiger, statisch auslieferbarer Verzeichnisbaum der Webseite.

## Requirements

### Requirement: Aufruf
Der Generator SHALL ein einzelnes Programm `flauschiversum` sein, das ohne Python, Node.js oder andere externe Programme läuft. `flauschiversum build` baut die Seite. Optional sind `--root <Pfad>` (Projektverzeichnis, Standard: aktuelles Verzeichnis), `--out <Pfad>` (Ausgabeverzeichnis, Standard `dist/`) und `--cache <Pfad>` (Bild-Cache, Standard `.cache/`). Relative Pfade beziehen sich auf das Projektverzeichnis.

#### Scenario: Build mit eigenen Pfaden
- **WHEN** `flauschiversum build --out /tmp/site --cache /tmp/cache` im Repository ausgeführt wird
- **THEN** liegt die fertige Seite in `/tmp/site`, und die Bildvarianten sind in `/tmp/cache` zwischengespeichert

#### Scenario: Aufruf aus einem anderen Verzeichnis
- **WHEN** `flauschiversum build --root ~/flauschiversum` in einem beliebigen Verzeichnis ausgeführt wird
- **THEN** wird die Seite aus `~/flauschiversum/content` nach `~/flauschiversum/dist` gebaut

### Requirement: Vollständiger statischer Verzeichnisbaum
Der Build SHALL alle in `url-scheme` genannten URLs als Dateien im Ausgabeverzeichnis erzeugen. Eine URL, die auf `/` endet, wird als `index.html` in dem entsprechenden Verzeichnis abgelegt. Für die Auslieferung braucht es nur einen Webserver für statische Dateien.

#### Scenario: Post als Verzeichnis
- **WHEN** der Post mit der URL `/2014/01/alraune/` gebaut wird
- **THEN** existiert `<dst>/2014/01/alraune/index.html`

#### Scenario: Bildvariante als Datei
- **WHEN** der Post `/2014/01/alraune/` das Bild `DSCF0527.jpg` enthält
- **THEN** existiert `<dst>/2014/01/alraune/700x/DSCF0527.jpg`

### Requirement: Statische Dateien kopieren
Der Build SHALL alle Dateien aus `content/static/` byte-genau an ihren Pfad relativ zu `content/static/` in das Ausgabeverzeichnis kopieren. Dazu kommen die vom Generator mitgelieferten Skripte und Styles für Karussell und Vollbild.

#### Scenario: Skript kopieren
- **WHEN** `content/static/script/beispiel.js` existiert
- **THEN** ist `<out>/script/beispiel.js` eine identische Kopie

#### Scenario: Schrift kopieren
- **WHEN** `content/static/fonts/Lobster.otf` existiert
- **THEN** ist `<out>/fonts/Lobster.otf` eine identische Kopie

### Requirement: Stylesheet erzeugen
Der Build SHALL `content/style/main.css` samt seinen `@import`-Dateien zu einer einzigen, minimierten CSS-Datei `<out>/style.css` zusammenführen.

#### Scenario: Gestaltung geändert
- **WHEN** in `content/style/theme.css` eine Farbe geändert und neu gebaut wird
- **THEN** enthält `<out>/style.css` die neue Farbe

### Requirement: Wiederholter Build
Ein erneuter Build ohne Änderungen an den Quellen SHALL ein byte-gleiches Ausgabeverzeichnis ergeben, ohne eine Datei neu zu schreiben.

#### Scenario: Erneuter Build
- **WHEN** zweimal hintereinander ohne Änderungen gebaut wird
- **THEN** meldet der zweite Build 0 geschriebene und 0 gelöschte Dateien, und die Änderungszeiten aller Dateien sind unverändert

### Requirement: Nur geänderte Dateien schreiben
Der Build SHALL eine Ausgabedatei nur schreiben, wenn sie fehlt oder ihr Inhalt sich geändert hat. Am Ende meldet er die Zahl der neuen, geänderten, unveränderten und gelöschten Dateien.

#### Scenario: Neuer Post
- **WHEN** ein neuer Post mit drei Bildern hinzukommt und neu gebaut wird
- **THEN** werden nur seine Seite, seine neun Bildvarianten, die Indexseiten, der bisher neueste Post (wegen der Navigation), `rss.xml` und `sitemap.xml` geschrieben

### Requirement: Verwaiste Dateien entfernen
Der Build SHALL nach dem Schreiben alle Dateien und leeren Verzeichnisse im Ausgabeverzeichnis löschen, die er in diesem Lauf nicht erzeugt hat. Eine Option `--clean` gibt es nicht mehr.

#### Scenario: Post gelöscht
- **WHEN** ein Post-Verzeichnis gelöscht und neu gebaut wird
- **THEN** sind seine Seite und seine Bildvarianten aus dem Ausgabeverzeichnis entfernt

#### Scenario: Ausgabeverzeichnis ist kein Build-Ziel
- **WHEN** `--out` auf ein nicht leeres Verzeichnis zeigt, das keine Markierungsdatei eines früheren Builds enthält
- **THEN** bricht der Build ab, ohne etwas zu löschen

### Requirement: Inhalte prüfen
`flauschiversum check` SHALL alle Inhalte prüfen, ohne etwas zu schreiben: Frontmatter, doppelte URLs, Bildreferenzen und interne Links. Fehler beenden das Programm mit einem Exit-Code ungleich 0. Warnungen werden nur ausgegeben. `build` führt dieselben Prüfungen aus.

#### Scenario: Kaputter interner Link
- **WHEN** ein Post auf `/2014/01/gibts-nicht/` verlinkt und diese URL nicht erzeugt wird
- **THEN** meldet `check` eine Warnung mit Post und Link

#### Scenario: Alles in Ordnung
- **WHEN** `check` auf den aktuellen Inhalten läuft
- **THEN** endet es mit Exit-Code 0

### Requirement: Vorschau-Server
`flauschiversum serve` SHALL die Seite bauen, sie unter `http://localhost:<Port>` (Standard 8080) ausliefern und bei jeder Änderung unter `content/` oder `assets/` neu bauen. Seiten im Browser laden danach automatisch neu. Änderungen an Templates erfordern ein Neukompilieren.

#### Scenario: Post bearbeiten
- **WHEN** während `serve` läuft ein Satz in einer `index.md` geändert und gespeichert wird
- **THEN** zeigt der Browser die geänderte Seite nach wenigen Sekunden ohne manuelles Neuladen

### Requirement: Templates beim Kompilieren geprüft
Die HTML-Templates SHALL beim Kompilieren des Generators geprüft und eingebunden werden. Ein Syntaxfehler oder ein Verweis auf ein nicht vorhandenes Feld verhindert das Kompilieren, statt erst beim Rendern aufzufallen. Zur Laufzeit liest der Generator keine Template-Dateien.

#### Scenario: Tippfehler im Template
- **WHEN** im Post-Template `{{ post.titel }}` statt `{{ post.title }}` steht
- **THEN** schlägt `cargo build` mit einer Fehlermeldung fehl, die das Template nennt

### Requirement: Stylesheet ohne Präprozessor
Die Stylesheet-Quellen SHALL Standard-CSS sein, ohne LESS, Sass oder einen anderen Präprozessor. Der Build fasst sie nur zusammen (`@import`) und minimiert sie. Ein separates Werkzeug wie Node.js ist dafür nicht nötig.

#### Scenario: Farbe zentral ändern
- **WHEN** in `content/style/theme.css` der Wert von `--theme-green` geändert wird
- **THEN** übernehmen alle Stellen, die `var(--theme-green)` oder daraus abgeleitete Farben (relative Farbsyntax) verwenden, nach dem Build die neue Farbe

### Requirement: Verständliche Fehlermeldungen
Fehler und Warnungen SHALL die betroffene Datei (relativer Pfad) und, wo möglich, Zeile oder Feld nennen, und zwar auf Deutsch.

#### Scenario: Ungültiges Datum
- **WHEN** eine `index.md` `date: 31.10.2020` enthält
- **THEN** nennt die Fehlermeldung den Pfad der Datei, das Feld `date` und das erwartete Format `YYYY-MM-DD`

### Requirement: Suchindex im Build
Der Build SHALL aus allen veröffentlichten Posts und Einzelseiten den Suchindex erzeugen und seine Dateien wie alle anderen Ausgaben ins Ausgabeverzeichnis schreiben und im Manifest führen. Der Index kommt aus dem Speicher des Builds, ohne zusätzliches Programm und ohne Zwischenverzeichnis.

#### Scenario: Ein Befehl genügt
- **WHEN** `flauschiversum build` auf einem frischen Checkout läuft
- **THEN** enthält das Ausgabeverzeichnis danach `pagefind/` mit dem Suchindex, ohne dass ein weiteres Programm gestartet wurde

#### Scenario: Verwaister Index
- **WHEN** ein Post gelöscht und neu gebaut wird
- **THEN** sind die nur von ihm benötigten Indexdateien aus `pagefind/` entfernt

### Requirement: Optionen für Basis-URL und Staging
`build`, `check` und `serve` SHALL die Optionen `--base-url <URL>` und `--staging <BRANCH>` annehmen. `--staging` schaltet den Staging-Modus ein, sein Wert ist der Branchname für die Staging-Leiste. Beide Optionen ändern nur Absolut-URLs und die Kennzeichnung der Seite, nicht die Inhalte. Die Basis-URL kann auch über `FLAUSCHI_BASE_URL` gesetzt werden; die Option hat Vorrang.

#### Scenario: Staging bauen
- **WHEN** `flauschiversum build --base-url https://staging.flauschiversum.de --staging claude/beautiful-darwin-66rwab` läuft
- **THEN** entsteht eine Seite mit Staging-Absolut-URLs, `noindex` und der Staging-Leiste mit dem Branchnamen

#### Scenario: Staging ohne Branchnamen
- **WHEN** `--staging` ohne Wert oder mit leerem Wert angegeben wird
- **THEN** bricht der Build ab und nennt, dass ein Branchname erwartet wird

#### Scenario: Staging lokal ansehen
- **WHEN** `flauschiversum serve --staging mein-branch` läuft
- **THEN** zeigt die Vorschau auf `http://localhost:<Port>/` die Staging-Leiste mit „mein-branch“

### Requirement: Fortschrittsmeldungen
`flauschiversum build`, `serve` (bei jedem Neubau) und `check` SHALL auf der Fehlerausgabe zu jedem Arbeitsschritt melden, woran sie gerade arbeiten (Inhalte lesen, Bilder verarbeiten, Beiträge rendern, Suchindex bauen, Dateien schreiben und so weiter). Jede Meldung nennt die seit dem Start verstrichene Zeit, und die Meldung des nächsten Schrittes nennt die Dauer des vorherigen, wenn sie länger als 0,5 Sekunden war.

#### Scenario: Kaltstart
- **WHEN** `flauschiversum build` mit leerem Cache gestartet wird
- **THEN** erscheint sofort die erste Meldung, und jeder weitere Schritt meldet sich beim Beginn mit der verstrichenen Zeit

#### Scenario: Zusammenfassung bleibt
- **WHEN** ein Build endet
- **THEN** stehen auf der Standardausgabe weiterhin die beiden Zusammenfassungszeilen mit Zahl der Beiträge, Bilder und Dateien

### Requirement: Zwischenstand bei Bildern und Suchindex
Bei der Verarbeitung der Medien SHALL der Generator höchstens alle 2 Sekunden einen Zwischenstand mit verarbeiteter Anzahl, Gesamtzahl, Prozent und der Aufteilung in neu berechnet und aus dem Cache melden, und beim letzten Medium immer. Beim Suchindex SHALL er melden, ob er aus dem Cache übernommen oder neu berechnet wird, und im zweiten Fall darauf hinweisen, dass es dauern kann.

#### Scenario: Zwischenstand
- **WHEN** mehr als zwei Sekunden lang Bilder verarbeitet werden
- **THEN** erscheint etwa alle zwei Sekunden eine Zeile wie `Bilder und Medien: 1298/3197 (40 %) · 1297 neu berechnet, 1 aus dem Cache`

#### Scenario: Suchindex aus dem Cache
- **WHEN** der Suchindex unverändert im Cache liegt
- **THEN** meldet der Generator, dass er aus dem Cache übernommen wurde

#### Scenario: Suchindex neu
- **WHEN** der Suchindex neu berechnet werden muss
- **THEN** meldet der Generator das vor dem Start der Berechnung mit dem Hinweis, dass es einige Minuten dauern kann

### Requirement: Tests und Bibliothek bleiben still
Die Fortschrittsmeldungen SHALL nur von den Kommandos ausgelöst werden; Tests und Aufrufe von `generate` ohne Kommando erzeugen keine Ausgabe.

#### Scenario: Test-Build
- **WHEN** ein Test die Seite über `generate` baut
- **THEN** werden keine Fortschrittsmeldungen ausgegeben

### Requirement: Weiterleitungen prüfen und erzeugen
Der Build SHALL die Datei `content/redirects.yaml` (Liste `von: nach`, jeweils interne Pfade mit führendem und abschließendem `/`) auswerten und für jeden Eintrag unter dem alten Pfad eine Weiterleitungsseite erzeugen. Verweist ein Eintrag auf sich selbst, hat ein Ziel, das nicht existiert, oder liegt die alte Adresse auf einer vorhandenen Seite, bricht der Build mit der Angabe des Eintrags ab. Die Datei darf fehlen.

#### Scenario: Gültige Weiterleitung
- **WHEN** `content/redirects.yaml` den Eintrag `/kategorie/basteln/: /tag/basteln/` enthält
- **THEN** gibt es unter `/kategorie/basteln/` eine Weiterleitungsseite auf `/tag/basteln/`

#### Scenario: Ziel existiert nicht
- **WHEN** ein Eintrag auf `/tag/gibtsnicht/` verweist
- **THEN** bricht der Build ab und nennt den Eintrag

#### Scenario: Alte Adresse belegt
- **WHEN** die alte Adresse eines Eintrags bereits eine Seite hat
- **THEN** bricht der Build ab und nennt die Adresse

#### Scenario: Keine Datei
- **WHEN** `content/redirects.yaml` fehlt
- **THEN** baut der Build ohne Weiterleitungen und ohne Fehler

### Requirement: Schlagworte prüfen
Der Build SHALL jedes Schlagwort eines Posts gegen `content/tags.yaml` prüfen. Ein unbekanntes Schlagwort bricht den Build ab und nennt Post, Wert und – wenn vorhanden – das ähnlichste erlaubte Schlagwort. Doppelte Schlagworte in einem Post und Schlagworte, die weniger als zwei Posts tragen, werden als Warnung gemeldet. Hat ein Post noch ein Feld `category`, meldet der Build eine Warnung mit dem Hinweis, den Wert als Schlagwort zu führen; das Feld wird sonst ignoriert.

#### Scenario: Tippfehler im Schlagwort
- **WHEN** ein Post `tags: [Kazen]` angibt und „Katzen“ erlaubt ist
- **THEN** bricht der Build ab und nennt den Post, „Kazen“ und den Vorschlag „Katzen“

#### Scenario: Altes Feld category
- **WHEN** ein Post noch `category: basteln` im Frontmatter hat
- **THEN** baut der Build weiter und meldet eine Warnung, die Post und Feld nennt und rät, „basteln“ in `tags` zu führen

#### Scenario: Schlagwort nur einmal benutzt
- **WHEN** ein erlaubtes Schlagwort nur von einem Post getragen wird
- **THEN** meldet der Build eine Warnung, und die Seite des Schlagworts wird trotzdem erzeugt
