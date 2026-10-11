# Spec Delta

## ADDED Requirements

### Requirement: Einordnung des Beitrags
Die Post-Seite SHALL im Kopf unter dem Autor die Kategorie (falls vorhanden) als Link auf `/kategorie/<name>/` und die Schlagworte (falls vorhanden) als Links auf `/tag/<name>/` zeigen.

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
