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
