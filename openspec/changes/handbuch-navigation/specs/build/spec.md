# Spec Delta

## ADDED Requirements

### Requirement: Kategorien und Schlagworte prüfen
Der Build SHALL jede Kategorie und jedes Schlagwort eines Posts gegen `content/categories.yaml` und `content/tags.yaml` prüfen. Eine unbekannte Kategorie oder ein unbekanntes Schlagwort bricht den Build ab und nennt Post, Wert und – wenn vorhanden – das ähnlichste erlaubte Schlagwort. Doppelte Schlagworte in einem Post, ein nicht vorhandenes Kategoriebild und Schlagworte, die weniger als zwei Posts tragen, werden als Warnung gemeldet bzw. bei fehlendem Bild als Fehler.

#### Scenario: Tippfehler im Schlagwort
- **WHEN** ein Post `tags: [Kazen]` angibt und „Katzen“ erlaubt ist
- **THEN** bricht der Build ab und nennt den Post, „Kazen“ und den Vorschlag „Katzen“

#### Scenario: Unbekannte Kategorie
- **WHEN** ein Post `category: stricken` angibt, die Liste diese aber nicht enthält
- **THEN** bricht der Build ab und nennt den Post, „stricken“ und die erlaubten Kategorien

#### Scenario: Schlagwort nur einmal benutzt
- **WHEN** ein erlaubtes Schlagwort nur von einem Post getragen wird
- **THEN** meldet der Build eine Warnung, und die Seite des Schlagworts wird trotzdem erzeugt

### Requirement: Suchindex im Build
Der Build SHALL aus allen veröffentlichten Posts und Einzelseiten den Suchindex erzeugen und seine Dateien wie alle anderen Ausgaben ins Ausgabeverzeichnis schreiben und im Manifest führen. Der Index kommt aus dem Speicher des Builds, ohne zusätzliches Programm und ohne Zwischenverzeichnis.

#### Scenario: Ein Befehl genügt
- **WHEN** `flauschiversum build` auf einem frischen Checkout läuft
- **THEN** enthält das Ausgabeverzeichnis danach `pagefind/` mit dem Suchindex, ohne dass ein weiteres Programm gestartet wurde

#### Scenario: Verwaister Index
- **WHEN** ein Post gelöscht und neu gebaut wird
- **THEN** sind die nur von ihm benötigten Indexdateien aus `pagefind/` entfernt
