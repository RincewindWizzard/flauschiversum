# Spec Delta

## ADDED Requirements

### Requirement: Begrüßungstext der Startseite
Der Begrüßungstext der Startseite SHALL in `content/start.md` als Markdown stehen (ohne Frontmatter oder mit `title`, der ignoriert wird). Fehlt die Datei, zeigt die Startseite die Überschrift und die Beiträge ohne Begrüßung und der Build meldet eine Warnung.

#### Scenario: Text ändern
- **WHEN** der Inhalt von `content/start.md` geändert und neu gebaut wird
- **THEN** zeigt die Startseite den neuen Text

#### Scenario: Datei fehlt
- **WHEN** `content/start.md` nicht existiert
- **THEN** zeigt die Startseite keinen Begrüßungstext, und der Build meldet eine Warnung mit dem Pfad

## MODIFIED Requirements

### Requirement: Listen der Kategorien und Schlagworte
Erlaubte Kategorien SHALL in `content/categories.yaml` stehen (je Eintrag `id`, `name`, `image` als Pfad einer statischen Datei, `motto`, optional `intro` als kurzer Einleitungstext in Markdown für die Kategorieseite), erlaubte Schlagworte in `content/tags.yaml` (je Eintrag `name`). Der Wert von `category` entspricht der `id`. Schlagworte werden über ihren Namen ohne Beachtung von Groß-/Kleinschreibung zugeordnet.

#### Scenario: Eintrag einer Kategorie
- **WHEN** `content/categories.yaml` den Eintrag `id: filzen`, `name: Filzen`, `image: /images/Filzen.jpg` und ein `motto` enthält
- **THEN** ist `category: filzen` in Posts erlaubt, und die Kategorie heißt auf der Seite „Filzen“

#### Scenario: Schlagwort in anderer Schreibweise
- **WHEN** `content/tags.yaml` das Schlagwort „Hüte“ enthält und ein Post `tags: [hüte]` angibt
- **THEN** trägt der Post das Schlagwort „Hüte“

#### Scenario: Einleitungstext
- **WHEN** eine Kategorie ein Feld `intro` hat
- **THEN** zeigt die Kategorieseite diesen Text über der Beitragsliste
