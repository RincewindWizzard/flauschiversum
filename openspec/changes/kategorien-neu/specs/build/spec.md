## ADDED Requirements

### Requirement: Jeder Beitrag hat eine Kategorie
Der Build SHALL jeden Beitrag ohne `category` als Fehler melden. `check` meldet dies ebenfalls und zählt alle betroffenen Beiträge.

#### Scenario: Beitrag ohne Kategorie
- **WHEN** ein Beitrag im Frontmatter keine `category` hat
- **THEN** bricht `build` und `check` mit Fehler ab und nennt den Pfad des Beitrags

#### Scenario: Alle Beiträge haben eine Kategorie
- **WHEN** alle Beiträge eine erlaubte Kategorie haben
- **THEN** läuft `check` ohne Fehler zu diesem Thema durch
