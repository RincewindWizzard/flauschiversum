## 1. Umsetzung

- [x] 1.1 `src/progress.rs` mit `enable`, `begin`, `step`, `note`, `count`; prüfen: Kaltstart zeigt Schritte mit Zeiten und Dauer des vorherigen Schritts
- [x] 1.2 Schritte in `site::generate`, Zähler in `media::build`, Hinweise in `search::build`, Aktivierung in `build`, `serve` und `check`; prüfen: Lauf mit leerem Cache zeigt Zwischenstände alle 2 s und Abschluss 3197/3197, Lauf mit vollem Cache zeigt „aus dem Cache“ und „Suchindex aus dem Cache übernommen“
- [x] 1.3 `cargo test` grün (Tests bleiben still); prüfen: kein Testlauf gibt Fortschrittszeilen aus

## 2. Dokumentation

- [x] 2.1 README (Fortschrittsmeldungen) ergänzen; prüfen: Beschreibung stimmt mit der Ausgabe überein

## Workflow follow-up

- Nach der Abnahme `/opsx:archive build-fortschritt` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
