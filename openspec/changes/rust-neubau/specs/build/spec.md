# Spec Delta

## MODIFIED Requirements

### Requirement: Aufruf
Der Generator SHALL ein einzelnes Programm `flauschiversum` sein, das ohne Python, Node.js oder andere externe Programme läuft. `flauschiversum build` baut die Seite. Optional sind `--root <Pfad>` (Projektverzeichnis, Standard: aktuelles Verzeichnis), `--out <Pfad>` (Ausgabeverzeichnis, Standard `dist/`) und `--cache <Pfad>` (Bild-Cache, Standard `.cache/`). Relative Pfade beziehen sich auf das Projektverzeichnis.

#### Scenario: Build mit eigenen Pfaden
- **WHEN** `flauschiversum build --out /tmp/site --cache /tmp/cache` im Repository ausgeführt wird
- **THEN** liegt die fertige Seite in `/tmp/site`, und die Bildvarianten sind in `/tmp/cache` zwischengespeichert

#### Scenario: Aufruf aus einem anderen Verzeichnis
- **WHEN** `flauschiversum build --root ~/flauschiversum` in einem beliebigen Verzeichnis ausgeführt wird
- **THEN** wird die Seite aus `~/flauschiversum/content` nach `~/flauschiversum/dist` gebaut

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

## ADDED Requirements

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
- **THEN** übernehmen alle Stellen, die `var(--theme-green)` oder daraus abgeleitete `color-mix()`-Farben verwenden, nach dem Build die neue Farbe

### Requirement: Verständliche Fehlermeldungen
Fehler und Warnungen SHALL die betroffene Datei (relativer Pfad) und, wo möglich, Zeile oder Feld nennen, und zwar auf Deutsch.

#### Scenario: Ungültiges Datum
- **WHEN** eine `index.md` `date: 31.10.2020` enthält
- **THEN** nennt die Fehlermeldung den Pfad der Datei, das Feld `date` und das erwartete Format `YYYY-MM-DD`
