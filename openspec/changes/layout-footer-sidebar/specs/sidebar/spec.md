# Spec Delta

## Purpose

Beschreibt die Seitenspalten des Flauschiversums: Kategorienliste und Tag-Cloud, mit denen Besucher von jeder Seite aus zu Themen springen können.

## ADDED Requirements

### Requirement: Kategorien in der Seitenspalte
Die Seitenspalten SHALL die Kategorien aus `content/categories.yaml` in der Reihenfolge der Datei als Liste mit Links auf `/kategorie/<name>/` und der Anzahl der veröffentlichten Beiträge zeigen: ab 80 em in der linken Navigationsspalte unter den Seitenlinks, darunter in der Sidebar vor der Tag-Cloud. Es ist stets genau eine Liste sichtbar. Kategorien ohne veröffentlichten Beitrag fehlen.

#### Scenario: Kategorie mit Anzahl
- **WHEN** die Kategorie „Filzen“ 37 veröffentlichte Beiträge hat
- **THEN** zeigt die Kategorienliste „Filzen (37)“ als Link auf `/kategorie/filzen/`

#### Scenario: Kategorie ohne Beiträge
- **WHEN** eine Kategorie nur geplante Beiträge hat
- **THEN** fehlt sie in der Seitenspalte

#### Scenario: Links auf dem Desktop, in der Sidebar auf kleineren Fenstern
- **WHEN** dieselbe Seite bei 1440 px und bei 1000 px Breite geöffnet wird
- **THEN** steht die Kategorienliste bei 1440 px links unter der Navigation und bei 1000 px in der Sidebar, und nie doppelt sichtbar

### Requirement: Tag-Cloud
Die Sidebar SHALL alle Schlagworte mit mindestens einem veröffentlichten Beitrag als Tag-Cloud zeigen: alphabetisch nach deutscher Sortierung (Umlaute wie ihre Grundbuchstaben, ohne Beachtung der Groß-/Kleinschreibung), jedes als Link auf `/tag/<name>/`, mit der Anzahl im Titel des Links („Hüte: 21 Beiträge“). Am Ende steht ein Link „Alle Themen“ auf `/themen/`.

#### Scenario: Alphabetische Reihenfolge
- **WHEN** die Schlagworte „Taschen“, „Hüte“ und „Katzen“ existieren
- **THEN** stehen sie in der Reihenfolge „Hüte“, „Katzen“, „Taschen“

#### Scenario: Schlagwort ohne veröffentlichten Beitrag
- **WHEN** ein Schlagwort nur von einem geplanten Beitrag getragen wird
- **THEN** erscheint es nicht in der Cloud

#### Scenario: Link zur Übersicht
- **WHEN** die Sidebar angezeigt wird
- **THEN** führt „Alle Themen“ auf `/themen/`

### Requirement: Größe der Tag-Cloud-Einträge
Die Schriftgröße eines Eintrags der Tag-Cloud SHALL mit der Zahl der Beiträge wachsen. Es gibt fünf Größenstufen von etwa 0,85 em bis 1,6 em; die Stufe richtet sich nach dem Logarithmus der Anzahl zwischen dem seltensten (Stufe 1) und dem häufigsten Schlagwort (Stufe 5). Haben alle Schlagworte gleich viele Beiträge, gilt die mittlere Stufe 3.

#### Scenario: Größe nach Häufigkeit
- **WHEN** „Hüte“ 21 und „Spinnen“ 9 Beiträge haben
- **THEN** ist „Hüte“ größer dargestellt als „Spinnen“

#### Scenario: Alle Schlagworte gleich häufig
- **WHEN** alle Schlagworte gleich viele Beiträge haben
- **THEN** haben alle dieselbe mittlere Größe, und es gibt keinen Fehler durch Division durch null

#### Scenario: Lesbarkeit der kleinsten Einträge
- **WHEN** die Cloud angezeigt wird
- **THEN** ist kein Eintrag kleiner als etwa 0,85 em, und jeder Link hat eine Klickfläche von mindestens 24 px Höhe

### Requirement: Sidebar ohne JavaScript und ohne Dienste von Dritten
Die Sidebar SHALL ohne JavaScript vollständig sein und bei jedem Seitenaufruf aus demselben HTML entstehen; ihre Zahlen und Größen berechnet der Build. Sie lädt keine Ressourcen von fremden Servern.

#### Scenario: JavaScript deaktiviert
- **WHEN** eine Seite ohne JavaScript geöffnet wird
- **THEN** sind Kategorien und Tag-Cloud sichtbar

#### Scenario: Gleiche Sidebar auf allen Seiten
- **WHEN** zwei verschiedene Seiten mit Sidebar verglichen werden
- **THEN** sind Kategorien, Schlagworte, Anzahlen und Größen identisch
