# Spec Delta

## ADDED Requirements

### Requirement: Weiterleitungen prüfen und erzeugen
Der Build SHALL die Datei `content/redirects.yaml` (Liste `von: nach`, jeweils interne Pfade mit führendem und abschließendem `/`) auswerten und für jeden Eintrag unter dem alten Pfad eine Weiterleitungsseite erzeugen. Verweist ein Eintrag auf sich selbst, hat ein Ziel, das nicht existiert, oder liegt die alte Adresse auf einer vorhandenen Seite, bricht der Build mit der Angabe des Eintrags ab. Die Datei darf fehlen.

#### Scenario: Gültige Weiterleitung
- **WHEN** `content/redirects.yaml` den Eintrag `/kategorie/basteln/: /tag/basteln/` enthält
- **THEN** gibt es unter `/kategorie/basteln/` eine Weiterleitungsseite auf `/tag/basteln/`

#### Scenario: Ziel existiert nicht
- **WHEN** ein Eintrag auf `/tag/gibtsnicht/` verweist
- **THEN** bricht der Build ab und nennt den Eintrag

#### Scenario: Alte Adresse belegt
- **WHEN** die alte Adresse eines Eintrags bereits eine Seite hat
- **THEN** bricht der Build ab und nennt die Adresse

#### Scenario: Keine Datei
- **WHEN** `content/redirects.yaml` fehlt
- **THEN** baut der Build ohne Weiterleitungen und ohne Fehler

### Requirement: Schlagworte prüfen
Der Build SHALL jedes Schlagwort eines Posts gegen `content/tags.yaml` prüfen. Ein unbekanntes Schlagwort bricht den Build ab und nennt Post, Wert und – wenn vorhanden – das ähnlichste erlaubte Schlagwort. Doppelte Schlagworte in einem Post und Schlagworte, die weniger als zwei Posts tragen, werden als Warnung gemeldet. Hat ein Post noch ein Feld `category`, meldet der Build eine Warnung mit dem Hinweis, den Wert als Schlagwort zu führen; das Feld wird sonst ignoriert.

#### Scenario: Tippfehler im Schlagwort
- **WHEN** ein Post `tags: [Kazen]` angibt und „Katzen“ erlaubt ist
- **THEN** bricht der Build ab und nennt den Post, „Kazen“ und den Vorschlag „Katzen“

#### Scenario: Altes Feld category
- **WHEN** ein Post noch `category: basteln` im Frontmatter hat
- **THEN** baut der Build weiter und meldet eine Warnung, die Post und Feld nennt und rät, „basteln“ in `tags` zu führen

#### Scenario: Schlagwort nur einmal benutzt
- **WHEN** ein erlaubtes Schlagwort nur von einem Post getragen wird
- **THEN** meldet der Build eine Warnung, und die Seite des Schlagworts wird trotzdem erzeugt

## REMOVED Requirements

### Requirement: Kategorien und Schlagworte prüfen
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Schlagworte prüfen“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Schlagworte prüfen“.
