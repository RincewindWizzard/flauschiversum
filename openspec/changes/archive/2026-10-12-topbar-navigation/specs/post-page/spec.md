# Spec Delta

## ADDED Requirements

### Requirement: Schlagworte am Beitrag
Die Post-Seite SHALL im Kopf unter dem Vorschaubild bzw. dem Titel die Schlagworte (falls vorhanden) als Links auf `/tag/<name>/` zeigen. Eine Kategorie gibt es nicht.

#### Scenario: Schlagworte
- **WHEN** der Post „Alraune“ die Schlagworte „Basteln“, „LARP“ und „Hexen“ trägt
- **THEN** zeigt die Seite „Basteln“, „LARP“ und „Hexen“ mit Links auf `/tag/basteln/`, `/tag/larp/` und `/tag/hexen/`

#### Scenario: Ohne Schlagworte
- **WHEN** ein Post keine Schlagworte hat
- **THEN** zeigt der Kopf keine Zeile zur Einordnung

## REMOVED Requirements

### Requirement: Einordnung des Beitrags
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Schlagworte am Beitrag“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Schlagworte am Beitrag“.
