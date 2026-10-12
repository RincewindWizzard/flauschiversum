## 1. Beitragskopf und Ende

- [ ] 1.1 Datum unter dem Titel in `templates/post.html` (`<time datetime>`, Daten aus `render.rs`) und Gestaltung (`header.css`: mittig, Fließtextgröße, `--muted`); prüfen: Rust-Snapshot der Beitragsseite, Browser-Test: Datum steht zwischen Titel und Text, Schriftgröße mindestens Fließtext, Kontrast mindestens 4,5 : 1 in Hell und Dunkel
- [ ] 1.2 Absatz „Schreibe Ihr eine Nachricht!“ aus dem Kopf nach Text und Autorenzeile, vor „Ähnliche Beiträge“ verschieben; prüfen: Browser-Test: Hinweis steht unter dem letzten Absatz, nicht im Kopf, `mailto:` mit Betreff „[Flauschiversum] <Titel>“ unverändert, Test `layout.mjs` zu mailto grün

## 2. Ähnliche Beiträge

- [ ] 2.1 `RelatedLink` durch die Entry-Daten ersetzen (Bild, Titel, Datum, Kurztext), Karte mit `ul.entries` und `partials/entry.html`, Chips entfernen, `related.css` auf die Karte reduzieren; prüfen: Rust-Test (Einträge haben Bild, Datum, Kurztext), Browser-Test: Eintrag in der Karte hat dieselben Maße und Schriften wie ein Eintrag auf der Startseite (Bildgröße 5 em, Titel kursiv, Datum, Kurztext)

## 3. Wasserzeichen

- [ ] 3.1 Neues `assets/badge.png` (800 × 133, Lavendel-Fläche, Indigo-Schrift in Lobster, Akzentrand) mit einem Skript aus `content/static/fonts/Lobster.otf` erzeugen und Skript im Change ablegen; prüfen: Pixeltest: keine Grün-/Graugrüntöne, Kontrast Schrift/Fläche mindestens 7 : 1, Größe unverändert
- [ ] 3.2 Tests der Bildverarbeitung (`media`) und Snapshots anpassen, Build komplett neu berechnen lassen; prüfen: `cargo test --release -- --include-ignored` grün, Vergleichsbild eines hellen und eines dunklen Fotos (700x) zur Abnahme

## 4. Absicherung und Doku

- [ ] 4.1 Browser-Tests (`layout.mjs`, `navigation.mjs`, `frontend.mjs`, `theme.mjs`) um Datum, Hinweis am Ende und Listenaufbau der ähnlichen Beiträge ergänzen; prüfen: alle Skripte grün
- [ ] 4.2 README (Beitragsseite, Wasserzeichen) aktualisieren; Vergleichsbilder (Beitrag oben/unten, Hell und Dunkel, Handy) zur Abnahme; prüfen: Freigabe im Gespräch

## Workflow follow-up

- Nach der Abnahme `/opsx:archive post-ansicht-ueberarbeiten` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
