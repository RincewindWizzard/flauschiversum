# Tasks

## 1. Basis-URL und Staging-Modus im Generator

- [ ] 1.1 `Base` (Origin und Pfadanteil) einführen: Option `--base-url` und Umgebungsvariable `FLAUSCHI_BASE_URL` für `build`, `check` und `serve`, Validierung (Schema `http(s)://`, kein `/` am Ende), `BLOG_URL` aus `config.rs`/`urls.rs`/`seo.rs`/`feed.rs`/`render.rs` durch die Basis ersetzen; prüfen: Unit-Tests für gültige und ungültige Werte (Meldung mit Wert und erwartetem Format) und für `urls::absolute` mit Unterpfad
- [ ] 1.2 Interne Verweise mit Pfadpräfix ausgeben: `urls::href`, `{{ base }}` in allen Templates (Navigation, Suchformulare, Favicon, Suchseite), Präfix für Markdown-Ziele und Bildpfade in `markdown.rs`, Feed-Inhalte; prüfen: Test baut die Fixture-Site mit Basis `https://example.org/flauschiversum` und findet in keiner HTML-Seite ein `href`/`src`/`action` mit führendem `/` ohne Präfix (außer protokollrelativ und Ankern)
- [ ] 1.3 Schriften und Skripte ohne feste Pfade: `fonts.css` relativ, `media.js` mit `new URL(…, import.meta.url)` für Importe und Stylesheet; prüfen: Playwright-Test für Karussell und Vollbild gegen die Seite im Unterpfad (Schritt 1.5) und unveränderte Ausgabe ohne Basis
- [ ] 1.4 Suche im Unterpfad: `data-base` am Suchcontainer, `search.js` setzt `baseUrl` (bzw. `processResult`), sodass Ergebnis-Links und -Bilder das Präfix tragen; prüfen: Browser-Test im Unterpfad sucht „katze“ und klickt einen Treffer, der unter `/<Unterpfad>/…` öffnet
- [ ] 1.5 `serve` mit Basis: Server liefert unter dem Pfadanteil aus, nimmt ihn von Anfragen ab, Live-Reload und Weiterleitung von Verzeichnissen kennen ihn; prüfen: `serve --base-url http://localhost:8099/flauschiversum` antwortet auf `/flauschiversum/` mit 200 und auf `/` mit 404, Unit-Test für die Pfadauflösung
- [ ] 1.6 Link-Prüfung (`check_links`) kennt die Basis; prüfen: Fixture-Build mit Unterpfad meldet keine zusätzlichen Warnungen, ein absichtlich kaputter Link wird weiter gemeldet
- [ ] 1.7 Staging-Modus (`--staging <Branch>`, Wert Pflicht): `noindex` auf jeder HTML-Seite, Leiste „Staging: <Branch>“ ganz oben auf jeder Seite (maskiert, umbrechend), `robots.txt` mit `Disallow: /` ohne Sitemap-Zeile, keine `sitemap.xml`; prüfen: Snapshot-/Integrationstests für Staging (Leiste mit Branchname auf Start-, Beitrags- und Suchseite) und für Produktion (keine Leiste); Browser-Test: Leiste am Handy ohne waagerechtes Scrollen bei einem langen Branchnamen; Produktiv-Build ohne Optionen ist byte-gleich zu vorher (`cargo test --release -- --ignored` und Vergleich der Ausgabe vor/nach der Änderung)

## 2. Pipeline

- [ ] 2.1 `.github/workflows/staging.yml` mit Auslöser `push` auf `dev` und `claude/**` und `workflow_dispatch`, minimalen Berechtigungen, `concurrency` und den Jobs `test`, `build` und `deploy` (Entscheidung 4); Rust-Cache und Bild-Cache (Entscheidung 5); prüfen: YAML-Syntax mit `actionlint` (oder `python -c yaml.safe_load`) und Durchsicht gegen die aktuellen Versionen der verwendeten Aktionen
- [ ] 2.2 Smoke-Test im Job `deploy` (Startseite, `/suche/`, `/pagefind/pagefind.js`, `/style.css`, ein fester Beitrag, `noindex` auf der Startseite), mit klarer Fehlermeldung je URL; prüfen: das Skript läuft lokal gegen `serve --base-url …` grün und gegen eine absichtlich falsche Basis rot
- [ ] 2.3 `dist/staging.txt` mit Branch und Commit schreiben und im Protokoll ausgeben; prüfen: Lauf zeigt beides, die Datei ist unter der Staging-Adresse abrufbar
- [ ] 2.4 Größenwarnung: Der Build-Job gibt die Größe von `dist/` aus und warnt ab 800 MB; prüfen: Ausgabe im Lauf (lokal mit dem Skript nachgestellt)

## 3. Einrichtung und Dokumentation

- [ ] 3.1 `README.md`: Abschnitt „Staging“ (Auslöser, Adresse, Optionen `--base-url` und `--staging`, lokale Vorschau im Unterpfad) und `openspec/config.yaml` (Projektkontext) aktualisieren; prüfen: die beschriebenen Befehle funktionieren wie angegeben
- [ ] 3.2 Einmalige Einrichtung mit der Nutzerin klären und durchführen: Pages-Quelle „GitHub Actions“, Umgebung `github-pages` für `dev` und `claude/*` freigeben (`dev` existiert bereits); prüfen: der erste Lauf nach einem Push ist grün, `https://rincewindwizzard.github.io/flauschiversum/` antwortet mit der Startseite
- [ ] 3.3 Abnahme am veröffentlichten Staging: Navigation, Suche, Karussell, Vollbild, ein Beitrag, Feed-Adresse; prüfen: Freigabe im Gespräch

## Workflow follow-up

- Nach der Abnahme `/opsx:archive staging-pages-deployment` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
- Danach (eigener Change): Produktiv-Deployment aus `main` per SSH; die Hauptspecs `deployment` und `static-pages` beschreiben noch das Verhalten vor dem Neubau.
