# post-page Specification

## Purpose

Beschreibt die Seite eines einzelnen Posts: Kopf mit Titel, Vorschaubild, Autor und Kontaktmöglichkeit sowie den aus Markdown erzeugten Inhalt.

## Requirements

### Requirement: Post-Kopf
Unter dem gemeinsamen Kopfbereich SHALL die Post-Seite den Titel des Posts als Überschrift zeigen, darunter das Vorschaubild (Größe `200x`, nur wenn `image` gesetzt ist) und die Zeile „Von <author>“. Der Autorenname verlinkt auf `/about/`.

#### Scenario: Post mit Autor und Bild
- **WHEN** die Seite des Posts „Gib´s mir!… - Audrey II“ mit `author: Tomcat` und `image: "Audrey1.jpg"` geöffnet wird
- **THEN** zeigt sie die Überschrift „Gib´s mir!… - Audrey II“, das Bild `200x/Audrey1.jpg` und „Von Tomcat“ mit Link auf `/about/`

### Requirement: Kontakt per E-Mail
Die Post-Seite SHALL den Hinweis „Schreibe Ihr eine Nachricht!“ zeigen. „Nachricht“ ist dabei ein `mailto:`-Link an `ermeline@flauschiversum.de` mit dem Betreff „[Flauschiversum] <Titel des Posts>“.

#### Scenario: Nachricht zum Post
- **WHEN** auf der Seite des Posts „Alraune“ auf „Nachricht“ geklickt wird
- **THEN** öffnet sich eine E-Mail an `ermeline@flauschiversum.de` mit dem Betreff „[Flauschiversum] Alraune“

### Requirement: Post-Inhalt
Der Markdown-Inhalt des Posts SHALL als HTML im Hauptbereich der Seite erscheinen. Überschriften, Absätze, Listen, Links und eingebettetes HTML bleiben erhalten. Bilder werden nach der Spec `post-images` dargestellt.

#### Scenario: Interner Link im Text
- **WHEN** der Post-Text `[Wolle&Hobby](/2013/07/wolle-hobby/)` enthält
- **THEN** enthält die Seite einen Link mit dem Text „Wolle&Hobby“ auf `/2013/07/wolle-hobby/`

### Requirement: Excerpt nur in Übersichten
Das Frontmatter-Feld `excerpt` SHALL nicht automatisch auf der Post-Seite erscheinen. Es wird nur auf den Indexseiten und im Feed verwendet.

#### Scenario: Excerpt nicht doppelt
- **WHEN** eine Post-Seite geöffnet wird
- **THEN** erscheint der Text aus `excerpt` nur, wenn er auch im Markdown-Inhalt steht

### Requirement: Nur veröffentlichte Posts
Für einen Post mit einem Datum in der Zukunft (bezogen auf den Zeitpunkt des Builds) SHALL weder eine Seite noch eine Bildvariante noch ein anderes Medium veröffentlicht werden. Er erscheint erst nach einem Build, der nach seinem Datum stattfindet.

#### Scenario: Geplanter Post
- **WHEN** ein Post das Datum von morgen hat und heute gebaut wird
- **THEN** existiert seine URL im Ausgabeverzeichnis nicht

#### Scenario: Geplanter Post wird fällig
- **WHEN** derselbe Post nach seinem Datum erneut gebaut wird
- **THEN** ist er unter seiner URL erreichbar und erscheint auf den Indexseiten

### Requirement: Navigation zwischen Posts
Unter dem Inhalt SHALL die Post-Seite Links auf den nächstälteren und den nächstneueren veröffentlichten Post zeigen, jeweils mit deren Titel. Beim ältesten bzw. neuesten Post entfällt der jeweilige Link.

#### Scenario: Neuester Post
- **WHEN** die Seite des neuesten Posts geöffnet wird
- **THEN** gibt es einen Link auf den nächstälteren Post und keinen auf einen neueren
