# Design

## Context

Der Generator (Rust) erzeugt heute eine Seite für den Wurzelpfad von `https://flauschiversum.de`: Die Konstante `BLOG_URL` steckt in `urls::absolute`, `seo`, `feed` und `render`; interne Links sind wurzelbezogen (`/themen/`, `/style.css`) in Templates, im gerenderten Markdown, in `fonts.css` (`url(/fonts/…)`), in `media.js` (statische und dynamische Importe) und in der Suchseite (Pagefind liefert wurzelbezogene Ergebnis-URLs). Es gibt keine CI (`.github/` fehlt); der Default-Branch heißt `master`, `dev` existiert noch nicht. Die gebaute Seite hat rund 10.000 Dateien und 546 MB, der Bild-Cache 1,1 GB, ein kalter Build dauert etwa 25 s auf 4 Kernen. GitHub Pages liefert ein Projekt-Repository unter `https://<user>.github.io/<repo>/`, also im Unterpfad. Der Nutzer hat diesen Unterpfad gewählt (keine eigene Domain) und das Staging als nicht indexierbar gewünscht.

## Goals / Non-Goals

**Goals:**
- Jeder Stand von `dev` ist nach wenigen Minuten unter der Pages-Adresse sichtbar, vorher geprüft und getestet.
- Dieselbe Quelle baut Staging und (später) Produktion; nur Basis-URL und Staging-Modus unterscheiden sich.
- Produktiv-Ausgabe bleibt byte-gleich zu heute, wenn keine Optionen gesetzt sind.

**Non-Goals:**
- Produktiv-Deployment aus `main` (SSH-Push) und eigene Staging-Domain.
- Vorschau je Pull Request, mehrere gleichzeitige Staging-Stände.
- Die Pages-Einstellungen im Repository per Code verwalten.

## Decisions

### 1. Basis-URL als Laufzeitwert, Pfade beim Rendern präfixen

`BLOG_URL` wird zu einem Wert `Base { origin, path }` (aus `--base-url` bzw. `FLAUSCHI_BASE_URL`, Standard `https://flauschiversum.de`), der in `Common` und in `urls` zur Verfügung steht. Internes Linken läuft über eine Funktion `urls::href(pfad)`, die `path` voranstellt; Templates erhalten `{{ base }}` (Pfadanteil ohne Schrägstrich am Ende, leer im Normalfall). Markdown-Ziele, die mit `/` beginnen, und Bildpfade werden in `markdown.rs` beim Rendern präfixiert (dort werden Ziele bereits umgeschrieben). Die Dateien im Ausgabeverzeichnis bleiben an denselben Stellen; nur der Inhalt der Verweise ändert sich.

*Alternativen:* (a) Ein Nachbearbeitungsschritt, der fertige HTML-Seiten per Textsuche nach `href="/`, `src="/` usw. umschreibt: wenig Eingriff, aber unzuverlässig bei rohem HTML im Markdown, `srcset`, Inline-Skripten und doppelt umschriebenen Verweisen. (b) Relative Verweise (`../../themen/`): braucht je Seite die Tiefe, ändert alle Ausgaben und bricht den Feed. Gewählt ist das zentrale Präfixieren, weil es genau eine Stelle je Verweisart gibt und die Link-Prüfung (`check_links`) die Basis kennen kann.

### 2. Schriften, Skripte und Suche ohne feste Pfade

- `fonts.css`: `url(fonts/Lobster.otf)` relativ zu `/style.css`, das im Wurzelverzeichnis der Seite liegt, funktioniert in jeder Basis und braucht keine Umschreibung.
- `media.js`: statische und dynamische Importe sowie der Stylesheet-Verweis werden relativ zum Modul aufgelöst (`new URL('../vendor/…', import.meta.url)`), nicht mehr wurzelbezogen.
- Suchseite: Das Template übergibt die Basis per `data-base` am Suchcontainer. `search.js` setzt Pagefinds `baseUrl` darauf; Ergebnis-Links und -Bilder erhalten das Präfix. Ob `baseUrl` Bilder und Links vollständig abdeckt, klärt die erste Aufgabe der Suche mit einem Browser-Test im Unterpfad; reicht es nicht, setzt `search.js` das Präfix selbst auf die Ergebnisse (die Oberfläche bietet dafür `processResult`).
- Live-Reload (`/__livereload`) und der Vorschau-Server (`serve`) kennen die Basis: Der Server liefert die Seite unter dem Pfadanteil aus und nimmt ihn von der Anfrage ab.

### 3. Staging-Modus als eigener Schalter

`--staging <Branch>` ist unabhängig von der Basis-URL und trägt den Branchnamen als Wert: Es setzt `noindex` in `layout.html` (das Feld `Head.noindex` gibt es bereits für die Suchseite), liefert `robots.txt` mit `Disallow: /` und lässt die Sitemap weg und blendet über `layout.html` eine Leiste „Staging: <Branch>“ vor dem Seitenkopf ein (einfaches Element mit eigener Farbe in `topics.css`/`layout.css`, ohne JavaScript, `overflow-wrap: anywhere` für lange Namen). Der Name wird HTML-maskiert. Feed und kanonische URLs folgen der Basis. Produktion nutzt den Schalter nie; so kann der spätere Prod-Workflow denselben Build mit `--base-url https://flauschiversum.de` ohne `--staging` aufrufen.

*Alternative:* `noindex` automatisch aus „Basis ≠ flauschiversum.de“ ableiten. Verworfen: zu implizit, und eine künftige Produktiv-Domain-Änderung würde die Seite unbemerkt aus dem Index nehmen.

### 4. Workflow `.github/workflows/staging.yml`

Auslöser: `push` auf `dev` und auf `claude/**` und `workflow_dispatch`. Berechtigungen: `contents: read`, `pages: write`, `id-token: write`. `concurrency: { group: pages-staging, cancel-in-progress: true }` für alle Branches gemeinsam, damit der neueste Stand gewinnt; es gibt nur eine Pages-Seite. Der Branchname kommt aus `github.ref_name` (bei `workflow_dispatch` der gewählte Branch) und wird an `--staging` übergeben; ein Schritt schreibt Branch und Commit nach `dist/staging.txt`, damit sichtbar ist, welcher Stand online ist.

Jobs:
1. **test** (parallel zum Bauen): Rust-Toolchain (stable), `Swatinem/rust-cache`, `cargo test`.
2. **build**: Checkout, Toolchain, `rust-cache`, `actions/configure-pages` (liefert `base_url`), Cache für `.cache/` (siehe 5), `cargo run --release -- check --staging --base-url "$BASE"`, dann `build` mit denselben Optionen, `actions/upload-pages-artifact` für `dist/`.
3. **deploy** (`needs: [test, build]`, `environment: github-pages`): `actions/deploy-pages`; danach der **Smoke-Test** mit `curl --fail` auf Startseite, `/suche/`, `/pagefind/pagefind.js`, `/style.css` und einen festen Beitrag, plus `grep noindex` auf der Startseite.

Aktionen werden auf Hauptversionen oder Commit-SHAs festgelegt; beim Anlegen werden die aktuellen Versionen geprüft.

*Alternative:* Zweig `gh-pages` mit einer Push-Aktion. Verworfen: der Weg über `deploy-pages` braucht keinen Schreibzugriff auf Branches und hinterlässt keine 550-MB-Commits im Repository.

### 5. Bild-Cache im Actions-Cache

`actions/cache` mit Schlüssel `images-${{ github.run_id }}` und `restore-keys: images-`, Pfad `.cache/`. So wird immer der letzte Stand geladen und nach dem Lauf ein neuer gespeichert. Der Cache ist 1,1 GB groß (Limit je Repository 10 GB); er wird nur im Job `build` genutzt. Fällt der Cache aus, rechnet der Build alle Bilder neu (ca. 25 s auf 4 Kernen, auf Actions-Runnern mit 4 Kernen ähnlich), die Seite bleibt korrekt.

### 6. Claude-Branches veröffentlichen mit

`claude/**` löst denselben Workflow aus wie `dev`, damit Änderungen aus Claude-Sitzungen ohne manuelles Anstoßen sichtbar werden. Das Muster `claude/**` passt auf alle Sitzungsbranches. Der Workflow läuft nur für Pushes in dieses Repository (nicht für Forks), und `contents: read` plus Pages-Schreibrecht reichen aus.

*Alternative:* nur `dev` veröffentlichen und Claude-Branches per Pull Request nach `dev` bringen. Verworfen: Jede Änderung bräuchte einen manuellen Schritt, bevor man sie sehen kann.

### 7. Einmalige Einrichtung bleibt manuell

Zwei Schritte kann der Workflow nicht selbst erledigen; die Aufgaben nennen sie als Anleitung an die Nutzerin: Settings → Pages → Source „GitHub Actions“, und in Settings → Environments → `github-pages` die Deployment-Branches `dev` und `claude/*` hinzufügen (voreingestellt ist nur der Default-Branch; die Umgebung erlaubt Namensmuster mit `*`). Ohne den letzten Schritt scheitert `deploy` mit einer Schutzmeldung der Umgebung. Der Branch `dev` ist bereits angelegt.

## Risks / Trade-offs

- [Mehrere Branches überschreiben sich gegenseitig auf der einen Staging-Seite] → Bewusst so; `staging.txt` und das Protokoll zeigen den Stand. Ein Zustand von `dev` wird nicht automatisch wiederhergestellt, wenn ein Claude-Branch danach pusht; ein manueller Lauf auf `dev` (`workflow_dispatch`) holt ihn zurück.

- [Pages-Limit: 1 GB je Seite, 550 MB heute; weiteres Wachstum durch neue Posts] → Smoke-Test und Build melden die Größe; bei Annäherung an 800 MB Warnung im Workflow-Log. Langfristig die 1000x-Varianten oder Videos auslagern (eigener Change).
- [Hochladen von 10.000 Dateien dauert] → `upload-pages-artifact` packt ein Tar; ein Lauf soll unter ca. 10 Minuten bleiben, andernfalls Bilder-Varianten prüfen.
- [Private Repositories: Pages braucht je nach Tarif einen bezahlten Plan] → Der Lauf meldet beim ersten Deployment einen klaren Fehler; Sichtbarkeit des Repositories vorab prüfen.
- [Unterpfad-Umschreibung übersieht einen Verweis] → Der Build prüft interne Verweise gegen die Basis (`check_links`), ein Test baut die Fixture-Site mit Unterpfad und prüft, dass kein `href="/…"` ohne Präfix übrig bleibt, und Browser-Tests laufen gegen `serve --base-url`.
- [Pagefind-Ergebnisse ohne Präfix] → Siehe Entscheidung 2; der Browser-Test im Unterpfad deckt das ab.
- [Gebrochene Aktionen/Versionen] → Festgelegte Versionen, `workflow_dispatch` zum erneuten Starten.

## Migration Plan

1. Code für Basis-URL und Staging-Modus umsetzen und lokal mit `build --base-url http://localhost:8099/flauschiversum --staging` und `serve` prüfen; Produktiv-Ausgabe ohne Optionen bleibt byte-gleich (Gesamtseitentest).
2. Workflow anlegen und auf einem Test-Branch mit `workflow_dispatch` prüfen, soweit die Umgebungsregeln das zulassen; sonst direkt auf `dev`.
3. Nutzerin stellt Pages und Umgebung ein (Entscheidung 7); der erste Push auf einen Claude-Branch oder `dev` löst den ersten Lauf aus.
4. Rollback: Workflow deaktivieren oder löschen; die zuletzt veröffentlichte Seite bleibt, bis Pages ausgeschaltet wird. Produktivseite unberührt.
