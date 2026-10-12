## ADDED Requirements

### Requirement: Cache leeren
`flauschiversum clean` SHALL den Cache leeren. Es akzeptiert `--root <Pfad>` und `--cache <Pfad>` wie `build` (Standard `.cache/` im Projektverzeichnis). Es entfernt nur die vom Generator angelegten Einträge des Cache-Verzeichnisses (Bildvarianten `v1/`, Vorschau-Ausgabe `serve/`, Suchindex-Dateien `search-*.bin` und `search-*.tmp`) und lässt alle anderen Dateien und das Verzeichnis selbst bestehen. Es meldet die Zahl der entfernten Dateien und den freigegebenen Platz. Ein fehlender oder bereits leerer Cache ist kein Fehler.

#### Scenario: Cache leeren
- **WHEN** nach einem Build `flauschiversum clean` ausgeführt wird
- **THEN** sind `v1/`, `serve/` und alle `search-*.bin` im Cache-Verzeichnis entfernt, und die Ausgabe nennt Anzahl der Dateien und freigegebenen Platz

#### Scenario: Fremde Dateien bleiben
- **WHEN** im Cache-Verzeichnis zusätzlich eine Datei `notizen.txt` liegt
- **THEN** bleibt sie nach `flauschiversum clean` erhalten

#### Scenario: Wiederholt aufrufen
- **WHEN** `flauschiversum clean` zweimal hintereinander ausgeführt wird oder das Cache-Verzeichnis nicht existiert
- **THEN** meldet das Kommando, dass der Cache leer ist, und endet mit Exit-Code 0

#### Scenario: Eigener Cache-Pfad
- **WHEN** `flauschiversum clean --cache /tmp/cache` ausgeführt wird
- **THEN** wird nur dieser Cache geleert, nicht `.cache/`

#### Scenario: Neuer Build nach dem Leeren
- **WHEN** nach `flauschiversum clean` erneut gebaut wird
- **THEN** werden Bilder und Suchindex neu berechnet, und die Seite ist inhaltlich identisch zum Build davor

### Requirement: Schutz beim Leeren
`flauschiversum clean` SHALL mit Fehlermeldung und Exit-Code 1 abbrechen und nichts löschen, wenn der Cache-Pfad das Projektverzeichnis oder ein Elternverzeichnis davon ist, ein Verzeichnis mit `content/` oder `Cargo.toml` ist oder das Wurzelverzeichnis ist. Symbolische Links werden nicht verfolgt.

#### Scenario: Cache-Pfad ist das Projektverzeichnis
- **WHEN** `flauschiversum clean --cache .` ausgeführt wird
- **THEN** bricht das Kommando mit einer Meldung ab, die den Pfad und den Grund nennt, und löscht nichts

#### Scenario: Link im Cache
- **WHEN** im Cache-Verzeichnis ein Link auf ein anderes Verzeichnis liegt, dessen Name einem bekannten Eintrag entspricht
- **THEN** wird nur der Link entfernt, nicht das Ziel

### Requirement: Ausgabeverzeichnis optional leeren
`flauschiversum clean --dist` SHALL zusätzlich das Ausgabeverzeichnis (`--out`, Standard `dist/`) leeren, aber nur, wenn es als vom Generator erzeugt erkennbar ist. Ohne `--dist` bleibt es unangetastet.

#### Scenario: Mit --dist
- **WHEN** nach einem Build `flauschiversum clean --dist` ausgeführt wird
- **THEN** sind Cache und Ausgabeverzeichnis geleert

#### Scenario: Fremdes Ausgabeverzeichnis
- **WHEN** `--out` auf ein Verzeichnis zeigt, das nicht vom Generator erzeugt wurde
- **THEN** bricht das Kommando ab, und das Verzeichnis bleibt unverändert

#### Scenario: Ohne --dist
- **WHEN** `flauschiversum clean` ohne `--dist` ausgeführt wird
- **THEN** bleibt `dist/` unverändert
