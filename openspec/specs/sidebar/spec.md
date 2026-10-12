# sidebar Specification

## Purpose

Beschreibt die Seitenspalten des Flauschiversums: die Tag-Cloud, mit der Besucher von jeder Seite aus zu Themen springen können.

## Requirements

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

### Requirement: Dezente Gewichtung der Tag-Cloud
Die Häufigkeit eines Schlagworts SHALL in der Tag-Cloud nur zurückhaltend sichtbar sein: über die angezeigte Zahl der Beiträge, eine leicht wachsende Schriftgröße (fünf Stufen von 0,9 bis 1,1 em) und eine schwache Tönung des Chips. Alle Einträge haben dieselbe Textfarbe. Die Stufe richtet sich nach dem Logarithmus der Anzahl zwischen dem seltensten (Stufe 1) und dem häufigsten Schlagwort (Stufe 5); haben alle gleich viele Beiträge, gilt die mittlere Stufe 3.

#### Scenario: Größe nach Häufigkeit
- **WHEN** „Hüte“ 21 und „Spinnen“ 9 Beiträge haben
- **THEN** ist „Hüte“ größer dargestellt als „Spinnen“

#### Scenario: Alle Schlagworte gleich häufig
- **WHEN** alle Schlagworte gleich viele Beiträge haben
- **THEN** haben alle dieselbe mittlere Größe, und es gibt keinen Fehler durch Division durch null

#### Scenario: Ruhiges Gesamtbild
- **WHEN** die Cloud angezeigt wird
- **THEN** unterscheiden sich die Schriftgrößen höchstens zwischen 0,9 und 1,1 em, alle Einträge haben dieselbe Textfarbe, und jeder Eintrag zeigt die Zahl seiner Beiträge
