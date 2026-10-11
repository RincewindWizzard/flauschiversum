# Design

## Context

Der Generator (Rust) erzeugt heute eine Seite für den Wurzelpfad von `https://flauschiversum.de`: Die Konstante `BLOG_URL` steckt in `urls::absolute`, `seo`, `feed` und `render` (kanonische Verweise, Open Graph, Feed, Sitemap). Interne Links sind wurzelbezogen (`/themen/`, `/style.css`). Es gibt keine CI (`.github/` fehlt); der Default-Branch heißt `master`, `dev` ist inzwischen angelegt. Die gebaute Seite hat rund 10.000 Dateien und 546 MB, der Bild-Cache 1,1 GB, ein kalter Build dauert etwa 25 s auf 4 Kernen. Das Staging bekommt die eigene Domain `staging.flauschiversum.de`, liegt also wie Produktion im Wurzelpfad; ein Pfadpräfix für interne Links ist nicht nötig (und würde das Testen erschweren). Das Staging soll nicht indexierbar sein und den Branchnamen oben zeigen.

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

### 1. Basis-URL als Laufzeitwert (nur Schema und Host)

`BLOG_URL` wird zu einem Wert, der aus `--base-url` bzw. `FLAUSCHI_BASE_URL` stammt (Standard `https://flauschiversum.de`), geprüft auf `http(s)://host` ohne Pfad und ohne `/` am Ende, und in `Common`/`urls` bereitsteht. Er ersetzt die Konstante in `urls::absolute`, `seo`, `feed` und `render`. Interne Verweise, Skripte, Schriften und Suche bleiben unverändert; alle Browser-Tests laufen weiter gegen `serve` im Wurzelpfad.

*Alternative:* Unterpfad auf `github.io/flauschiversum/` mit Pfadpräfix für alle Links, `media.js`, Schriften und Suche. Verworfen: großer Eingriff in Templates, Markdown, Skripte und Suche, und jede Prüfung bräuchte eine zweite Konfiguration. Die eigene Domain vermeidet das vollständig.

### 2. Domain über Pages „Custom domain“

DNS: `CNAME staging → rincewindwizzard.github.io`. In den Pages-Einstellungen wird `staging.flauschiversum.de` eingetragen und „Enforce HTTPS“ aktiviert; da per Actions deployt wird, braucht es keine `CNAME`-Datei im Artefakt. `actions/configure-pages` liefert die Custom domain als `base_url`, der Workflow übergibt sie an `--base-url`. Empfohlen ist die Domain-Verifizierung im GitHub-Konto (TXT `_github-pages-challenge-…`), um Übernahmen bei verwaistem CNAME zu verhindern.

### 3. Staging-Modus als eigener Schalter

`--staging <Branch>` ist unabhängig von der Basis-URL und trägt den Branchnamen als Wert: Es setzt `noindex` in `layout.html` (das Feld `Head.noindex` gibt es bereits für die Suchseite), liefert `robots.txt` mit `Disallow: /` und lässt die Sitemap weg und blendet über `layout.html` eine Leiste „Staging: <Branch>“ vor dem Seitenkopf ein (einfaches Element mit eigener Farbe in `topics.css`/`layout.css`, ohne JavaScript, `overflow-wrap: anywhere` für lange Namen). Der Name wird HTML-maskiert. Feed und kanonische URLs folgen der Basis. Produktion nutzt den Schalter nie; so kann der spätere Prod-Workflow denselben Build ohne `--staging` aufrufen.

*Alternative:* `noindex` automatisch aus „Basis ≠ flauschiversum.de“ ableiten (Staging wäre dann an die Domain gebunden). Verworfen: zu implizit, und eine künftige Produktiv-Domain-Änderung würde die Seite unbemerkt aus dem Index nehmen.

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

Zwei Schritte kann der Workflow nicht selbst erledigen; die Aufgaben nennen sie als Anleitung an die Nutzerin: DNS-Eintrag `staging`, Settings → Pages → Source „GitHub Actions“ und Custom domain, und in Settings → Environments → `github-pages` die Deployment-Branches `dev` und `claude/*` hinzufügen (voreingestellt ist nur der Default-Branch; die Umgebung erlaubt Namensmuster mit `*`). Ohne den letzten Schritt scheitert `deploy` mit einer Schutzmeldung der Umgebung. Der Branch `dev` ist bereits angelegt.

## Risks / Trade-offs

- [Mehrere Branches überschreiben sich gegenseitig auf der einen Staging-Seite] → Bewusst so; `staging.txt` und das Protokoll zeigen den Stand. Ein Zustand von `dev` wird nicht automatisch wiederhergestellt, wenn ein Claude-Branch danach pusht; ein manueller Lauf auf `dev` (`workflow_dispatch`) holt ihn zurück.

- [Pages-Limit: 1 GB je Seite, 550 MB heute; weiteres Wachstum durch neue Posts] → Smoke-Test und Build melden die Größe; bei Annäherung an 800 MB Warnung im Workflow-Log. Langfristig die 1000x-Varianten oder Videos auslagern (eigener Change).
- [Hochladen von 10.000 Dateien dauert] → `upload-pages-artifact` packt ein Tar; ein Lauf soll unter ca. 10 Minuten bleiben, andernfalls Bilder-Varianten prüfen.
- [DNS oder Zertifikat noch nicht bereit: Pages meldet „DNS check unsuccessful“, HTTPS ist erst nach einigen Minuten bis Stunden verfügbar] → Die Einrichtung (Aufgabe 3.2) erfolgt vor dem ersten Lauf; bis dahin ist die Seite unter `https://rincewindwizzard.github.io/flauschiversum/` mit kaputten Links erreichbar, was für den Smoke-Test nicht zählt.
- [Private Repositories: Pages braucht je nach Tarif einen bezahlten Plan] → Der Lauf meldet beim ersten Deployment einen klaren Fehler; Sichtbarkeit des Repositories vorab prüfen.
- [Gebrochene Aktionen/Versionen] → Festgelegte Versionen, `workflow_dispatch` zum erneuten Starten.

## Migration Plan

1. Code für Basis-URL und Staging-Modus umsetzen und lokal mit `build --base-url https://staging.flauschiversum.de --staging <Branch>` und `serve --staging <Branch>` prüfen; Produktiv-Ausgabe ohne Optionen bleibt byte-gleich (Gesamtseitentest).
2. Workflow anlegen und auf einem Test-Branch mit `workflow_dispatch` prüfen, soweit die Umgebungsregeln das zulassen; sonst direkt auf `dev`.
3. Nutzerin setzt DNS und stellt Pages, Custom domain und Umgebung ein (Entscheidungen 2 und 7); der erste Push auf einen Claude-Branch oder `dev` löst den ersten Lauf aus.
4. Rollback: Workflow deaktivieren oder löschen; die zuletzt veröffentlichte Seite bleibt, bis Pages ausgeschaltet wird. Produktivseite unberührt.
