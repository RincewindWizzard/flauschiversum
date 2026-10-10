# content-format Specification

## Purpose

Beschreibt, wie Inhalte des Flauschiversums als Dateien vorliegen (Posts, Seiten, Bilder), damit die vorhandenen 194 Posts und 3 Seiten ohne inhaltliche Änderung weiter verarbeitet werden können.

## Requirements

### Requirement: Post als Verzeichnis
Ein Post SHALL ein Verzeichnis unter `src/posts/` sein, das genau eine Datei `index.md` und die zugehörigen Bilddateien enthält. Der Name des Verzeichnisses hat keinen Einfluss auf die Darstellung oder die URL des Posts.

#### Scenario: Post wird erkannt
- **WHEN** `src/posts/alraune/index.md` existiert
- **THEN** wird daraus ein Post erzeugt, und die Bilder in `src/posts/alraune/` gehören zu diesem Post

#### Scenario: Verzeichnisname ist unerheblich
- **WHEN** das Verzeichnis `src/posts/wollfass/` einen Post mit dem Titel „Das Wollfass“ enthält
- **THEN** richtet sich die URL nach dem Titel und nicht nach `wollfass`

### Requirement: Frontmatter eines Posts
Die `index.md` eines Posts SHALL mit einem YAML-Frontmatter beginnen. Pflichtfelder sind `title` (Text) und `date` (`YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`). Optional sind `author` (Text), `image` (Dateiname eines Bildes im Post-Verzeichnis), `excerpt` (Kurztext) und `category`.

#### Scenario: Datum mit Uhrzeit
- **WHEN** das Frontmatter `date: 2014-01-04 19:10:28` enthält
- **THEN** gilt der Post als am 4. Januar 2014 veröffentlicht

#### Scenario: Datum ohne Uhrzeit
- **WHEN** das Frontmatter `date: 2020-10-31` enthält
- **THEN** gilt der Post als am 31. Oktober 2020 veröffentlicht

#### Scenario: Kategorie ohne Wirkung
- **WHEN** ein Post `category: filzen` angibt
- **THEN** hat das keinen sichtbaren Einfluss auf die erzeugte Seite

### Requirement: Markdown-Inhalt
Der Text nach dem Frontmatter SHALL als Markdown interpretiert werden. Eingebettetes HTML (z.B. `<br/>`, `<br></br>`) SHALL unverändert übernommen werden. Die Dateien sind UTF-8-kodiert.

#### Scenario: Eingebettetes HTML
- **WHEN** eine Seite `Nadja Bender<br/>` enthält
- **THEN** erscheint im erzeugten HTML ein Zeilenumbruch an dieser Stelle

### Requirement: Bilder eines Posts
Als Bilder eines Posts SHALL alle Dateien mit der Endung `.jpg` oder `.JPG` im Post-Verzeichnis gelten. Dateinamen können Umlaute und Leerzeichen enthalten (z.B. `kürbis_thumb.jpg`, `Schwacher Moment.jpg`). Andere Dateien im Post-Verzeichnis (`.jpeg`, `.mp4`, `.xmp`, `.xcf`, `.html`) werden nicht veröffentlicht.

#### Scenario: Bild mit Leerzeichen im Namen
- **WHEN** `src/posts/winter-is-coming/Schwacher Moment.jpg` existiert
- **THEN** wird es als Bild dieses Posts veröffentlicht

#### Scenario: Nicht unterstützte Datei
- **WHEN** ein Post-Verzeichnis eine `.xcf`-Datei enthält
- **THEN** wird diese Datei nicht veröffentlicht

### Requirement: Bildreferenzen im Markdown
Bilder SHALL im Markdown eines Posts mit `![Alt-Text](Dateiname)` und einem relativen Dateinamen aus dem Post-Verzeichnis eingebunden werden. Der Alt-Text ist die Bildbeschreibung.

#### Scenario: Bild einbinden
- **WHEN** ein Post `![Alraune oben](DSCF0527.jpg)` enthält
- **THEN** wird das Bild `DSCF0527.jpg` des Posts mit der Beschreibung „Alraune oben“ angezeigt

### Requirement: Bildgruppen
Bilder in direkt aufeinanderfolgenden Zeilen ohne Leerzeile dazwischen (ein Markdown-Absatz) SHALL eine zusammengehörige Bildgruppe bilden. Eine Leerzeile beendet die Gruppe.

#### Scenario: Vier Bilder hintereinander
- **WHEN** vier Zeilen `![…](…)` ohne Leerzeile aufeinander folgen
- **THEN** bilden sie eine Bildgruppe mit vier Bildern

#### Scenario: Durch Text getrennt
- **WHEN** zwischen zwei Bildzeilen ein Textabsatz steht
- **THEN** gehören die beiden Bilder zu verschiedenen Gruppen

### Requirement: Einzelseiten
Eine Einzelseite SHALL eine Markdown-Datei unter `src/pages/` mit einem Frontmatter-Feld `title` sein. Der Dateiname hat keinen Einfluss auf die URL. Bilder auf Seiten werden mit absoluten Pfaden auf statische Dateien eingebunden (z.B. `/images/Ermeline.jpg`).

#### Scenario: Seite mit statischem Bild
- **WHEN** `src/pages/autoren.md` den Titel „About“ hat und `![Ermeline](/images/Ermeline.jpg)` enthält
- **THEN** wird eine Seite „About“ erzeugt, die das statische Bild `/images/Ermeline.jpg` zeigt

### Requirement: Statische Dateien und Stylesheet-Quellen
Dateien unter `src/static/` SHALL unverändert veröffentlicht werden. Die Gestaltung SHALL in LESS-Dateien unter `src/style/` mit dem Einstieg `src/style/main.less` vorliegen.

#### Scenario: Schrift als statische Datei
- **WHEN** `src/static/fonts/Lobster.otf` existiert
- **THEN** wird die Datei unverändert veröffentlicht
