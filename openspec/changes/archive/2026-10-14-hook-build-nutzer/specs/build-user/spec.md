## Purpose

Beschreibt, wie der Git-Hook die Seite unter einem eigenen, unprivilegierten Benutzer baut und deployt: Aufbau und Rechte des Benutzers `flauschiversum`, der feste Einstiegsbefehl mit `sudo`, die Sperre gegen gleichzeitige Läufe und die Einrichtung des Servers.

## ADDED Requirements

### Requirement: Eigener Benutzer für Build und Deployment
Der Server SHALL für Build und Deployment einen eigenen Benutzer `flauschiversum` mit Home `/var/share/flauschiversum` haben, ohne Passwort und ohne Login-Shell. In seinem Home liegen die Rust-Toolchain (per `rustup`, nicht systemweit), der SSH-Schlüssel und `~/.ssh/config` mit dem Host-Eintrag `flauschiversum` sowie Compiler-Ergebnisse und Bild-Cache (`~/.cache/flauschiversum`). Das Home ist für andere Benutzer nicht lesbar, Schlüssel haben die Rechte 600.

#### Scenario: Benutzer eingerichtet
- **WHEN** die Einrichtung abgeschlossen ist
- **THEN** existiert der Benutzer `flauschiversum` mit Home `/var/share/flauschiversum`, ohne Passwort und mit der Shell `/usr/sbin/nologin`, und `cargo --version` gelingt für ihn

#### Scenario: Schlüssel geschützt
- **WHEN** der Hook-Benutzer (`git`) versucht, `/var/share/flauschiversum/.ssh/deploy_key` zu lesen
- **THEN** wird der Zugriff verweigert

#### Scenario: Zugang zum Webserver
- **WHEN** als `flauschiversum` `ssh flauschiversum true` ausgeführt wird
- **THEN** gelingt die Anmeldung ohne Passwort und ohne Rückfrage zum Server-Schlüssel

### Requirement: Hook übergibt nur die Commit-ID
Der Hook `post-receive` SHALL bei einem Push auf den Hauptbranch des Repositorys (den Branch, auf den `HEAD` des Bare-Repositorys zeigt, z. B. `main`; mit `DEPLOY_REF` festlegbar) nur `sudo -n -u flauschiversum /usr/local/libexec/flauschiversum/deploy-commit <Commit-ID>` aufrufen und selbst nichts bauen oder deployen. Die Ausgabe des Befehls erreicht den Pusher. Pushes auf andere Branches und gelöschte Branches lösen nichts aus. Scheitert `sudo` (fehlende Regel, Passwortabfrage), meldet der Hook den Fehler mit Hinweis auf die Einrichtung und blockiert nicht.

#### Scenario: Push auf den Hauptbranch
- **WHEN** auf den Hauptbranch gepusht wird
- **THEN** ruft der Hook den Einstiegsbefehl als `flauschiversum` mit der gepushten Commit-ID auf, und der Pusher sieht dessen Ausgabe

#### Scenario: Push auf einen anderen Branch
- **WHEN** auf `dev` gepusht wird
- **THEN** passiert nichts

#### Scenario: sudo nicht eingerichtet
- **WHEN** die sudoers-Regel fehlt
- **THEN** meldet der Hook „Deployment FEHLGESCHLAGEN“ mit Hinweis auf die Einrichtung, der Push selbst bleibt angenommen

### Requirement: Fester, root-eigener Einstiegsbefehl
Der Befehl `/usr/local/libexec/flauschiversum/deploy-commit` SHALL `root` gehören und weder vom Hook-Benutzer noch von `flauschiversum` änderbar sein. Er nimmt genau ein Argument, eine Commit-ID aus 40 Hexziffern (Kleinbuchstaben), und weist alles andere ab. Das Repository und weitere Einstellungen liest er nur aus `/etc/flauschiversum/deploy.conf` (root-eigen), nie aus Argumenten oder der Umgebung des Aufrufers. Er exportiert den Stand dieses Commits aus dem Repository in einen festen Arbeitsbaum unter `~/.cache/flauschiversum/src` (nur geänderte Dateien werden angefasst, damit Cargo unveränderten Code nicht neu übersetzt), baut und deployt dort mit `flauschiversum deploy` (Compiler-Ergebnisse und Bild-Cache dauerhaft unter `~/.cache/flauschiversum`) und schreibt ein Protokoll.

#### Scenario: Gültiger Commit
- **WHEN** der Befehl mit der ID eines vorhandenen Commits aufgerufen wird
- **THEN** wird dessen Stand gebaut und deployt, und der Arbeitsbaum steht auf diesem Commit

#### Scenario: Ungültige Argumente
- **WHEN** der Befehl ohne Argument, mit zu kurzer oder zu langer ID, mit Großbuchstaben, mit Zusätzen (`abc…; rm -rf /`) oder mit mehr als einem Argument aufgerufen wird
- **THEN** bricht er ab, ohne etwas zu exportieren oder auszuführen

#### Scenario: Unbekannter Commit
- **WHEN** die ID nicht im Repository existiert
- **THEN** bricht der Befehl mit einer Fehlermeldung ab und deployt nichts

#### Scenario: Protokoll
- **WHEN** ein Lauf endet, erfolgreich oder nicht
- **THEN** steht seine Ausgabe in `~/.cache/flauschiversum/deploy.log`

### Requirement: Eng begrenzte Rechte
Die sudoers-Regel SHALL dem Hook-Benutzer nur diesen einen Befehl als `flauschiversum` erlauben, ohne Passwort. Der Build-Benutzer SHALL das Repository nur lesen können. Er hat keine sudo-Rechte, und seine Dateien (Schlüssel, Toolchain, Cache) liegen ausschließlich in seinem Home.

#### Scenario: Anderer Befehl
- **WHEN** der Hook-Benutzer `sudo -n -u flauschiversum /bin/sh` ausführt
- **THEN** wird das von sudo abgelehnt

#### Scenario: Repository schreiben
- **WHEN** der Build-Benutzer versucht, im Repository eine Datei anzulegen
- **THEN** wird der Zugriff verweigert

### Requirement: Keine gleichzeitigen Läufe
Der Einstiegsbefehl SHALL gleichzeitige Aufrufe durch eine Sperre im Home des Build-Benutzers (`~/.cache/flauschiversum/deploy.lock`) nacheinander ausführen, nicht parallel.

#### Scenario: Zwei Pushes kurz hintereinander
- **WHEN** zwei Pushes fast gleichzeitig eintreffen
- **THEN** baut und deployt der zweite Lauf erst, wenn der erste beendet ist

### Requirement: Dokumentierte Einrichtung
Die README SHALL die vollständige Einrichtung Schritt für Schritt beschreiben: Pakete, Benutzer mit Home `/var/share/flauschiversum`, Installation von Rust und Cargo per `rustup`, SSH-Schlüssel und Host-Eintrag mit Prüfung des Server-Fingerabdrucks, Einstiegsbefehl und Konfiguration, sudoers-Regel (mit `visudo -cf` geprüft), Hook, erster Lauf von Hand, Fehlersuche und Aktualisieren von Rust. Jeder Befehl ist kopierbar und nennt, als welcher Benutzer er läuft.

#### Scenario: Einrichtung von Grund auf
- **WHEN** ein Server ohne Rust und ohne den Benutzer nach der README eingerichtet wird
- **THEN** gelingt am Ende ein Push auf den Hauptbranch, der die Seite baut und nach `/httpdocs/` deployt

### Requirement: Geprüfter Ablauf ohne Root
Hook und Einstiegsbefehl SHALL ohne Root und ohne echten Benutzerwechsel automatisiert geprüft werden (Ersatzprogramme für `sudo` und `cargo`, lokales Repository): Branch-Auswahl, Übergabe der Commit-ID, Abweisung ungültiger Argumente, unbekannter Commit, Aufräumen, Protokoll, Sperre und Fehlermeldung bei fehlendem sudo.

#### Scenario: Tests laufen
- **WHEN** `scripts/test-git-hook.sh` ausgeführt wird
- **THEN** werden alle genannten Fälle geprüft, ohne dass ein Benutzer angelegt oder `sudo` benötigt wird
