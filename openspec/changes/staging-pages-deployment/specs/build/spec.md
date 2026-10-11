# Spec Delta

## ADDED Requirements

### Requirement: Optionen für Basis-URL und Staging
`build`, `check` und `serve` SHALL die Optionen `--base-url <URL>` und `--staging <BRANCH>` annehmen. `--staging` schaltet den Staging-Modus ein; der Wert ist der Name des Branches, der in der Staging-Leiste erscheint. Beide haben keine Wirkung auf die Inhalte der Posts und Seiten, nur auf die erzeugten Verweise und auf die Indexierbarkeit. Die Basis-URL kann alternativ über die Umgebungsvariable `FLAUSCHI_BASE_URL` gesetzt werden; die Option hat Vorrang.

#### Scenario: Staging bauen
- **WHEN** `flauschiversum build --base-url https://rincewindwizzard.github.io/flauschiversum --staging claude/beautiful-darwin-66rwab` läuft
- **THEN** entsteht eine Seite mit Unterpfad-Verweisen, `noindex` und der Staging-Leiste mit dem Branchnamen

#### Scenario: Staging ohne Branchnamen
- **WHEN** `--staging` ohne Wert oder mit leerem Wert angegeben wird
- **THEN** bricht der Build ab und nennt, dass ein Branchname erwartet wird

#### Scenario: Vorschau mit Unterpfad
- **WHEN** `flauschiversum serve --base-url https://example.org/vorschau` läuft
- **THEN** ist die Vorschau unter `http://localhost:<Port>/vorschau/` erreichbar, und alle Links darin funktionieren

#### Scenario: Link-Prüfung mit Basis
- **WHEN** mit einer Basis-URL gebaut wird
- **THEN** prüft der Build interne Verweise gegen den Unterpfad und meldet keine falschen Warnungen
