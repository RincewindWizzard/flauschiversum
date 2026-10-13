## Context

`dist/` entsteht mit `flauschiversum build` und enthält rund 550 MB (Bilder, Suchindex, HTML). Der Build schreibt nur Geändertes und legt `dist/.flauschiversum-manifest` an (Pfad → Inhalts-Hash), das auch bestimmt, was beim nächsten Build als „verwaist“ gilt. `scripts/` kennt `smoke-test.sh` (Staging) und `dist-size.sh`. Die Staging-Pipeline `.github/workflows/staging.yml` läuft bei `dev` und `claude/**`; `master` ist der Standardbranch und veröffentlicht bisher nichts. Zielserver: SSH-Host `flauschiversum` (Eintrag samt Schlüssel in `~/.ssh/config`), Zielpfad `/httpdocs/`. Ob der Server ein vollständiges Linux mit `rsync` ist, ist nicht bekannt (Annahme unten).

## Goals / Non-Goals

**Goals:** ein Befehl, der die fertige Seite sicher auf den Server legt; gleiche Logik in Action und Hook; keine Zugangsdaten im Repository; schnelle Folge-Deployments (nur Geändertes).
**Non-Goals:** Serverinstallation, Blue/Green mit Verzeichniswechsel (das Zielverzeichnis ist fest vorgegeben), Rollback-Automatik (ein Rollback ist ein Deployment des alten Commits).

## Decisions

### 1. Ein Skript für alle Umgebungen

`scripts/deploy.sh [--dry-run] [--no-build]` in Bash (`set -euo pipefail`), konfigurierbar über Umgebungsvariablen mit den Standardwerten `DEPLOY_TARGET=flauschiversum:/httpdocs/`, `FLAUSCHI_BASE_URL=https://flauschiversum.de`, `FLAUSCHI_CACHE_DIR=.cache`. Ablauf:

1. Sperre (`flock` auf `.deploy.lock`), damit nie zwei Deployments gleichzeitig laufen.
2. Ohne `--no-build`: `cargo run --release --locked -- check` und `build --base-url "$FLAUSCHI_BASE_URL" --cache "$FLAUSCHI_CACHE_DIR"` (ohne `--staging`).
3. Prüfung von `dist/`: `index.html` und `.flauschiversum-manifest` vorhanden; kein `class="staging-bar"`, kein `noindex` auf der Startseite, kanonische URL beginnt mit der Basis-URL, sonst Abbruch („falscher Build“). Größe über `scripts/dist-size.sh` (Warnung).
4. Erreichbarkeit: `ssh -o BatchMode=yes <Host> true` (kein Passwort-Prompt in CI), danach `command -v rsync` auf dem Server.
5. Übertragung in zwei Durchgängen (Entscheidung 3) und Aufräumen (Entscheidung 2).

Das Ziel `host:pfad` wird in Host und Pfad zerlegt. Ist das Ziel ein lokaler Pfad (kein Doppelpunkt), arbeitet das Skript ohne SSH: Das ist die Testumgebung (`scripts/test-deploy.sh` deployt in ein temporäres Verzeichnis, ohne Server), aber auch ein gültiger Einsatz, wenn der Build auf dem Webserver selbst läuft (dann zeigt `DEPLOY_TARGET` auf `/httpdocs/`).

### 2. Nur eigene Dateien löschen (Manifest statt `--delete`)

`rsync --delete` würde in `/httpdocs/` alles entfernen, was nicht in `dist/` liegt, auch fremde Dateien (Verifizierungsdateien, `.well-known`, Reste der alten Seite). Stattdessen: Das Skript liest das Manifest des letzten Deployments vom Server (`<Ziel>/.flauschiversum-manifest`, fehlt es: leere Liste), vergleicht es mit dem neuen und löscht nur Pfade, die früher geliefert wurden und jetzt fehlen (`ssh host 'cd /httpdocs && xargs -0 rm -f'`, anschließend leere Verzeichnisse darunter `rmdir`). Fremde Dateien bleiben. Das erste Deployment hat kein Manifest, es überlagert nur; das Skript meldet vorher die Zahl der Einträge im Ziel, die nicht aus `dist/` stammen. Pfade in der Löschliste werden gegen `..` und absolute Anteile geprüft. `--dry-run` zeigt Übertragung (`rsync -n`) und Löschliste, ändert nichts.

### 3. Was übertragen wird und in welcher Reihenfolge

Die Übertragungsliste kommt aus den Manifesten, nicht aus Zeitstempeln: In der Action liegt `dist/` nicht zwischen Läufen vor (alle Dateien sind frisch geschrieben), `rsync` würde allein nach Zeit und Größe alles neu hochladen. Das Skript vergleicht deshalb das lokale Manifest (`Pfad → Hash`) mit dem Manifest auf dem Server und überträgt nur Pfade, deren Hash abweicht oder die dort fehlen (kleines Python-3-Stück aus der Standardbibliothek, `python3` ist auf Runnern und Entwicklungsrechnern vorhanden). Die Liste geht mit `rsync --files-from` in zwei Durchgängen: (a) alles außer `*.html`, `*.xml` (Bilder, Stylesheet, Skripte, Suchindex), (b) HTML-Dateien, Feed und Sitemap. Danach die Löschung veralteter Dateien (Entscheidung 2) und **zuletzt** das neue Manifest. Dadurch verweisen neue Seiten nie auf noch nicht vorhandene Medien; bricht ein Durchgang ab, bleibt das Manifest auf dem Server unverändert, und die nächste Ausführung holt den Rest nach (sie überträgt wieder alles, was dem Manifest nicht entspricht). `--delay-updates` legt je Verzeichnis erst am Ende um. Rechte: `--chmod=D755,F644`, unabhängig von der umask des Absenders. Ohne vorhandenes Server-Manifest (erstes Deployment) wird alles übertragen.

*Alternative:* `rsync -a --delete` des ganzen Verzeichnisses. Verworfen: löscht fremde Dateien und lädt in der Action alles neu hoch.

### 4. GitHub Action

`.github/workflows/production.yml`: Trigger `push` auf `master` und `workflow_dispatch`; `concurrency: production` ohne Abbrechen (ein laufendes Deployment wird nie durch ein neueres unterbrochen); `permissions: contents: read`; `environment: production` (dort lassen sich Schutzregeln ergänzen). Schritte: Checkout, Rust und Cache wie im Staging, `cargo test --locked`, Bild-Cache (`actions/cache`, gleicher Schlüssel wie Staging), SSH einrichten, `scripts/deploy.sh`, Smoke-Test.

SSH einrichten: aus den Secrets `DEPLOY_SSH_KEY` (privater Schlüssel, nur für diesen Zweck), `DEPLOY_KNOWN_HOSTS` (Zeile(n) aus `ssh-keyscan`, vorab geprüft – kein blindes `ssh-keyscan` im Lauf), `DEPLOY_HOST`, `DEPLOY_USER`, optional `DEPLOY_PORT` entsteht `~/.ssh/config` mit `Host flauschiversum`, `IdentityFile`, `IdentitiesOnly yes`, `StrictHostKeyChecking yes`. Damit findet `deploy.sh` genau die Umgebung vor, die auch lokal besteht. Der Schlüssel wird nie ausgegeben und nach dem Lauf entfernt.

*Hinweis:* Das Staging-Spec verlangt „keine eigenen Secrets“ für den Staging-Workflow; das gilt weiter, nur der Produktions-Workflow nutzt Secrets.

### 5. Git-Hook für Umgebungen ohne GitHub Actions

`scripts/git-hooks/post-receive`: Liest die Zeilen (`alt neu ref`); nur für `refs/heads/master` (die Ref ist per `DEPLOY_REF` änderbar): legt ein temporäres Arbeitsverzeichnis an (`git archive`, dann `tar`), exportiert `CARGO_TARGET_DIR` und `FLAUSCHI_CACHE_DIR` auf dauerhafte Verzeichnisse (`$HOME/.cache/flauschiversum/{target,images}`, damit Bilder und Compiler-Ergebnisse nicht jedes Mal neu entstehen), ruft `scripts/deploy.sh` auf, schreibt das Protokoll nach `$HOME/.cache/flauschiversum/deploy.log` und räumt auf. Das Ergebnis wird dem Pusher gemeldet (Hook-Ausgabe). `scripts/install-git-hook.sh <bare-Repository>` kopiert den Hook mit Ausführungsrecht. Läuft der Hook auf dem Webserver selbst, genügt `DEPLOY_TARGET=/httpdocs/`.

*Annahme:* „Git-Hook je nach Umgebung“ heißt: ein serverseitiger Hook für ein Repository auf eigenem Git-Server. Ein lokaler `pre-push`-Hook wäre eine Alternative, wurde aber verworfen, weil er vor dem Gelingen des Pushs deployen und bei jedem Entwickler laufen würde.

### 6. Smoke-Test im Produktionsmodus

`scripts/smoke-test.sh <Basis-URL>` bekommt `SMOKE_MODE=production`: Dieselben Statusprüfungen (`/`, `/suche/`, `/pagefind/pagefind.js`, `/style.css`, Beitrag), zusätzlich `/robots.txt` und `/sitemap.xml` (200), die Startseite enthält **kein** `noindex` und **keine** Staging-Leiste, und die kanonische URL beginnt mit der Basis-URL. Der Staging-Modus bleibt der Standard.

### 7. Annahmen und offene Voraussetzungen

- `rsync` ist auf dem Server vorhanden. Fehlt es (reiner SFTP-Zugang), bricht das Skript mit klarer Meldung ab; ein SFTP-Pfad wäre ein eigener Change.
- Der Zielpfad `/httpdocs/` ist absolut auf dem Server, der Benutzer hat dort Schreibrechte.
- Der Branch heißt `master` (Standardbranch des Repositorys); `dev` und `claude/**` bleiben Staging.
- Das HTTPS-Zertifikat der Produktionsdomain liegt beim Server (nicht Teil dieses Changes).

## Risks / Trade-offs

- [Falscher Build geht live (z. B. mit Staging-Leiste)] → Prüfung von `dist/` im Skript und Smoke-Test nach dem Deployment.
- [Löschen fremder Dateien] → Löschen nur über das Manifest; `--dry-run`; Pfadprüfung der Löschliste.
- [Halb übertragene Seite] → Zwei Durchgänge in fester Reihenfolge, Manifest zuletzt, `--delay-updates`; die nächste Ausführung heilt.
- [SSH-Schlüssel in der CI] → eigener, nur für diesen Server gültiger Schlüssel, feste `known_hosts`, `environment: production` für Schutzregeln; Schlüssel nie in Logs.
- [Erste Übertragung dauert lang (550 MB)] → einmalig; danach geht nur, was sich laut Manifest geändert hat, auch aus der Action ohne zwischengespeichertes `dist/`.
- [Manifest auf dem Server veraltet oder von Hand gelöscht] → Die nächste Ausführung überträgt dann alles und löscht nichts (kein Manifest, keine Löschliste); das Ergebnis ist richtig, nur langsamer.
