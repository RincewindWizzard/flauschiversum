## Why

Der Cache (`.cache/`) wächst auf über 1 GB (Bildvarianten in `v1/`, Suchindex `search-*.bin`, Ausgabe der Vorschau in `serve/`). Wer Platz braucht, den Cache nach einem Wechsel des Badge-Wasserzeichens oder der Bildgrößen erneuern will oder einen verdächtigen Build reproduzieren möchte, muss heute von Hand löschen. Das ist fehleranfällig, weil der Cache-Pfad per `--cache` frei wählbar ist und ein falscher `rm -rf` Inhalte kosten könnte.

## What Changes

- Neues Kommando `flauschiversum clean`, das den Cache leert. Es kennt `--root` und `--cache` wie `build` (Standard `.cache/` im Projektverzeichnis).
- Es löscht nur, was der Generator selbst dort anlegt (`v1/`, `serve/`, `search-*.bin`, `search-*.tmp`), nicht beliebige Dateien. Eigene Dateien im Cache-Verzeichnis und alles außerhalb bleiben unberührt; das Verzeichnis selbst bleibt bestehen.
- Es meldet, was entfernt wurde (Anzahl Dateien und freigegebener Platz), und ist wiederholbar: ein fehlender oder leerer Cache ist kein Fehler.
- Schutz vor Fehlbedienung: Ein `--cache` auf das Projektverzeichnis selbst, auf ein Verzeichnis, das `content/` enthält, oder auf das Stammverzeichnis wird abgelehnt.
- Optional `--dist`, das zusätzlich das Ausgabeverzeichnis (`dist/`) leert, mit demselben Schutz wie der Build (nur ein vom Generator erzeugtes Ausgabeverzeichnis).
- Der nächste Build danach rechnet alles neu (Bilder, Suchindex) und erzeugt dieselbe Seite wie zuvor.

Nicht Teil dieses Changes: automatisches Aufräumen alter Cache-Einträge, Cache-Größenbegrenzung, Änderungen am Cache-Format.

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `build`: neues Kommando `clean`.

## Impact

- `src/main.rs` (Unterkommando `Clean`), neues Modul `src/clean.rs`, `src/output.rs` (Erkennung des Ausgabeverzeichnisses, falls `--dist`), README (Kommando dokumentieren), Tests (Unit-Tests mit Temp-Verzeichnissen, ein Ende-zu-Ende-Test: Build, Clean, Build).
