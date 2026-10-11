# Spec Delta

## ADDED Requirements

### Requirement: Optionen für Basis-URL und Staging
`build`, `check` und `serve` SHALL die Optionen `--base-url <URL>` und `--staging <BRANCH>` annehmen. `--staging` schaltet den Staging-Modus ein, sein Wert ist der Branchname für die Staging-Leiste. Beide Optionen ändern nur Absolut-URLs und die Kennzeichnung der Seite, nicht die Inhalte. Die Basis-URL kann auch über `FLAUSCHI_BASE_URL` gesetzt werden; die Option hat Vorrang.

#### Scenario: Staging bauen
- **WHEN** `flauschiversum build --base-url https://staging.flauschiversum.de --staging claude/beautiful-darwin-66rwab` läuft
- **THEN** entsteht eine Seite mit Staging-Absolut-URLs, `noindex` und der Staging-Leiste mit dem Branchnamen

#### Scenario: Staging ohne Branchnamen
- **WHEN** `--staging` ohne Wert oder mit leerem Wert angegeben wird
- **THEN** bricht der Build ab und nennt, dass ein Branchname erwartet wird

#### Scenario: Staging lokal ansehen
- **WHEN** `flauschiversum serve --staging mein-branch` läuft
- **THEN** zeigt die Vorschau auf `http://localhost:<Port>/` die Staging-Leiste mit „mein-branch“
