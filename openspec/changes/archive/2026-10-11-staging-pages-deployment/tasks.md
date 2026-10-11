# Tasks

## 1. Basis-URL und Staging-Modus im Generator

- [x] 1.1 Basis-URL einführen: Option `--base-url` und Umgebungsvariable `FLAUSCHI_BASE_URL` für `build`, `check` und `serve`, Validierung (`http(s)://host`, kein Pfad, kein `/` am Ende), `BLOG_URL` aus `config.rs`/`urls.rs`/`seo.rs`/`feed.rs`/`render.rs` durch die Basis ersetzen; prüfen: Unit-Tests für gültige und ungültige Werte (Meldung mit Wert und erwartetem Format) und Test, dass die Fixture-Site mit Basis `https://staging.flauschiversum.de` in `canonical`, Open Graph, Feed und strukturierten Daten diese Basis trägt und interne Links unverändert bleiben
- [x] 1.2 Staging-Modus (`--staging <Branch>`, Wert Pflicht): `noindex` auf jeder HTML-Seite, Leiste „Staging: <Branch>“ ganz oben auf jeder Seite (maskiert, umbrechend, Stil in `layout.css`), `robots.txt` mit `Disallow: /` ohne Sitemap-Zeile, keine `sitemap.xml`; prüfen: Snapshot-/Integrationstests für Staging (Leiste mit Branchname auf Start-, Beitrags- und Suchseite) und für Produktion (keine Leiste, kein zusätzliches `noindex`); Browser-Test: Leiste am Handy ohne waagerechtes Scrollen bei einem langen Branchnamen
- [x] 1.3 Produktiv-Ausgabe unverändert: Ausgabe ohne Optionen vor und nach der Änderung vergleichen; prüfen: `cargo test --release -- --ignored` grün und Vergleich der beiden `dist/`-Verzeichnisse (Hash-Liste) ohne Unterschied
- [x] 1.4 `serve --staging <Branch>` zeigt die Leiste, `serve --base-url …` übernimmt die Basis in die Absolut-URLs; prüfen: Playwright-Test gegen `serve --staging test/branch` findet die Leiste

## 2. Pipeline

- [x] 2.1 `.github/workflows/staging.yml` mit Auslöser `push` auf `dev` und `claude/**` und `workflow_dispatch`, minimalen Berechtigungen, `concurrency` und den Jobs `test`, `build` und `deploy` (Entscheidung 4); Rust-Cache und Bild-Cache (Entscheidung 5); `--staging "$GITHUB_REF_NAME"` und `--base-url` aus `configure-pages`; prüfen: YAML-Syntax mit `actionlint` (oder `python -c yaml.safe_load`) und Durchsicht gegen die aktuellen Versionen der verwendeten Aktionen
- [x] 2.2 Smoke-Test im Job `deploy` (Startseite, `/suche/`, `/pagefind/pagefind.js`, `/style.css`, ein fester Beitrag, `noindex` und Staging-Leiste auf der Startseite), mit klarer Fehlermeldung je URL; prüfen: das Skript läuft lokal gegen `serve --staging …` grün und gegen eine absichtlich falsche Adresse rot
- [x] 2.3 `dist/staging.txt` mit Branch und Commit schreiben und im Protokoll ausgeben; prüfen: Lauf zeigt beides, die Datei ist unter der Staging-Adresse abrufbar
- [x] 2.4 Größenwarnung: Der Build-Job gibt die Größe von `dist/` aus und warnt ab 800 MB; prüfen: Ausgabe im Lauf (lokal mit dem Skript nachgestellt)

## 3. Einrichtung und Dokumentation

- [x] 3.1 `README.md`: Abschnitt „Staging“ (Auslöser, Adresse, Optionen `--base-url` und `--staging`, lokale Vorschau mit Leiste) und `openspec/config.yaml` (Projektkontext) aktualisieren; prüfen: die beschriebenen Befehle funktionieren wie angegeben
- [x] 3.2 Einrichtung mit der Nutzerin abstimmen und durchführen: DNS `CNAME staging → rincewindwizzard.github.io`, Pages-Quelle „GitHub Actions“, Custom domain `staging.flauschiversum.de` mit „Enforce HTTPS“, Umgebung `github-pages` für `dev` und `claude/*` (bereits freigegeben); prüfen: der erste Lauf nach einem Push ist grün und `https://staging.flauschiversum.de/` antwortet mit der Startseite samt Staging-Leiste
- [x] 3.3 Abnahme am veröffentlichten Staging: Navigation, Suche, Karussell, Vollbild, ein Beitrag, Feed-Adresse, Branchname in der Leiste; prüfen: Freigabe im Gespräch

## Workflow follow-up

- Nach der Abnahme `/opsx:archive staging-pages-deployment` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
- Danach (eigener Change): Produktiv-Deployment aus `main` per SSH; die Hauptspecs `deployment` und `static-pages` beschreiben noch das Verhalten vor dem Neubau.
