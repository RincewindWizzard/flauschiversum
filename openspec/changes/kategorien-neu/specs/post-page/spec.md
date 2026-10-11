## REMOVED Requirements

### Requirement: Einordnung des Beitrags
**Reason**: Die Kategorie ist Pflicht, die Fälle ohne Kategorie entfallen; ersetzt durch „Einordnung mit Kategorie“.
**Migration**: Siehe ADDED Requirement „Einordnung mit Kategorie“.

## ADDED Requirements

### Requirement: Einordnung mit Kategorie
Die Post-Seite SHALL im Kopf unter dem Vorschaubild bzw. dem Titel die Kategorie als Link auf `/kategorie/<name>/` und die Schlagworte (falls vorhanden) als Links auf `/tag/<name>/` zeigen.

#### Scenario: Kategorie und Schlagworte
- **WHEN** der Post „Alraune“ in der Kategorie „Basteln & Upcycling“ liegt und die Schlagworte „LARP“ und „Hexen“ trägt
- **THEN** zeigt die Seite „Basteln & Upcycling“ mit Link auf `/kategorie/basteln/` und „LARP“ und „Hexen“ mit Links auf `/tag/larp/` und `/tag/hexen/`

#### Scenario: Ohne Schlagworte
- **WHEN** ein Post keine Schlagworte hat
- **THEN** zeigt die Seite nur die Kategorie

### Requirement: Ähnliche Beiträge auf der Seite
Die Post-Seite SHALL nach dem Autor und vor der Navigation zum älteren und neueren Beitrag eine Karte „Ähnliche Beiträge“ mit den Links zu den ähnlichen Beiträgen zeigen (Auswahl nach Capability `related-posts`). Gibt es keine, entfällt die Karte.

#### Scenario: Karte vorhanden
- **WHEN** zu einem Beitrag ähnliche Beiträge existieren
- **THEN** zeigt die Seite die Karte „Ähnliche Beiträge“ mit Titeln als Links
