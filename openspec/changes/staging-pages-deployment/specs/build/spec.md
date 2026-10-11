# Spec Delta

## ADDED Requirements

### Requirement: Optionen für Basis-URL und Staging
`build`, `check` und `serve` SHALL die Optionen `--base-url <URL>` und `--staging` annehmen. Beide haben keine Wirkung auf die Inhalte der Posts und Seiten, nur auf die erzeugten Verweise und auf die Indexierbarkeit. Die Basis-URL kann alternativ über die Umgebungsvariable `FLAUSCHI_BASE_URL` gesetzt werden; die Option hat Vorrang.

#### Scenario: Staging bauen
- **WHEN** `flauschiversum build --base-url https://rincewindwizzard.github.io/flauschiversum --staging` läuft
- **THEN** entsteht eine Seite mit Unterpfad-Verweisen und `noindex`

#### Scenario: Vorschau mit Unterpfad
- **WHEN** `flauschiversum serve --base-url https://example.org/vorschau` läuft
- **THEN** ist die Vorschau unter `http://localhost:<Port>/vorschau/` erreichbar, und alle Links darin funktionieren

#### Scenario: Link-Prüfung mit Basis
- **WHEN** mit einer Basis-URL gebaut wird
- **THEN** prüft der Build interne Verweise gegen den Unterpfad und meldet keine falschen Warnungen
