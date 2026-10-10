# rss-feed Specification

## Purpose

Beschreibt den RSS-Feed, über den Leser neue Posts des Flauschiversums in einem Feedreader abonnieren können.

## Requirements

### Requirement: Feed-Format und Kanal
Unter `/rss.xml` SHALL ein RSS-2.0-Feed in UTF-8 ausgeliefert werden. Der Kanal hat den Titel „Flauschiversum“, den Link `https://flauschiversum.de/`, die Beschreibung „Mehr als nur eine Masche“ und den Zeitpunkt des Builds als `lastBuildDate`.

#### Scenario: Feed im Feedreader
- **WHEN** `https://flauschiversum.de/rss.xml` in einem Feedreader abonniert wird
- **THEN** erscheint der Kanal als „Flauschiversum“ mit der Beschreibung „Mehr als nur eine Masche“

### Requirement: Feed-Einträge
Der Feed SHALL für jeden Post einen Eintrag enthalten mit: dem Titel des Posts, der absoluten Post-URL als `link`, dem `excerpt` als `description` und der absoluten Post-URL als `guid` (mit `isPermaLink="false"`).

#### Scenario: Eintrag eines Posts
- **WHEN** der Post „Kollegenkarten“ vom März 2018 im Feed steht
- **THEN** hat sein Eintrag den Titel „Kollegenkarten“, den Link und die GUID `https://flauschiversum.de/2018/03/kollegenkarten/` und als Beschreibung den Excerpt des Posts

### Requirement: Stabile Eintragskennung
Die `guid` eines Eintrags SHALL sich nicht ändern, solange sich die URL des Posts nicht ändert, damit Feedreader bereits gelesene Posts nicht erneut als neu anzeigen.

#### Scenario: Neuer Build
- **WHEN** die Seite ohne inhaltliche Änderung neu gebaut wird
- **THEN** behalten alle Einträge ihre bisherige `guid`
