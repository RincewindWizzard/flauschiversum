## Context

`scripts/git-hooks/post-receive` exportiert den gepushten Stand per `git archive` in ein temporäres Verzeichnis und führt `cargo run --release --locked -- deploy` aus, alles als der Benutzer, der den Push annimmt. Das Deployment (`flauschiversum deploy`) arbeitet über das `ssh` des Systems und den Host-Eintrag `flauschiversum`; die Sperre `.deploy.lock` liegt im Arbeitsverzeichnis, also im jeweiligen temporären Verzeichnis (wirkt dort nicht zwischen Pushes). Der Git-Server ist ein Linux-Rechner mit einem Bare-Repository; der Benutzer, der pusht und dessen Hook läuft, ist im Folgenden `git` (Annahme, steht in der Anleitung als Platzhalter). Der Build braucht Rust (Release-Build mit C-Abhängigkeiten wie `libwebp` und `blake3`, daher eine C-Toolchain), rund 1 GB Bild-Cache und mehrere Minuten beim ersten Lauf.

## Goals / Non-Goals

**Goals:** Build und Deployment unter einem eigenen, unprivilegierten Konto; der Hook-Benutzer bekommt weder dessen Rechte noch Schlüssel; kein beliebiger Befehl über `sudo`; vollständige, kopierbare Einrichtungsanleitung.
**Non-Goals:** Container oder Chroot, mehrere Projekte, Einrichten des Git-Servers, Rotation von Schlüsseln.

## Decisions

### 1. Aufbau und Rechte

| Was | Wem | Rechte |
|---|---|---|
| Benutzer `flauschiversum`, Home `/var/share/flauschiversum` | eigene Gruppe `flauschiversum`, Shell `/usr/sbin/nologin`, kein Passwort | Home `750` (Gruppe und andere ohne Zugriff auf `.ssh`, Schlüssel `600`) |
| `/usr/local/libexec/flauschiversum/deploy-commit` | `root:root` | `755`, nur `root` schreibt |
| `/etc/flauschiversum/deploy.conf` | `root:root` | `644`, enthält `REPO=/srv/git/flauschiversum.git`, optional `TARGET` und `REF` |
| `/etc/sudoers.d/flauschiversum` | `root:root` | `440`, geprüft mit `visudo -cf` |
| Bare-Repository | Benutzer `git` | Gruppe `git`, gruppenlesbar; `flauschiversum` ist Mitglied der Gruppe `git` (nur lesen) |

Der Build-Benutzer bekommt keine Login-Shell: `sudo -u flauschiversum <Befehl>` startet den Befehl direkt, die Shell im Passwort-Eintrag ist dafür unerheblich. Für einmalige Einrichtungsschritte (rustup, SSH-Test) dient `sudo -u flauschiversum -H bash -c '…'` als root.

### 2. Der feste Einstiegsbefehl `deploy-commit`

`/usr/local/libexec/flauschiversum/deploy-commit <40-stellige Commit-ID>` läuft als `flauschiversum` (per `sudo` aus dem Hook) und tut:

1. Argument prüfen: genau eine Zeichenfolge aus 40 Hexziffern, sonst Abbruch. Keine weiteren Argumente, keine Umgebungsvariablen aus dem Aufrufer (`sudo` setzt die Umgebung zurück; das Skript liest nur `/etc/flauschiversum/deploy.conf`, für Tests über `FLAUSCHI_CONF` umlenkbar).
2. Sperre: `flock` auf `~/.cache/flauschiversum/deploy.lock` um den ganzen Lauf; ein gleichzeitiger zweiter Push wartet, bis der erste fertig ist (Standard: höchstens 30 Minuten), und baut dann seinen Stand.
3. Stand exportieren: `git --git-dir="$REPO" -c safe.directory="$REPO" archive <ID> | tar -x -C <mktemp -d>` (`safe.directory`, weil das Repository einem anderen Benutzer gehört; die ID muss im Repository existieren).
4. Im Export: `PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR="$HOME/.cache/flauschiversum/target" cargo run --release --locked -- deploy --cache "$HOME/.cache/flauschiversum/images"` (Ziel aus `TARGET` der Konfiguration, sonst der Standard `flauschiversum:/httpdocs/`).
5. Protokoll nach `~/.cache/flauschiversum/deploy.log` (zusätzlich auf der Standardausgabe, die der Hook an den Pusher durchreicht), temporäres Verzeichnis wird immer entfernt, Exit-Code spiegelt das Ergebnis.

Der Befehl nimmt weder Repository-Pfad noch Befehlszeile aus Argumenten: Selbst wenn der Hook-Benutzer kompromittiert ist, kann er nur „deploye Commit X aus dem festgelegten Repository“ auslösen, und X muss dort existieren.

### 3. `sudo`-Regel

```
git ALL=(flauschiversum) NOPASSWD: /usr/local/libexec/flauschiversum/deploy-commit [0-9a-f]*
```

Der Platzhalter ist in sudoers kein regulärer Ausdruck (er passt auch auf anderes); die eigentliche Prüfung macht das Skript. Dazu `Defaults:git !requiretty`, falls `requiretty` aktiv ist. `NOEXEC` ist hier nicht möglich (das Skript startet Programme).

### 4. Hook

`scripts/git-hooks/post-receive` behält Struktur und Ausgabe (`==> Deployment …`, Meldung an den Pusher), ruft aber statt des Builds `${DEPLOY_SUDO:-sudo} -n -u "${DEPLOY_USER:-flauschiversum}" "${DEPLOY_COMMIT_COMMAND:-/usr/local/libexec/flauschiversum/deploy-commit}" "$new"` auf. `-n` verhindert eine Passwortabfrage (der Push würde sonst hängen); ein Fehlschlag von `sudo` wird mit einem Hinweis auf die Einrichtung gemeldet. Die bisherigen Variablen für den Fall ohne eigenen Build-Benutzer (`DEPLOY_COMMAND`, `DEPLOY_STATE_DIR`) entfallen; der Test nutzt `DEPLOY_SUDO` und `DEPLOY_COMMIT_COMMAND` als Ersatzprogramme. `scripts/install-git-hook.sh` bleibt gleich.

### 5. Einrichtung (Inhalt der README, Reihenfolge)

Als `root` auf dem Server (Debian/Ubuntu; Platzhalter `git` für den Benutzer des Git-Servers und `/srv/git/flauschiversum.git` für das Bare-Repository):

1. Pakete: `apt install git curl ca-certificates build-essential pkg-config openssh-client util-linux sudo` (`build-essential` für den C-Compiler und Linker, `util-linux` für `flock`).
2. Benutzer: `mkdir -p /var/share && useradd --system --create-home --home-dir /var/share/flauschiversum --shell /usr/sbin/nologin --user-group flauschiversum && chmod 750 /var/share/flauschiversum`; Lesezugriff auf das Repository: `usermod -aG git flauschiversum` und das Repository gruppenlesbar machen (`chmod -R g+rX /srv/git/flauschiversum.git`, `git config core.sharedRepository group` im Repository).
3. Rust und Cargo (als `flauschiversum`, nur im Home): `sudo -u flauschiversum -H bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable'`; Kontrolle `sudo -u flauschiversum -H bash -c '~/.cargo/bin/cargo --version'`. Aktualisieren später mit `rustup update`.
4. SSH-Zugang zum Webserver: `sudo -u flauschiversum -H bash -c 'install -d -m 700 ~/.ssh && ssh-keygen -t ed25519 -N "" -C flauschiversum-deploy -f ~/.ssh/deploy_key'`, den öffentlichen Schlüssel (`~/.ssh/deploy_key.pub`) beim Hoster eintragen; `~/.ssh/config` mit `Host flauschiversum`, `HostName`, `User`, `IdentityFile ~/.ssh/deploy_key`, `IdentitiesOnly yes`; Server-Schlüssel mit `ssh-keyscan` holen, **Fingerabdruck mit dem des Hosters vergleichen** und in `~/.ssh/known_hosts` ablegen; Kontrolle: `sudo -u flauschiversum -H ssh flauschiversum true`.
5. Einstiegsbefehl und Konfiguration: `install -d -o root -g root -m 755 /usr/local/libexec/flauschiversum /etc/flauschiversum`; `install -o root -g root -m 755 scripts/server/deploy-commit /usr/local/libexec/flauschiversum/`; `/etc/flauschiversum/deploy.conf` aus `scripts/server/deploy.conf.example` anlegen (`REPO=…`).
6. sudoers: `scripts/server/sudoers` nach `/etc/sudoers.d/flauschiversum` (Benutzername des Git-Servers anpassen), `chmod 440`, prüfen mit `visudo -cf /etc/sudoers.d/flauschiversum`.
7. Hook: `scripts/install-git-hook.sh /srv/git/flauschiversum.git`.
8. Erster Lauf von Hand: `sudo -u git sudo -n -u flauschiversum /usr/local/libexec/flauschiversum/deploy-commit <Commit-ID>` (dauert wegen des ersten Release-Builds mehrere Minuten), bevor der erste Push den Hook auslöst. Vorher `flauschiversum deploy --dry-run` als `flauschiversum` im Export prüfen.
9. Fehlersuche: Protokoll `/var/share/flauschiversum/.cache/flauschiversum/deploy.log`; häufige Ursachen (sudo verlangt Passwort, `safe.directory`, fehlendes `cc`, SSH-Schlüssel nicht eingetragen oder falscher Fingerabdruck).

### 6. Tests

Ohne Root und ohne echten Benutzerwechsel: `scripts/test-git-hook.sh` ersetzt `sudo` durch ein Stück Shell, das nur den Befehl ausführt (`DEPLOY_SUDO`), und prüft: Push auf `master` ruft den Einstiegsbefehl mit genau der Commit-ID auf, Push auf `dev` nicht, Fehler (auch fehlschlagendes `sudo`) erreichen den Pusher mit Hinweis. `deploy-commit` wird mit `FLAUSCHI_CONF` und einem Ersatz für `cargo` getestet: falsche IDs (zu kurz, Großbuchstaben, Zusätze, leer) werden abgewiesen, unbekannte Commits ebenso, ein gültiger Commit wird exportiert und deployt, das temporäre Verzeichnis ist danach weg, zwei gleichzeitige Aufrufe laufen nacheinander (Sperre), das Protokoll entsteht.

*Alternativen:* (a) `su`/`sudo -i` mit freier Befehlszeile: erlaubt dem Hook-Benutzer beliebige Befehle als Build-Benutzer. (b) Gruppenrechte statt `sudo` (Hook und Build im selben Konto): löst die Trennung nicht. (c) Container/Chroot: mehr Aufwand, hier nicht nötig. (d) systemd-Dienst, der auf einen Auslöser wartet: sauberer, aber deutlich mehr Einrichtung.

## Risks / Trade-offs

- [`sudoers`-Regel zu weit] → fester Pfad des Befehls (root-eigen, nicht überschreibbar), Argument wird im Skript streng geprüft, kein Pfad und keine Variablen aus dem Aufrufer.
- [Hook-Benutzer liest den Schlüssel] → Home `750` mit Gruppe `flauschiversum`, Schlüssel `600`; der Hook-Benutzer ist nicht in der Gruppe.
- [Der Build-Benutzer manipuliert das Repository] → nur lesende Gruppenrechte; schreibende Rechte hat nur `git`.
- [Der Build führt Code aus dem Repository aus (Rust `build.rs`, Abhängigkeiten)] → läuft unprivilegiert, ohne sudo-Rechte und ohne Schreibzugriff außerhalb seines Homes; wer pushen darf, kann also Code im Konto `flauschiversum` und damit das Deployment ausführen (gewollt, entspricht der Rolle des Pushers).
- [Erster Build dauert und braucht Speicher] → in der Anleitung genannt (≈ 2 GB RAM, mehrere Minuten); Folge-Builds nutzen den Cache.
- [Pfad `/var/share` existiert nicht] → die Anleitung legt ihn an; `/var/lib/flauschiversum` wäre der übliche Ort, der Pfad ist aber wie gewünscht gesetzt.
- [Zwei Pushes kurz hintereinander] → Sperre; der zweite wartet, baut dann seinen Stand (der erste Stand wird nie von einem älteren überschrieben, weil die Reihenfolge der Sperre die Reihenfolge der Pushes spiegelt – bei Gleichzeitigkeit nicht garantiert; `deploy` deployt immer den Stand des eigenen Commits, daher kann bei Überholung kurz ein älterer Stand live sein, der nächste Push korrigiert das).
