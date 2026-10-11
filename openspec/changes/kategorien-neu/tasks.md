## 1. Taxonomie und Inhalte

- [ ] 1.1 `zuordnung.md` mit der Autorin abstimmen (Kategorien, Zuordnung, neue Schlagworte); prüfen: Freigabe liegt vor
- [ ] 1.2 `content/categories.yaml` auf die zehn Kategorien umstellen (Wolle zuerst, `filzen` entfernt, neue ids `papier`, `schmuck`, `feste`, `material`, `sonstiges`) mit `image` aus vorhandenen Beitragsbildern und Entwurf-`motto`; prüfen: `check` meldet keine fehlenden Bilder
- [ ] 1.3 `content/tags.yaml` um Filzen, Leder, Holz, Bücher, Linktipps ergänzen; prüfen: Schlagworte alphabetisch, `check` ohne Fehler
- [ ] 1.4 Einmaliges Skript schreibt `category` und fehlende `tags` in das Frontmatter aller betroffenen Posts laut `zuordnung.md`; prüfen: `git diff` ändert nur Frontmatter, 194 Beiträge haben eine Kategorie, Beitrags-URLs unverändert

## 2. Pflichtfeld

- [ ] 2.1 `category` in `src/content.rs` zum Pflichtfeld machen und Code für „ohne Kategorie“ entfernen; prüfen: Unit-Test „Post ohne Kategorie“ bricht mit Pfad und Feld ab, Snapshots angepasst

## 3. Ähnliche Beiträge

- [ ] 3.1 Berechnung in `src/topics.rs` (Wert, Auffüllen, Sortierung, Maximum 6) mit Unit-Tests für alle Szenarien aus `related-posts`; prüfen: `cargo test`
- [ ] 3.2 Karte „Ähnliche Beiträge“ in `templates/post.html` und CSS; prüfen: Browser-Test auf einem Filz-Beitrag, Karte zeigt Titel und gemeinsame Schlagworte, nicht auf Seiten ohne Treffer

## 4. Abschluss

- [ ] 4.1 Vollständigen Build und `check` laufen lassen, Browser-Tests und Screenshots (Kategorie „Wolle“, Beitrag mit ähnlichen Beiträgen) der Autorin vorlegen; prüfen: Abnahme
- [ ] 4.2 README aktualisieren (Kategorien, Pflichtfeld, Ähnliche Beiträge); prüfen: Doku nennt die zehn Kategorien
