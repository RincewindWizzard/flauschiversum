## Why

Das Favicon ist noch ein winziges 16×16-Icon mit 16 Farben aus dem alten Auftritt. Die Seite hat inzwischen ein Gesicht, das Maskottchen Flauschi in der Topbar, und das Favicon soll es aufgreifen, damit Tabs, Lesezeichen und der Startbildschirm auf dem Handy zur Seite passen.

## What Changes

- **Favicon aus dem Maskottchen:** Das normale Flauschi (nicht die überraschte Variante) ist künftig das Favicon. Weil das Maskottchen viele feine Fransen, Verläufe und kleine Details hat, die bei 16 bis 32 px verschwimmen, gibt es dafür eine vereinfachte Variante: kräftiger violett-indigoner Körper mit Verlauf wie im Original, zwei große Augen mit Pupillen, ohne Fransen, Mund und Kleindetails, auf transparentem Grund.
- **Mehrere Formate:** `favicon.svg` (modernes Format, skaliert scharf), `favicon.ico` mit 16, 32 und 48 px (bleibt unter `/favicon.ico` erreichbar, Cool URIs und Browser, die es ungefragt abrufen) und `apple-touch-icon.png` (180 px, mit Hintergrundfläche, weil iOS keine Transparenz mag). Die Seiten verlinken alle drei.
- **Helle und dunkle Tab-Leisten:** Das Icon ist auf hellem und dunklem Grund erkennbar (Körper mittelhell mit hellem Rand der Augen), ohne eigenes Dark-Mode-Umschalten.
- **Nicht Teil dieses Changes:** Änderungen am Maskottchen in der Topbar, Web-App-Manifest und größere Icons (192/512 px).

## Capabilities

### New Capabilities

- `favicon`: Gestalt, Dateien und Einbindung des Favicons aus dem Maskottchen.

### Modified Capabilities

- `site-layout`: Grundgerüst jeder Seite bindet die Icon-Dateien ein.
- `url-scheme`: Die Icon-Dateien sind unter festen Adressen erreichbar.

## Impact

- `content/static/favicon.svg`, `favicon.ico` (neu), `apple-touch-icon.png` (neu), `templates/layout.html` (drei `link`-Zeilen), Skript zum Erzeugen im Change, Snapshots, ein Rust-Test und ein Browser-Test, README.
- Die Adresse `/favicon.ico` bleibt; der Dateiinhalt ändert sich.
