# Spec Delta

## MODIFIED Requirements

### Requirement: Handbuch-Startseite
Die Startseite SHALL unter `/` und `/index.html` erreichbar sein und, in dieser Reihenfolge, zeigen: das Suchfeld, die Kategoriekacheln und den Abschnitt „Neu im Flauschiversum“. Die häufigen Themen stehen als Tag-Cloud in der Sidebar. Sie enthält keine chronologische Liste aller Beiträge und keine Paginierung.

#### Scenario: Aufbau der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** erscheinen nacheinander Suchfeld, Kategoriekacheln und „Neu im Flauschiversum“, daneben bzw. darunter die Sidebar mit der Tag-Cloud

#### Scenario: Keine Blätterliste
- **WHEN** `/` geöffnet wird
- **THEN** gibt es dort keine Seitennummern und keinen Link „weiterlesen“ außer im Abschnitt „Neu im Flauschiversum“

## REMOVED Requirements

### Requirement: Häufige Themen
**Reason**: Die Tag-Cloud in der Sidebar (Spec `sidebar`) zeigt alle Schlagworte nach Häufigkeit skaliert und ersetzt die Liste der zwölf häufigsten Schlagworte im Inhalt der Startseite.
**Migration**: Der Abschnitt entfällt; der Link „Alle Themen“ steht am Ende der Tag-Cloud.
