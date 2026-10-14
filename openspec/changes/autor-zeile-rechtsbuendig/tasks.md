## 1. Stylesheet

- [x] 1.1 In `content/style/page.css` die Regel `main.page p.author { text-align: right; margin-top: 2em; }` ergänzen; prüfen: Build erfolgreich, Beitrag mit Autor (z. B. „Alraune“) im Browser hell und dunkel angesehen, Zeile steht rechts mit Abstand zum Text, Link und Wortlaut unverändert

## 2. Tests

- [x] 2.1 Playwright-Test für Ausrichtung und Abstand von `p.author` (hell und dunkel) und Prüfung, dass Beiträge ohne Autor die Zeile nicht zeigen; prüfen: Test grün, `cargo test --release -- --include-ignored` grün

## Workflow follow-up

- Nach Freigabe `/opsx:archive autor-zeile-rechtsbuendig` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
