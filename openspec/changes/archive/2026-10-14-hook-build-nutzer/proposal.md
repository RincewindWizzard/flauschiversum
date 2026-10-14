## Why

Der Git-Hook `post-receive` baut und deployt bisher unter dem Benutzer, der den Push entgegennimmt (typisch `git`). Dieser Benutzer hat damit Zugriff auf den SSH-Schlüssel zum Webserver, und der Build (Rust-Compiler, Abhängigkeiten, Bild-Cache, Netzzugriff) läuft in einem Konto, das eigentlich nur Repositories verwalten soll. Ein eigener, unprivilegierter Benutzer `flauschiversum` (Home `/var/share/flauschiversum`) soll den Build ausführen: mit eigener Rust-Installation, eigenem SSH-Schlüssel und eigenem Cache. Außerdem fehlt eine vollständige Anleitung, wie man diesen Server von Grund auf einrichtet, einschließlich der Installation von Rust und Cargo.

## What Changes

- **Eigener Build-Benutzer:** Benutzer `flauschiversum` mit Home `/var/share/flauschiversum`, ohne Passwort und ohne Login-Shell. Er besitzt alles, was zum Bauen und Deployen nötig ist: Rust-Toolchain (per `rustup` in seinem Home, nicht systemweit), SSH-Schlüssel und `~/.ssh/config` mit dem Host-Eintrag `flauschiversum`, Compiler-Ergebnisse und Bild-Cache unter `~/.cache/flauschiversum`.
- **Hook übergibt nur den Commit:** `post-receive` ruft bei einem Push auf `master` per `sudo` genau einen festen Befehl als `flauschiversum` auf und übergibt nur die Commit-ID. Der Befehl (`/usr/local/libexec/flauschiversum/deploy-commit`) gehört `root`, ist weder für den Hook-Benutzer noch für `flauschiversum` änderbar, prüft die Commit-ID (40 Hexziffern), kennt das Repository aus einer root-eigenen Konfiguration (nicht aus Argumenten oder der Umgebung), exportiert den Stand in ein temporäres Verzeichnis, baut und deployt dort mit `flauschiversum deploy` und räumt auf.
- **Eng begrenzte Rechte:** Eine Regel in `/etc/sudoers.d/` erlaubt dem Hook-Benutzer nur diesen einen Befehl als `flauschiversum`. Der Build-Benutzer darf das Repository lesen, aber nicht ändern; der Hook-Benutzer kommt nicht an den SSH-Schlüssel.
- **Gemeinsame Sperre:** Gleichzeitige Pushes bauen nacheinander (eine Sperre im Home des Build-Benutzers), statt sich in getrennten temporären Verzeichnissen zu überholen.
- **Schritt-für-Schritt-Anleitung in der README:** Benutzer anlegen, Pakete, Rust und Cargo per `rustup`, SSH-Schlüssel und Host-Eintrag, erste manuelle Prüfung, sudoers-Regel, Hook installieren, Test-Push, Fehlersuche, Aktualisieren von Rust.
- **Nicht Teil dieses Changes:** Betrieb des Git-Servers selbst (Anlegen des Bare-Repositories, SSH-Zugang der Entwickler), Einrichten des Webhostings, Container oder Chroot.

## Capabilities

### New Capabilities

- `build-user`: Der Build läuft im Hook unter dem eigenen Benutzer `flauschiversum` mit festem, root-eigenem Einstiegsbefehl, getrennten Rechten und dokumentierter Einrichtung.

### Modified Capabilities

Keine. (Die Anforderung „Deployment per Git-Hook“ der Spec `production-deployment` aus dem Change `prod-deployment-ssh` bekommt beim Archivieren beider Changes einen Verweis auf `build-user`.)

## Impact

- Neu: `scripts/server/deploy-commit`, `scripts/server/sudoers`, `scripts/server/deploy.conf.example`; geändert: `scripts/git-hooks/post-receive` (delegiert per `sudo`), `scripts/install-git-hook.sh` (Benutzer als Option), `scripts/test-git-hook.sh`, README (Server-Einrichtung).
- Voraussetzungen auf dem Server: Linux mit `sudo`, `git`, `tar`, `curl`, C-Toolchain (`build-essential`), `openssh-client`, `flock` (util-linux), Root-Zugang für die einmalige Einrichtung, rund 3 GB Platz und 2 GB Arbeitsspeicher für den ersten Release-Build.
- Der Pfad `/var/share/flauschiversum` ist wie gewünscht gesetzt (er gehört nicht zum Standard-Dateisystemaufbau von Debian oder Ubuntu und muss angelegt werden).
