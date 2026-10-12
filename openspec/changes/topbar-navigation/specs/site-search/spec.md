# Spec Delta

## MODIFIED Requirements

### Requirement: Durchsuchter Bestand
Die Suche SHALL die Texte aller veröffentlichten Beiträge und Einzelseiten sowie die Schlagworte der Beiträge berücksichtigen. Startseite, Beitragslisten, Themenseiten und die Suchseite selbst sind nicht Teil des Bestands.

#### Scenario: Begriff aus dem Text
- **WHEN** ein Begriff nur im Fließtext eines Beitrags von 2014 vorkommt
- **THEN** erscheint dieser Beitrag unter den Treffern

#### Scenario: Begriff aus einem Schlagwort
- **WHEN** nach „Halloween“ gesucht wird
- **THEN** erscheinen auch Beiträge mit dem Schlagwort „Halloween“, in deren Text das Wort nicht steht

#### Scenario: Geplanter Beitrag
- **WHEN** ein Beitrag ein Datum in der Zukunft hat
- **THEN** erscheint er in keinem Suchergebnis

## ADDED Requirements

### Requirement: Filter nach Thema
Die Suchseite SHALL die Treffer nach Thema (Schlagwort) einschränken können. Die Auswahl heißt „Thema“ und zeigt jedes Schlagwort mit der Zahl der Treffer.

#### Scenario: Auf Thema einschränken
- **WHEN** nach „Mütze“ gesucht und das Thema „Wolle“ gewählt wird
- **THEN** bleiben nur Treffer mit dem Schlagwort „Wolle“ übrig

#### Scenario: Beitrag ohne Schlagworte
- **WHEN** ein Beitrag keine Schlagworte hat
- **THEN** erscheint er in der Suche ohne Filter, aber in keiner Themenauswahl

### Requirement: Konzentrische Lupe
Der Suchknopf SHALL als Kreis im Suchfeld sitzen, dessen Mittelpunkt auf dem Mittelpunkt der Rundung am rechten Rand des Feldes liegt (konzentrisch): oben, unten und rechts hat er denselben Abstand zum Rand des Feldes. Das gilt für jedes Suchfeld (Topbar, Suchseite) in jeder Schriftgröße.

#### Scenario: Gleiche Abstände
- **WHEN** ein Suchfeld angezeigt wird
- **THEN** sind die Abstände der Lupe zum Feldrand oben, unten und rechts gleich (Abweichung höchstens 1,5 px), und der Knopf ist rund

#### Scenario: Größeres Suchfeld
- **WHEN** das Suchfeld der Suchseite, das größer als das der Topbar ist, angezeigt wird
- **THEN** sind die Abstände ebenfalls gleich

## REMOVED Requirements

### Requirement: Filter nach Kategorie
**Reason**: Es gibt keine Kategorien mehr; der Filter nach Schlagwort („Thema“) ersetzt ihn.
**Migration**: Siehe „Filter nach Thema“.
