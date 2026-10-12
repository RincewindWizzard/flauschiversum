## Why

Von den 194 Beiträgen hatten nur 160 eine Kategorie, und die Kategorien trennten Technik statt Art des Beitrags. Beim Neuordnen zeigte sich, dass Kategorien (sie müssten sich gegenseitig ausschließen) und Schlagworte sich nicht sauber trennen lassen: Es gibt immer Überschneidungen. Deshalb gibt es **keine Kategorien mehr**; alle Themen laufen über Schlagworte (siehe Änderung `topbar-navigation`, die Kategorien, ihre URLs und das Menü entfernt). Dieser Change behält, was davon bleibt: die Neuordnung der Schlagworte für alle 194 Beiträge und die Auswahl ähnlicher Beiträge, die jetzt allein über gemeinsame Schlagworte läuft.

## What Changes

- **Schlagworte statt Kategorien:** Jeder Beitrag trägt mindestens ein passendes Schlagwort; der frühere Kategoriewert (und die neu abgeleitete Zuordnung aus `zuordnung.md`) steht als Schlagwort in `tags`. Neue Schlagworte u. a. Filzen, Leder, Holz, Bücher, Linktipps. Das Feld `category` ist aus allen Beiträgen entfernt.
- **Ähnliche Beiträge:** Jede Beitragsseite zeigt bis zu 6 ähnliche Beiträge, bestimmt über gemeinsame Schlagworte (seltene zählen mehr). Es gibt kein Auffüllen und keinen Bonus für Kategorien; ein Beitrag ohne gemeinsame Schlagworte hat keine Karte.
- **Stabile URLs:** Beitrags-URLs ändern sich nicht; die alten `/kategorie/<name>/`-Adressen bleiben als Weiterleitungen (in `topbar-navigation`).

## Capabilities

### New Capabilities
- `related-posts`: Auswahl und Anzeige ähnlicher Beiträge über gemeinsame Schlagworte.

### Modified Capabilities
- `post-page`: Abschnitt „Ähnliche Beiträge“ (die Einordnung über Schlagworte regelt `topbar-navigation`).

## Impact

- Frontmatter von `content/posts/*/index.md` (`category` entfernt, Schlagworte ergänzt), `content/tags.yaml`.
- `src/topics.rs` (Ähnlichkeit), `src/render.rs`, `templates/post.html`, CSS, Tests.
- Abhängigkeit: `topbar-navigation` entfernt Kategorien aus Code, Specs, Menü und URLs; beide Changes ändern verschiedene Anforderungen und lassen sich unabhängig archivieren.
