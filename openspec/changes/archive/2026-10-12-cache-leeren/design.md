## Context

Der Cache liegt standardmäßig in `.cache/` (Option `--cache`) und enthält laut Code: Bildvarianten und ihre Metadaten unter `v1/` (`media/mod.rs`, Dateien `<Schlüssel>.<ext>` und `.json`), den Suchindex als `search-<Hash>.bin` (und kurzzeitig `search-<Hash>.tmp`, `search.rs`) und bei `serve` die Ausgabe der Vorschau unter `serve/` (`main.rs`). Gemessen: 1,1 GB. `output::write` löscht im Ausgabeverzeichnis nur Dateien, die der Generator selbst geschrieben hat (Manifest), und weigert sich bei fremden Verzeichnissen; dieselbe Vorsicht gilt hier.

## Goals / Non-Goals

**Goals:** ein sicheres `flauschiversum clean`, das den Cache leert und Rückmeldung gibt.
**Non-Goals:** automatische Bereinigung, Größenlimit, neues Cache-Format.

## Decisions

### 1. Nur bekannte Einträge löschen

`clean` entfernt im Cache-Verzeichnis ausschließlich `v1/` (rekursiv), `serve/` (rekursiv) und Dateien `search-*.bin` und `search-*.tmp`. Alles andere bleibt. Das Verzeichnis selbst bleibt erhalten (kein Fehler, wenn es fehlt). So kann auch ein versehentlich auf ein fremdes Verzeichnis gerichtetes `--cache` nichts Eigenes zerstören, solange der Schutz (nächster Punkt) greift. Neue Cache-Arten brauchen künftig einen Eintrag in einer Liste in `clean.rs` und einen Test, der prüft, dass jede Datei, die ein Build im Cache anlegt, von `clean` erfasst wird (Build in Temp-Verzeichnis, Clean, Verzeichnis leer).

*Alternative:* das ganze Verzeichnis löschen. Verworfen: gefährlich bei falschem `--cache`.

### 2. Schutz vor falschen Pfaden

Vor dem Löschen wird der aufgelöste Cache-Pfad geprüft und das Kommando bricht mit Fehlermeldung ab, wenn er (a) das Projektverzeichnis selbst oder ein Elternverzeichnis davon ist, (b) ein Verzeichnis ist, das `content/` oder `Cargo.toml` enthält, oder (c) das Wurzelverzeichnis `/` ist. Symbolische Links werden nicht verfolgt (nur der Link selbst würde entfernt, nie das Ziel). Die Meldung nennt den Pfad und den Grund.

### 3. Ausgabe und Rückgabewert

Ausgabe auf Deutsch wie die anderen Kommandos: `Cache geleert: 3196 Dateien, 1,1 GB frei (.cache)` bzw. `Cache ist bereits leer (.cache)`. Exit-Code 0 bei Erfolg und bei leerem oder fehlendem Cache, 1 bei Abbruch durch den Pfadschutz oder bei einem Löschfehler (Datei, Fehler werden genannt, die übrigen Einträge werden trotzdem entfernt).

### 4. `--dist`

Mit `--dist` (und `--out`, Standard `dist/`) wird zusätzlich das Ausgabeverzeichnis geleert, aber nur, wenn es als vom Generator erzeugt erkennbar ist (Manifest wie in `output::write`); sonst Abbruch mit derselben Fehlermeldung wie beim Build. Ohne `--dist` bleibt `dist/` unangetastet.

### 5. Parallelität

Läuft gleichzeitig `serve` oder `build`, kann `clean` Dateien löschen, die gerade gelesen werden. Das Kommando prüft das nicht (kein Sperrmechanismus in `build`), warnt in der Hilfe aber davor; der nächste Build erzeugt alles neu.

## Risks / Trade-offs

- [Neue Cache-Datei wird vergessen] → Test über einen echten Build deckt alle Einträge ab.
- [Falscher `--cache`] → Pfadschutz und Beschränkung auf bekannte Namen.
- [Langer erster Build danach (mehrere Minuten bei 3000+ Bildern)] → Hinweis in der Ausgabe und im README.
