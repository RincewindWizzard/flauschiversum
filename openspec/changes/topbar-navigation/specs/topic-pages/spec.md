# Spec Delta

## ADDED Requirements

### Requirement: Eintrag in der Themenseite
Jeder Eintrag auf Schlagwortseiten SHALL das Vorschaubild (falls vorhanden), den Titel als Link auf den Beitrag, den Kurztext und das Datum im Format „<Tag>. <Monatsname> <Jahr>“ zeigen.

#### Scenario: Eintrag mit Bild
- **WHEN** der Beitrag „Kürbis“ in einer Liste steht
- **THEN** zeigt der Eintrag das Bild `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg`, den Titel mit Link auf `/2020/10/kurbis/`, den Kurztext und „31. Oktober 2020“

## MODIFIED Requirements

### Requirement: Schlagwortseite
Für jedes Schlagwort mit mindestens einem veröffentlichten Beitrag SHALL es unter `/tag/<name>/` eine Seite mit Name, Anzahl und allen zugehörigen veröffentlichten Beiträgen geben, neueste zuerst, auf einer einzigen Seite ohne Paginierung, und daneben das Archiv der Beiträge dieses Schlagworts nach Jahr und Monat (Spec `post-archive`). Schlagworte heißen in der Oberfläche „Themen“; es gibt keinen Reim und keinen Einleitungstext. Auf dem Handy stehen Titel, Archiv und Beiträge in dieser Reihenfolge.

#### Scenario: Schlagwort Katzen
- **WHEN** das Schlagwort „Katzen“ von 54 veröffentlichten Beiträgen getragen wird
- **THEN** zeigt `/tag/katzen/` alle 54 Beiträge, neueste zuerst

#### Scenario: Schlagwort ohne veröffentlichten Beitrag
- **WHEN** ein Schlagwort nur von einem geplanten Beitrag getragen wird
- **THEN** existiert keine Seite für dieses Schlagwort

#### Scenario: Archiv des Themas
- **WHEN** `/tag/filzen/` bei 1440 px Breite geöffnet wird
- **THEN** steht rechts das Archiv mit genau den Beiträgen des Schlagworts „Filzen“

#### Scenario: Ältere Beiträge sind erreichbar
- **WHEN** ein Beitrag von 2013 das Schlagwort „Wolle“ trägt
- **THEN** steht er auf `/tag/wolle/` unten in der Liste und im Archiv unter 2013

#### Scenario: Handy reihenfolge
- **WHEN** `/tag/wolle/` bei 360 px Breite geöffnet wird
- **THEN** stehen Titel, Archiv und Beiträge untereinander in dieser Reihenfolge, ohne waagerechtes Scrollen

### Requirement: Themenübersicht
Unter `/themen/` SHALL eine Übersicht aller Schlagworte stehen, alphabetisch (deutsche Sortierung), jeweils als Link auf `/tag/<name>/` mit der Anzahl der veröffentlichten Beiträge. Die Seite ist über „Themen“ in der Topbar erreichbar.

#### Scenario: Alle Themen auf einer Seite
- **WHEN** es 18 Schlagworte gibt
- **THEN** zeigt `/themen/` alle 18 Schlagworte mit je einer Anzahl

#### Scenario: Alphabetische Schlagworte
- **WHEN** die Schlagworte „Taschen“, „Hüte“ und „Katzen“ existieren
- **THEN** stehen sie in der Reihenfolge „Hüte“, „Katzen“, „Taschen“

### Requirement: Zählung nur veröffentlichter Beiträge
Alle Anzahlen und Listen der Themenseiten SHALL nur veröffentlichte Beiträge berücksichtigen. Geplante Beiträge erscheinen weder in Listen noch in Zählungen.

#### Scenario: Geplanter Beitrag
- **WHEN** das Schlagwort „Malen“ 6 veröffentlichte und einen geplanten Beitrag hat
- **THEN** zeigt die Seite „6 Beiträge“ und listet den geplanten Beitrag nicht

## REMOVED Requirements

### Requirement: Kategorieseite
**Reason**: Kategorien und Schlagworte lassen sich nicht sauber trennen, Kategorien müssten sich gegenseitig ausschließen. Themen werden nur noch über Schlagworte geführt.
**Migration**: `/kategorie/<name>/` leitet auf die Seite des entsprechenden Schlagworts weiter (Spec `url-scheme`, `content/redirects.yaml`).

### Requirement: Schlagwörter innerhalb einer Kategorie
**Reason**: Es gibt keine Kategorien mehr; jede Themenseite zeigt das Archiv nach Monat und Jahr.
**Migration**: Schlagworte über `/themen/`, die Startseite oder die Beiträge erreichen.

### Requirement: Beiträge ohne Kategorie
**Reason**: Es gibt keine Kategorien; jeder Beitrag trägt Schlagworte.
**Migration**: Keine.

### Requirement: Eintrag in einer Themenliste
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Eintrag in der Themenseite“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Eintrag in der Themenseite“.
