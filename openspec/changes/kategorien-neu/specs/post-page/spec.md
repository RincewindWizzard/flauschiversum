## ADDED Requirements

### Requirement: Ähnliche Beiträge auf der Seite
Die Post-Seite SHALL nach dem Autor und vor der Navigation zum älteren und neueren Beitrag eine Karte „Ähnliche Beiträge“ mit den Links zu den ähnlichen Beiträgen zeigen (Auswahl nach Capability `related-posts`). Gibt es keine, entfällt die Karte.

#### Scenario: Karte vorhanden
- **WHEN** zu einem Beitrag ähnliche Beiträge existieren
- **THEN** zeigt die Seite die Karte „Ähnliche Beiträge“ mit Titeln als Links

#### Scenario: Keine ähnlichen Beiträge
- **WHEN** ein Beitrag kein Schlagwort mit einem anderen Beitrag teilt
- **THEN** zeigt die Seite keine Karte
