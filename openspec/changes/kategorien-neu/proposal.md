## Why

Heute haben nur 160 von 194 Beiträgen eine Kategorie, und die sechs Kategorien trennen Technik statt Art des Beitrags (Filzen und Wolle überlappen, Schmuck, Papier, Feste, Einkaufstipps und Persönliches haben kein Zuhause). Jeder Beitrag soll genau einer Kategorie angehören, und Schlagworte sollen die feinere Unterscheidung tragen. Auf Beiträgen zum Filzen sollen darüber ähnliche Beiträge zur Auswahl erscheinen.

## What Changes

- **Neue Kategorien (10):** Wolle (Spinnen, Weben, Stricken, Häkeln, Filzen), Nähen & Quilten, Basteln & Upcycling, Brandings & Leder, Malen & Zeichnen, Papier & Bücher, Schmuck & Perlen, Feste & Jahreszeiten, Material & Einkaufstipps, Sonstiges. Die Kategorie „Filzen“ entfällt; ihre Beiträge gehen nach Wolle und erhalten das Schlagwort „Filzen“. Die vollständige Zuordnung aller 194 Beiträge steht in `zuordnung.md` zur Prüfung.
- **Pflichtfeld:** `category` ist für jeden Beitrag Pflicht; fehlt sie, bricht der Build ab. Der Meta-Beitrag ohne Kategorie kommt nach „Sonstiges“.
- **Neue Schlagworte:** Filzen, Leder, Holz, Bücher, Linktipps. Bestehende Schlagworte bleiben.
- **Ähnliche Beiträge:** Jede Beitragsseite zeigt eine Auswahl von bis zu 6 ähnlichen Beiträgen, bestimmt über gemeinsame Schlagworte (seltene zählen mehr) und gleiche Kategorie.
- **Stabile URLs:** Bestehende Kategorie-URLs außer `/kategorie/filzen/` (existierte nur auf Staging) bleiben; Beitrags-URLs ändern sich nicht.
- Kategorien bekommen `image` und `motto` aus vorhandenen Beitragsbildern bzw. Entwürfen zur Prüfung.

## Capabilities

### New Capabilities
- `related-posts`: Auswahl und Anzeige ähnlicher Beiträge.

### Modified Capabilities
- `content-format`: Kategorie wird Pflichtfeld, Liste der Kategorien.
- `topic-pages`: Fall „ohne Kategorie“ entfällt.
- `post-page`: Einordnung immer mit Kategorie, Abschnitt „Ähnliche Beiträge“.
- `build`: Jeder Beitrag braucht eine Kategorie.

## Impact

- Frontmatter von bis zu 194 `content/posts/*/index.md` (Kategorie, Schlagworte), `content/categories.yaml`, `content/tags.yaml`, neue Kategoriebilder.
- `src/content.rs` (Pflichtfeld), `src/topics.rs` (Ähnlichkeit), `src/render.rs`, `templates/post.html`, CSS, Tests, README.
- Abhängigkeit: `topbar-navigation` zeigt alle Kategorien dynamisch; beide Changes ändern `categories.yaml`.
