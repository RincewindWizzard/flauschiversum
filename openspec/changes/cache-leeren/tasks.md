## 1. Kommando

- [ ] 1.1 `src/clean.rs`: bekannte Cache-Einträge (`v1/`, `serve/`, `search-*.bin`, `search-*.tmp`) entfernen, Zähler für Dateien und Bytes, Pfadschutz (Projektverzeichnis und Eltern, `content/` oder `Cargo.toml` enthalten, `/`), Links nicht verfolgen; prüfen: Unit-Tests für Löschen, fremde Datei bleibt, leerer und fehlender Cache, Pfadschutz, Link wird nur selbst entfernt
- [ ] 1.2 Unterkommando `clean` in `src/main.rs` (`--root`, `--cache`, `--dist`, `--out`), deutsche Ausgabe mit Dateizahl und Größe, Exit-Codes; prüfen: `flauschiversum clean --help` und manuelle Läufe (voll, leer, `--cache .`)
- [ ] 1.3 `--dist`: Ausgabeverzeichnis nur leeren, wenn es vom Generator stammt (Prüfung wie in `output::write`); prüfen: Test mit eigenem und mit fremdem Ausgabeverzeichnis

## 2. Absicherung

- [ ] 2.1 Ende-zu-Ende-Test mit der Test-Site: Build, Clean (Cache leer), erneuter Build mit gleichem Ergebnis (Dateien byte-gleich bis auf Zeitstempel), `serve`-Verzeichnis eingeschlossen; prüfen: `cargo test` grün, und ein Test stellt sicher, dass jede vom Build angelegte Cache-Datei von `clean` erfasst wird

## 3. Dokumentation

- [ ] 3.1 README (Kommandos, Cache, Hinweis auf lange Neuberechnung) und GitHub-Workflow-Hinweis (Cache dort über `actions/cache`) ergänzen; prüfen: Beschreibung stimmt mit der Ausgabe von `--help` überein

## Workflow follow-up

- Nach der Abnahme `/opsx:archive cache-leeren` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
