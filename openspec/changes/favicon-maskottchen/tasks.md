## 1. Icon gestalten

- [ ] 1.1 Vereinfachtes `favicon.svg` aus `flauschi_normal.svg` ableiten (Körper mit Verlauf, zwei Augen mit Pupillen und Glanz, heller Rand, ohne Fransen und Details, `viewBox` 0 0 64 64, unter 3 KB) nach `content/static/favicon.svg`; prüfen: Vorschaubild bei 16, 32 und 180 px auf hellem und dunklem Grund liegt zur Abnahme vor, Augen bei 16 px mindestens 3 px groß
- [ ] 1.2 Skript `assets/make_favicons.mjs` im Change (Playwright/Chromium für das Rendern, ImageMagick für ICO) erzeugt `favicon.ico` (16, 32, 48 px) und `apple-touch-icon.png` (180 px auf hellem Grund) in `content/static/`; prüfen: `identify` zeigt drei Bilder in der ICO-Datei und 180 × 180 ohne transparente Pixel im PNG

## 2. Einbindung und Tests

- [ ] 2.1 `templates/layout.html`: drei Verweise (`favicon.ico` mit `sizes`, `favicon.svg`, `apple-touch-icon.png`); prüfen: Snapshots aktualisiert, Rust-Test: jede Seite verlinkt alle drei
- [ ] 2.2 Rust-Test für die Dateien (ICO-Kopf mit drei Bildern, SVG wohlgeformt mit `viewBox` und kleiner als 10 KB, PNG 180 × 180) und Browser-Test (Adressen liefern 200 mit passendem Medientyp, Verweise im Kopf); prüfen: `cargo test --release -- --include-ignored` und alle Playwright-Skripte grün

## 3. Dokumentation

- [ ] 3.1 README (Favicon, Skript zum Neuerzeugen) aktualisieren und Vergleichsbilder alt/neu zur Abnahme vorlegen; prüfen: Freigabe im Gespräch

## Workflow follow-up

- Nach der Abnahme `/opsx:archive favicon-maskottchen` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
