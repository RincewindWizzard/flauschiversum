## Context

`templates/layout.html` verlinkt nur `/favicon.ico` (16×16, 16 Farben, 318 Byte, aus dem alten Auftritt). Das Maskottchen liegt als `content/static/images/flauschi_normal.svg` (5,9 KB, 478 × 425, Verläufe in Violett `#b859fd`, Indigo `#596af4` und Cyan `#59fdf4`, mehrere Gruppen für Fransen, Augen und Details). Der Build liefert alles unter `content/static/` unverändert aus. Im Build-Umfeld sind ImageMagick (`convert`), Pillow und Chromium (Playwright) vorhanden, aber kein `rsvg-convert`.

## Goals / Non-Goals

**Goals:** ein erkennbares, hübsches Maskottchen-Favicon in allen gängigen Formaten; die alte Adresse bleibt.
**Non-Goals:** Manifest, Maskable- und 512-px-Icons, Dark-Mode-Variante, Änderung des Topbar-Maskottchens.

## Decisions

### 1. Vereinfachte Variante

Aus `flauschi_normal.svg` wird `favicon.svg` abgeleitet, mit `viewBox` 0 0 64 64 und nur den tragenden Formen: runder, leicht unregelmäßiger Körper mit dem Verlauf des Originals (Violett → Indigo), zwei große weiße Augen mit dunklen Pupillen und je einem Glanzpunkt, ein dünner heller Rand um den Körper, damit die Silhouette auf dunklen Tab-Leisten nicht verschwindet. Fransen, Mund und Mikrodetails entfallen: Bei 16 px wären sie Rauschen. Der Ausschnitt ist so, dass die Augen in 16 px mindestens 3 px groß bleiben. Die Datei wird von Hand aus dem Original abgeleitet und liegt als Quelle in `content/static/favicon.svg` (kleiner als 3 KB).

*Alternative:* das Original unverändert verkleinern. Verworfen: bei 16 px ein verwaschener Fleck.

### 2. Rasterformate aus der SVG

Ein Skript (`openspec/changes/favicon-maskottchen/assets/make_favicons.mjs`, Playwright mit dem vorhandenen Chromium) rendert `favicon.svg` bei 16, 32, 48 und 180 px mit transparentem Hintergrund (180 px auf Hintergrundfläche in Lavendel `#f6f3fc`, damit iOS keine schwarze Fläche setzt) und baut mit ImageMagick `favicon.ico` aus den drei kleinen Größen (PNG-komprimiert) sowie `apple-touch-icon.png`. Die Ergebnisse werden eingecheckt; das Skript läuft nicht im Build. Der Build bleibt ohne neue Abhängigkeit.

### 3. Einbindung

```html
<link rel="icon" href="/favicon.ico" sizes="48x48">
<link rel="icon" href="/favicon.svg" type="image/svg+xml" sizes="any">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
```

Reihenfolge nach der gängigen Empfehlung: `.ico` zuerst mit `sizes`, damit Chromium die SVG wählt und Safari/ältere die `.ico`. Ohne Hash-Parameter (Favicons werden von Browsern ohnehin eigenständig gecacht; eine Änderung wird beim nächsten Besuch sichtbar).

### 4. Prüfung

Rust-Test: Die drei Dateien werden ausgeliefert, `favicon.ico` ist eine gültige ICO-Datei mit 16-, 32- und 48-px-Bild (Kopf lesen), `favicon.svg` ist wohlgeformtes XML mit `viewBox`, Dateigrößen unter 10 KB (SVG) bzw. 40 KB (apple). Browser-Test: Die Seiten verlinken die drei Icons, jede Adresse liefert `200` und den richtigen Medientyp; Screenshot des Icons bei 16 px auf hellem und dunklem Grund zur Abnahme.

## Risks / Trade-offs

- [Vereinfachung gefällt nicht oder wirkt zu fremd] → Abnahme mit Vorher/Nachher-Bild bei 16, 32 und 180 px; die SVG lässt sich leicht anpassen.
- [Browser cachen das alte Favicon hartnäckig] → neue SVG-Adresse hilft Chromium und Firefox; für `.ico` bleibt es beim Browser-Cache (bekannt, unkritisch).
- [Transparenz im 180-px-Icon] → eigene Hintergrundfläche für apple-touch-icon.
