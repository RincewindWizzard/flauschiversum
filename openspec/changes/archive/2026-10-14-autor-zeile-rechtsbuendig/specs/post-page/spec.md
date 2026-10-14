## MODIFIED Requirements

### Requirement: Autor am Ende des Beitrags
Hat der Post einen Autor, SHALL die Seite nach dem Beitragstext die Zeile „Von <Autor>“ zeigen, wobei der Name auf `/about/` verlinkt. Die Zeile steht nach dem Inhalt und vor den Links zum älteren und neueren Beitrag. Die Zeile SHALL rechtsbündig stehen und nach oben einen sichtbaren Abstand zum Beitragstext haben, der mindestens eine Zeilenhöhe beträgt. Hat der Post keinen Autor, gibt es die Zeile nicht.

#### Scenario: Autor nach dem Text
- **WHEN** die Seite des Posts „Alraune“ mit `author: Ermeline` geöffnet wird
- **THEN** steht „Von Ermeline“ mit Link auf `/about/` nach dem letzten Absatz und vor „älterer Beitrag“

#### Scenario: Rechtsbündig mit Abstand
- **WHEN** die Seite eines Posts mit Autor geöffnet wird
- **THEN** ist die Zeile „Von …“ rechtsbündig, und zwischen dem letzten Absatz des Textes und der Zeile liegt ein Abstand von mehr als einer Zeilenhöhe

#### Scenario: Ohne Autor
- **WHEN** ein Post kein Feld `author` hat
- **THEN** zeigt die Seite keine Zeile „Von …“
