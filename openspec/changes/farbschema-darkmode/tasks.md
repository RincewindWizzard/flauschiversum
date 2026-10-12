## 1. Entscheidung

- [x] 1.1 Themenfamilie gewählt: Flauschi-Nebel; prüfen: Entscheidung steht in `proposal.md` und `design.md` (Feinheiten wie Sternendichte und Akzent können bei der Abnahme der Screenshots angepasst werden)

## 2. Tokens

- [x] 2.1 `content/style/theme.css` auf Tokens beider Modi umstellen (`:root`, `prefers-color-scheme: dark`, `data-theme`), `color-scheme` setzen, Tönungen per `color-mix()`; prüfen: Seite sieht im Hellmodus fast wie heute (neue Farben der Familie) aus, `cargo test` grün
- [x] 2.2 Feste Hex-Farben aus allen anderen Stylesheets entfernen (Tag-Cloud, Hover, Chips, Archiv, Karten, Suche, Footer, Medien, Seiten; Staging-Leiste bleibt); prüfen: `grep` nach Hex außerhalb von `theme.css` findet nur die Staging-Leiste, ein Test sichert das ab

## 3. Dunkelmodus

- [x] 3.1 Umschalter (Knopf mit Sonne/Mond als inline SVG, Stufen Automatisch/Hell/Dunkel, `aria-pressed`, 44 px) in der Topbar und `assets/js/theme.js`; kleines Skript im `<head>` setzt `data-theme` vor dem Zeichnen; `localStorage` in `try/catch`; prüfen: Playwright-Test: Wahl bleibt nach Neuladen, ohne JavaScript folgt die Seite dem System, kein helles Aufblitzen (Hintergrundfarbe direkt nach `DOMContentLoaded` dunkel)
- [x] 3.2 Pagefind-, PhotoSwipe- und Medien-Farben an Tokens hängen; Maskottchen-Schein im Dunkelmodus; prüfen: Screenshot der Suche, Lightbox und Seite in beiden Modi

## 4. Universum

- [x] 4.1 `content/style/starfield.css` und zwei kleine Sternen-SVG-Kacheln unter `content/static/`: Sterne und Nebel hinter allem, im Hellmodus zart, bei `prefers-contrast: more` und `forced-colors` aus; kein Animationseffekt; prüfen: Test: Sterne liegen hinter dem Inhalt (`z-index`), fangen keine Klicks, Textflächen haben einfarbigen Hintergrund
- [x] 4.2 Sternchen-Trenner in der Fußzeile; prüfen: Screenshot

## 5. Absicherung

- [x] 5.1 `tests/browser/theme.mjs`: alle Farbpaare beider Modi (Text/Hintergrund, Link/Karte, grauer Text, Knopf, Rand, Akzent) über berechnete Stile gegen 4,5 : 1 bzw. 3 : 1 prüfen, auf mehreren Seiten (Start, Beitrag, Kategorie, Suche, Impressum); Simulation von Protanopie, Deuteranopie und Tritanopie für Link/Karte; prüfen: grün
- [x] 5.2 Bestehende Browser-Tests (Kontrast, Farben in `layout.mjs`) auf Tokens umstellen, Snapshots anpassen; prüfen: `cargo test` und alle Playwright-Skripte grün
- [ ] 5.3 Vergleichsbilder (Hell und Dunkel: Startseite, Beitrag, Kategorie, Suche, Handy) zur Abnahme; prüfen: Freigabe im Gespräch am Staging

## 6. Dokumentation

- [x] 6.1 README (Farbsystem, Umschalter, Tokens in `theme.css`, Kontrasttest) und `openspec/config.yaml` aktualisieren; prüfen: Beschreibung stimmt mit dem Code überein

## Workflow follow-up

- Nach der Abnahme `/opsx:archive farbschema-darkmode` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
