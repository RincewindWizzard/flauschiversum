# home-page Specification

## Purpose

Beschreibt die Startseite des Flauschiversums als Einstieg in das Handbuch: Besucher finden Anleitungen über Suche, Kategorien und Themen, statt durch eine chronologische Liste zu blättern.

## Requirements

### Requirement: Handbuch-Startseite
Die Startseite SHALL unter `/` und `/index.html` erreichbar sein und, in dieser Reihenfolge, zeigen: das Suchfeld, die Kategoriekacheln und den Abschnitt „Neu im Flauschiversum“. Die häufigen Themen stehen als Tag-Cloud in der Sidebar. Sie enthält keine chronologische Liste aller Beiträge und keine Paginierung.

#### Scenario: Aufbau der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** erscheinen nacheinander Suchfeld, Kategoriekacheln und „Neu im Flauschiversum“, daneben bzw. darunter die Sidebar mit der Tag-Cloud

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

### Requirement: Neu im Flauschiversum
Der Abschnitt „Neu im Flauschiversum“ SHALL die 4 neuesten veröffentlichten Beiträge zeigen, jeweils mit Vorschaubild, Titel als Link und Datum, sowie einen Link „Alle Beiträge“ auf `/beitraege/`.

#### Scenario: Neueste Beiträge
- **WHEN** der neueste Beitrag „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag im Abschnitt, mit Vorschaubild und dem Datum „31. Oktober 2020“

#### Scenario: Geplanter Beitrag
- **WHEN** ein Beitrag ein Datum in der Zukunft hat
- **THEN** erscheint er nicht im Abschnitt
