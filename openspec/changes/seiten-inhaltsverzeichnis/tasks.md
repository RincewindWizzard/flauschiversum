## 1. Überschriften und Verzeichnisdaten

- [ ] 1.1 `markdown::render_page` liefert zusätzlich die Überschriften (`TocEntry { level, id, text }`) und setzt `id` an jede Überschrift (`urls::slugify`, Doppelte nummeriert, Kollision mit festen Seitenrahmen-IDs vermeiden); prüfen: Unit-Tests für Text mit Umlaut und Sonderzeichen („Ufo's“), doppelte Überschriften, Code und Auszeichnung in der Überschrift, `##Titel` ohne Leerzeichen, Überschrift in Codeblock zählt nicht
- [ ] 1.2 Verschachtelung nach relativer Ebene (höchstens drei Stufen) als Datenstruktur für das Template; prüfen: Unit-Tests für gemischte Ebenen (`#` und `##`), nur Ebene 2, drei und vier Ebenen, weniger als zwei Überschriften ergibt kein Verzeichnis

## 2. Seite und Gestaltung

- [ ] 2.1 `templates/page.html`: Karte „Inhalt“ im `aside`-Block mit `details`/`nav`/verschachtelter Liste, Body-Klasse `has-aside aside-early` nur mit Verzeichnis, `src/render.rs`/`src/site.rs` geben die Daten weiter; prüfen: Rust-Test auf allen Seiten (Karte nur bei mindestens zwei Überschriften, jedes `id` einmalig, jeder Verzeichnislink zeigt auf ein vorhandenes `id`), Snapshot einer Seite mit Verzeichnis
- [ ] 2.2 `content/style/toc.css` (Karte, Einrückung je Stufe, Hervorhebung, Fokus, `scroll-margin-top` für Überschriften) und `assets/js/toc.js` (Aufklappen nach Fensterbreite, `aria-current` per `IntersectionObserver`), Skript nur auf Seiten mit Verzeichnis; prüfen: Playwright bei 1440 und 360 px: Karte rechts und sticky bzw. oben und zugeklappt, Klick auf einen Link bringt die Überschrift unter die Topbar, Hervorhebung wechselt beim Scrollen, ohne JavaScript alle Links sichtbar, kein waagerechtes Scrollen

## 3. Absicherung und Doku

- [ ] 3.1 Browser-Tests (`layout.mjs`, `navigation.mjs`, `theme.mjs`: Kontrast der Karte in Hell und Dunkel) ergänzen, Snapshots der Seiten aktualisieren; prüfen: `cargo test --release -- --include-ignored` und alle Playwright-Skripte grün
- [ ] 3.2 README (Einzelseiten, Inhaltsverzeichnis, Anker) aktualisieren; Vergleichsbilder (`/tipps/`, `/autoren/`, Impressum; Desktop, Handy, Hell und Dunkel) zur Abnahme; prüfen: Freigabe im Gespräch

## Workflow follow-up

- Nach der Abnahme `/opsx:archive seiten-inhaltsverzeichnis` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
