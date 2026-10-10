# Design

## Context

Die Motivation steht in `proposal.md`, das heutige Verhalten in den Specs unter `openspec/specs/`. Für die Umsetzung sind diese Rahmenbedingungen wichtig:

- **Datenmenge:** 194 Posts, 3 Seiten und 3195 Bilder (357 MB). Fast alle Originale haben höchstens 1000 px an der längsten Kante. Teuer ist nur die Bildverarbeitung: Mit Python und einem Kern dauert sie ca. 90–140 s, HTML dagegen weniger als 1 s.
- **Referenz:** Gesichert sind die Live-Seite mit ihren 194 Post-URLs und 2857 Medien-Referenzen. Die URL-Liste wird zur Test-Fixture.
- **Inhalte:** 82 Originale tragen einen veralteten EXIF-Drehungs-Tag (Werte 3, 6 oder 8), obwohl ihre Pixel bereits aufrecht sind. Das ist an der Live-Seite geprüft.
- **Umgebung:** Rust 1.97 (stable). Der Build soll später in einer GitHub Action laufen (eigener Change) und darf deshalb keine Systembibliotheken außer einem C-Compiler voraussetzen.

## Goals / Non-Goals

**Goals:**
- Ein einziges Binary `flauschiversum` mit den Befehlen `build`, `check` und `serve`. Konfiguration über Konstanten und CLI-Optionen, keine eigene Konfigurationsdatei.
- Ein Kaltstart-Build aller Bilder in unter 2 Minuten auf 4 Kernen. Ein Build ohne Änderungen in wenigen Sekunden, ohne dass ein Bild dekodiert wird.
- Jede Abweichung vom alten Verhalten ist durch eine Spec gedeckt. Ein Test sichert alle Live-URLs.

**Non-Goals:**
- Deployment, GitHub Action, Server-Konfiguration.
- Ein allgemeiner Static-Site-Generator: Pfade, Texte und Größen sind fest auf das Flauschiversum zugeschnitten.
- Videos umkodieren, Kategorien- oder Archivseiten, Suche.

## Decisions

### 1. Repository-Layout

```
Cargo.toml, Cargo.lock
src/            Rust-Code (main.rs, cli, content, markdown, media/, render, feed, seo, output, serve)
templates/      Askama-Templates, beim Kompilieren eingebunden (layout.html, index.html, post.html, page.html, partials/…)
assets/         vom Generator ausgeliefert: badge.png, js/carousel.js, vendor/photoswipe/…
content/        posts/, pages/, static/, style/   (per `git mv` aus src/)
tests/          Integrationstests + fixtures/ (Mini-Site, live-urls.txt)
```

Der Umzug `src/` → `content/` passiert in einem **eigenen Commit nur mit `git mv`**. So erkennt Git die Umbenennung, und die Historie bleibt nachvollziehbar.
*Alternative:* Code in `generator/` und Inhalte in `src/` lassen. Verworfen, weil es von der Rust-Konvention abweicht und `src/` dann zweideutig wäre.

### 2. Pipeline

```
load+validate (seriell, schnell) -> media (rayon, parallel) -> render (rayon) -> write (diff + manifest)
```

Alle Ausgaben entstehen als `(Pfad, Bytes)` in einer Map. Erst der Writer berührt die Platte. Dadurch lassen sich `check` (ohne Writer), `build` und `serve` mit demselben Kern umsetzen.

### 3. Bibliotheken

| Zweck | Crate | Begründung / Alternative |
|---|---|---|
| CLI | `clap` (derive) | Standard |
| Frontmatter | `serde` + `serde_yaml_ng` | Gepflegter Fork von `serde_yaml`. Datumswerte werden als String gelesen und selbst geparst, damit beide Formate exakt gelten. |
| Datum/Zeitzone | `jiff` | Zeitzone Europe/Berlin mit eingebauter Zeitzonendatenbank. `chrono-tz` wäre die Alternative. |
| Markdown | `pulldown-cmark` (+ HTML-Durchreichung) | Event-Stream für die Erkennung von Bildgruppen. `comrak` ist schwerer und bringt hier keinen Vorteil. |
| Templates | `askama` | Templates werden beim Kompilieren gegen typisierte Structs geprüft und zu Rust-Code. Das ist schnell, fehlende Felder sind Compilerfehler, und die Syntax ist Jinja-ähnlich, sodass die Templates leicht übertragbar sind. *Alternativen:* `minijinja` und `tera` (zur Laufzeit interpretiert, Fehler erst beim Rendern), `maud` (HTML als Rust-Makro, typsicher, aber für Layouts schlechter lesbar). Nachteil von Askama: Wer ein Template ändert, muss neu kompilieren (siehe Vorschau-Server). |
| Dekodieren | `image` (jpeg, png, webp, gif) | — |
| Skalieren | `fast_image_resize` (Lanczos3, SIMD) | Viel schneller als `image::imageops`. |
| JPEG schreiben | `jpeg-encoder` | Reines Rust, optimierte Huffman-Tabellen. `mozjpeg` ist kleiner in der Ausgabe, braucht aber CMake und NASM. |
| WebP schreiben | `webp` (libwebp über `cc`) | `image` kann nur verlustfreies WebP. Braucht nur einen C-Compiler. |
| Metadaten einfügen | `img-parts` | Ersetzt oder entfernt Segmente bzw. Chunks. Die EXIF-, XMP- und IPTC-Bytes erzeugen wir selbst (feste kleine Strukturen). |
| Hash | `blake3` | Schnell, für Cache-Schlüssel und Ausgabe-Diff. |
| Parallelität | `rayon` | — |
| RSS | `rss` (mit Extensions für `dc`, `content`, `atom`) | — |
| CSS | `lightningcss`, **nur** Bundler + Minifier | Es gibt keinen Präprozessor mehr: Quellen sind Standard-CSS (Custom Properties, Nesting, `color-mix()`, alles in aktuellen Browsern verfügbar). lightningcss löst `@import` auf, minimiert und senkt Nesting für ältere Browser ab. *Alternativen:* Sass mit `grass` (unnötig, die Sprachmittel hat CSS heute selbst), Dateien einzeln ausliefern (mehr Requests, kein Minify). |
| Vorschau | `tiny_http` + `notify` | Leichtgewichtig. Live-Reload per Polling eines Build-Zählers. `axum` wäre für diesen Zweck zu schwer. |
| Tests | `insta`, `tempfile` | Snapshot-Tests für das erzeugte HTML. |

### 4. Permalinks

Ein einmaliges Migrationsskript ordnet jedem Post anhand von Datum und Titel seine Live-URL aus `tests/fixtures/live-urls.txt` zu und **fügt nur eine Zeile `permalink: …` nach `title:` ein**. Das YAML wird dabei nicht neu serialisiert. Prüfung: `git diff --stat` zeigt pro `index.md` genau eine eingefügte Zeile. Der Sonderfall `gibs-mir-audrey-ii` ergibt sich automatisch, weil die Liste maßgeblich ist.

Für neue Posts ohne `permalink` wird der Slug aus dem **Verzeichnisnamen** gebildet (deutsche Transliteration `ü`→`ue`). Der Titel ist dafür ungeeignet, weil er sich ändern kann.

### 5. Markdown → Medien-HTML

Die Event-Folge von pulldown-cmark wird pro Absatz gepuffert. Enthält ein Absatz Bilder, geht der Text als `<p>` hinaus (falls nicht leer). Danach folgt die Mediengruppe:

- 1 Medium → `<figure class="media">` mit `<a href="1000x/…" data-pswp-*><img …></a>` und `<figcaption>`
- n Medien → `<div class="carousel"><ul>` mit einem `<li><figure>…</figure></li>` pro Medium

Video wird zu `<video autoplay muted loop playsinline preload="metadata">`, GIF zu `<img>` auf das Original. Breite und Höhe stammen aus der Medienanalyse (Schritt Media), die vor dem Rendern läuft. Referenzen auf unbekannte Dateien bleiben unverändert und führen zu einer Warnung.

### 6. Bildverarbeitung

Für jedes Original:

1. Bytes lesen, Hash `h_orig`. Die Schlüssel für die drei Varianten werden berechnet aus `h_orig`, Hash der Badge-Vorlage (nur bei 700/1000), Breite, Qualitätsstufe, Metadaten-Text und `PIPELINE_VERSION`.
2. Liegen alle drei Varianten im Cache (`.cache/v1/<schlüssel>.<ext>`), wird nur kopiert. Abmessungen und Dateigrößen stehen in `.cache/v1/<schlüssel>.json`.
3. Sonst: einmal dekodieren, EXIF-Orientierung anwenden, kaskadiert skalieren (Original → 1000 → 700 → 200). Bei 700 und 1000 das Badge aufbringen (Alpha-Blending). Kodieren im Originalformat. JPEG und WebP starten mit Qualität 82 und reduzieren in Schritten von 6 bis höchstens 1 MB erreicht ist. Danach die Rechte-Metadaten einsetzen. Andere Metadaten werden nie übernommen, weil wir aus den Pixeldaten neu kodieren.
4. Animierte GIFs (mehr als 1 Frame) und Videos werden nur kopiert.

**Badge:** Eine Vorlage `assets/badge.png` (ca. 800 px breit, mit transparentem Rand) wird **einmal** erzeugt (Lobster, `#333` mit 75 % Deckkraft, Fläche `#bdf239` mit 43 % Deckkraft, Radius halbe Höhe) und eingecheckt. Pro Variante wird sie auf ein Fünftel der Bildbreite skaliert (mindestens 60 px) und mit 2,5 % Randabstand gesetzt. Bei Varianten unter 240 px Breite entfällt sie.
*Alternative:* Schrift zur Laufzeit rendern (`ab_glyph`). Verworfen: mehr Code, und das Ergebnis wäre nicht garantiert identisch mit dem abgestimmten Entwurf.

**Veraltete Drehungs-Tags:** Ein einmaliges Skript setzt bei den 82 betroffenen Originalen den Orientation-Wert **im EXIF-Block auf 1**. Das ist ein Byte-Patch im IFD-Eintrag, ohne neu zu kodieren. Prüfung: Die dekodierten Pixel sind vorher und nachher identisch.

### 7. Ausgabe und Inkrementalität

- Ins Ausgabeverzeichnis kommt `.flauschiversum-manifest` (JSON: Pfad → blake3, Größe). Seine Existenz kennzeichnet ein Build-Ziel.
- Schreiben: Stimmt der Hash im Manifest überein und hat die Datei auf der Platte die erwartete Größe, wird nichts geschrieben. Sonst wird in eine temporäre Datei geschrieben und umbenannt.
- Löschen: Alles im Ausgabeverzeichnis, was nicht im neuen Manifest steht, außer dem Manifest selbst. Danach leere Verzeichnisse.
- Ist `--out` nicht leer und ohne Manifest, bricht der Build ab (Schutz vor `--out ~`).
- HTML, Feed, Sitemap und CSS werden immer neu gerendert. Der Diff verhindert unnötige Schreibvorgänge. `lastBuildDate` im Feed und das Jahr in der Fußzeile hängen nur an Inhalten bzw. am Jahr, damit Ausgaben stabil bleiben.

### 8. Frontend

- **CSS:** Die 417 Zeilen LESS werden von Hand in CSS überführt. Variablen werden zu Custom Properties (`--theme-green` …), die vier `lighten()`-Aufrufe zu `color-mix(in oklab, var(--theme-bright), white 40%)` o.ä. (visuell abgeglichen), LESS-Verschachtelung bleibt als natives Nesting. Neu hinzu kommen die Regeln für `figure.media`, `.carousel` (`scroll-snap-type: x mandatory`) und die neue `h1`-Struktur. Das bisherige Aussehen bleibt erhalten.
- **Karussell:** Ohne JavaScript ist es eine horizontal scrollbare Liste mit Snap. `assets/js/carousel.js` (Vanilla, ca. 100 Zeilen) ergänzt Pfeile, Punkte und Tastatursteuerung und synchronisiert die Punkte per `IntersectionObserver`.
- **Vollbild:** PhotoSwipe 5 (MIT) wird unter `assets/vendor/photoswipe/` mitgeliefert und als ES-Modul lazy geladen. Eine Galerie entspricht einer Mediengruppe. `data-pswp-width/height` kommen aus der Medienanalyse.
- **Reduzierte Bewegung:** Bei `prefers-reduced-motion` pausiert ein kleines Skript Videos und blendet die Bedienelemente ein.

### 9. Vorschau-Server

`serve` baut in ein temporäres Ausgabeverzeichnis und liefert es mit `tiny_http` aus (inkl. `index.html` für Verzeichnisse). `notify` beobachtet `content/` und `assets/`. Templates sind einkompiliert. Für Arbeit an Templates wird `serve` per `cargo watch -x 'run -- serve'` gestartet, das bei Änderungen neu kompiliert und neu startet. Die Seite lädt danach ebenfalls automatisch neu, weil das Live-Reload-Skript nach dem Neustart einen neuen Zählerstand sieht. Nach einem Rebuild (entprellt) erhöht sich ein Build-Zähler, den die Seite über `/__livereload` jede Sekunde abfragt (Server-Sent Events wurden verworfen, weil `tiny_http` gestreamte Antworten puffert). Nur im `serve`-Modus wird dafür ein kleines Skript in jede Seite eingefügt.

### 10. Tests

- **Unit:** Slug, Datum und Zeitzone, „veröffentlicht“, Paginierungsfenster, Kürzung der Meta-Beschreibung, Erkennung von Bildgruppen, Cache-Schlüssel, Byte-Strukturen der Metadaten.
- **Integration:** Eine Mini-Site unter `tests/fixtures/site` (3 Posts, darunter einer geplant, Einzelbild, Karussell, Video, PNG) wird gebaut. Geprüft werden Snapshots von HTML, Feed und Sitemap, Verhalten bei einem zweiten Build (0 Schreibvorgänge) und das Löschen verwaister Dateien.
- **Gesamtseite:** Ein Test baut `content/` und prüft, dass jede URL aus `live-urls.txt` existiert und alle Medien-Referenzen auflösen. Wegen der Laufzeit ist er als `#[ignore]` markiert und läuft mit `cargo test -- --ignored`.

## Risks / Trade-offs

- [Die Badge-Vorlage sieht anders aus als im Entwurf] → Vorlage mit denselben Parametern erzeugen. Vor dem Abschluss eine Vergleichsgrafik (alt/neu, hell/dunkel) zur Abnahme vorlegen.
- [Das `webp`-Crate braucht einen C-Compiler] → In jeder üblichen CI vorhanden. Im Repo gibt es derzeit keine WebP-Bilder, notfalls ginge es verlustfrei über `image`.
- [JPEG-Encoder in reinem Rust erzeugt größere Dateien als mozjpeg] → Grenze und Qualität sind messbar (Stichprobe: Median ~70 KB bei 1000 px). Austauschbar hinter einer Funktion.
- [Die manuelle Umstellung von LESS auf CSS verändert das Aussehen] → Screenshot-Vergleich alt (Live) gegen neu für Start-, Post- und About-Seite.
- [Der Byte-Patch der Drehungs-Tags beschädigt Dateien] → Nur Dateien mit genau einem Orientation-Eintrag patchen und danach die Pixelgleichheit prüfen. Git erlaubt die Rücknahme.
- [Großer Diff durch Umzug und Permalinks] → Eigene Commits: (1) `git mv`, (2) Permalinks, (3) Kategorien und Drehungs-Tags, (4) Code.
- [Die Löschlogik der Ausgabe trifft falsche Dateien] → Manifest-Pflicht für nicht leere Ziele. Dafür gibt es einen Integrationstest.

## Migration Plan

1. Inhalte nach `content/` verschieben, Permalinks setzen, Kategorien und Drehungs-Tags korrigieren. Jeder Schritt bekommt einen eigenen Commit.
2. Den Rust-Generator parallel zum alten Code aufbauen, bis der Gesamtseiten-Test grün ist und die Screenshots abgenommen sind.
3. Python-Code, LESS, `docker/`, `docker-compose.yml`, `requirements.txt` und `test/` entfernen. README und `openspec/config.yaml` aktualisieren.
4. **Rollback:** Bis zum neuen Deployment-Change bleibt die Live-Seite unberührt. Ein Zurück auf den Stand vor dem Change ist ein einfacher Git-Revert.
