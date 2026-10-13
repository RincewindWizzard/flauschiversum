## Purpose

Beschreibt, wie die gebaute Seite auf den Produktivserver gelangt: der Befehl `flauschiversum deploy`, der per SSH nach `flauschiversum:/httpdocs/` überträgt, und die beiden Wege, es automatisch auszulösen (GitHub Action oder Git-Hook), mit Schutz gegen falsche Builds und gegen das Löschen fremder Dateien.

## ADDED Requirements

### Requirement: Deploy-Befehl
Das Werkzeug SHALL einen Unterbefehl `flauschiversum deploy` bieten, der die Seite für Produktion baut (Basis-URL `https://flauschiversum.de`, ohne Staging-Modus) und `dist/` über SSH nach `flauschiversum:/httpdocs/` überträgt, ohne dass der Server `rsync` haben muss. `flauschiversum` ist ein Host-Eintrag der SSH-Konfiguration des Aufrufers einschließlich Schlüssel; das Werkzeug enthält und verlangt weder Benutzer noch Schlüssel noch Passwort. Mit `--dry-run` zeigt es, was übertragen und gelöscht würde, und ändert nichts; mit `--no-build` überträgt es das vorhandene Ausgabeverzeichnis. Ziel (`--target`, `DEPLOY_TARGET`) und Weg (`--method`) sind änderbar, ein Ziel ohne `host:` ist ein lokaler Pfad. Die Logik liegt im Werkzeug; Build-Pipeline und Hook enthalten keine eigene Übertragungslogik in Shell- oder Python-Skripten.

#### Scenario: Deployment von Hand
- **WHEN** `flauschiversum deploy` auf einem Rechner mit SSH-Eintrag `flauschiversum` ausgeführt wird
- **THEN** baut es die Seite, überträgt sie nach `/httpdocs/` und meldet Anzahl und Größe der übertragenen und gelöschten Dateien

#### Scenario: Trockenlauf
- **WHEN** `flauschiversum deploy --dry-run` ausgeführt wird
- **THEN** nennt es Weg, zu übertragende und zu löschende Dateien, und auf dem Server ändert sich nichts

#### Scenario: Keine Zugangsdaten im Repository
- **WHEN** das Repository nach Schlüsseln, Passwörtern und Benutzernamen des Servers durchsucht wird
- **THEN** findet sich nichts davon; nur der Hostalias `flauschiversum` kommt vor

#### Scenario: Staging verboten
- **WHEN** `flauschiversum deploy --staging x` aufgerufen wird
- **THEN** bricht der Befehl ab und überträgt nichts

### Requirement: Nur Produktionsbuilds und lauffähige Ziele
Der Befehl SHALL vor der Übertragung abbrechen, wenn `dist/` nicht der Produktionsbuild ist (Staging-Leiste oder `noindex` auf der Startseite, kanonische URL nicht unter der Basis-URL, Manifest oder Startseite fehlen), wenn das Ziel nicht ohne Passwortabfrage per SSH erreichbar ist oder wenn auf dem Server kein Übertragungsweg (`tar`, SFTP oder `rsync`) nutzbar ist. Es nennt jeweils den Grund.

#### Scenario: Staging-Build
- **WHEN** `dist/` mit `--staging` gebaut wurde und `flauschiversum deploy --no-build` aufgerufen wird
- **THEN** bricht der Befehl mit der Meldung „Staging-Build“ ab und überträgt nichts

#### Scenario: Server nicht erreichbar
- **WHEN** der SSH-Eintrag `flauschiversum` fehlt oder der Schlüssel nicht passt
- **THEN** bricht der Befehl vor der Übertragung mit einer verständlichen Meldung ab

### Requirement: Fremde Dateien bleiben unberührt
Der Befehl SHALL auf dem Server nur Dateien löschen, die ein früheres Deployment selbst abgelegt hat. Grundlage ist das Manifest `.flauschiversum-manifest` im Zielverzeichnis: Gelöscht werden Pfade, die im Manifest des Servers stehen und im neuen fehlen, danach leere Verzeichnisse darunter. Beim ersten Deployment (kein Manifest auf dem Server) wird nichts gelöscht. Pfade der Löschliste mit `..` oder absolutem Anteil werden abgewiesen.

#### Scenario: Entfernter Beitrag
- **WHEN** ein Beitrag aus der Seite entfällt und neu deployed wird
- **THEN** verschwinden seine Dateien auf dem Server, und eine Datei `/httpdocs/google123.html`, die nicht von der Seite stammt, bleibt

#### Scenario: Erstes Deployment
- **WHEN** im Zielverzeichnis kein Manifest liegt
- **THEN** werden alle Dateien der Seite hochgeladen, nichts gelöscht, und der Befehl nennt die Zahl der fremden Einträge im Ziel

### Requirement: Übertragung nur des Geänderten in fester Reihenfolge
Der Befehl SHALL nur Dateien übertragen, deren Inhalts-Hash vom Manifest des Servers abweicht oder dort fehlt, unabhängig von Dateizeiten. Zuerst werden Medien, Stylesheet, Skripte und Suchindex an ihren Platz gebracht, danach HTML-Dateien, Feed und Sitemap, danach werden veraltete Dateien gelöscht, zuletzt wird das neue Manifest geschrieben. Dateien erhalten die Rechte 644 und Verzeichnisse 755, unabhängig vom Übertragungsweg und der umask des Absenders.

#### Scenario: Nur Text geändert
- **WHEN** nach einem Deployment ein Beitragstext geändert wird und neu deployed wird
- **THEN** werden nur die betroffenen HTML-Dateien und Listen übertragen, nicht die Bilder

#### Scenario: Abbruch mitten in der Übertragung
- **WHEN** die Verbindung während der Übertragung abbricht
- **THEN** bleibt das Manifest auf dem Server unverändert, die Seite bleibt abrufbar, und der nächste Lauf überträgt den Rest

#### Scenario: Neue Seite braucht neue Bilder
- **WHEN** ein neuer Beitrag mit Bildern deployed wird
- **THEN** liegen die Bilder auf dem Server, bevor die HTML-Seite des Beitrags dort erscheint

### Requirement: Übertragung ohne rsync
Der Befehl SHALL die Dateien über den ersten nutzbaren Weg übertragen: `tar` über SSH (das Archiv erzeugt das Werkzeug selbst), sonst SFTP. Der Weg lässt sich über `DEPLOY_METHOD` festlegen und wird im Trockenlauf genannt. Beide Wege laden zuerst in ein Zwischenverzeichnis im Zielverzeichnis; ein gemeinsamer Einbauschritt (ein vom Werkzeug erzeugtes gerades Skript aus `mkdir`, `chmod`, `mv`, `rm`, `rmdir`) setzt die Rechte und verschiebt die Dateien einzeln an ihren Platz, sodass keine halb geschriebene Datei abrufbar ist. Das Zwischenverzeichnis wird auch bei Fehlern entfernt.

#### Scenario: Server ohne rsync
- **WHEN** auf dem Server `rsync` fehlt (netcup-Webhosting) und `tar` vorhanden ist
- **THEN** überträgt der Befehl per `tar` über SSH und meldet den gewählten Weg

#### Scenario: Kein tar
- **WHEN** auf dem Server `tar` fehlt, aber SFTP geht
- **THEN** überträgt der Befehl per SFTP

#### Scenario: Kein Weg
- **WHEN** keiner der drei Wege nutzbar ist
- **THEN** bricht der Befehl vor dem Hochladen mit einer Meldung ab, die die geprüften Werkzeuge nennt

#### Scenario: Aufräumen nach Abbruch
- **WHEN** die Übertragung abbricht
- **THEN** bleibt kein Zwischenverzeichnis im Ziel zurück

### Requirement: Keine parallelen Deployments
Der Befehl SHALL pro Arbeitsverzeichnis höchstens einmal gleichzeitig laufen; ein zweiter Aufruf bricht mit einer Meldung ab.

#### Scenario: Zweiter Aufruf
- **WHEN** während eines laufenden Deployments `flauschiversum deploy` erneut gestartet wird
- **THEN** beginnt keine zweite Übertragung

### Requirement: Deployment per GitHub Action
Der Workflow `.github/workflows/production.yml` SHALL bei jedem Push auf `master` und auf manuellen Start die Tests ausführen, die SSH-Umgebung aus Repository-Secrets (`DEPLOY_SSH_KEY`, `DEPLOY_KNOWN_HOSTS`, `DEPLOY_HOST`, `DEPLOY_USER`, optional `DEPLOY_PORT`) herstellen und `flauschiversum deploy` ausführen. Die SSH-Konfiguration trägt den Hostalias `flauschiversum`, prüft den Server strikt gegen `DEPLOY_KNOWN_HOSTS` und nutzt nur den hinterlegten Schlüssel. Schlägt ein Test fehl, wird nicht deployed. Läufe werden nie abgebrochen, sondern nacheinander ausgeführt. Der Workflow fordert nur lesenden Zugriff auf das Repository an und läuft in der Umgebung `production`.

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
Für Umgebungen ohne GitHub Actions SHALL das Repository einen `post-receive`-Hook (`scripts/git-hooks/post-receive`) und ein Installationsskript (`scripts/install-git-hook.sh`) liefern. Bei einem Push auf `refs/heads/master` baut der Hook in einem temporären Verzeichnis aus dem gepushten Stand und ruft dort `flauschiversum deploy` auf, mit dauerhaftem Bild- und Compiler-Cache außerhalb des temporären Verzeichnisses. Pushes auf andere Branches lösen nichts aus. Der Hook meldet Erfolg oder Fehler an den Pusher und schreibt ein Protokoll.

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
Der Befehl SHALL durch automatisierte Tests (`cargo test`) ohne Server geprüft werden, mit einem lokalen Zielverzeichnis und kleinen Ersatzprogrammen für `ssh` und `sftp`: erstes Deployment, Folge-Deployment mit geänderter und entfernter Datei, fremde Datei im Ziel, die beiden Übertragungswege, Reihenfolge des Einbaus, Staging-Build und falsche Basis-URL als Abbruchgrund, gefährliche Pfade im Server-Manifest, nicht erreichbarer Server, fehlender Übertragungsweg, abgebrochene Übertragung, Trockenlauf und Sperre. Der Hook wird mit `scripts/test-git-hook.sh` gegen ein lokales Bare-Repository geprüft.

#### Scenario: Prüfung läuft ohne SSH
- **WHEN** `cargo test deploy` ausgeführt wird
- **THEN** werden alle genannten Fälle gegen temporäre Verzeichnisse geprüft, ohne Netzwerk und ohne Server
