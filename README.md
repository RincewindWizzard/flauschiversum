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
```

Optionen (für alle Befehle): `--root <Pfad>` (Projektverzeichnis, Standard `.`), `--out <Pfad>` (Standard `dist/`),
`--cache <Pfad>` (Bild-Cache, Standard `.cache/`), bei `serve` zusätzlich `--port <Nummer>`.

Der erste Build berechnet alle Bilder (ca. 3200 Originale, rund 20–30 Sekunden auf 4 Kernen). Danach kommen die
Bilder aus `.cache/`, und ein Build ohne Änderungen dauert unter einer Sekunde. Geschrieben werden nur Dateien,
deren Inhalt sich geändert hat; was nicht mehr erzeugt wird, wird aus `dist/` gelöscht.

## Einen Post schreiben

Jeder Post ist ein Verzeichnis unter `content/posts/` mit einer `index.md` und den zugehörigen Medien:

```markdown
---
title: "Grüne Mütze"
date: 2026-11-01 10:00:00        # oder nur 2026-11-01; Zeitzone Europe/Berlin
author: Ermeline
image: "muetze.jpg"              # Vorschaubild auf der Startseite und beim Teilen
excerpt: "Kurzer Text für die Startseite, den Feed und Suchmaschinen."
category: wolle                  # optional: eine Kategorie aus content/categories.yaml (die `id`)
tags: [Mützen und Stirnbänder, Häkeln]   # optional: Schlagworte aus content/tags.yaml
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
  `700x/` (im Text) und `1000x/` (Vollbild), mit Wasserzeichen-Badge (außer `200x`) und Rechteangaben in den
  Metadaten. GPS- und Kameradaten werden entfernt. Handyfotos dürfen groß sein; sie werden automatisch verkleinert.
- **Animierte GIFs und Videos** (`.mp4`, `.webm`) werden unverändert ausgeliefert. Videos laufen stumm in Schleife.
  Ab 20 MB warnt der Build.
- **Bildgruppen:** Stehen mehrere Bilder ohne Leerzeile untereinander, entsteht ein wischbares Karussell; ein Klick
  öffnet das Vollbild. Der Text in `![…]` ist die Bildunterschrift.

### Kategorien, Schlagworte und Suche

Besucher finden Beiträge über die Startseite (`/`), die Themenübersicht (`/themen/`), je eine Seite pro Kategorie
(`/kategorie/<name>/`) und Schlagwort (`/tag/<name>/`) sowie die Suche (`/suche/`). Die chronologische Liste liegt
unter `/beitraege/`.

- **Kategorie:** höchstens eine, optional. Die erlaubten stehen in `content/categories.yaml` (`id`, `name`, `image`
  als Pfad unter `content/static/`, `motto`). Neue Kategorie = neuer Eintrag dort; sie erscheint dann als Kachel.
- **Schlagworte:** beliebig viele, optional, als Liste oder durch Komma getrennt. Erlaubt sind nur die aus
  `content/tags.yaml`; ein Tippfehler bricht den Build ab und nennt das ähnlichste Schlagwort („Meintest du …?“).
  Ein neues Schlagwort zuerst dort eintragen. Ein Schlagwort mit weniger als zwei Beiträgen erzeugt eine Warnung.
- **Suche:** Der Index entsteht beim Build mit [Pagefind](https://pagefind.app/) (als Rust-Bibliothek, kein
  zusätzliches Programm) und liegt unter `/pagefind/`. Durchsucht werden Titel, Text, Kategorie und Schlagworte aller
  Beiträge und Einzelseiten; die Suche läuft im Browser, ohne Dienste von Dritten. Das Ergebnis des Index wird in
  `.cache/` zu den Eingaben gemerkt, damit ein Build ohne Änderungen nichts neu schreibt.
- Die Pfade `/themen/`, `/suche/`, `/beitraege/`, `/kategorie/`, `/tag/` und `/pagefind/` sind reserviert.

Einzelseiten (About, Impressum, Tipps und Tricks) liegen als Markdown unter `content/pages/`; ihre URL ergibt sich
aus dem Titel. Dateien unter `content/static/` werden unverändert veröffentlicht.

## Verzeichnisse

| Pfad | Inhalt |
|---|---|
| `content/posts/` | Posts mit ihren Medien |
| `content/pages/` | Einzelseiten |
| `content/categories.yaml`, `content/tags.yaml` | erlaubte Kategorien und Schlagworte |
| `content/static/` | unverändert ausgelieferte Dateien (Schriften, Favicon, Bilder der Einzelseiten) |
| `content/style/` | Gestaltung als normales CSS (`main.css` mit `@import`), wird zu `/style.css` gebündelt |
| `templates/` | HTML-Templates ([Askama](https://askama.readthedocs.io/)), werden beim Kompilieren geprüft |
| `assets/` | vom Generator ausgelieferte Skripte (Karussell, Vollbild mit PhotoSwipe, Suchseite) und die Badge-Vorlage |
| `src/` | Rust-Code des Generators |
| `tests/` | Referenz der Live-URLs und Browser-Tests |
| `openspec/` | Anforderungen und Änderungen ([OpenSpec](https://github.com/Fission-AI/OpenSpec)) |

Änderungen an `templates/` werden erst nach erneutem Kompilieren sichtbar; für die Arbeit daran
`cargo watch -x 'run -- serve'` verwenden.

## Tests

```sh
cargo test                       # Unit- und Integrationstests (Test-Site, Snapshots)
cargo test --release -- --ignored   # baut die ganze Seite und prüft alle 194 Live-URLs
```

Browser-Tests (Karussell, Vollbild, Handy, ohne JavaScript, Live-Reload, Startseite, Suche) brauchen Node.js und Chromium:

```sh
cargo run --release -- serve --port 8099 &
npm i playwright-core
node tests/browser/frontend.mjs /pfad/zu/chromium
node tests/browser/navigation.mjs /pfad/zu/chromium
```

## Deployment

Das frühere Docker-Deployment wurde entfernt. Die gebaute Seite in `dist/` muss nur auf einen Webserver für
statische Dateien kopiert werden. Ein automatisches Deployment per GitHub Action ist als eigene Änderung geplant.
