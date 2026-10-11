# Spec Delta

## ADDED Requirements

### Requirement: Felder eines Posts
Die `index.md` eines Posts SHALL mit einem YAML-Frontmatter beginnen. Pflichtfelder sind `title` und `date` (`YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`, Ortszeit Europe/Berlin). Optional sind `permalink`, `author`, `image`, `excerpt`, `category` (höchstens eine) und `tags` (Liste von Schlagworten). Fehlt ein Pflichtfeld oder ist ein Feld ungültig, bricht der Build mit Datei und Feld ab.

#### Scenario: Datum mit Uhrzeit
- **WHEN** das Frontmatter `date: 2014-01-04 19:10:28` enthält
- **THEN** gilt der Post als am 4. Januar 2014 um 19:10:28 Uhr deutscher Zeit veröffentlicht

#### Scenario: Datum ohne Uhrzeit
- **WHEN** das Frontmatter `date: 2020-10-31` enthält
- **THEN** gilt der Post als am 31. Oktober 2020 um 0:00 Uhr deutscher Zeit veröffentlicht

#### Scenario: Fehlender Titel
- **WHEN** eine `index.md` kein Feld `title` hat
- **THEN** bricht der Build ab und nennt den Pfad der Datei und das fehlende Feld `title`

#### Scenario: Vorschaubild existiert nicht
- **WHEN** `image` einen Dateinamen nennt, der im Post-Verzeichnis nicht als Bild existiert
- **THEN** bricht der Build ab und nennt Post und Dateinamen

#### Scenario: Post ohne Kategorie
- **WHEN** eine `index.md` kein Feld `category` hat
- **THEN** ist der Post gültig, erscheint auf keiner Kategorieseite und bleibt in der chronologischen Liste, in der Suche und auf den Seiten seiner Schlagworte

#### Scenario: Schlagworte als Liste
- **WHEN** das Frontmatter `tags: [Katzen, Hüte]` oder dieselbe Liste in YAML-Listenschreibweise enthält
- **THEN** trägt der Post die Schlagworte „Katzen“ und „Hüte“

### Requirement: Listen der Kategorien und Schlagworte
Erlaubte Kategorien SHALL in `content/categories.yaml` stehen (je Eintrag `id`, `name`, `image` als Pfad einer statischen Datei, `motto`), erlaubte Schlagworte in `content/tags.yaml` (je Eintrag `name`). Der Wert von `category` entspricht der `id`. Schlagworte werden über ihren Namen ohne Beachtung von Groß-/Kleinschreibung zugeordnet.

#### Scenario: Eintrag einer Kategorie
- **WHEN** `content/categories.yaml` den Eintrag `id: filzen`, `name: Filzen`, `image: /images/Filzen.jpg` und ein `motto` enthält
- **THEN** ist `category: filzen` in Posts erlaubt, und die Kategorie heißt auf der Seite „Filzen“

#### Scenario: Schlagwort in anderer Schreibweise
- **WHEN** `content/tags.yaml` das Schlagwort „Hüte“ enthält und ein Post `tags: [hüte]` angibt
- **THEN** trägt der Post das Schlagwort „Hüte“

## REMOVED Requirements

### Requirement: Frontmatter eines Posts
**Reason**: Ersetzt durch „Felder eines Posts“. `category` hat jetzt Wirkung und einen festen Wertevorrat, neu ist `tags`.
**Migration**: Alle Posts erhalten `tags`; fehlende Kategorien werden nur ergänzt, wo eine klar passt. Beides wird vorgeschlagen und vor dem Eintragen geprüft.
