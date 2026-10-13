## Why

Das Flauschiversum hat Staging auf GitHub Pages, aber keinen Weg auf den Produktivserver: Das alte Docker-Deployment wurde entfernt, und `deployment` beschreibt nur das Ergebnis, nicht den Weg der Dateien. Die fertige Seite soll reproduzierbar, ohne Handarbeit und ohne Zugangsdaten im Repository per SSH auf den Server (`flauschiversum:/httpdocs/`) gelegt werden, wahlweise automatisch aus einer GitHub Action oder aus einem Git-Hook, je nachdem, wo das Repository gerade liegt.

## What Changes

- **Deploy-Skript `scripts/deploy.sh`:** Baut die Seite für Produktion (Basis-URL `https://flauschiversum.de`, ohne Staging-Leiste) und überträgt `dist/` über SSH (nur, was sich laut Manifest geändert hat) an `flauschiversum:/httpdocs/`. Der Server (netcup-Webhosting) hat kein `rsync` – die Meldung „rsync: Kommando nicht gefunden“ kam beim Versuch von Hand –, deshalb ist die Übertragung austauschbar: `rsync`, wenn vorhanden, sonst `tar` über SSH, sonst SFTP; das Skript erkennt den Weg selbst. `flauschiversum` ist ein Host-Eintrag in der SSH-Konfiguration (inklusive Schlüssel); das Skript kennt weder Benutzer noch Schlüssel noch Port. Optionen: `--dry-run` (nur zeigen), `--no-build` (vorhandenes `dist/` nehmen). Dasselbe Skript läuft lokal, in der Action und im Hook.
- **Sicher gegen Fehlgriffe:** Das Skript bricht ab, wenn `dist/` mit `--staging` oder einer anderen Basis-URL gebaut wurde. Gelöscht werden auf dem Server nur Dateien, die ein früherer Deploy selbst dort abgelegt hat (Abgleich über die Datei `.flauschiversum-manifest`, die der Build ohnehin schreibt); fremde Dateien in `/httpdocs/` bleiben unberührt. Das erste Deployment überlagert nur.
- **Reihenfolge für ein ruhiges Umschalten:** Zuerst Medien, Stylesheet, Skripte und Suchindex, dann die HTML-Seiten, zuletzt Entfernen veralteter Dateien und das neue Manifest. Jede Datei wird erst in ein Zwischenverzeichnis hochgeladen und dann einzeln an ihren Platz verschoben, sodass Besucher nie halb geschriebene Dateien sehen. Bricht die Übertragung ab, bleibt die alte Seite lauffähig.
- **GitHub Action `production.yml`:** Läuft bei jedem Push auf `master` (und von Hand), testet, schreibt die SSH-Konfiguration aus Repository-Secrets, ruft `scripts/deploy.sh` auf und prüft die Live-Seite mit einem Smoke-Test im Produktionsmodus.
- **Git-Hook `post-receive`:** Für Umgebungen ohne GitHub Actions (ein eigener Git-Server): Bei einem Push auf `master` baut der Hook in einem temporären Verzeichnis und ruft `scripts/deploy.sh` auf. Ein Installationsskript legt ihn ab.
- **Smoke-Test für Produktion:** `scripts/smoke-test.sh` bekommt einen Produktionsmodus (kein `noindex`, keine Staging-Leiste, kanonische URL stimmt, `robots.txt` und Sitemap vorhanden).
- **Nicht Teil dieses Changes:** Einrichten des Servers (Webserver, TLS, DNS), Rollback-Automatik, Löschen des alten Inhalts in `/httpdocs/` beim ersten Mal (steht als Hinweis in der Anleitung).

## Capabilities

### New Capabilities

- `production-deployment`: Weg der gebauten Seite auf den Produktivserver per SSH: Skript, Schutz vor falschen Builds und Löschungen, Automatisierung über GitHub Action oder Git-Hook, Prüfung der Live-Seite.

### Modified Capabilities

Keine.

## Impact

- Neu: `scripts/deploy.sh`, `scripts/test-deploy.sh`, `scripts/git-hooks/post-receive`, `scripts/install-git-hook.sh`, `.github/workflows/production.yml`; geändert: `scripts/smoke-test.sh`, README (Einrichtung, Secrets, erstes Deployment).
- Voraussetzungen: SSH-Zugang mit Host-Eintrag `flauschiversum` (nicht im Repository) mit einer Shell, auf dem Server mindestens `sh`, `mv`, `find`, `chmod`, `rm` und eines von `tar` (empfohlen), `sftp` oder `rsync`, Repository-Secrets `DEPLOY_SSH_KEY`, `DEPLOY_KNOWN_HOSTS`, `DEPLOY_HOST`, `DEPLOY_USER` (optional `DEPLOY_PORT`) für die Action.
- Die Staging-Pipeline bleibt unverändert.
