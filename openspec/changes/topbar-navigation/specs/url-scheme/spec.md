# Spec Delta

## ADDED Requirements

### Requirement: Weiterleitung alter Adressen
Jede Adresse aus `content/redirects.yaml` SHALL eine statische HTML-Seite unter `<alter Pfad>index.html` erhalten, die mit `<meta http-equiv="refresh" content="0; url=<neuer Pfad>">` sofort weiterleitet, eine kanonische URL (`rel="canonical"`) auf das Ziel und `noindex` trägt und einen sichtbaren Link auf das Ziel zeigt. Weiterleitungsseiten sind keine Seiten im Sinne von Kopf, Fußzeile oder Staging-Leiste.

#### Scenario: Alte Kategorieadresse
- **WHEN** `/kategorie/basteln/` aufgerufen wird und die Datei `/kategorie/basteln/: /tag/basteln/` enthält
- **THEN** leitet der Browser sofort auf `/tag/basteln/` weiter

#### Scenario: Suchmaschine
- **WHEN** eine Suchmaschine `/kategorie/basteln/` liest
- **THEN** findet sie `noindex` und die kanonische URL `/tag/basteln/`

#### Scenario: Ohne JavaScript
- **WHEN** JavaScript deaktiviert ist
- **THEN** funktioniert die Weiterleitung trotzdem, und der Link im Text führt auf das Ziel

### Requirement: URLs der Themen
Es SHALL folgende URLs geben: `/themen/` (Übersicht), `/tag/<name>/` je Schlagwort und `/suche/` (Suchseite). Der Name ist der Slug des Namens des Schlagworts (Kleinschreibung, `ä`→`ae`, `ö`→`oe`, `ü`→`ue`, `ß`→`ss`). Die früheren Adressen `/kategorie/<name>/` bleiben als Weiterleitungen erhalten.

#### Scenario: Schlagwort mit Umlaut
- **WHEN** das Schlagwort „Hüte“ existiert
- **THEN** ist seine Seite unter `/tag/huete/` erreichbar

#### Scenario: Frühere Kategorieadresse
- **WHEN** `/kategorie/naehen/` aufgerufen wird
- **THEN** leitet sie auf `/tag/naehen/` weiter

## MODIFIED Requirements

### Requirement: Reservierte Pfade
Einzelseiten und Posts SHALL keine URL verwenden, die mit den Pfaden `/themen/`, `/suche/`, `/beitraege/`, `/tag/` oder `/pagefind/` übereinstimmt oder darunter liegt. Eine Kollision bricht den Build ab.

#### Scenario: Einzelseite mit reserviertem Titel
- **WHEN** eine Einzelseite den Titel „Themen“ hat
- **THEN** bricht der Build ab und nennt die Datei und die kollidierende URL `/themen/`

## REMOVED Requirements

### Requirement: URLs der Themenseiten
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „URLs der Themen“ ohne Kategorien neu gefasst.
**Migration**: Siehe „URLs der Themen“.
