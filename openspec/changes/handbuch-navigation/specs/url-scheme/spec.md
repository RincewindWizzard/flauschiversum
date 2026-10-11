# Spec Delta

## MODIFIED Requirements

### Requirement: URLs der Indexseiten
Die Startseite SHALL unter `/` und `/index.html` erreichbar sein. Die chronologische Liste SHALL als erste Seite unter `/beitraege/` und mit den Folgeseiten unter `/page/<N>/` und `/page/<N>/index.html` erreichbar sein, mit N ab 1 für die zweite Seite der Liste.

#### Scenario: Zweite Indexseite
- **WHEN** es mehr als 4 veröffentlichte Posts gibt
- **THEN** ist die zweite Seite der Liste unter `/page/1/index.html` und `/page/1/` erreichbar

#### Scenario: Erste Seite der Liste
- **WHEN** es mindestens einen veröffentlichten Post gibt
- **THEN** ist die erste Seite der Liste unter `/beitraege/` erreichbar

#### Scenario: Startseite
- **WHEN** `/` aufgerufen wird
- **THEN** wird die Handbuch-Startseite ausgeliefert und nicht die chronologische Liste

## ADDED Requirements

### Requirement: URLs der Themenseiten
Es SHALL folgende URLs geben: `/themen/` (Übersicht), `/kategorie/<name>/` je Kategorie, `/tag/<name>/` je Schlagwort und `/suche/` (Suchseite). Der Name ist der Slug der `id` der Kategorie bzw. des Namens des Schlagworts (Kleinschreibung, `ä`→`ae`, `ö`→`oe`, `ü`→`ue`, `ß`→`ss`).

#### Scenario: Kategorie mit Umlaut
- **WHEN** die Kategorie `id: nähen` heißt
- **THEN** ist ihre Seite unter `/kategorie/naehen/` erreichbar

#### Scenario: Schlagwort mit Umlaut
- **WHEN** das Schlagwort „Hüte“ existiert
- **THEN** ist seine Seite unter `/tag/huete/` erreichbar

### Requirement: Reservierte Pfade
Einzelseiten und Posts SHALL keine URL verwenden, die mit den Pfaden `/themen/`, `/suche/`, `/beitraege/`, `/kategorie/`, `/tag/` oder `/pagefind/` übereinstimmt oder darunter liegt. Eine Kollision bricht den Build ab.

#### Scenario: Einzelseite mit reserviertem Titel
- **WHEN** eine Einzelseite den Titel „Themen“ hat
- **THEN** bricht der Build ab und nennt die Datei und die kollidierende URL `/themen/`
