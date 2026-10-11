# Proposal

## Why

Änderungen am Flauschiversum lassen sich bisher nur lokal ansehen; es gibt weder eine öffentliche Vorschau noch ein automatisches Deployment (das alte Docker-Deployment wurde entfernt). Ein Staging auf GitHub Pages, das bei jedem Stand von `dev` und von den Arbeitsbranches `claude/**` neu gebaut wird, macht Ergebnisse für Review und Abnahme sofort sichtbar, ohne den Produktivserver anzufassen. Das Produktiv-Deployment aus `main` ist bewusst nicht Teil dieses Changes.

## What Changes

- **Pipeline:** Ein GitHub-Actions-Workflow baut die Seite bei jedem Push auf den Branch `dev` (auch durch Merge) und auf Branches `claude/**` und veröffentlicht sie auf GitHub Pages. So ist die Arbeit von Claude sofort sichtbar, ohne dass jemand von Hand ein Deployment anstößt. Es gibt nur eine Staging-Seite: Der zuletzt gepushte Stand gewinnt. Vorher laufen die Inhaltsprüfung (`check`) und `cargo test`; schlägt eine fehl, wird nichts veröffentlicht. Nach dem Deployment prüft ein Smoke-Test die wichtigsten URLs der veröffentlichten Seite.
- **Unterpfad:** Das Staging liegt unter `https://rincewindwizzard.github.io/flauschiversum/` und damit nicht im Wurzelpfad. Der Generator bekommt dafür die Option `--base-url`; alle internen Verweise (Links, Bilder, Stylesheet, Schriften, Skripte, Suche) tragen dann den Pfadanteil der Basis-URL. Ohne die Option bleibt alles wie heute (`https://flauschiversum.de`, Wurzelpfad).
- **Branchname sichtbar:** Mit `--staging <Branch>` zeigt jede Seite ganz oben eine Leiste „Staging: <Branch>“, damit sofort erkennbar ist, welcher Stand online ist.
- **Nicht indexierbar:** Der Staging-Modus (`--staging`) setzt außerdem auf jeder Seite `noindex`, `robots.txt` sperrt alles und die Sitemap entfällt, damit das Staging nicht neben der Produktivseite in Suchmaschinen auftaucht. Canonical-URLs, Feed und Sitemap-Verweise zeigen auf die Basis-URL des Stagings.
- **Cache:** Der Bild-Cache (`.cache/`) wird zwischen den Läufen im GitHub-Actions-Cache gehalten, damit nicht bei jedem Push 3200 Bilder neu berechnet werden.
- **Einmalige Einrichtung im Repository (nicht per Code möglich):** Pages-Quelle auf „GitHub Actions“ stellen und in der Umgebung `github-pages` die Branches `dev` und `claude/**` für Deployments erlauben (`dev` ist bereits angelegt).

Nicht Teil dieses Changes: Produktiv-Deployment aus `main` (per SSH auf den Server), eigene Domain für das Staging, Vorschauen je Pull Request.

## Capabilities

### New Capabilities

- `staging-deployment`: Die Pipeline, die `dev` nach jedem Push baut, prüft, auf GitHub Pages veröffentlicht und die veröffentlichte Seite stichprobenartig testet.

### Modified Capabilities

- `url-scheme`: Eine konfigurierbare Basis-URL mit Pfadanteil; alle internen Verweise und Absolut-URLs richten sich danach.
- `site-layout`: Staging-Leiste mit Branchname am Seitenanfang.
- `seo`: Staging-Modus ohne Indexierung (noindex, robots.txt, keine Sitemap) und Basis-URL in kanonischen Verweisen.
- `build`: Die Optionen `--base-url` und `--staging <Branch>` für `build`, `check` und `serve`.

## Impact

- **Code:** `src/main.rs` (Optionen), `src/config.rs`/`src/urls.rs` (Basis-URL statt Konstante), `src/render.rs`, `src/seo.rs`, `src/feed.rs`, `src/site.rs` (Verweise, Staging-Modus), Templates (`layout.html` u.a.), `assets/js/media.js` und `search.js`, `content/style/fonts.css`; neu `.github/workflows/staging.yml`.
- **Abhängigkeiten:** Nur GitHub-Actions-Bausteine (checkout, Rust-Toolchain, Cache, `configure-pages`, `upload-pages-artifact`, `deploy-pages`); keine neuen Rust-Crates.
- **Betrieb:** GitHub Pages muss für das Repository eingeschaltet sein (bei privaten Repositories ist das Tarif-abhängig). Die gebaute Seite ist rund 550 MB groß; das Limit für eine Pages-Seite liegt bei 1 GB.
- **URLs:** Produktiv ändert sich nichts. Die Standard-Basis-URL bleibt `https://flauschiversum.de`.
