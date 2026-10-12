# related-posts Specification

## Purpose

Auswahl ähnlicher Beiträge für jede Beitragsseite anhand gemeinsamer Schlagworte.

## Requirements

### Requirement: Ähnlichkeit über Schlagworte
Für jeden veröffentlichten Beitrag SHALL der Build bis zu 6 andere veröffentlichte Beiträge als ähnlich auswählen. Der Wert eines Kandidaten ist die Summe über alle gemeinsamen Schlagworte von `1 / Anzahl der Beiträge mit diesem Schlagwort`. Kandidaten ohne gemeinsames Schlagwort zählen nicht. Der Beitrag selbst und nicht veröffentlichte Beiträge sind ausgeschlossen.

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

### Requirement: Kein Auffüllen
Hat ein Beitrag weniger als 6 Kandidaten mit gemeinsamem Schlagwort, SHALL die Auswahl entsprechend kürzer bleiben. Hat er keine Schlagworte oder teilt keines, ist die Auswahl leer.

#### Scenario: Beitrag ohne Schlagworte
- **WHEN** ein Beitrag keine Schlagworte hat
- **THEN** ist die Auswahl leer

#### Scenario: Nur ein Kandidat
- **WHEN** nur ein anderer Beitrag ein gemeinsames Schlagwort hat
- **THEN** besteht die Auswahl aus diesem einen Beitrag

### Requirement: Anzeige wie in der Beitragsliste
Jeder Eintrag der Auswahl SHALL im Aufbau der Einträge der Beitragsliste dargestellt werden: Vorschaubild (falls vorhanden), Titel als Link, Datum und Kurztext. Gemeinsame Schlagworte werden nicht eigens angezeigt.

#### Scenario: Eintrag mit Bild
- **WHEN** der ähnliche Beitrag „Kürbis“ ein Vorschaubild hat
- **THEN** zeigt der Eintrag das runde Bild, den Titel mit Link, „31. Oktober 2020“ und den Kurztext wie in der Liste auf der Startseite

#### Scenario: Ohne Bild und Kurztext
- **WHEN** ein ähnlicher Beitrag weder Vorschaubild noch Kurztext hat
- **THEN** zeigt der Eintrag Titel und Datum, ohne leere Flächen
