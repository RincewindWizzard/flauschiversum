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
| `scripts/` | Smoke-Test der Staging-Seite, Größenprüfung von `dist/` |
| `.github/workflows/` | Staging-Pipeline |
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

## Deployment

Das frühere Docker-Deployment wurde entfernt. Die gebaute Seite in `dist/` muss nur auf einen Webserver für
statische Dateien kopiert werden. Das Produktiv-Deployment aus `main` ist als eigene Änderung geplant.

### Favicon

Das Favicon ist das normale Maskottchen Flauschi in vereinfachter Fassung (`content/static/favicon.svg`, ohne Fransen und
Kleindetails, damit es bei 16 px erkennbar bleibt). `favicon.ico` (16, 32, 48 px) und `apple-touch-icon.png` (180 px)
entstehen daraus mit `node openspec/changes/favicon-maskottchen/assets/make_favicons.mjs` (aus dem Projektverzeichnis; Chromium
und ImageMagick nötig, `--preview` erzeugt Vorschaubilder; nach dem Archivieren liegt das Skript unter
`openspec/changes/archive/2026-10-12-favicon-maskottchen/assets/`). Die Dateien sind eingecheckt, der Build erzeugt sie nicht.
