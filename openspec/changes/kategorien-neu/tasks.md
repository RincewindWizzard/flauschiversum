## 1. Schlagworte

- [x] 1.1 `zuordnung.md` mit der Autorin abstimmen; prüfen: Freigabe liegt vor
- [x] 1.2 `content/tags.yaml` um Filzen, Leder, Holz, Bücher, Linktipps und die früheren Kategorienamen ergänzen; prüfen: Schlagworte alphabetisch, `check` ohne Fehler
- [x] 1.3 Einmaliges Skript entfernt `category` aus dem Frontmatter aller Posts und ergänzt das zugeordnete Schlagwort; prüfen: `git diff` ändert nur Frontmatter, Beitrags-URLs unverändert

## 2. Ähnliche Beiträge

- [x] 2.1 Berechnung in `src/topics.rs` (Wert, Sortierung, Maximum 6) mit Unit-Tests für alle Szenarien aus `related-posts`; prüfen: `cargo test`
- [x] 2.2 Karte „Ähnliche Beiträge“ in `templates/post.html` und CSS; prüfen: Browser-Test auf einem Filz-Beitrag, Karte zeigt Titel und gemeinsame Schlagworte, nicht auf Seiten ohne Treffer

## 3. Abschluss

- [ ] 3.1 Vollständigen Build und `check` laufen lassen, Browser-Tests und Screenshots (Beitrag mit ähnlichen Beiträgen) der Autorin vorlegen; prüfen: Abnahme
- [x] 3.2 README aktualisieren (Schlagworte, Ähnliche Beiträge); prüfen: Doku nennt keine Kategorien
