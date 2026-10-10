# Spec Delta

## Purpose

Beschreibt, wie die gebaute Seite unter https://flauschiversum.de/ ausgeliefert wird. Der Weg der Dateien auf den Server ist nicht Teil des Ist-Zustands, weil das alte Deployment nicht mehr funktioniert.

## ADDED Requirements

### Requirement: Auslieferung über HTTPS
Die Seite SHALL von einem Webserver für statische Dateien unter https://flauschiversum.de/ mit gültigem TLS-Zertifikat ausgeliefert werden.

#### Scenario: Startseite abrufen
- **WHEN** `https://flauschiversum.de/` abgerufen wird
- **THEN** wird die erste Indexseite mit Status 200 über eine gültige TLS-Verbindung ausgeliefert

### Requirement: Verzeichnis-URLs
Für eine URL, die auf `/` endet, SHALL der Webserver die `index.html` im entsprechenden Verzeichnis ausliefern.

#### Scenario: Verzeichnis-URL
- **WHEN** `https://flauschiversum.de/page/1/` abgerufen wird
- **THEN** wird der Inhalt von `/page/1/index.html` mit Status 200 ausgeliefert

### Requirement: Bedingte Anfragen
Der Webserver SHALL für ausgelieferte Dateien `ETag` und `Last-Modified` senden und bedingte Anfragen für unveränderte Dateien mit 304 beantworten.

#### Scenario: Erneuter Abruf
- **WHEN** ein Browser eine bereits geladene Seite mit `If-None-Match` erneut anfragt und sie sich nicht geändert hat
- **THEN** antwortet der Server mit 304
