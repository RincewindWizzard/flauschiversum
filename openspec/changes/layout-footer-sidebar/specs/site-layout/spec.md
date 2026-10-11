# Spec Delta

## MODIFIED Requirements

### Requirement: Hauptnavigation
Jede Seite SHALL im Kopf eine Navigation mit genau diesen Einträgen in dieser Reihenfolge haben: „Start“ (`/`), „Themen“ (`/themen/`) und „Beiträge“ (`/beitraege/`). „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“ stehen in der Fußzeile.

#### Scenario: Feed abonnieren
- **WHEN** in der Fußzeile „Abonnieren“ angeklickt wird
- **THEN** wird `/rss.xml` geöffnet

#### Scenario: Themen finden
- **WHEN** „Themen“ angeklickt wird
- **THEN** wird `/themen/` geöffnet

#### Scenario: Kopf ohne seltene Seiten
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Navigation im Kopf kein „About“, „Impressum“, „Tipps und Tricks“ und „Abonnieren“

### Requirement: Fußzeile
Jede Seite SHALL am Ende eine Fußzeile haben. Sie enthält die Links „Tipps und Tricks“ (`/tipps-und-tricks/`), „About“ (`/about/`), „Impressum“ (`/impressum/`) und „Abonnieren“ (`/rss.xml`), die Kontaktwege nach der Spec `contact-links` und den Hinweis „© <Jahr> Ermeline Wollknoll“. Das Jahr ist das Jahr des Builds. Die Fußzeile ist auch ohne JavaScript vollständig und bricht bei schmalen Fenstern um, ohne waagerechtes Scrollen zu verursachen.

#### Scenario: Jahr im Copyright
- **WHEN** die Seite im Jahr 2021 gebaut wurde
- **THEN** steht in der Fußzeile „© 2021 Ermeline Wollknoll“

#### Scenario: Seitenlinks in der Fußzeile
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Fußzeile die Links „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“ mit den genannten Zielen

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** sind alle Links der Fußzeile sichtbar und erreichbar, und die Seite scrollt nicht waagerecht

#### Scenario: Reihenfolge am Seitenende
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** steht die Fußzeile nach dem Hauptinhalt als letztes sichtbares Element

## ADDED Requirements

### Requirement: Zweispaltiges Layout mit Sidebar
Auf breiten Fenstern (ab 64 em) SHALL jede Seite außer der Suchseite und der Themenübersicht den Inhalt links und eine Sidebar rechts zeigen, in einem Seitenrahmen von bis zu etwa 1200 px Breite. Der Lesetext von Beiträgen und Einzelseiten hat höchstens etwa 40 em Breite und steht in der Inhaltsspalte; Kacheln und Listen füllen die Inhaltsspalte.

#### Scenario: Desktop
- **WHEN** eine Beitragsseite bei 1440 px Breite geöffnet wird
- **THEN** steht die Sidebar rechts neben dem Text, und der Text hat höchstens etwa 40 em Breite

#### Scenario: Liste auf dem Desktop
- **WHEN** eine Kategorieseite bei 1440 px Breite geöffnet wird
- **THEN** füllt die Liste der Beiträge die Inhaltsspalte neben der Sidebar

### Requirement: Sidebar auf schmalen Fenstern und Ausnahmen
Auf Fenstern unter 64 em SHALL die Sidebar nach dem Inhalt und vor der Fußzeile stehen. Die Sidebar steht in der Quelle nach dem Hauptinhalt und ist als `aside` mit Beschriftung ausgezeichnet. Die Suchseite und die Themenübersicht haben keine Sidebar und sind einspaltig.

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** steht die Sidebar nach dem Inhalt und vor der Fußzeile, und die Seite scrollt nicht waagerecht

#### Scenario: Suche einspaltig
- **WHEN** die Suchseite bei 1440 px Breite geöffnet wird
- **THEN** gibt es keine Sidebar, und Filter und Trefferliste nutzen die Breite der Inhaltsspalte

#### Scenario: Themenübersicht
- **WHEN** `/themen/` geöffnet wird
- **THEN** gibt es keine Sidebar, weil Kategorien und Schlagworte im Inhalt stehen
