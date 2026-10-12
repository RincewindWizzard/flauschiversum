# post-page Specification

## Purpose

Beschreibt die Seite eines einzelnen Posts: Kopf mit Titel, Vorschaubild, Autor und Kontaktmöglichkeit sowie den aus Markdown erzeugten Inhalt.

## Requirements

### Requirement: Post-Kopf
Unter dem gemeinsamen Kopfbereich SHALL die Post-Seite den Titel des Posts als Überschrift zeigen, darunter das Vorschaubild (Größe `200x`, nur wenn `image` gesetzt ist). Der Autor steht nicht im Kopf, sondern am Ende des Beitrags.

#### Scenario: Post mit Autor und Bild
- **WHEN** die Seite des Posts „Gib´s mir!… - Audrey II“ mit `author: Tomcat` und `image: "Audrey1.jpg"` geöffnet wird
- **THEN** zeigt der Kopf die Überschrift „Gib´s mir!… - Audrey II“ und das Bild `200x/Audrey1.jpg`, aber nicht „Von Tomcat“

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

### Requirement: Autor am Ende des Beitrags
Hat der Post einen Autor, SHALL die Seite nach dem Beitragstext die Zeile „Von <Autor>“ zeigen, wobei der Name auf `/about/` verlinkt. Die Zeile steht nach dem Inhalt und vor den Links zum älteren und neueren Beitrag. Hat der Post keinen Autor, gibt es die Zeile nicht.

#### Scenario: Autor nach dem Text
- **WHEN** die Seite des Posts „Alraune“ mit `author: Ermeline` geöffnet wird
- **THEN** steht „Von Ermeline“ mit Link auf `/about/` nach dem letzten Absatz und vor „älterer Beitrag“

#### Scenario: Ohne Autor
- **WHEN** ein Post kein Feld `author` hat
- **THEN** zeigt die Seite keine Zeile „Von …“

### Requirement: Schlagworte am Beitrag
Die Post-Seite SHALL im Kopf unter dem Vorschaubild bzw. dem Titel die Schlagworte (falls vorhanden) als Links auf `/tag/<name>/` zeigen. Eine Kategorie gibt es nicht.

#### Scenario: Schlagworte
- **WHEN** der Post „Alraune“ die Schlagworte „Basteln“, „LARP“ und „Hexen“ trägt
- **THEN** zeigt die Seite „Basteln“, „LARP“ und „Hexen“ mit Links auf `/tag/basteln/`, `/tag/larp/` und `/tag/hexen/`

#### Scenario: Ohne Schlagworte
- **WHEN** ein Post keine Schlagworte hat
- **THEN** zeigt der Kopf keine Zeile zur Einordnung

### Requirement: Ähnliche Beiträge auf der Seite
Die Post-Seite SHALL nach dem Autor und vor der Navigation zum älteren und neueren Beitrag eine Karte „Ähnliche Beiträge“ mit den Links zu den ähnlichen Beiträgen zeigen (Auswahl nach Capability `related-posts`). Gibt es keine, entfällt die Karte.

#### Scenario: Karte vorhanden
- **WHEN** zu einem Beitrag ähnliche Beiträge existieren
- **THEN** zeigt die Seite die Karte „Ähnliche Beiträge“ mit Titeln als Links

#### Scenario: Keine ähnlichen Beiträge
- **WHEN** ein Beitrag kein Schlagwort mit einem anderen Beitrag teilt
- **THEN** zeigt die Seite keine Karte
