# build Specification

## Purpose

Beschreibt den Aufruf des Generators und sein Ergebnis: aus den Quellen in `src/` entsteht ein vollständiger, statisch auslieferbarer Verzeichnisbaum der Webseite.

## Requirements

### Requirement: Aufruf
Der Generator SHALL im Wurzelverzeichnis des Repositorys mit `python3 main.py` gestartet werden. Optional sind `--dst <Pfad>` (Ausgabeverzeichnis, Standard `build/`), `--cache <Pfad>` (Bild-Cache, Standard `.cache/`) und `--clean <Wert>` (Ausgabeverzeichnis vorher leeren).

#### Scenario: Build mit eigenen Pfaden
- **WHEN** `python3 main.py --dst /var/www/flauschiversum --cache /var/share/flauschiversum-cache` ausgeführt wird
- **THEN** liegt die fertige Seite in `/var/www/flauschiversum`, und die Bildvarianten sind in `/var/share/flauschiversum-cache` zwischengespeichert

### Requirement: Vollständiger statischer Verzeichnisbaum
Der Build SHALL alle in `url-scheme` genannten URLs als Dateien im Ausgabeverzeichnis erzeugen. Eine URL, die auf `/` endet, wird als `index.html` in dem entsprechenden Verzeichnis abgelegt. Für die Auslieferung braucht es nur einen Webserver für statische Dateien.

#### Scenario: Post als Verzeichnis
- **WHEN** der Post mit der URL `/2014/01/alraune/` gebaut wird
- **THEN** existiert `<dst>/2014/01/alraune/index.html`

#### Scenario: Bildvariante als Datei
- **WHEN** der Post `/2014/01/alraune/` das Bild `DSCF0527.jpg` enthält
- **THEN** existiert `<dst>/2014/01/alraune/700x/DSCF0527.jpg`

### Requirement: Statische Dateien kopieren
Der Build SHALL alle Dateien aus `src/static/` byte-genau an ihren Pfad relativ zu `src/static/` in das Ausgabeverzeichnis kopieren.

#### Scenario: Skript kopieren
- **WHEN** `src/static/script/slideshow.js` existiert
- **THEN** ist `<dst>/script/slideshow.js` eine identische Kopie

### Requirement: Stylesheet erzeugen
Der Build SHALL `src/style/main.less` samt seinen Importen zu einer einzigen CSS-Datei `<dst>/style.css` übersetzen.

#### Scenario: Gestaltung geändert
- **WHEN** in `src/style/lib/theme.less` eine Farbe geändert und neu gebaut wird
- **THEN** enthält `<dst>/style.css` die neue Farbe

### Requirement: Wiederholter Build
Ein erneuter Build ohne Änderungen an den Quellen SHALL dieselbe Menge an Dateien im Ausgabeverzeichnis ergeben.

#### Scenario: Erneuter Build
- **WHEN** zweimal hintereinander ohne Änderungen gebaut wird
- **THEN** enthält das Ausgabeverzeichnis nach dem zweiten Build dieselben Dateien wie nach dem ersten
