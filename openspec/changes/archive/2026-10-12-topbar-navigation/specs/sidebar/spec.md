# Spec Delta

## MODIFIED Requirements

### Requirement: Tag-Cloud
Die Startseite SHALL alle Schlagworte mit mindestens einem veröffentlichten Beitrag als Tag-Cloud auf einer Karte zeigen: alphabetisch nach deutscher Sortierung (Umlaute wie ihre Grundbuchstaben, ohne Beachtung der Groß-/Kleinschreibung), jedes als ruhiger Chip und Link auf `/tag/<name>/`, mit der Zahl der Beiträge klein und gedämpft im Chip und im Titel des Links („Hüte: 21 Beiträge“). Am Ende steht ein Link „Alle Themen“ auf `/themen/`. Auf keiner anderen Seite erscheint die Tag-Cloud.

#### Scenario: Alphabetische Reihenfolge
- **WHEN** die Schlagworte „Taschen“, „Hüte“ und „Katzen“ existieren
- **THEN** stehen sie in der Reihenfolge „Hüte“, „Katzen“, „Taschen“

#### Scenario: Schlagwort ohne veröffentlichten Beitrag
- **WHEN** ein Schlagwort nur von einem geplanten Beitrag getragen wird
- **THEN** erscheint es nicht in der Cloud

#### Scenario: Link zur Übersicht
- **WHEN** die Startseite angezeigt wird
- **THEN** führt „Alle Themen“ auf `/themen/`

#### Scenario: Nur auf der Startseite
- **WHEN** eine Beitragsseite, eine Themenseite oder `/beitraege/` geöffnet wird
- **THEN** gibt es dort keine Tag-Cloud

## REMOVED Requirements

### Requirement: Kategorien in der Seitenspalte
**Reason**: Es gibt keine Kategorien mehr, Themen sind Schlagworte (Link „Themen“ in der Topbar); Seitenspalten gibt es auch nicht mehr.
**Migration**: Themen über „Themen“ in der Topbar erreichen.

### Requirement: Sidebar ohne JavaScript und ohne Dienste von Dritten
**Reason**: Die Sidebar entfällt. Die Tag-Cloud auf der Startseite entsteht weiterhin im Build und ohne JavaScript; das folgt aus „Inhalte ohne JavaScript nutzbar“ und „Keine Ressourcen von Drittanbietern“.
**Migration**: Keine.
