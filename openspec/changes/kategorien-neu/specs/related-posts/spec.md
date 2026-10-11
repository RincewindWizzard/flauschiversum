## Purpose

Auswahl ähnlicher Beiträge für jede Beitragsseite anhand von Schlagworten und Kategorie.

## ADDED Requirements

### Requirement: Ähnlichkeit über Schlagworte
Für jeden veröffentlichten Beitrag SHALL der Build bis zu 6 andere veröffentlichte Beiträge als ähnlich auswählen. Der Wert eines Kandidaten ist die Summe über alle gemeinsamen Schlagworte von `1 / Anzahl der Beiträge mit diesem Schlagwort`, plus 0,25 bei gleicher Kategorie, sofern mindestens ein Schlagwort gemeinsam ist. Kandidaten ohne gemeinsames Schlagwort zählen nicht. Der Beitrag selbst und nicht veröffentlichte Beiträge sind ausgeschlossen.

#### Scenario: Seltenes Schlagwort zählt mehr
- **WHEN** Beitrag A mit B nur das häufige Schlagwort „Wolle“ (50 Beiträge) und mit C das seltene „Gelenkpuppen“ (2 Beiträge) teilt
- **THEN** steht C in der Auswahl von A vor B

#### Scenario: Höchstens sechs
- **WHEN** mehr als 6 Kandidaten ein gemeinsames Schlagwort haben
- **THEN** zeigt die Auswahl nur die 6 mit dem höchsten Wert

### Requirement: Feste Reihenfolge
Die ähnlichen Beiträge SHALL nach Wert absteigend, dann nach Datum absteigend, dann nach URL aufsteigend geordnet sein, sodass wiederholte Builds dasselbe Ergebnis liefern.

#### Scenario: Deterministische Reihenfolge
- **WHEN** zwei Kandidaten denselben Wert haben
- **THEN** steht der mit dem neueren Datum zuerst, bei gleichem Datum der mit der alphabetisch früheren URL

#### Scenario: Geplanter Beitrag
- **WHEN** ein Kandidat ein Datum in der Zukunft hat
- **THEN** erscheint er nicht in der Auswahl

### Requirement: Auffüllen aus der Kategorie
Bleiben weniger als 6 Kandidaten mit gemeinsamem Schlagwort, SHALL die Auswahl mit den neuesten Beiträgen derselben Kategorie aufgefüllt werden, die noch nicht enthalten sind. Hat ein Beitrag keine Schlagworte, besteht die Auswahl nur daraus.

#### Scenario: Beitrag ohne Schlagworte
- **WHEN** ein Beitrag keine Schlagworte hat und seine Kategorie weitere Beiträge enthält
- **THEN** zeigt die Auswahl die neuesten anderen Beiträge dieser Kategorie

#### Scenario: Kategorie mit einem Beitrag
- **WHEN** ein Beitrag weder gemeinsame Schlagworte noch weitere Beiträge in seiner Kategorie hat
- **THEN** ist die Auswahl leer

### Requirement: Anzeige der gemeinsamen Schlagworte
Jeder Eintrag der Auswahl SHALL den Titel als Link und, wenn vorhanden, die gemeinsamen Schlagworte zeigen. Aufgefüllte Einträge zeigen keine Schlagworte.

#### Scenario: Gemeinsame Schlagworte
- **WHEN** zwei Beiträge die Schlagworte „Filzen“ und „Katzen“ teilen
- **THEN** zeigt der Eintrag „Filzen“ und „Katzen“ unter dem Titel
