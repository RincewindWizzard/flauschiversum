## Purpose

Beschreibt, wie die gebaute Seite auf den Produktivserver gelangt: ein Skript, das per SSH nach `flauschiversum:/httpdocs/` überträgt, und die beiden Wege, es automatisch auszulösen (GitHub Action oder Git-Hook), mit Schutz gegen falsche Builds und gegen das Löschen fremder Dateien.

## ADDED Requirements

### Requirement: Deploy-Skript
Das Repository SHALL ein Skript `scripts/deploy.sh` enthalten, das die Seite für Produktion baut (Basis-URL `https://flauschiversum.de`, ohne Staging-Modus) und `dist/` per `rsync` über SSH nach `flauschiversum:/httpdocs/` überträgt. `flauschiversum` ist ein Host-Eintrag der SSH-Konfiguration des Aufrufers einschließlich Schlüssel; das Skript enthält und verlangt weder Benutzer noch Schlüssel noch Passwort. Mit `--dry-run` zeigt es, was übertragen und gelöscht würde, und ändert nichts; mit `--no-build` überträgt es das vorhandene `dist/`. Ziel und Basis-URL sind über `DEPLOY_TARGET` und `FLAUSCHI_BASE_URL` änderbar, ein Ziel ohne `host:` ist ein lokaler Pfad.

#### Scenario: Deployment von Hand
- **WHEN** `scripts/deploy.sh` auf einem Rechner mit SSH-Eintrag `flauschiversum` ausgeführt wird
- **THEN** baut es die Seite, überträgt sie nach `/httpdocs/` und meldet Anzahl und Größe der übertragenen und gelöschten Dateien

#### Scenario: Trockenlauf
- **WHEN** `scripts/deploy.sh --dry-run` ausgeführt wird
- **THEN** listet es die zu übertragenden und die zu löschenden Dateien, und auf dem Server ändert sich nichts

#### Scenario: Keine Zugangsdaten im Repository
- **WHEN** das Repository nach Schlüsseln, Passwörtern und Benutzernamen des Servers durchsucht wird
- **THEN** findet sich nichts davon; nur der Hostalias `flauschiversum` kommt vor

### Requirement: Nur Produktionsbuilds und lauffähige Ziele
Das Skript SHALL vor der Übertragung abbrechen, wenn `dist/` nicht der Produktionsbuild ist (Staging-Leiste oder `noindex` auf der Startseite, kanonische URL nicht unter der Basis-URL, Manifest oder Startseite fehlen), wenn das Ziel nicht ohne Passwortabfrage per SSH erreichbar ist oder wenn auf dem Server `rsync` fehlt. Es nennt jeweils den Grund.

#### Scenario: Staging-Build
- **WHEN** `dist/` mit `--staging` gebaut wurde und `scripts/deploy.sh --no-build` aufgerufen wird
- **THEN** bricht das Skript mit der Meldung „Staging-Build“ ab und überträgt nichts

#### Scenario: Server nicht erreichbar
- **WHEN** der SSH-Eintrag `flauschiversum` fehlt oder der Schlüssel nicht passt
- **THEN** bricht das Skript vor der Übertragung mit einer verständlichen Meldung ab

### Requirement: Fremde Dateien bleiben unberührt
Das Skript SHALL auf dem Server nur Dateien löschen, die ein früheres Deployment selbst abgelegt hat. Grundlage ist das Manifest `.flauschiversum-manifest` im Zielverzeichnis: Gelöscht werden Pfade, die im Manifest des Servers stehen und im neuen fehlen, danach leere Verzeichnisse darunter. Beim ersten Deployment (kein Manifest auf dem Server) wird nichts gelöscht. Pfade der Löschliste mit `..` oder absolutem Anteil werden abgewiesen.

#### Scenario: Entfernter Beitrag
- **WHEN** ein Beitrag aus der Seite entfällt und neu deployed wird
- **THEN** verschwinden seine Dateien auf dem Server, und eine Datei `/httpdocs/google123.html`, die nicht von der Seite stammt, bleibt

#### Scenario: Erstes Deployment
- **WHEN** im Zielverzeichnis kein Manifest liegt
- **THEN** werden alle Dateien der Seite hochgeladen, nichts gelöscht, und das Skript nennt die Zahl der fremden Einträge im Ziel

### Requirement: Übertragung nur des Geänderten in fester Reihenfolge
Das Skript SHALL nur Dateien übertragen, deren Inhalts-Hash vom Manifest des Servers abweicht oder dort fehlt, unabhängig von Dateizeiten. Zuerst werden Medien, Stylesheet, Skripte und Suchindex übertragen, danach HTML-Dateien, Feed und Sitemap, danach werden veraltete Dateien gelöscht, zuletzt wird das neue Manifest geschrieben. Dateien erhalten die Rechte 644 und Verzeichnisse 755.

#### Scenario: Nur Text geändert
- **WHEN** nach einem Deployment ein Beitragstext geändert wird und neu deployed wird
- **THEN** werden nur die betroffenen HTML-Dateien und Listen übertragen, nicht die Bilder

#### Scenario: Abbruch mitten in der Übertragung
- **WHEN** die Verbindung während der Übertragung abbricht
- **THEN** bleibt das Manifest auf dem Server unverändert, die Seite bleibt abrufbar, und der nächste Lauf überträgt den Rest

#### Scenario: Neue Seite braucht neue Bilder
- **WHEN** ein neuer Beitrag mit Bildern deployed wird
- **THEN** liegen die Bilder auf dem Server, bevor die HTML-Seite des Beitrags dort erscheint

### Requirement: Keine parallelen Deployments
Das Skript SHALL pro Arbeitsverzeichnis höchstens einmal gleichzeitig laufen; ein zweiter Aufruf wartet oder bricht mit einer Meldung ab.

#### Scenario: Zweiter Aufruf
- **WHEN** während eines laufenden Deployments `scripts/deploy.sh` erneut gestartet wird
- **THEN** beginnt keine zweite Übertragung

### Requirement: Deployment per GitHub Action
Der Workflow `.github/workflows/production.yml` SHALL bei jedem Push auf `master` und auf manuellen Start die Tests ausführen, die SSH-Umgebung aus Repository-Secrets (`DEPLOY_SSH_KEY`, `DEPLOY_KNOWN_HOSTS`, `DEPLOY_HOST`, `DEPLOY_USER`, optional `DEPLOY_PORT`) herstellen und `scripts/deploy.sh` ausführen. Die SSH-Konfiguration trägt den Hostalias `flauschiversum`, prüft den Server strikt gegen `DEPLOY_KNOWN_HOSTS` und nutzt nur den hinterlegten Schlüssel. Schlägt ein Test fehl, wird nicht deployed. Läufe werden nie abgebrochen, sondern nacheinander ausgeführt. Der Workflow fordert nur lesenden Zugriff auf das Repository an und läuft in der Umgebung `production`.

#### Scenario: Merge nach master
- **WHEN** ein Pull Request nach `master` gemergt wird
- **THEN** startet der Workflow, testet, deployed nach `flauschiversum:/httpdocs/` und prüft die Live-Seite

#### Scenario: Fehlschlagender Test
- **WHEN** ein Test fehlschlägt
- **THEN** wird nichts auf den Server übertragen

#### Scenario: Andere Branches
- **WHEN** auf `dev` oder `claude/**` gepusht wird
- **THEN** startet der Produktions-Workflow nicht

#### Scenario: Schlüssel bleibt geheim
- **WHEN** der Workflow ausgeführt wird
- **THEN** erscheint der private Schlüssel in keinem Log und nicht in einem Artefakt, und er liegt nach dem Lauf nicht mehr auf dem Runner

### Requirement: Deployment per Git-Hook
Für Umgebungen ohne GitHub Actions SHALL das Repository einen `post-receive`-Hook (`scripts/git-hooks/post-receive`) und ein Installationsskript (`scripts/install-git-hook.sh`) liefern. Bei einem Push auf `refs/heads/master` baut der Hook in einem temporären Verzeichnis aus dem gepushten Stand und ruft `scripts/deploy.sh` auf, mit dauerhaftem Bild- und Compiler-Cache außerhalb des temporären Verzeichnisses. Pushes auf andere Branches lösen nichts aus. Der Hook meldet Erfolg oder Fehler an den Pusher und schreibt ein Protokoll.

#### Scenario: Push auf master
- **WHEN** auf `master` des Git-Servers gepusht wird
- **THEN** wird die Seite gebaut und nach `/httpdocs/` übertragen, und der Pusher sieht das Ergebnis

#### Scenario: Push auf einen anderen Branch
- **WHEN** auf `dev` gepusht wird
- **THEN** passiert nichts

#### Scenario: Fehler im Build
- **WHEN** der Build mit einem Fehler im Inhalt abbricht
- **THEN** wird nichts übertragen, und der Pusher erhält die Fehlermeldung

### Requirement: Prüfung der Live-Seite
Nach einem Deployment SHALL der Smoke-Test (`scripts/smoke-test.sh` mit `SMOKE_MODE=production`) prüfen, dass Startseite, Suchseite, Suchindex, Stylesheet, ein Beitrag, `robots.txt` und `sitemap.xml` mit 200 antworten, die Startseite weder `noindex` noch eine Staging-Leiste enthält und ihre kanonische URL unter `https://flauschiversum.de` liegt. Im Standardmodus (Staging) verhält sich der Smoke-Test wie bisher.

#### Scenario: Live-Seite in Ordnung
- **WHEN** das Deployment abgeschlossen ist
- **THEN** antworten alle geprüften URLs mit 200, und der Lauf bleibt grün

#### Scenario: Staging-Leiste auf Produktion
- **WHEN** versehentlich ein Staging-Build live ist
- **THEN** schlägt der Smoke-Test fehl und nennt „Staging-Leiste auf der Produktionsseite“

### Requirement: Geprüfter Ablauf ohne Server
Das Deploy-Skript SHALL in einer automatisierten Prüfung (`scripts/test-deploy.sh`) gegen ein lokales Zielverzeichnis getestet werden: erstes Deployment, Folge-Deployment mit geänderter und entfernter Datei, fremde Datei im Ziel, Staging-Build als Abbruchgrund und Trockenlauf.

#### Scenario: Prüfung läuft ohne SSH
- **WHEN** `scripts/test-deploy.sh` ausgeführt wird
- **THEN** werden alle genannten Fälle gegen ein temporäres Verzeichnis geprüft, ohne Netzwerk und ohne Server
