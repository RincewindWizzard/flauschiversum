## Context

Kategorien liegen in `content/categories.yaml` (id, name, image, motto), Schlagworte in `content/tags.yaml`. `src/topics.rs` kennt Zählungen und die Cloud. Die Zuordnung der Beiträge wurde aus Titel, bisheriger Kategorie und Schlagworten abgeleitet (`zuordnung.md`).

## Goals / Non-Goals

**Goals:** jeder Beitrag in genau einer inhaltlich passenden Kategorie; Schlagworte für Feinunterscheidung; deterministische „Ähnliche Beiträge“.
**Non-Goals:** neue Beitragsinhalte, Änderungen an Beitrags-URLs, mehrere Kategorien pro Beitrag.

## Decisions

### 1. Taxonomie
| id | Name | Beiträge |
|----|------|----------|
| wolle | Wolle | 87 |
| nähen | Nähen & Quilten | 26 |
| basteln | Basteln & Upcycling | 31 |
| brandings | Brandings & Leder | 14 |
| malen | Malen & Zeichnen | 5 |
| papier | Papier & Bücher | 5 |
| schmuck | Schmuck & Perlen | 5 |
| feste | Feste & Jahreszeiten | 7 |
| material | Material & Einkaufstipps | 7 |
| sonstiges | Sonstiges | 7 |

Bestehende ids bleiben stabil, damit Kategorie-URLs nicht brechen; Anzeigenamen werden präzisiert. Wolle bleibt als Sammelkategorie (Spinnen, Weben, Stricken, Häkeln, Filzen) wie von der Autorin gewünscht; „Filzen“ wird Schlagwort (38 Beiträge), `/kategorie/filzen/` entfällt (nur auf Staging vorhanden). Auf `/tag/filzen/` steht dieselbe Auswahl als Schlagwortseite.

### 2. Zuordnung per Skript
Ein einmaliges Skript schreibt `category` und fehlende `tags` in das Frontmatter anhand der geprüften `zuordnung.md`-Daten, ohne den restlichen Text zu verändern (zeilenweiser Eingriff im Frontmatter). Danach läuft `check`.

### 3. Pflichtfeld
`Post::category` wird beim Laden zum Pflichtfeld; fehlt es, Fehler „Feld `category` fehlt“ mit Pfad. Der Code für „ohne Kategorie“ (Einordnung, Zählung) entfällt.

### 4. Ähnliche Beiträge
Für einen Beitrag P und jeden anderen veröffentlichten Beitrag Q: `score = Σ_{t ∈ Tags(P)∩Tags(Q)} 1/count(t)` + 0,25 bei gleicher Kategorie. Q mit Score 0 werden ausgelassen; bleiben weniger als 6 Treffer, wird mit den neuesten Beiträgen derselben Kategorie aufgefüllt. Sortierung: Score absteigend, Datum absteigend, URL aufsteigend. Anzeige als Karte „Ähnliche Beiträge“ nach dem Autor und vor der Navigation zum älteren/neueren Beitrag: Titel als Link, darunter die gemeinsamen Schlagworte (ohne Fallback-Treffer). Berechnung zur Build-Zeit in `topics.rs`; deterministisch.

### 5. Kategoriebilder und -mottos
Neue Kategorien erhalten `image` aus einem vorhandenen Beitragsbild (verkleinert über die Bildpipeline bzw. als statische Datei) und ein `motto` als Entwurf zur Prüfung durch die Autorin.

## Risks / Trade-offs

- [Falsche Zuordnung einzelner Beiträge] → Autorin prüft `zuordnung.md` vor Apply; Änderung später per Frontmatter trivial.
- [Wolle sehr groß (87)] → Schlagworte (Filzen, Stricken, Häkeln, …) und Archiv gliedern; ähnliche Beiträge helfen beim Finden.
- [Konflikt mit `topbar-navigation`] → beide lesen `categories.yaml` dynamisch; Reihenfolge der Datei bestimmt Menü, Wolle zuerst.
