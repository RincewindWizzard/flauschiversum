# content-format Specification

## Purpose

Beschreibt, wie Inhalte des Flauschiversums als Dateien vorliegen (Posts, Seiten, Bilder), damit die vorhandenen 194 Posts und 3 Seiten ohne inhaltliche Änderung weiter verarbeitet werden können.

## Requirements

### Requirement: Post als Verzeichnis
Ein Post SHALL ein Verzeichnis unter `content/posts/` sein, das genau eine Datei `index.md` und die zugehörigen Mediendateien enthält. Der Verzeichnisname bestimmt die URL nur bei Posts ohne `permalink`.

#### Scenario: Post wird erkannt
- **WHEN** `content/posts/alraune/index.md` existiert
- **THEN** wird daraus ein Post erzeugt, und die Mediendateien in `content/posts/alraune/` gehören zu diesem Post

#### Scenario: Verzeichnisname ist unerheblich
- **WHEN** das Verzeichnis `content/posts/wollfass/` einen Post mit `permalink: /2013/07/das-wollfass/` enthält
- **THEN** richtet sich die URL nach `permalink` und nicht nach `wollfass`

### Requirement: Frontmatter eines Posts
Die `index.md` eines Posts SHALL mit einem YAML-Frontmatter beginnen. Pflichtfelder sind `title` (Text) und `date` (`YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`, Ortszeit Europe/Berlin). Optional sind `permalink` (absolute URL-Pfad mit `/` am Anfang und Ende), `author` (Text), `image` (Dateiname eines Bildes im Post-Verzeichnis), `excerpt` (Kurztext) und `category`. Fehlt ein Pflichtfeld oder hat ein Feld ein ungültiges Format, bricht der Build mit einer Meldung ab, die Datei und Feld nennt.

#### Scenario: Datum mit Uhrzeit
- **WHEN** das Frontmatter `date: 2014-01-04 19:10:28` enthält
- **THEN** gilt der Post als am 4. Januar 2014 um 19:10:28 Uhr deutscher Zeit veröffentlicht

#### Scenario: Datum ohne Uhrzeit
- **WHEN** das Frontmatter `date: 2020-10-31` enthält
- **THEN** gilt der Post als am 31. Oktober 2020 um 0:00 Uhr deutscher Zeit veröffentlicht

#### Scenario: Kategorie ohne Wirkung
- **WHEN** ein Post `category: filzen` angibt
- **THEN** hat das keinen sichtbaren Einfluss auf die erzeugte Seite

#### Scenario: Fehlender Titel
- **WHEN** eine `index.md` kein Feld `title` hat
- **THEN** bricht der Build ab und nennt den Pfad der Datei und das fehlende Feld `title`

#### Scenario: Vorschaubild existiert nicht
- **WHEN** `image` einen Dateinamen nennt, der im Post-Verzeichnis nicht als Bild existiert
- **THEN** bricht der Build ab und nennt Post und Dateinamen

### Requirement: Markdown-Inhalt
Der Text nach dem Frontmatter SHALL als Markdown interpretiert werden. Eingebettetes HTML (z.B. `<br/>`, `<br></br>`) SHALL unverändert übernommen werden. Die Dateien sind UTF-8-kodiert.

#### Scenario: Eingebettetes HTML
- **WHEN** eine Seite `Nadja Bender<br/>` enthält
- **THEN** erscheint im erzeugten HTML ein Zeilenumbruch an dieser Stelle

### Requirement: Bilder eines Posts
Als Bilder eines Posts SHALL alle Dateien mit den Endungen `.jpg`, `.jpeg`, `.png` und `.webp` (unabhängig von Groß-/Kleinschreibung) sowie nicht animierte `.gif` im Post-Verzeichnis gelten. Dateinamen können Umlaute und Leerzeichen enthalten. Begleitdateien (`.xmp`, `.xcf`, `.html` und andere) werden nicht veröffentlicht.

#### Scenario: Bild mit Leerzeichen im Namen
- **WHEN** `content/posts/winter-is-coming/Schwacher Moment.jpg` existiert
- **THEN** wird es als Bild dieses Posts veröffentlicht

#### Scenario: JPEG mit langer Endung
- **WHEN** `content/posts/fur-blumenkinder/20170212_141939.jpeg` existiert
- **THEN** wird es als Bild dieses Posts veröffentlicht

#### Scenario: Nicht unterstützte Datei
- **WHEN** ein Post-Verzeichnis eine `.xcf`-Datei enthält
- **THEN** wird diese Datei nicht veröffentlicht

### Requirement: Bildreferenzen im Markdown
Bilder, Animationen und Videos SHALL im Markdown eines Posts mit `![Alt-Text](Dateiname)` und einem relativen Dateinamen aus dem Post-Verzeichnis eingebunden werden. Der Alt-Text ist die Beschreibung. Verweist eine relative Referenz auf eine Datei, die nicht als Medium des Posts existiert, gibt der Build eine Warnung mit Post und Dateinamen aus.

#### Scenario: Bild einbinden
- **WHEN** ein Post `![Alraune oben](DSCF0527.jpg)` enthält
- **THEN** wird das Bild `DSCF0527.jpg` des Posts mit der Beschreibung „Alraune oben“ angezeigt

#### Scenario: Video einbinden
- **WHEN** ein Post `![Karteikartenfresser in Aktion](VID_20130115_184912.mp4)` enthält
- **THEN** wird an dieser Stelle das Video mit der Beschreibung „Karteikartenfresser in Aktion“ angezeigt

#### Scenario: Tippfehler im Dateinamen
- **WHEN** ein Post `![Foto](DSCF0572.jpg)` enthält, die Datei aber nicht existiert
- **THEN** meldet der Build eine Warnung mit Post und `DSCF0572.jpg`

### Requirement: Bildgruppen
Bilder in direkt aufeinanderfolgenden Zeilen ohne Leerzeile dazwischen (ein Markdown-Absatz) SHALL eine zusammengehörige Bildgruppe bilden. Eine Leerzeile beendet die Gruppe.

#### Scenario: Vier Bilder hintereinander
- **WHEN** vier Zeilen `![…](…)` ohne Leerzeile aufeinander folgen
- **THEN** bilden sie eine Bildgruppe mit vier Bildern

#### Scenario: Durch Text getrennt
- **WHEN** zwischen zwei Bildzeilen ein Textabsatz steht
- **THEN** gehören die beiden Bilder zu verschiedenen Gruppen

### Requirement: Einzelseiten
Eine Einzelseite SHALL eine Markdown-Datei unter `content/pages/` mit einem Frontmatter-Feld `title` sein. Der Dateiname hat keinen Einfluss auf die URL. Bilder auf Seiten werden mit absoluten Pfaden auf statische Dateien eingebunden (z.B. `/images/Ermeline.jpg`).

#### Scenario: Seite mit statischem Bild
- **WHEN** `content/pages/autoren.md` den Titel „About“ hat und `![Ermeline](/images/Ermeline.jpg)` enthält
- **THEN** wird eine Seite „About“ erzeugt, die das statische Bild `/images/Ermeline.jpg` zeigt

### Requirement: Statische Dateien und Stylesheet-Quellen
Dateien unter `content/static/` SHALL unverändert veröffentlicht werden. Die Gestaltung SHALL als normales CSS unter `content/style/` mit dem Einstieg `content/style/main.css` vorliegen. `@import` wird beim Build aufgelöst.

#### Scenario: Schrift als statische Datei
- **WHEN** `content/static/fonts/Lobster.otf` existiert
- **THEN** wird die Datei unverändert veröffentlicht

#### Scenario: Stylesheet aus mehreren Dateien
- **WHEN** `content/style/main.css` `@import "theme.css";` enthält
- **THEN** enthält `/style.css` den Inhalt von `theme.css` an dieser Stelle

### Requirement: Animationen und Videos eines Posts
Als Animationen SHALL animierte `.gif`-Dateien gelten, als Videos Dateien mit den Endungen `.mp4` und `.webm` (unabhängig von Groß-/Kleinschreibung) im Post-Verzeichnis. Sie werden unverändert veröffentlicht. Ist eine solche Datei größer als 20 MB, gibt der Build eine Warnung aus.

#### Scenario: Großes Video
- **WHEN** ein Post ein Video mit 35 MB enthält
- **THEN** wird es veröffentlicht, und der Build warnt mit Dateiname und Größe

### Requirement: Einheitliche Kategorien
Der Wert von `category` SHALL für dieselbe Kategorie in allen Posts gleich geschrieben sein. Bestehende Posts mit `naehen` werden auf `nähen` vereinheitlicht.

#### Scenario: Vereinheitlichte Kategorie
- **WHEN** die Inhalte nach dem Neubau durchsucht werden
- **THEN** kommt `category: naehen` nicht mehr vor
