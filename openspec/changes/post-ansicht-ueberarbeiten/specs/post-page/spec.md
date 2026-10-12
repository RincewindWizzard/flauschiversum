## MODIFIED Requirements

### Requirement: Post-Kopf
Unter dem gemeinsamen Kopfbereich SHALL die Post-Seite den Titel des Posts als Überschrift zeigen, darunter das Veröffentlichungsdatum im Format „<Tag>. <Monatsname> <Jahr>“ als `<time datetime>` in gut lesbarer Gestalt (mindestens Schriftgröße des Fließtexts, Kontrast mindestens 4,5 : 1 in Hell und Dunkel), darunter das Vorschaubild (Größe `200x`, nur wenn `image` gesetzt ist). Der Autor steht nicht im Kopf, sondern am Ende des Beitrags.

#### Scenario: Post mit Autor und Bild
- **WHEN** die Seite des Posts „Gib´s mir!… - Audrey II“ mit `author: Tomcat` und `image: "Audrey1.jpg"` geöffnet wird
- **THEN** zeigt der Kopf die Überschrift „Gib´s mir!… - Audrey II“ und das Bild `200x/Audrey1.jpg`, aber nicht „Von Tomcat“

#### Scenario: Datum über dem Beitrag
- **WHEN** die Seite des Posts „Kürbis“ vom 31. Oktober 2020 geöffnet wird
- **THEN** steht „31. Oktober 2020“ unter dem Titel und vor dem Beitragstext, in mindestens der Schriftgröße des Fließtexts und mit Kontrast mindestens 4,5 : 1

### Requirement: Kontakt per E-Mail
Die Post-Seite SHALL am Ende des Beitrags, nach dem Text und der Zeile „Von <Autor>“ und vor der Karte „Ähnliche Beiträge“, den Hinweis „Schreibe Ihr eine Nachricht!“ zeigen, nicht im Kopf. „Nachricht“ ist dabei ein `mailto:`-Link an `ermeline@flauschiversum.de` mit dem Betreff „[Flauschiversum] <Titel des Posts>“.

#### Scenario: Nachricht zum Post
- **WHEN** auf der Seite des Posts „Alraune“ auf „Nachricht“ geklickt wird
- **THEN** öffnet sich eine E-Mail an `ermeline@flauschiversum.de` mit dem Betreff „[Flauschiversum] Alraune“

#### Scenario: Am Ende des Beitrags
- **WHEN** die Seite eines Posts geöffnet wird
- **THEN** steht der Hinweis nach dem letzten Absatz des Textes, nicht im Kopf über dem Beitrag

### Requirement: Ähnliche Beiträge auf der Seite
Die Post-Seite SHALL nach dem Kontakt-Hinweis und vor der Navigation zum älteren und neueren Beitrag eine Karte „Ähnliche Beiträge“ zeigen. Die Einträge sind genauso aufgebaut wie die Einträge der Beitragsliste der Startseite (Vorschaubild rund, Titel als Link, Datum, Kurztext). Gibt es keine, entfällt die Karte.

#### Scenario: Karte vorhanden
- **WHEN** zu einem Beitrag ähnliche Beiträge existieren
- **THEN** zeigt die Seite die Karte „Ähnliche Beiträge“, und jeder Eintrag hat denselben Aufbau und dieselbe Gestalt wie ein Eintrag auf der Startseite

#### Scenario: Keine ähnlichen Beiträge
- **WHEN** ein Beitrag kein Schlagwort mit einem anderen Beitrag teilt
- **THEN** zeigt die Seite keine Karte
