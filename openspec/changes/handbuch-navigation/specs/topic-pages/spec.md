# Spec Delta

## Purpose

Beschreibt die Seiten, über die Besucher Beiträge nach Thema finden: eine Seite je Kategorie, eine Seite je Schlagwort und eine Übersicht aller Themen.

## ADDED Requirements

### Requirement: Kategorieseite
Für jede Kategorie mit mindestens einem veröffentlichten Beitrag SHALL es unter `/kategorie/<name>/` eine Seite geben. Sie zeigt Name, Reim und Anzahl der Beiträge und danach alle veröffentlichten Beiträge der Kategorie, neueste zuerst, auf einer einzigen Seite ohne Paginierung.

#### Scenario: Alle Beiträge auf einer Seite
- **WHEN** die Kategorie „Basteln“ 54 veröffentlichte Beiträge hat
- **THEN** zeigt `/kategorie/basteln/` alle 54 Beiträge, ohne Seitennummern

#### Scenario: Ältere Beiträge sind erreichbar
- **WHEN** ein Beitrag von 2013 in der Kategorie „Wolle“ liegt
- **THEN** steht er auf `/kategorie/wolle/` unten in der Liste

### Requirement: Eintrag in einer Themenliste
Jeder Eintrag auf Kategorie- und Schlagwortseiten SHALL das Vorschaubild (falls vorhanden), den Titel als Link auf den Beitrag, den Kurztext und das Datum im Format „<Tag>. <Monatsname> <Jahr>“ zeigen. Auf Schlagwortseiten steht zusätzlich die Kategorie als Link.

#### Scenario: Eintrag mit Bild
- **WHEN** der Beitrag „Kürbis“ (Kategorie „Basteln“) in einer Liste steht
- **THEN** zeigt der Eintrag das Bild `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg`, den Titel mit Link auf `/2020/10/kurbis/`, den Kurztext und „31. Oktober 2020“

#### Scenario: Kategorie auf der Schlagwortseite
- **WHEN** der Beitrag „Kürbis“ auf der Seite des Schlagworts „Halloween“ steht
- **THEN** zeigt der Eintrag zusätzlich „Basteln“ als Link auf `/kategorie/basteln/`

### Requirement: Schlagwörter innerhalb einer Kategorie
Die Kategorieseite SHALL über der Liste die Schlagworte zeigen, die in den Beiträgen der Kategorie vorkommen, mit der Anzahl innerhalb der Kategorie und als Links auf die Schlagwortseiten, absteigend nach Anzahl.

#### Scenario: Themen in der Kategorie
- **WHEN** in der Kategorie „Filzen“ 12 Beiträge „Katzen“ und 5 Beiträge „Hüte“ tragen
- **THEN** zeigt `/kategorie/filzen/` „Katzen (12)“ vor „Hüte (5)“

### Requirement: Schlagwortseite
Für jedes Schlagwort mit mindestens einem veröffentlichten Beitrag SHALL es unter `/tag/<name>/` eine Seite mit Name, Anzahl und allen zugehörigen veröffentlichten Beiträgen geben, neueste zuerst, ohne Paginierung.

#### Scenario: Schlagwort Katzen
- **WHEN** das Schlagwort „Katzen“ von 54 veröffentlichten Beiträgen getragen wird
- **THEN** zeigt `/tag/katzen/` alle 54 Beiträge, neueste zuerst

#### Scenario: Schlagwort ohne veröffentlichten Beitrag
- **WHEN** ein Schlagwort nur von einem geplanten Beitrag getragen wird
- **THEN** existiert keine Seite für dieses Schlagwort

### Requirement: Themenübersicht
Unter `/themen/` SHALL eine Übersicht stehen: zuerst alle Kategorien in der Reihenfolge von `content/categories.yaml`, danach alle Schlagworte alphabetisch, jeweils als Link mit Anzahl der veröffentlichten Beiträge.

#### Scenario: Alle Themen auf einer Seite
- **WHEN** es 6 Kategorien und 18 Schlagworte gibt
- **THEN** zeigt `/themen/` alle 6 Kategorien und alle 18 Schlagworte mit je einer Anzahl

#### Scenario: Alphabetische Schlagworte
- **WHEN** die Schlagworte „Taschen“, „Hüte“ und „Katzen“ existieren
- **THEN** stehen sie in der Reihenfolge „Hüte“, „Katzen“, „Taschen“

### Requirement: Zählung nur veröffentlichter Beiträge
Alle Anzahlen und Listen der Themenseiten SHALL nur veröffentlichte Beiträge berücksichtigen. Geplante Beiträge erscheinen weder in Listen noch in Zählungen.

#### Scenario: Geplanter Beitrag
- **WHEN** die Kategorie „Malen“ 6 veröffentlichte und einen geplanten Beitrag hat
- **THEN** zeigt die Seite „6 Beiträge“ und listet den geplanten Beitrag nicht
