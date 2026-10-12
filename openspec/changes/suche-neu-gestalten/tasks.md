## 1. Seite und Skript

- [x] 1.1 Großes Formular aus `templates/search.html` entfernen, Hinweis für Besucher ohne JavaScript im `noscript` behalten, Body-Klasse `has-aside` und `aside`-Block mit `#search-filters` für die Suchseite (`templates/search.html`, `src/render.rs`), `assets/js/search.js` ohne Formular (Fokus ohne `q`, Feld füllen mit `q`, Adresse aktuell halten); prüfen: Rust-Test (kein `id="search-form"`, Body-Klasse), Playwright: genau ein sichtbares Suchfeld im Inhalt, Topbar-Suchfeld bleibt, ohne JavaScript Hinweis und Links vorhanden

## 2. Gestaltung

- [x] 2.1 Modulare Pagefind-Oberfläche statt der fertigen verwenden (`pagefind-modular-ui.js` ausliefern, Cache-Schlüssel anheben), Stile der Seite in `search.css` (Schrift Lora, Farben aus Seitenvariablen) und Reste der alten Oberfläche aus `topics.css` entfernen; prüfen: berechnete Schriftgrößen und -familie im Browser-Test (Titel, Ausschnitt, Filter mindestens 16 px, Etikett mindestens 14 px, Lora)
- [x] 2.2 Treffer im Stil der Beitragslisten (gestrichelte Trenner, runde Vorschaubilder 5 em, Titel kursiv in Linkfarbe, `mark` in `--accent-soft` fett, Etiketten als ruhige Chips), Löschen-, Mehr-laden- und Filter-Bedienelemente im Seitenstil, Fokusrahmen sichtbar; prüfen: Screenshot-Vergleich mit `screenshots/vorher.png`, Kontrast von Text, Link und Hervorhebung mindestens 4,5 : 1
- [x] 2.3 Seitenaufbau: Themenfilter aus dem Ereignis `filters` in die Seitenleiste `aside.card#search-filters` rendern (außerhalb von `main`, Gitterspalte und `sticky` wie das Archiv), unter 64 em eine Spalte mit Filterkarte oberhalb der Treffer; prüfen: Playwright bei 1440, 1100 und 360 px: Filter liegt in `aside` und nicht in `main`, Hauptspalte breiter als 1000 px, Filterkarte rechts bzw. oben, Filter bleibt bedienbar, kein waagerechtes Scrollen

## 3. Absicherung

- [x] 3.1 Bestehende Suchtests (Treffer, Hervorhebung, Umlaute, Themenfilter, Adresse aktualisieren) anpassen und Snapshot der Suchseite aktualisieren; prüfen: `cargo test` und `node tests/browser/navigation.mjs` grün
- [x] 3.2 Vergleichsbilder (vorher/nachher, 1440 und 360 px, mit Filter) zur Abnahme; prüfen: Freigabe im Gespräch am Staging

## 4. Dokumentation

- [x] 4.1 README (Suchseite) aktualisieren; prüfen: Beschreibung stimmt mit dem Code überein

## Workflow follow-up

- Nach der Abnahme `/opsx:archive suche-neu-gestalten` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
