## Why

Das Flauschiversum hat Staging auf GitHub Pages, aber keinen Weg auf den Produktivserver: Das alte Docker-Deployment wurde entfernt, und `deployment` beschreibt nur das Ergebnis, nicht den Weg der Dateien. Die fertige Seite soll reproduzierbar, ohne Handarbeit und ohne Zugangsdaten im Repository per SSH auf den Server (`flauschiversum:/httpdocs/`) gelegt werden, wahlweise automatisch aus einer GitHub Action oder aus einem Git-Hook. Die Logik gehört ins Werkzeug selbst (`flauschiversum deploy`) und nicht in ein Skript aus Bash und Python, das die Build-Pipeline schwer wartbar macht. Der Server (netcup-Webhosting) hat außerdem kein `rsync`.

## What Changes

- **Neuer Befehl `flauschiversum deploy`:** Baut die Seite für Produktion (Basis-URL `https://flauschiversum.de`, ohne Staging) und überträgt `dist/` über SSH nach `flauschiversum:/httpdocs/`. `flauschiversum` ist ein Host-Eintrag der SSH-Konfiguration (inklusive Schlüssel); das Werkzeug ruft das `ssh` des Systems auf und kennt weder Benutzer noch Schlüssel noch Port. Optionen: `--dry-run`, `--no-build`, `--target`, `--method`.
- **Ohne rsync:** Das Werkzeug packt die zu übertragenden Dateien selbst und schickt sie als `tar`-Strom über SSH; fehlt `tar` auf dem Server, nutzt es SFTP. Übertragen wird nur, was sich laut Manifest (`.flauschiversum-manifest`) geändert hat.
- **Sicher gegen Fehlgriffe:** `deploy` bricht ab, wenn `dist/` mit `--staging` oder einer anderen Basis-URL gebaut wurde. Gelöscht werden auf dem Server nur Dateien, die ein früheres Deployment selbst dort abgelegt hat (Abgleich über das Manifest); fremde Dateien in `/httpdocs/` bleiben. Das erste Deployment überlagert nur.
- **Ruhiges Umschalten:** Alles geht in ein Zwischenverzeichnis und wird dort einzeln an den Platz verschoben: erst Medien, Stylesheet, Skripte und Suchindex, dann die HTML-Seiten, zuletzt Entfernen veralteter Dateien und das neue Manifest. Bricht etwas ab, bleibt die alte Seite lauffähig.
- **GitHub Action `production.yml`:** Läuft bei jedem Push auf `master` (und von Hand), testet, schreibt die SSH-Konfiguration aus Repository-Secrets, ruft `flauschiversum deploy` auf und prüft die Live-Seite mit dem Smoke-Test im Produktionsmodus.
- **Git-Hook `post-receive`:** Für Umgebungen ohne GitHub Actions (eigener Git-Server): Bei einem Push auf `master` exportiert der Hook den Stand in ein temporäres Verzeichnis und ruft dort `flauschiversum deploy` auf. Ein Installationsskript legt ihn ab.
- **Smoke-Test für Produktion:** `scripts/smoke-test.sh` bekommt einen Produktionsmodus (kein `noindex`, keine Staging-Leiste, kanonische URL stimmt, `robots.txt` und Sitemap vorhanden).
- **Nicht Teil dieses Changes:** Einrichten des Servers (Webserver, TLS, DNS), Rollback-Automatik, Löschen des alten Inhalts in `/httpdocs/` beim ersten Mal (steht als Hinweis in der Anleitung), Umschreiben des Smoke-Tests in Rust.

## Capabilities

### New Capabilities

- `production-deployment`: Weg der gebauten Seite auf den Produktivserver per SSH: Befehl `deploy`, Schutz vor falschen Builds und Löschungen, Automatisierung über GitHub Action oder Git-Hook, Prüfung der Live-Seite.

### Modified Capabilities

Keine.

## Impact

- Neu: `src/deploy.rs` (Unterbefehl `deploy`), Abhängigkeiten `tar` und `flate2`, `scripts/git-hooks/post-receive`, `scripts/install-git-hook.sh`, `scripts/test-git-hook.sh`, `.github/workflows/production.yml`; geändert: `src/main.rs`, `src/output.rs` (Manifest lesbar), `scripts/smoke-test.sh`, README (Einrichtung, Secrets, erstes Deployment).
- Voraussetzungen: SSH-Zugang mit Host-Eintrag `flauschiversum` (nicht im Repository) mit einer Shell; auf dem Server `tar` und `gzip` (empfohlen) oder das SFTP-Subsystem; für die Action die Repository-Secrets `DEPLOY_SSH_KEY`, `DEPLOY_KNOWN_HOSTS`, `DEPLOY_HOST`, `DEPLOY_USER` (optional `DEPLOY_PORT`).
- Die Staging-Pipeline bleibt unverändert.
