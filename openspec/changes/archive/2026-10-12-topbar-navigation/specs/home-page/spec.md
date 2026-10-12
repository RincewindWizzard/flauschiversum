# Spec Delta

## ADDED Requirements

### Requirement: Startseite mit Begrüßung
Die Startseite SHALL unter `/` und `/index.html` erreichbar sein und, in dieser Reihenfolge, zeigen: einen kurzen Begrüßungstext aus `content/start.md` und direkt danach die letzten Beiträge. Der Titel „Flauschiversum“ steht nur in der Topbar und wird im Inhalt nicht wiederholt. Es gibt kein Suchfeld im Inhalt (die Suche steht in der Topbar), keine Kacheln und keine Blätterliste.

#### Scenario: Aufbau der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** erscheinen nacheinander der Begrüßungstext und die letzten Beiträge, ohne wiederholte Überschrift „Flauschiversum“

#### Scenario: Begrüßungstext aus der Datei
- **WHEN** `content/start.md` den Text „Willkommen im Flauschiversum!“ enthält
- **THEN** steht er am Anfang des Inhalts der Startseite

#### Scenario: Keine Blätterliste
- **WHEN** `/` geöffnet wird
- **THEN** gibt es dort keine Seitennummern

### Requirement: Letzte Beiträge auf der Startseite
Die Startseite SHALL die zehn neuesten veröffentlichten Beiträge zeigen, jeweils mit Vorschaubild (falls vorhanden), Titel als Link, Datum und Kurztext, sowie am Ende einen Link „Alle Beiträge“ auf `/beitraege/`. Geplante Beiträge fehlen.

#### Scenario: Neueste Beiträge
- **WHEN** der neueste Beitrag „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag, mit Vorschaubild und dem Datum „31. Oktober 2020“

#### Scenario: Zehn Einträge
- **WHEN** es 194 veröffentlichte Beiträge gibt
- **THEN** zeigt die Startseite genau zehn Einträge und den Link „Alle Beiträge“

#### Scenario: Geplanter Beitrag
- **WHEN** ein Beitrag ein Datum in der Zukunft hat
- **THEN** erscheint er nicht auf der Startseite

## REMOVED Requirements

### Requirement: Handbuch-Startseite
**Reason**: Ersetzt durch „Startseite mit Begrüßung“; Suchfeld, Kacheln und „Neu im Flauschiversum“ entfallen im Inhalt.
**Migration**: Die Suche steht in der Topbar, die Themen unter „Themen“, die Beiträge direkt unter der Begrüßung.

### Requirement: Kategoriekacheln
**Reason**: Es gibt keine Kategorien mehr; Themen stehen als Tag-Cloud auf der Startseite und unter „Themen“ in der Topbar.
**Migration**: Themen über die Tag-Cloud oder `/themen/` erreichen.

### Requirement: Neu im Flauschiversum
**Reason**: Ersetzt durch „Letzte Beiträge auf der Startseite“ mit zehn statt vier Einträgen.
**Migration**: Keine.
