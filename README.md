# Flauschiversum

Quellen und Generator für [flauschiversum.de](https://flauschiversum.de/) – „Mehr als nur eine Masche“.

Der Generator ist ein einzelnes Rust-Programm. Er baut aus den Inhalten unter `content/` eine statische Webseite,
die jeder Webserver ausliefern kann. Python, Node.js oder andere Werkzeuge werden nicht benötigt.

## Schnellstart

Voraussetzung: [Rust](https://rustup.rs/) (stable) und ein C-Compiler (für die WebP-Kodierung).

```sh
cargo run --release -- build     # Seite nach dist/ bauen
cargo run --release -- check     # Inhalte prüfen, ohne etwas zu schreiben
cargo run --release -- serve     # Vorschau unter http://localhost:8080/ mit Live-Reload
cargo run --release -- clean     # Cache leeren (mit --dist auch dist/)
```

Optionen (für alle Befehle): `--root <Pfad>` (Projektverzeichnis, Standard `.`), `--out <Pfad>` (Standard `dist/`),
`--cache <Pfad>` (Bild-Cache, Standard `.cache/`), `--base-url <URL>` (Standard `https://flauschiversum.de`, siehe
[Staging](#staging)) und `--staging <Branch>`; bei `serve` zusätzlich `--port <Nummer>`.

`clean` leert den Cache (`.cache/`: Bildvarianten, Suchindex, Vorschau-Ausgabe von `serve`) und meldet Dateizahl und
Größe; `clean --dist` leert zusätzlich das Ausgabeverzeichnis, aber nur, wenn es von diesem Generator stammt. Es löscht
nur bekannte Cache-Einträge (fremde Dateien bleiben) und bricht ab, wenn `--cache` auf das Projektverzeichnis oder ein
Verzeichnis mit `content/` zeigt. Nicht gleichzeitig mit `build` oder `serve` ausführen. Danach braucht der nächste Build
einige Minuten, weil Bilder und Suchindex neu berechnet werden.
Im GitHub-Workflow (`.github/workflows/staging.yml`) liegt `.cache/` in `actions/cache`; ein Neustart ohne Cache lässt sich dort über das Löschen des Caches in den Actions-Einstellungen erzwingen, `clean` wird dort nicht gebraucht.

Der erste Build berechnet alle Bilder (ca. 3200 Originale, rund 20–30 Sekunden auf 4 Kernen). Danach kommen die
Bilder aus `.cache/`, und ein Build ohne Änderungen dauert unter einer Sekunde. Geschrieben werden nur Dateien,
deren Inhalt sich geändert hat; was nicht mehr erzeugt wird, wird aus `dist/` gelöscht.

`build`, `serve` und `check` melden auf der Konsole (stderr), woran sie gerade arbeiten, mit verstrichener Zeit und
Dauer des vorherigen Schritts. Bei den Bildern gibt es alle zwei Sekunden einen Zwischenstand (Zähler, Prozent, neu
berechnet / aus dem Cache), beim Suchindex den Hinweis, ob er aus dem Cache kommt oder neu berechnet wird. Die beiden
Zusammenfassungszeilen am Ende stehen weiter auf stdout.

## Einen Post schreiben

Jeder Post ist ein Verzeichnis unter `content/posts/` mit einer `index.md` und den zugehörigen Medien:

```markdown
---
title: "Grüne Mütze"
date: 2026-11-01 10:00:00        # oder nur 2026-11-01; Zeitzone Europe/Berlin
author: Ermeline
image: "muetze.jpg"              # Vorschaubild auf der Startseite und beim Teilen
excerpt: "Kurzer Text für die Startseite, den Feed und Suchmaschinen."
tags: [Wolle, Mützen und Stirnbänder, Häkeln]   # optional: Themen (Schlagworte) aus content/tags.yaml
---

Text in Markdown …

![Die fertige Mütze](muetze.jpg)
![Von hinten](hinten.jpg)
![So sieht das Muster aus](muster.mp4)

![Ein einzelnes Bild](detail.jpg)
```

- **URL:** Ohne `permalink` lautet sie `/<Jahr>/<Monat>/<Verzeichnisname>/`, z.B. `/2026/11/gruene-muetze/` für das
  Verzeichnis `grüne-mütze`. Bestehende Posts tragen ihre bisherige URL als `permalink:` – diese Zeile nie ändern.
- **Veröffentlichung:** Posts mit einem Datum in der Zukunft erscheinen erst beim ersten Build nach diesem Zeitpunkt.
- **Bilder** (`.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`) werden in drei Größen ausgeliefert: `200x/` (Vorschau),
  `700x/` (im Text) und `1000x/` (Vollbild), mit Wasserzeichen-Badge im Farbschema „Flauschi-Nebel“ (Lavendel, Indigo; außer `200x`; neu erzeugbar mit `openspec/changes/archive/2026-10-12-post-ansicht-ueberarbeiten/assets/make_badge.py`; Änderung am Badge berechnet alle Bilder neu) und Rechteangaben in den
  Metadaten. GPS- und Kameradaten werden entfernt. Handyfotos dürfen groß sein; sie werden automatisch verkleinert.
- **Animierte GIFs und Videos** (`.mp4`, `.webm`) werden unverändert ausgeliefert. Videos laufen stumm in Schleife.
  Ab 20 MB warnt der Build.
- **Bildgruppen:** Stehen mehrere Bilder ohne Leerzeile untereinander, entsteht ein wischbares Karussell; ein Klick
  öffnet das Vollbild. Der Text in `![…]` ist die Bildunterschrift.

### Themen und Suche

Besucher finden Beiträge über die Topbar (Titel mit dem Maskottchen Flauschi links, rechts die Links **Beiträge**,
**Themen** und **Tipps und Tricks** sowie die Suche; auf dem Handy hinter einem Hamburger-Knopf; „About me“ und
„Impressum“ stehen im Footer), die Startseite (`/`), die Themenübersicht (`/themen/`), je eine Seite pro Thema
(`/tag/<name>/`) sowie die Suche (`/suche/`). Es gibt keine Kategorien: alles läuft über Themen (Schlagworte).

- **Startseite:** Überschrift, Begrüßungstext aus `content/start.md` (Markdown, ohne oder mit Frontmatter), die zehn
  neuesten Beiträge und rechts die Tag-Cloud (nur dort).
- **Beiträge** (`/beitraege/`): die zehn neuesten Beiträge und das Archiv nach Jahr und Monat (einklappbar; neuestes Jahr
  und Monat offen). Die früheren Blätterseiten (`/page/<N>/`) bleiben unter
  ihren Adressen erreichbar, werden aber nicht mehr verlinkt.
- **Einzelseiten** (`content/pages/*.md`): Seiten mit mindestens zwei Überschriften zeigen in der Seitenleiste ein
  **Inhaltsverzeichnis**, das der Build automatisch aus den Überschriften erzeugt (nichts zu pflegen). Jede Überschrift
  bekommt dafür ein `id` aus ihrem Text (`/tipps-und-tricks/#ufo-s`), Doppelte werden nummeriert. Am Handy steht das
  Verzeichnis zugeklappt über dem Text, mit JavaScript wird der aktuelle Abschnitt hervorgehoben. Beiträge haben keins.
- **Themenseite:** Name mit Anzahl, alle Beiträge des Themas und daneben das Archiv nach Jahr und Monat nur dieses
  Themas. „Themen“ in der Topbar führt auf die Übersicht aller Themen.
- **Alte Adressen:** `content/redirects.yaml` ordnet frühere Adressen neuen zu (`/kategorie/basteln/: /tag/basteln/`);
  der Build erzeugt dafür Weiterleitungsseiten (mit `noindex` und kanonischer URL) und prüft Ziel und Kollisionen.
- **Kategorien** gibt es nicht mehr. Ein altes Feld `category` im Frontmatter wird mit einer Warnung ignoriert; der Wert
  gehört als Schlagwort in `tags`.
- **Ähnliche Beiträge:** Unter jedem Beitrag stehen bis zu sechs ähnliche Beiträge. Ausgewählt wird über gemeinsame
  Schlagworte (seltene zählen mehr, `1 / Anzahl der Beiträge mit dem Schlagwort`); ohne gemeinsames Schlagwort gibt es
  keine Karte. Je genauer die Schlagworte, desto besser die Auswahl.
  Die Einträge sehen aus wie die Beitragsliste der Startseite (Bild, Titel, Datum, Kurztext). Auf der Beitragsseite steht das
  Datum unter dem Titel und der Hinweis „Schreibe Ihr eine Nachricht!“ am Ende, vor den ähnlichen Beiträgen.
- **Themen (Schlagworte):** beliebig viele, optional, als Liste oder durch Komma getrennt. Erlaubt sind nur die aus
  `content/tags.yaml`; ein Tippfehler bricht den Build ab und nennt das ähnlichste Schlagwort („Meintest du …?“).
  Ein neues Schlagwort zuerst dort eintragen. Ein Schlagwort mit weniger als zwei Beiträgen erzeugt eine Warnung.
- **Suche:** Der Index entsteht beim Build mit [Pagefind](https://pagefind.app/) (als Rust-Bibliothek, kein
  zusätzliches Programm) und liegt unter `/pagefind/`. Durchsucht werden Titel, Text und Schlagworte aller
  Beiträge und Einzelseiten; die Treffer lassen sich nach „Thema“ filtern (Seitenleiste neben den Treffern; die Seite nutzt die modulare Pagefind-Oberfläche und rendert Treffer wie die Beitragslisten); die Suche läuft im Browser, ohne Dienste von Dritten. Das Ergebnis des Index wird in
  `.cache/` zu den Eingaben gemerkt, damit ein Build ohne Änderungen nichts neu schreibt.
- Die Pfade `/themen/`, `/suche/`, `/beitraege/`, `/tag/` und `/pagefind/` sind reserviert.

### Kontakt, Social-Links und Karten

Die Kontaktadresse und die Social-Links (Instagram, Mastodon) stehen als Konstanten in `src/config.rs`
(`CONTACT_EMAIL`, `INSTAGRAM_URL`, `MASTODON_URL`) und erscheinen im Footer jeder Seite; die Icons sind inline SVG in
`templates/partials/icon-*.html`. Jeder `mailto:`-Betreff enthält „Flauschiversum“ (Spamfilter): `render::mailto()` bricht
sonst den Build ab. Die Menüs der Topbar, das Archiv und die Tag-Cloud entstehen
automatisch aus `content/tags.yaml` und den Beiträgen; die Schriftgröße eines Schlagworts
folgt der Zahl seiner Beiträge. Das Maskottchen (`content/static/images/flauschi_normal.svg` und `flauschi_surprised.svg`,
beim Überfahren des Titels) hat in beiden Dateien dieselbe `viewBox`, damit der Wechsel nichts verschiebt.

Einzelseiten (About, Impressum, Tipps und Tricks) liegen als Markdown unter `content/pages/`; ihre URL ergibt sich
aus dem Titel. Dateien unter `content/static/` werden unverändert veröffentlicht.

### Farben: Hell, Dunkel und das Universum

Das Farbschema heißt „Flauschi-Nebel“ (Violett und Indigo wie das Maskottchen, Gold und Cyan als Sternenlicht) und
gibt es hell und dunkel. Alle Farben stehen als Variablen nur in `content/style/theme.css` (ein Test sucht feste
Farben in den anderen Stylesheets). Der Modus folgt der Systemeinstellung (`prefers-color-scheme`); der Knopf in der
Topbar schaltet zwischen Automatisch, Hell und Dunkel um und merkt sich die Wahl im Browser (`localStorage`). Ein
kleines Skript im `<head>` setzt sie vor dem ersten Zeichnen, damit nichts aufblitzt. Die Sterne (`stars-small.svg`,
`stars-large.svg` unter `content/static/images/`, als Maske in Farbe eingefärbt) und der Nebel liegen in
`content/style/starfield.css` hinter dem Inhalt; die Fläche hinter dem Text ist einfarbig, die Dekoration bewegt
sich nicht und entfällt bei erhöhtem Kontrast. Kontraste und Farbfehlsichtigkeit (Protanopie, Deuteranopie,
Tritanopie) prüft `tests/browser/theme.mjs`.

## Verzeichnisse

| Pfad | Inhalt |
|---|---|
| `content/posts/` | Posts mit ihren Medien |
| `content/pages/` | Einzelseiten |
| `content/tags.yaml` | erlaubte Themen (Schlagworte) |
| `content/redirects.yaml` | Weiterleitungen alter Adressen |
| `content/static/` | unverändert ausgelieferte Dateien (Schriften, Favicon in drei Formaten, Bilder der Einzelseiten) |
| `content/style/` | Gestaltung als normales CSS (`main.css` mit `@import`), wird zu `/style.css` gebündelt |
| `templates/` | HTML-Templates ([Askama](https://askama.readthedocs.io/)), werden beim Kompilieren geprüft |
| `assets/` | vom Generator ausgelieferte Skripte (Karussell, Vollbild mit PhotoSwipe, Suchseite, Topbar und Archiv) und die Badge-Vorlage |
| `src/` | Rust-Code des Generators |
| `tests/` | Referenz der Live-URLs und Browser-Tests |
| `scripts/` | Git-Hook fürs Deployment, Server-Skripte (`scripts/server/`: `deploy-commit`, sudoers, Konfiguration) und ihr Test, Smoke-Test, Größenprüfung von `dist/` |
| `.github/workflows/` | Staging- und Produktions-Pipeline |
| `openspec/` | Anforderungen und Änderungen ([OpenSpec](https://github.com/Fission-AI/OpenSpec)) |

Änderungen an `templates/` werden erst nach erneutem Kompilieren sichtbar; für die Arbeit daran
`cargo watch -x 'run -- serve'` verwenden.

## Tests

```sh
cargo test                       # Unit- und Integrationstests (Test-Site, Snapshots)
cargo test --release -- --ignored   # baut die ganze Seite und prüft alle 194 Live-URLs
```

Browser-Tests (Karussell, Vollbild, Handy, ohne JavaScript, Live-Reload, Startseite, Topbar, Archiv, Suche und Footer) brauchen Node.js und Chromium:

```sh
cargo run --release -- serve --port 8099 &
npm i playwright-core
node tests/browser/frontend.mjs /pfad/zu/chromium
node tests/browser/navigation.mjs /pfad/zu/chromium
node tests/browser/layout.mjs /pfad/zu/chromium
node tests/browser/topbar.mjs /pfad/zu/chromium
node tests/browser/theme.mjs /pfad/zu/chromium
# Staging-Leiste: Vorschau mit `serve --port 8098 --staging <langer Branchname>` starten
node tests/browser/staging.mjs /pfad/zu/chromium <Branchname>
```

## Staging

Jeder Push auf `dev` und auf Branches `claude/**` baut die Seite und veröffentlicht sie auf GitHub Pages unter
**https://staging.flauschiversum.de/** (Workflow `.github/workflows/staging.yml`). Es gibt eine einzige Staging-Seite:
der zuletzt gepushte Stand gewinnt. Vorher laufen `cargo test` und die Inhaltsprüfung; schlägt eine fehl, wird nichts
veröffentlicht. Danach prüft `scripts/smoke-test.sh` die veröffentlichte Seite. Von Hand starten: Actions → Staging →
„Run workflow“ (z.B. auf `dev`, um dessen Stand zurückzuholen).

Die Staging-Seite ist erkennbar und nicht indexierbar:

- ganz oben eine rote Leiste „Staging: <Branch>“, dazu `/staging.txt` mit Branch, Commit und Lauf,
- `noindex` auf jeder Seite, `robots.txt` sperrt alles, keine Sitemap,
- kanonische Verweise, Feed und Open Graph zeigen auf `https://staging.flauschiversum.de`.

Lokal nachstellen:

```sh
cargo run --release -- build --base-url https://staging.flauschiversum.de --staging mein-branch
cargo run --release -- serve --staging mein-branch      # Vorschau mit Leiste
scripts/smoke-test.sh http://localhost:8080             # gegen die laufende Vorschau
```

Einmalige Einrichtung (GitHub und DNS): `CNAME staging → rincewindwizzard.github.io`, Settings → Pages → Source
„GitHub Actions“ und Custom domain `staging.flauschiversum.de` mit „Enforce HTTPS“, Settings → Environments →
`github-pages` → Deployment-Branches `dev` und `claude/*`. Die Seite ist rund 550 MB groß; das Pages-Limit liegt bei
1 GB (der Workflow warnt ab 800 MB).

## Produktiv-Deployment

Die fertige Seite geht per SSH auf den Server (`flauschiversum:/httpdocs/`). Das erledigt das Werkzeug selbst, von Hand,
aus einer GitHub Action oder aus einem Git-Hook:

```bash
cargo run --release -- deploy --dry-run   # nur zeigen: Weg, zu übertragende und zu löschende Dateien (ändert nichts)
cargo run --release -- deploy             # Seite für https://flauschiversum.de bauen und übertragen
cargo run --release -- deploy --no-build  # vorhandenes dist/ übertragen (muss ein Produktionsbuild sein)
```

**Voraussetzung:** ein Host-Eintrag `flauschiversum` in `~/.ssh/config` mit Benutzer und Schlüssel (nichts davon steht im
Repository), auf dem Server eine Shell. Kontrolle: `ssh flauschiversum true` muss ohne Passwortabfrage funktionieren.
Das Werkzeug ruft dafür das `ssh` (und bei Bedarf `sftp`) des Systems auf, liest also die SSH-Konfiguration wie gewohnt.

**Wie übertragen wird:** Der Server (netcup-Webhosting) hat kein `rsync`, deshalb packt das Werkzeug die zu übertragenden
Dateien selbst und schickt sie als Strom an `tar` auf dem Server; fehlt dort `tar`, geht es per SFTP (`--method tar|sftp`
legt den Weg fest, `--dry-run` nennt den gewählten). Übertragen wird nur, was sich laut Manifest
(`.flauschiversum-manifest`, schreibt der Build) geändert hat **oder auf dem Server fehlt** (das Werkzeug prüft, ob die Dateien
wirklich noch dort liegen; ein von Hand geleertes Verzeichnis wird also komplett neu befüllt, auch wenn die Punktdatei
`.flauschiversum-manifest` stehen blieb). `--full` überträgt unabhängig davon alles neu, z. B. nach Änderungen von Hand auf dem
Server. Beim Übertragen zeigt eine Fortschrittsanzeige Dateien, MB, Prozent, Geschwindigkeit und Restzeit (im Terminal eine
sich aktualisierende Zeile, in CI alle paar Sekunden eine Zeile). Alles geht zuerst in ein Zwischenverzeichnis im Ziel und wird
dann einzeln an seinen Platz verschoben: erst Bilder, Stylesheet, Skripte und Suchindex, dann die Seiten, dann werden
veraltete Dateien gelöscht, zuletzt wird das Manifest geschrieben. Bricht etwas ab, bleibt die alte Seite lauffähig, und der
nächste Lauf holt den Rest nach. Auf dem Server laufen dabei nur `tar`, `sh`, `mv`, `mkdir`, `chmod`, `rm` und `rmdir`.

**Sicherheit:** `deploy` bricht ab, wenn `dist/` ein Staging-Build ist, eine andere Basis-URL hat oder das Manifest fehlt
(`--staging` ist bei `deploy` nicht erlaubt). Auf dem Server werden **nur Dateien gelöscht, die ein früheres Deployment selbst
abgelegt hat**; fremde Dateien in `/httpdocs/` (Bestätigungsdateien, `.well-known`, Reste der alten Seite) bleiben. Das
**erste Deployment** überlagert nur und löscht nichts: Reste der alten Seite müssen einmal von Hand entfernt werden. Immer
zuerst `--dry-run`. Zwei Deployments gleichzeitig verhindert eine Sperrdatei (`.deploy.lock`).

Weitere Einstellungen: `--target` bzw. `DEPLOY_TARGET` (Standard `flauschiversum:/httpdocs/`; ein Pfad ohne `host:` ist ein lokales
Ziel, z. B. wenn der Build auf dem Webserver läuft), `--base-url`, `--cache`, `--out`. Getestet ist alles ohne Server mit
`cargo test deploy` (lokales Ziel und kleine Ersatzprogramme für `ssh` und `sftp`).

### Automatisch: GitHub Action

`.github/workflows/production.yml` läuft bei jedem Push auf `master` (und von Hand unter Actions → Produktion): Tests, dann
`flauschiversum deploy`, dann `scripts/smoke-test.sh` mit `SMOKE_MODE=production` gegen die Live-Seite. Einmalig einrichten
(Settings → Secrets and variables → Actions, am besten in der Umgebung `production`, dort lassen sich auch Freigaben verlangen):

1. Eigenes Schlüsselpaar nur für das Deployment: `ssh-keygen -t ed25519 -f deploy_key -C "flauschiversum-deploy" -N ""`, den
   öffentlichen Teil beim Hoster für den SSH-Zugang hinterlegen.
2. Secrets anlegen: `DEPLOY_SSH_KEY` (Inhalt von `deploy_key`), `DEPLOY_HOST`, `DEPLOY_USER`, optional `DEPLOY_PORT`.
3. `DEPLOY_KNOWN_HOSTS`: `ssh-keyscan -p <Port> <Host>` ausführen, den **Fingerabdruck mit dem vom Hoster genannten vergleichen**
   (`ssh-keygen -lf <Datei>`) und die Zeile(n) als Secret eintragen. Der Lauf prüft den Server strikt dagegen.
4. Empfohlen: Branch-Schutz für `master` (nur über Pull Requests), weil jeder Push dorthin live geht.

### Automatisch: Git-Hook (ohne GitHub Actions)

Für ein Repository auf einem eigenen Git-Server: Bei einem Push auf `master` baut der Hook `post-receive` den gepushten Stand
und deployt ihn. **Der Build läuft nicht unter dem Git-Benutzer, sondern unter dem eigenen Benutzer `flauschiversum`**
(Home `/var/share/flauschiversum`): Dort liegen der SSH-Schlüssel zum Webserver und die Rust-Installation. Der Hook darf
per `sudo` genau einen festen Befehl als dieser Benutzer starten (`deploy-commit <Commit-ID>`), sonst nichts; den Schlüssel
kann der Git-Benutzer nicht lesen. Die Ausgabe sieht der Pusher (`remote: …`), das Protokoll steht in
`/var/share/flauschiversum/.cache/flauschiversum/deploy.log`. Anderer Branch: `DEPLOY_REF=refs/heads/main` in der Umgebung des Hooks.

#### Server für das Deployment einrichten

Gilt für Debian/Ubuntu. Angenommen: der Git-Server läuft unter dem Benutzer **`git`**, das Bare-Repository liegt in
`/media/git/nadja/flauschiversum` (das ist das **Bare-Repository**, in das gepusht wird; `REPO`, der Hook und der Build
verwenden nur dieses). Die Skripte unter `scripts/` brauchst du einmalig als Dateien: dafür ein normaler Klon, z. B.
`git clone /media/git/nadja/flauschiversum ~/flauschiversum` (als `root`; ein anderer Ort geht genauso). Passe die Namen an. Alle Befehle als `root` (`sudo -i`), sofern nicht anders
steht. Bedarf: etwa 2 GB RAM und 3 GB Platz, der erste Build dauert einige Minuten.

**1. Pakete.** C-Compiler und Linker braucht Rust für Bibliotheken wie `libwebp`; `flock` stammt aus `util-linux`.

```bash
apt update
apt install -y git curl ca-certificates build-essential pkg-config openssh-client util-linux sudo
```

**2. Den Benutzer `flauschiversum` anlegen.** Ein Systembenutzer ohne Passwort und ohne Login-Shell, mit eigener Gruppe und
dem Home `/var/share/flauschiversum` (das Verzeichnis `/var/share` gibt es nicht überall, daher zuerst `mkdir -p`):

```bash
mkdir -p /var/share
useradd --system --create-home --home-dir /var/share/flauschiversum \
        --shell /usr/sbin/nologin --user-group flauschiversum
chmod 750 /var/share/flauschiversum        # Fremde (auch der Benutzer git) kommen nicht hinein
id flauschiversum                          # Kontrolle: uid=… gid=…(flauschiversum)
ls -ld /var/share/flauschiversum           # Kontrolle: drwxr-x--- flauschiversum flauschiversum
```

Eine Login-Shell braucht der Benutzer nicht: `sudo -u flauschiversum <Befehl>` startet den Befehl auch so. Alle
Einrichtungsbefehle unten laufen deshalb als `sudo -u flauschiversum -H …` von `root` aus.

Der Build muss das Repository nur **lesen** können. Prüfe, ob `flauschiversum` das darf (Git gehört das Repository dem
Benutzer `git`; als `root` oder `flauschiversum` verweigert Git den Zugriff mit „nicht in einem Git-Repository“, solange
das Verzeichnis nicht als sicher markiert ist – `deploy-commit` setzt dafür selbst `safe.directory`, deshalb dieselbe Option
hier in der Kontrolle):

```bash
sudo -u flauschiversum git -c safe.directory=/media/git/nadja/flauschiversum --git-dir=/media/git/nadja/flauschiversum rev-parse master
```

Das muss eine Commit-ID ausgeben. Scheitert es mit „Keine Berechtigung“, braucht `flauschiversum` Leserechte: entweder
Mitglied der Gruppe des Repositorys werden (`usermod -aG <Gruppe> flauschiversum`, Gruppe siehe `ls -ld`) oder das Repository
für andere lesbar machen (`chmod -R o+rX`). Ist es schon für alle lesbar, ist nichts zu tun. Auch die übergeordneten Verzeichnisse
(`/media/git`, `/media/git/nadja`) müssen durchsuchbar sein (`namei -l /media/git/nadja/flauschiversum` zeigt, wo es scheitert).

**3. Rust und Cargo installieren** (nur im Home von `flauschiversum`, systemweit wird nichts verändert):

```bash
sudo -u flauschiversum -H bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable'
sudo -u flauschiversum -H bash -c '~/.cargo/bin/cargo --version'    # Kontrolle: cargo 1.xx.x
```

`deploy-commit` setzt `~/.cargo/bin` selbst in den `PATH`. Später aktualisieren:
`sudo -u flauschiversum -H bash -c '~/.cargo/bin/rustup update'`.

**4. SSH-Zugang zum Webserver.** Eigenes Schlüsselpaar nur für das Deployment, erzeugt als `flauschiversum`:

```bash
sudo -u flauschiversum -H bash -c 'install -d -m 700 ~/.ssh && ssh-keygen -t ed25519 -N "" -C flauschiversum-deploy -f ~/.ssh/deploy_key'
cat /var/share/flauschiversum/.ssh/deploy_key.pub       # diesen öffentlichen Teil beim Hoster für den SSH-Zugang eintragen
```

Dann den Host-Eintrag `flauschiversum` anlegen (`HostName`, `User` und gegebenenfalls `Port` vom Hoster):

```bash
cat > /var/share/flauschiversum/.ssh/config <<'CONF'
Host flauschiversum
    HostName ssh.example.org
    User u12345
    IdentityFile ~/.ssh/deploy_key
    IdentitiesOnly yes
CONF
chown flauschiversum:flauschiversum /var/share/flauschiversum/.ssh/config
chmod 600 /var/share/flauschiversum/.ssh/config
```

Den Schlüssel des Webservers holen, **den Fingerabdruck mit dem vom Hoster genannten vergleichen** und erst dann übernehmen:

```bash
sudo -u flauschiversum -H bash -c 'ssh-keyscan -t ed25519 ssh.example.org > ~/.ssh/known_hosts.neu && ssh-keygen -lf ~/.ssh/known_hosts.neu'
# stimmt der Fingerabdruck?  Dann:
sudo -u flauschiversum -H bash -c 'mv ~/.ssh/known_hosts.neu ~/.ssh/known_hosts'
sudo -u flauschiversum -H ssh flauschiversum true       # Kontrolle: Ende ohne Passwortabfrage und ohne Fehler
```

**5. Den Einstiegsbefehl installieren** (gehört `root`, damit niemand sonst ändern kann, was per `sudo` läuft):

```bash
cd ~/flauschiversum        # das ausgecheckte Repository
install -d -o root -g root -m 755 /usr/local/libexec/flauschiversum /etc/flauschiversum
install -o root -g root -m 755 scripts/server/deploy-commit /usr/local/libexec/flauschiversum/deploy-commit
install -o root -g root -m 644 scripts/server/deploy.conf.example /etc/flauschiversum/deploy.conf
editor /etc/flauschiversum/deploy.conf                  # REPO=/media/git/nadja/flauschiversum prüfen/anpassen
```

**6. `sudo`-Regel für den Hook.** Der Benutzer `git` darf nur diesen einen Befehl als `flauschiversum` ausführen:

```bash
install -o root -g root -m 440 scripts/server/sudoers /etc/sudoers.d/flauschiversum
editor /etc/sudoers.d/flauschiversum                    # "git" durch den Benutzer ersetzen, falls der Git-Server anders heißt
visudo -cf /etc/sudoers.d/flauschiversum                # Kontrolle: parsed OK
sudo -l -U git                                          # Kontrolle: nennt (flauschiversum) NOPASSWD: …/deploy-commit
```

**7. Hook installieren** (als der Benutzer, dem das Repository gehört, hier `git`):

```bash
sudo -u git ~/flauschiversum/scripts/install-git-hook.sh /media/git/nadja/flauschiversum
```

**8. Erster Lauf von Hand**, damit der erste Release-Build nicht im Push hängt (Commit-ID:
`git --git-dir=/media/git/nadja/flauschiversum -c safe.directory=/media/git/nadja/flauschiversum rev-parse master`):

```bash
sudo -u git sudo -n -u flauschiversum /usr/local/libexec/flauschiversum/deploy-commit <40-stellige Commit-ID>
```

Erst danach pushen. Der erste Lauf überlagert nur, was schon auf dem Webserver liegt, und löscht nichts (siehe „Sicherheit“
oben). Vorab zeigt `deploy --dry-run` in einem Export, was passieren würde.

**Kontrolle der Trennung:** `sudo -u git cat /var/share/flauschiversum/.ssh/deploy_key` muss mit „Keine Berechtigung“ scheitern.

**Fehlersuche.** Zuerst das Protokoll lesen: `tail -n 50 /var/share/flauschiversum/.cache/flauschiversum/deploy.log`.

| Meldung / Symptom | Ursache |
|---|---|
| `sudo: a password is required` / `no tty present` | sudoers-Datei fehlt oder nennt den falschen Benutzer (Schritt 6) |
| `Permission denied` beim Export | `flauschiversum` darf das Repository nicht lesen (Schritt 2) |
| `linker 'cc' not found` | `build-essential` fehlt (Schritt 1) |
| `Permission denied (publickey)` / `Host key verification failed` | Schlüssel nicht beim Hoster eingetragen oder `known_hosts` fehlt (Schritt 4) |
| `Sperre nach … nicht erhalten` | ein anderes Deployment läuft noch (`LOCK_WAIT` in `deploy.conf`) |

### Favicon

Das Favicon ist das normale Maskottchen Flauschi in vereinfachter Fassung (`content/static/favicon.svg`, ohne Fransen und
Kleindetails, damit es bei 16 px erkennbar bleibt). `favicon.ico` (16, 32, 48 px) und `apple-touch-icon.png` (180 px)
entstehen daraus mit `node openspec/changes/favicon-maskottchen/assets/make_favicons.mjs` (aus dem Projektverzeichnis; Chromium
und ImageMagick nötig, `--preview` erzeugt Vorschaubilder; nach dem Archivieren liegt das Skript unter
`openspec/changes/archive/2026-10-12-favicon-maskottchen/assets/`). Die Dateien sind eingecheckt, der Build erzeugt sie nicht.


# Nichts
gar ncihtsa