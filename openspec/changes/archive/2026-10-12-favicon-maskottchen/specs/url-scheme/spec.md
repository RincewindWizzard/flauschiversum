## ADDED Requirements

### Requirement: Adressen der Favicon-Dateien
Die Favicon-Dateien SHALL unter `/favicon.ico`, `/favicon.svg` und `/apple-touch-icon.png` erreichbar sein (Dateien aus `content/static/`). Die bisherige Adresse `/favicon.ico` bleibt unverändert bestehen.

#### Scenario: Alle drei Adressen
- **WHEN** die drei Adressen abgerufen werden
- **THEN** antwortet der Server jeweils mit der Datei und dem passenden Medientyp
