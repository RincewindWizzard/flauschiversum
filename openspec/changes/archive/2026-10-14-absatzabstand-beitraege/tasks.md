## 1. Stylesheet

- [x] 1.1 In `content/style/page.css` die Abstandsregeln für `main.page` ergänzen (Absätze ohne Klasse, Listen, Zitate, `pre` mit `margin-bottom: 1.6em`; Überschriften `1.6em` davor und `0.5em` danach), das bisherige `padding-top` der `h2` und die `margin-bottom`-Regel der direkten `ul` damit zusammenführen; prüfen: `cargo run --release -- build` baut, `dist/style.css` enthält die Regel, Beitrag mit Absätzen, Liste, Überschrift und Bildern im Browser (hell und dunkel) angesehen, keine doppelten Abstände an Figuren und Bildergruppen

## 2. Tests

- [x] 2.1 Playwright-Test für den Absatzabstand (zwei aufeinanderfolgende Absätze, Abstand größer als die Zeilenhöhe, hell und dunkel) und Prüfung, dass `p.author`/`p.contact` und eine Übersichtsseite unverändert sind; prüfen: Test grün, `cargo test --release -- --include-ignored` grün (bei Snapshot-Änderungen die Änderungen prüfen und übernehmen)

## Workflow follow-up

- Nach Freigabe `/opsx:archive absatzabstand-beitraege` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
