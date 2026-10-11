# Spec Delta

## MODIFIED Requirements

### Requirement: Post-Kopf
Unter dem gemeinsamen Kopfbereich SHALL die Post-Seite den Titel des Posts als Überschrift zeigen, darunter das Vorschaubild (Größe `200x`, nur wenn `image` gesetzt ist). Der Autor steht nicht im Kopf, sondern am Ende des Beitrags.

#### Scenario: Post mit Autor und Bild
- **WHEN** die Seite des Posts „Gib´s mir!… - Audrey II“ mit `author: Tomcat` und `image: "Audrey1.jpg"` geöffnet wird
- **THEN** zeigt der Kopf die Überschrift „Gib´s mir!… - Audrey II“ und das Bild `200x/Audrey1.jpg`, aber nicht „Von Tomcat“

### Requirement: Einordnung des Beitrags
Die Post-Seite SHALL im Kopf unter dem Vorschaubild bzw. dem Titel die Kategorie (falls vorhanden) als Link auf `/kategorie/<name>/` und die Schlagworte (falls vorhanden) als Links auf `/tag/<name>/` zeigen.

#### Scenario: Kategorie und Schlagworte
- **WHEN** der Post „Alraune“ in der Kategorie „Basteln“ liegt und die Schlagworte „LARP“ und „Hexen“ trägt
- **THEN** zeigt die Seite „Basteln“ mit Link auf `/kategorie/basteln/` und „LARP“ und „Hexen“ mit Links auf `/tag/larp/` und `/tag/hexen/`

#### Scenario: Ohne Schlagworte
- **WHEN** ein Post keine Schlagworte hat
- **THEN** zeigt die Seite nur die Kategorie

#### Scenario: Ohne Kategorie
- **WHEN** ein Post keine Kategorie, aber das Schlagwort „Blog“ hat
- **THEN** zeigt die Seite nur „Blog“ mit Link auf `/tag/blog/`

#### Scenario: Ohne Einordnung
- **WHEN** ein Post weder Kategorie noch Schlagworte hat
- **THEN** zeigt der Kopf keine Zeile zur Einordnung

## ADDED Requirements

### Requirement: Autor am Ende des Beitrags
Hat der Post einen Autor, SHALL die Seite nach dem Beitragstext die Zeile „Von <Autor>“ zeigen, wobei der Name auf `/about/` verlinkt. Die Zeile steht nach dem Inhalt und vor den Links zum älteren und neueren Beitrag. Hat der Post keinen Autor, gibt es die Zeile nicht.

#### Scenario: Autor nach dem Text
- **WHEN** die Seite des Posts „Alraune“ mit `author: Ermeline` geöffnet wird
- **THEN** steht „Von Ermeline“ mit Link auf `/about/` nach dem letzten Absatz und vor „älterer Beitrag“

#### Scenario: Ohne Autor
- **WHEN** ein Post kein Feld `author` hat
- **THEN** zeigt die Seite keine Zeile „Von …“
