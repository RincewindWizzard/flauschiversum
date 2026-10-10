# Spec Delta

## Purpose

Beschreibt, wie Änderungen an den Quellen auf die öffentliche Seite https://flauschiversum.de/ gelangen und wie die gebaute Seite ausgeliefert wird.

## ADDED Requirements

### Requirement: Aktualisierung aus dem Git-Repository
Ein Deployment SHALL den aktuellen Stand der Quellen aus dem Git-Repository holen (beim ersten Mal klonen, danach `git pull`), die Seite mit einem dauerhaft erhaltenen Bild-Cache bauen und das Ergebnis auf den Auslieferungsserver übertragen.

#### Scenario: Neuer Post veröffentlicht
- **WHEN** ein neuer Post in das Repository gepusht und ein Deployment ausgelöst wird
- **THEN** ist der Post danach unter seiner URL auf https://flauschiversum.de/ erreichbar

#### Scenario: Wiederholtes Deployment
- **WHEN** ein Deployment läuft, nachdem sich nur Text geändert hat
- **THEN** werden die Bildvarianten aus dem Cache des vorherigen Deployments übernommen

### Requirement: Spiegelung auf den Server
Die Übertragung SHALL den Inhalt des Ausgabeverzeichnisses per SSH auf den Auslieferungsserver spiegeln. Dateien, die im Ausgabeverzeichnis nicht mehr existieren, werden auf dem Server gelöscht. Der Zugriff erfolgt mit einem eigenen Deploy-Schlüssel.

#### Scenario: Datei entfernt
- **WHEN** eine Datei nach dem Build nicht mehr im Ausgabeverzeichnis liegt
- **THEN** ist sie nach der Übertragung auch auf dem Server nicht mehr vorhanden

### Requirement: Auslieferung über HTTPS
Die Seite SHALL von einem Webserver für statische Dateien unter https://flauschiversum.de/ mit gültigem TLS-Zertifikat ausgeliefert werden. Für Verzeichnis-URLs liefert er die enthaltene `index.html` aus und unterstützt bedingte Anfragen (`ETag`, `Last-Modified`).

#### Scenario: Verzeichnis-URL
- **WHEN** `https://flauschiversum.de/page/1/` abgerufen wird
- **THEN** wird der Inhalt von `/page/1/index.html` mit Status 200 ausgeliefert

#### Scenario: Erneuter Abruf
- **WHEN** ein Browser eine bereits geladene Seite mit `If-None-Match` erneut anfragt und sie sich nicht geändert hat
- **THEN** antwortet der Server mit 304
