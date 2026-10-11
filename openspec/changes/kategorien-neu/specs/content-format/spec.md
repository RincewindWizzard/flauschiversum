## MODIFIED Requirements

### Requirement: Felder eines Posts
Die `index.md` eines Posts SHALL mit einem YAML-Frontmatter beginnen. Pflichtfelder sind `title`, `date` (`YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`, Ortszeit Europe/Berlin) und `category` (genau eine). Optional sind `permalink`, `author`, `image`, `excerpt` und `tags` (Liste von Schlagworten). Fehlt ein Pflichtfeld oder ist ein Feld ungültig, bricht der Build mit Datei und Feld ab.

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
- **THEN** bricht der Build ab und nennt den Pfad der Datei und das fehlende Feld `category`

#### Scenario: Schlagworte als Liste
- **WHEN** das Frontmatter `tags: [Katzen, Hüte]` oder dieselbe Liste in YAML-Listenschreibweise enthält
- **THEN** trägt der Post die Schlagworte „Katzen“ und „Hüte“

### Requirement: Listen der Kategorien und Schlagworte
Erlaubte Kategorien SHALL in `content/categories.yaml` stehen (je Eintrag `id`, `name`, `image` als Pfad einer statischen Datei, `motto`), erlaubte Schlagworte in `content/tags.yaml` (je Eintrag `name`). Der Wert von `category` entspricht der `id`. Die Reihenfolge der Datei bestimmt die Reihenfolge der Kategorien auf der Seite. Schlagworte werden über ihren Namen ohne Beachtung von Groß-/Kleinschreibung zugeordnet.

#### Scenario: Eintrag einer Kategorie
- **WHEN** `content/categories.yaml` den Eintrag `id: papier`, `name: Papier & Bücher`, `image: /images/Papier.jpg` und ein `motto` enthält
- **THEN** ist `category: papier` in Posts erlaubt, und die Kategorie heißt auf der Seite „Papier & Bücher“

#### Scenario: Schlagwort in anderer Schreibweise
- **WHEN** `content/tags.yaml` das Schlagwort „Hüte“ enthält und ein Post `tags: [hüte]` angibt
- **THEN** trägt der Post das Schlagwort „Hüte“

#### Scenario: Alphabetische Schlagworte
- **WHEN** die Schlagworte „Taschen“, „Hüte“ und „Katzen“ existieren
- **THEN** stehen sie in der Reihenfolge „Hüte“, „Katzen“, „Taschen“
