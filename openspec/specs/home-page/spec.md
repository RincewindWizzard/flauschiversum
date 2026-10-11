# home-page Specification

## Purpose

Beschreibt die Startseite des Flauschiversums als Einstieg in das Handbuch: Besucher finden Anleitungen über Suche, Kategorien und Themen, statt durch eine chronologische Liste zu blättern.

## Requirements

### Requirement: Handbuch-Startseite
Die Startseite SHALL unter `/` und `/index.html` erreichbar sein und, in dieser Reihenfolge, zeigen: das Suchfeld, die Kategoriekacheln, die häufigsten Themen und den Abschnitt „Neu im Flauschiversum“. Sie enthält keine chronologische Liste aller Beiträge und keine Paginierung.

#### Scenario: Aufbau der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** erscheinen nacheinander Suchfeld, Kategoriekacheln, häufige Themen und „Neu im Flauschiversum“

#### Scenario: Keine Blätterliste
- **WHEN** `/` geöffnet wird
- **THEN** gibt es dort keine Seitennummern und keinen Link „weiterlesen“ außer im Abschnitt „Neu im Flauschiversum“

### Requirement: Kategoriekacheln
Für jede Kategorie aus `content/categories.yaml` SHALL die Startseite eine Kachel in der Reihenfolge der Datei zeigen: Bild, Name, Reim und Anzahl der veröffentlichten Beiträge. Die ganze Kachel verlinkt auf die Kategorieseite.

#### Scenario: Kachel Filzen
- **WHEN** die Kategorie „Filzen“ 37 veröffentlichte Beiträge hat
- **THEN** zeigt die Kachel das Bild und den Reim der Kategorie, den Namen „Filzen“ und „37 Beiträge“ und verlinkt auf `/kategorie/filzen/`

#### Scenario: Kategorie ohne Beiträge
- **WHEN** eine Kategorie keinen veröffentlichten Beitrag hat
- **THEN** erscheint für sie keine Kachel

### Requirement: Häufige Themen
Die Startseite SHALL die bis zu 12 Schlagworte mit den meisten veröffentlichten Beiträgen als Links auf ihre Schlagwortseiten zeigen, absteigend nach Anzahl und bei gleicher Anzahl alphabetisch, jeweils mit der Anzahl. Ein Link „Alle Themen“ führt auf `/themen/`.

#### Scenario: Reihenfolge der Themen
- **WHEN** „Katzen“ 54 und „Hüte“ 37 Beiträge haben und kein anderes Schlagwort mehr
- **THEN** steht „Katzen (54)“ vor „Hüte (37)“

#### Scenario: Mehr als 12 Schlagworte
- **WHEN** es 20 Schlagworte gibt
- **THEN** zeigt die Startseite die 12 häufigsten und den Link „Alle Themen“

### Requirement: Neu im Flauschiversum
Der Abschnitt „Neu im Flauschiversum“ SHALL die 4 neuesten veröffentlichten Beiträge zeigen, jeweils mit Vorschaubild, Titel als Link und Datum, sowie einen Link „Alle Beiträge“ auf `/beitraege/`.

#### Scenario: Neueste Beiträge
- **WHEN** der neueste Beitrag „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag im Abschnitt, mit Vorschaubild und dem Datum „31. Oktober 2020“

#### Scenario: Geplanter Beitrag
- **WHEN** ein Beitrag ein Datum in der Zukunft hat
- **THEN** erscheint er nicht im Abschnitt
