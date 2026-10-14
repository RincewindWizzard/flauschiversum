## Context

`dist/` entsteht mit `flauschiversum build` und enthält rund 550 MB (10 146 Dateien: Bilder, Suchindex, HTML). Der Build schreibt nur Geändertes und legt `dist/.flauschiversum-manifest` an (Pfad → Hash und Länge). Die Staging-Pipeline `.github/workflows/staging.yml` läuft bei `dev` und `claude/**`; `master` ist der Standardbranch. Zielserver: SSH-Host `flauschiversum` (Eintrag samt Schlüssel in `~/.ssh/config`), Zielpfad `/httpdocs/`. Der Server ist ein netcup-Webhosting: eine SSH-Shell, aber **kein `rsync`** (`rsync: Kommando nicht gefunden`, vom Betreiber festgestellt). Welche Werkzeuge es sonst gibt (`tar`, SFTP), ist nicht geprüft; das Werkzeug erkennt es zur Laufzeit.

Ein erster Entwurf als Bash-Skript mit eingebettetem Python wurde verworfen: Er war lang, schwer lesbar und machte die Build-Pipeline unübersichtlich. Die Logik liegt jetzt im Werkzeug.

## Goals / Non-Goals

**Goals:** ein Befehl, der die fertige Seite sicher auf den Server legt; gleiche Logik in Action und Hook; keine Zugangsdaten im Repository; schnelle Folge-Deployments (nur Geändertes); Tests ohne Server.
**Non-Goals:** Serverinstallation, Blue/Green mit Verzeichniswechsel (das Zielverzeichnis ist fest vorgegeben), Rollback-Automatik (ein Rollback ist ein Deployment des alten Commits), SSH-Bibliothek im Werkzeug.

## Decisions

### 1. Unterbefehl `deploy` im Werkzeug

`flauschiversum deploy [--target host:/pfad/] [--dry-run] [--no-build] [--method auto|tar|sftp]` (Standardziel `flauschiversum:/httpdocs/`, auch über `DEPLOY_TARGET`; Basis-URL, Cache und Ausgabe wie bei `build`). Ablauf in `src/deploy.rs`:

1. Sperre über `.deploy.lock` (`File::try_lock`), damit nie zwei Deployments gleichzeitig laufen. `--staging` wird abgewiesen.
2. Ohne `--no-build`: derselbe Build wie `flauschiversum build` (der enthält die Prüfung der Inhalte), ohne Staging.
3. Prüfung von `dist/`: `index.html` und Manifest vorhanden; keine Staging-Leiste, kein `noindex` auf der Startseite, kanonische URL beginnt mit der Basis-URL, sonst Abbruch mit Grund.
4. Ziel erreichbar: `ssh -o BatchMode=yes <Host> true` (keine Passwortabfrage in CI). Übertragungsweg erkennen (Entscheidung 3).
5. Abgleich der Manifeste, Übertragen, Einbauen (Entscheidungen 2 und 3).

Das Ziel `host:pfad` wird zerlegt; ein Ziel ohne `host:` ist ein lokaler Pfad (Befehle laufen dann über `sh -c` lokal): Das ist die Testumgebung und zugleich der Weg, wenn der Build auf dem Webserver selbst läuft. SSH läuft über das `ssh`-Programm des Systems (so greift die SSH-Konfiguration samt Schlüssel; Programme sind für Tests austauschbar). Es gibt keine eigene SSH-Bibliothek.

### 2. Nur eigene Dateien löschen (Manifest statt `--delete`)

Das Werkzeug liest das Manifest des letzten Deployments vom Server (`ssh … cat <Ziel>/.flauschiversum-manifest`; fehlt es: leer), vergleicht es mit dem neuen und löscht nur Pfade, die früher geliefert wurden und jetzt fehlen, danach leere Verzeichnisse darunter. Fremde Dateien bleiben. Das erste Deployment hat kein Manifest und überlagert nur; das Werkzeug nennt vorher die Zahl der fremden Einträge im Ziel. Pfade aus dem Manifest werden geprüft (relativ, kein `..`, keine Anführungszeichen oder Zeilenumbrüche), sonst Abbruch ohne etwas zu ändern. `--dry-run` zeigt Weg, Übertragungs- und Löschliste und ändert nichts.

### 3. Übertragung ohne rsync, Einbau ohne Schleifen

**Was:** nur Pfade, deren Hash im Manifest abweicht oder fehlt (nicht nach Dateizeiten: in der Action liegt `dist/` nicht zwischen Läufen vor) **oder deren Datei laut Prüfung auf dem Server fehlt**: Das Manifest allein beweist nichts, es bleibt als Punktdatei liegen, wenn jemand das Verzeichnis mit `rm -rf *` leert. Die Prüfung ist ein gerades `sh -s`-Skript mit `[ -f … ] || echo …` je Eintrag; ihre Eingabe wird nebenher geschrieben, weil die Antwort bei Tausenden fehlenden Dateien größer als ein Pipe-Puffer ist (sonst Deadlock). `--full` überspringt den Vergleich und überträgt alles. Während der Übertragung zeigt eine Fortschrittszeile Dateien, MB, Prozent, Geschwindigkeit und Restzeit (Terminal: eine sich aktualisierende Zeile, sonst alle fünf Sekunden eine Zeile); getrennt in Medien/Skripte/Index und Seiten (`*.html`, `*.xml`).

**Transport** (Erkennung: `tar` und `gzip` per `command -v` auf dem Server, sonst `sftp -b -` mit einem Probebefehl):
- `tar`: Das Werkzeug erzeugt das Archiv selbst (Crates `tar` und `flate2`, Dateimodus 644) und schreibt es als Strom in `ssh <Host> tar -xzf - -C <Zwischenverzeichnis>`. Eine Verbindung, komprimiert, braucht lokal kein `tar`.
- `sftp`: Das Werkzeug erzeugt die Batch-Befehle (`-mkdir`, `put`, `chmod 644`) und schreibt sie an `sftp -b -`. Langsamer bei vielen Dateien, aber nur beim ersten Deployment relevant.

Beide schreiben in ein Zwischenverzeichnis `<Ziel>/.deploy-<Kennung>/` auf demselben Dateisystem; es wird auch bei Fehlern entfernt.

**Einbau:** Das Werkzeug erzeugt ein gerades Skript ohne Schleifen und Bedingungen und schickt es an `sh -s` auf dem Server: `mkdir -p` und `chmod 755` der Verzeichnisse, `mv -f` jeder Datei erst für Medien/Index, dann für Seiten, `rm -f` veralteter Dateien, `rmdir` leerer Verzeichnisse (tiefste zuerst), zuletzt `mv -f` des neuen Manifests und Entfernen des Zwischenverzeichnisses. Auf dem Server werden nur `tar`, `sh`, `mv`, `mkdir`, `chmod`, `rm`, `rmdir` und `cat` gebraucht. Dadurch verweisen neue Seiten nie auf noch nicht vorhandene Medien, Besucher sehen nie halb geschriebene Dateien (`mv` im selben Dateisystem ist je Datei atomar), und bei einem Abbruch bleibt das Manifest auf dem Server unverändert; die nächste Ausführung holt alles Fehlende nach.

*Alternativen:* (a) `rsync`: geht auf dem Server nicht. (b) `scp -r dist/`: überträgt immer alles, löscht nichts, zeigt Zwischenstände. (c) Bash-Skript mit Python: verworfen (Wartbarkeit). (d) SSH-Bibliothek (`russh`): eigene Schlüssel- und Konfigurationslogik, die `ssh` ohnehin liefert.

### 4. GitHub Action

`.github/workflows/production.yml`: Trigger `push` auf `master` und `workflow_dispatch`; `concurrency: production` ohne Abbrechen; `permissions: contents: read`; `environment: production`. Schritte: Checkout, Rust und Cache wie im Staging, `cargo test --locked`, Test des Hooks, Bild-Cache (`actions/cache`), SSH einrichten, `cargo run --release --locked -- deploy`, Smoke-Test.

SSH einrichten: aus den Secrets `DEPLOY_SSH_KEY`, `DEPLOY_KNOWN_HOSTS` (vorab geprüft, kein blindes `ssh-keyscan`), `DEPLOY_HOST`, `DEPLOY_USER`, optional `DEPLOY_PORT` entsteht `~/.ssh/config` mit `Host flauschiversum`, `IdentityFile`, `IdentitiesOnly yes`, `StrictHostKeyChecking yes`: genau die Umgebung, die auch lokal besteht. Der Schlüssel wird nie ausgegeben und nach dem Lauf entfernt. Das ist der einzige Shell-Anteil in der Pipeline.

*Hinweis:* Das Staging-Spec verlangt „keine eigenen Secrets“ für den Staging-Workflow; das gilt weiter.

### 5. Git-Hook für Umgebungen ohne GitHub Actions

`scripts/git-hooks/post-receive` (kurz): Bei einem Push auf `refs/heads/master` (änderbar über `DEPLOY_REF`) exportiert er den Stand per `git archive` in ein temporäres Verzeichnis und führt dort `cargo run --release --locked -- deploy` aus (`DEPLOY_COMMAND` ändert den Befehl), mit dauerhaftem Zustandsverzeichnis `~/.cache/flauschiversum` für Compiler-Ergebnisse und Bild-Cache. Die Ausgabe sieht der Pusher; ein Protokoll liegt im Zustandsverzeichnis; ein Fehler wird als solcher gemeldet. `scripts/install-git-hook.sh <Bare-Repository>` kopiert den Hook mit Ausführungsrecht und überschreibt keinen fremden Hook. `scripts/test-git-hook.sh` prüft ihn mit einem lokalen Bare-Repository.

*Annahme:* „Git-Hook je nach Umgebung“ heißt: ein serverseitiger Hook für ein Repository auf eigenem Git-Server. Ein lokaler `pre-push`-Hook wurde verworfen, weil er vor dem Gelingen des Pushs deployen würde.

### 6. Smoke-Test im Produktionsmodus

`scripts/smoke-test.sh <Basis-URL>` bekommt `SMOKE_MODE=production`: Statusprüfungen wie bisher, zusätzlich `/robots.txt` und `/sitemap.xml`, die Startseite enthält **kein** `noindex` und **keine** Staging-Leiste, die kanonische URL beginnt mit `SMOKE_CANONICAL` (Standard: die Basis-URL), `robots.txt` sperrt nicht alles. Der Staging-Modus bleibt der Standard. Der Smoke-Test bleibt ein kleines Shell-Skript (nur `curl`); eine Umsetzung in Rust wäre ein eigener Change.

### 7. Annahmen und offene Voraussetzungen

- Der Server hat eine Shell mit den Standardwerkzeugen (`sh`, `mv`, `chmod`, `rm`, `rmdir`) und `tar` mit `gzip` oder das SFTP-Subsystem. Fehlt beides, bricht der Befehl vor dem Hochladen mit klarer Meldung ab. Das ist am netcup-Webhosting noch nicht geprüft; `deploy --dry-run` nennt den gewählten Weg, bevor etwas hochgeladen wird.
- Der Zielpfad `/httpdocs/` ist auf dem Server erreichbar und beschreibbar; ein relativer Pfad (`flauschiversum:httpdocs/`) geht ebenso.
- Der Branch heißt `master` (Standardbranch); `dev` und `claude/**` bleiben Staging.
- Das HTTPS-Zertifikat der Produktionsdomain liegt beim Server (nicht Teil dieses Changes).

## Risks / Trade-offs

- [Falscher Build geht live (z. B. mit Staging-Leiste)] → Prüfung von `dist/` im Befehl und Smoke-Test nach dem Deployment.
- [Löschen fremder Dateien] → Löschen nur über das Manifest; `--dry-run`; Pfadprüfung; getestet.
- [Halb übertragene Seite] → Zwischenverzeichnis, feste Reihenfolge, Manifest zuletzt; die nächste Ausführung heilt; getestet (abgebrochener Transfer).
- [SSH-Schlüssel in der CI] → eigener, nur für diesen Server gültiger Schlüssel, feste `known_hosts`, `environment: production` für Schutzregeln; Schlüssel nie in Logs und nach dem Lauf entfernt.
- [Erste Übertragung dauert (550 MB)] → einmalig (lokal rund 40 s auf einer Platte, über das Netz entsprechend länger); danach geht nur, was sich laut Manifest geändert hat, auch aus der Action ohne zwischengespeichertes `dist/`.
- [Manifest auf dem Server veraltet oder gelöscht] → Die nächste Ausführung überträgt dann alles und löscht nichts; richtig, nur langsamer.
- [Webhosting hat weder `tar` noch SFTP] → Abbruch vor dem Hochladen mit Hinweis; ein weiterer Weg wäre ein eigener Change.
