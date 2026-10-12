# Spec Delta

## ADDED Requirements

### Requirement: Begrüßungstext der Startseite
Der Begrüßungstext der Startseite SHALL in `content/start.md` als Markdown stehen (ohne Frontmatter oder mit `title`, der ignoriert wird). Fehlt die Datei, zeigt die Startseite die Beiträge ohne Begrüßung und der Build meldet eine Warnung.

#### Scenario: Text ändern
- **WHEN** der Inhalt von `content/start.md` geändert und neu gebaut wird
- **THEN** zeigt die Startseite den neuen Text

#### Scenario: Datei fehlt
- **WHEN** `content/start.md` nicht existiert
- **THEN** zeigt die Startseite keinen Begrüßungstext, und der Build meldet eine Warnung mit dem Pfad

### Requirement: Liste der Weiterleitungen
Alte Adressen, die nicht mehr eigene Seiten sind, SHALL in `content/redirects.yaml` als Zuordnung `<alter Pfad>: <neuer Pfad>` stehen (Pfade mit führendem und abschließendem `/`). Es gibt keine weiteren Seitenarten für Weiterleitungen.

#### Scenario: Eintrag
- **WHEN** `content/redirects.yaml` die Zeile `/kategorie/naehen/: /tag/naehen/` enthält
- **THEN** leitet die alte Adresse auf die neue weiter (Spec `url-scheme`)

### Requirement: Frontmatter eines Posts
Die `index.md` eines Posts SHALL mit einem YAML-Frontmatter beginnen. Pflichtfelder sind `title` und `date` (`YYYY-MM-DD` oder `YYYY-MM-DD HH:MM:SS`, Ortszeit Europe/Berlin). Optional sind `permalink`, `author`, `image`, `excerpt` und `tags` (Liste von Schlagworten). Ein Feld `category` gibt es nicht mehr; Themen werden ausschließlich über Schlagworte geführt. Fehlt ein Pflichtfeld oder ist ein Feld ungültig, bricht der Build mit Datei und Feld ab.

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

#### Scenario: Post ohne Schlagworte
- **WHEN** eine `index.md` kein Feld `tags` hat
- **THEN** ist der Post gültig und bleibt in der chronologischen Liste und in der Suche, steht aber auf keiner Themenseite

#### Scenario: Schlagworte als Liste
- **WHEN** das Frontmatter `tags: [Katzen, Hüte]` oder dieselbe Liste in YAML-Listenschreibweise enthält
- **THEN** trägt der Post die Schlagworte „Katzen“ und „Hüte“

### Requirement: Liste der Schlagworte
Erlaubte Schlagworte SHALL in `content/tags.yaml` stehen (je Eintrag `name`). Schlagworte werden über ihren Namen ohne Beachtung von Groß-/Kleinschreibung zugeordnet.

#### Scenario: Schlagwort in anderer Schreibweise
- **WHEN** `content/tags.yaml` das Schlagwort „Hüte“ enthält und ein Post `tags: [hüte]` angibt
- **THEN** trägt der Post das Schlagwort „Hüte“

## REMOVED Requirements

### Requirement: Einheitliche Kategorien
**Reason**: Es gibt keine Kategorien mehr; Themen sind Schlagworte.
**Migration**: Der frühere Wert von `category` steht in jedem Post als Schlagwort in `tags`.

### Requirement: Felder eines Posts
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Frontmatter eines Posts“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Frontmatter eines Posts“.

### Requirement: Listen der Kategorien und Schlagworte
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Liste der Schlagworte“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Liste der Schlagworte“.
