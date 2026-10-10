# Spec Delta

## Purpose

Beschreibt den gemeinsamen Rahmen aller HTML-Seiten des Flauschiversums: Kopfbereich mit Logo und Navigation, Fußzeile, Seitentitel und eingebundene Ressourcen.

## ADDED Requirements

### Requirement: Grundgerüst jeder Seite
Jede HTML-Seite SHALL als deutschsprachiges Dokument (`lang="de"`, UTF-8) mit Viewport-Angabe für Mobilgeräte ausgeliefert werden. Sie bindet das Stylesheet `/style.css` und das Favicon `/favicon.ico` ein.

#### Scenario: Seite auf dem Handy
- **WHEN** eine beliebige Seite auf einem Mobilgerät geöffnet wird
- **THEN** wird sie in Gerätebreite dargestellt, nicht als verkleinerte Desktop-Ansicht

### Requirement: Kopfbereich
Jede Seite SHALL oben das Logo „Flauschiversum“ zeigen, in der Schrift Lobster und mit einem Link auf `/`. Darunter stehen der Untertitel „Mehr als nur eine Masche“ und der Zusatz „neue Posts an Sonntagen“.

#### Scenario: Logo führt zur Startseite
- **WHEN** auf einer Post-Seite das Logo angeklickt wird
- **THEN** öffnet sich die Startseite `/`

### Requirement: Hauptnavigation
Jede Seite SHALL eine Navigation mit diesen Einträgen in dieser Reihenfolge haben: „Beiträge“ (`/`), „Tipps und Tricks“ (`/tipps-und-tricks/`), „About“ (`/about/`), „Impressum“ (`/impressum/`), „Abonnieren“ (`/rss.xml`).

#### Scenario: Feed abonnieren
- **WHEN** „Abonnieren“ angeklickt wird
- **THEN** wird `/rss.xml` geöffnet

### Requirement: Fußzeile
Jede Seite SHALL eine Fußzeile mit „© <Jahr> Ermeline Wollknoll — Impressum“ haben, wobei „Impressum“ auf `/impressum/` verlinkt. Das Jahr ist das Jahr des Builds.

#### Scenario: Jahr im Copyright
- **WHEN** die Seite im Jahr 2021 gebaut wurde
- **THEN** steht in der Fußzeile „© 2021 Ermeline Wollknoll“

### Requirement: Seitentitel
Post-Seiten und Einzelseiten SHALL ihren Titel als HTML-`<title>` tragen, mit korrekt maskierten HTML-Sonderzeichen.

#### Scenario: Titel eines Posts
- **WHEN** die Seite des Posts „Alraune“ geöffnet wird
- **THEN** lautet der Dokumenttitel „Alraune“

### Requirement: Keine Ressourcen von Drittanbietern
Seiten SHALL keine Skripte, Stylesheets, Schriften oder Bilder von fremden Servern laden. Schriften (Lobster, Lora, Merriweather) und Skripte werden von der eigenen Domain ausgeliefert. Links auf fremde Seiten im Inhalt sind erlaubt.

#### Scenario: Seitenaufruf ohne Drittanfragen
- **WHEN** eine beliebige Seite vollständig geladen wird
- **THEN** wurden alle Ressourcen von `flauschiversum.de` geladen
