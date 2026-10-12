## Context

Die Suchseite (`templates/search.html`) enthält ein `<main class="topic search-page">` mit Überschrift „Suche“, einem großen Formular (`#search-form`, no-JS-Fallback), einem Container `#search` für die Pagefind-Oberfläche und einem Hinweis für Besucher ohne JavaScript. `assets/js/search.js` startet `PagefindUI`, blendet das Formular aus (`form.hidden = true`), liest `?q=` und hält die Adresse aktuell. Gemessen am Stand mit Topbar (Desktop, 1440 px): Seitentext 20,8 px; Treffertitel 16,8 px, Ausschnitt, Filter und Meldung 12,8 px, Kategorie-Etikett 11,2 px, alles in Systemschrift; Hauptspalte 1000 px; Filterfeld 243 px breit mit eckigem grauem Rahmen links neben den Treffern; Vorschaubilder quadratisch in unterschiedlichen Größen; Löschen-Knopf mit schwarzem Rahmen. Ein Screenshot des Zustands liegt in `screenshots/vorher.png`.

## Goals / Non-Goals

**Goals:** ein Suchfeld im Inhalt; Schrift und Größe wie der Rest der Seite; Treffer in der Hauptspalte, der Filter als echte Seitenleiste (`aside` des Seitenrahmens wie Archiv und Tag-Cloud, nicht Teil des `main`); Bedienelemente im Seitenstil.
**Non-Goals:** Suchlogik, Index, Filterinhalte, Dunkelmodus (Farben aus Variablen).

## Decisions

### 1. Ursache beheben statt überschreiben

Pagefind definiert seine Variablen auf `.pagefind-ui`; die Werte auf einem Elternelement greifen deshalb nicht. Die Seite setzt sie künftig auf `.search-page .pagefind-ui` (höhere Spezifität): `--pagefind-ui-font: var(--font-text)`, `--pagefind-ui-scale: 1.25` (skaliert Abstände, Schrift, Bilder der Oberfläche von 0,8 auf 1,25, also rund das 1,56-Fache: Ausschnitt 12,8 → 20 px bei Desktop-Schrift, wird gegebenenfalls auf 1,1 bis 1,2 justiert, Ziel Ausschnitt etwa 17 bis 18 px), `--pagefind-ui-primary`, `--pagefind-ui-text`, `--pagefind-ui-background`, `--pagefind-ui-border`, `--pagefind-ui-tag` aus den Seitenvariablen, `--pagefind-ui-image-border-radius: 50%`. Zusätzliche Regeln nur dort, wo die Variablen nicht reichen (Etiketten, Löschen-Knopf, Mehr-laden-Knopf, Filter-Häkchen, `mark`).

*Alternative:* Pagefind-CSS nicht laden und alles selbst stylen: Hoher Aufwand, bricht bei Updates. Verworfen.

### 2. Ein Suchfeld

Das große Formular wird gelöscht. Für Besucher ohne JavaScript bleibt der Hinweis mit Links auf `/themen/` und `/beitraege/` (das Suchfeld der Topbar bleibt sichtbar und schickt ein GET nach `/suche/?q=`; es zeigt dann den Hinweis). `search.js` startet die Oberfläche ohne Formular, fokussiert das Feld, wenn kein `q` gesetzt ist, und füllt es sonst. Das Feld der Oberfläche bekommt Rundung, Lora, Schrift 1,1 em, den Platzhalter „z. B. Mütze, Katze, Filz …“ und den Lösch-Knopf als schlichten Pill-Knopf in Akzent-Tönung. Die Überschrift „Suche“ bleibt (h1, Lobster).

### 3. Seitenaufbau wie Archiv und Cloud

Die Suchseite bekommt dieselbe Body-Klasse wie die Seiten mit Karte (`has-aside`, Rahmen 1200 px). Der Filter gehört nicht in den Hauptinhalt, sondern ist wie das Archiv und die Tag-Cloud eine echte Seitenleiste des Seitenrahmens: `templates/search.html` füllt den `aside`-Block des Layouts mit `<aside class="card filters" id="search-filters">` neben `.page-content` (nicht innerhalb von `<main>`). Die Oberfläche selbst rendert Pagefind als ein Formular mit Eingabefeld, einem `drawer` und darin Filter und Treffer; `search.js` verschiebt nach dem Start den Filterbereich (`.pagefind-ui__filter-panel`) per DOM in `#search-filters` (Pagefind bindet seine Ereignisse am Element, das Verschieben ändert nichts am Verhalten) und blendet die Karte aus, solange es keine Filter gibt. Die Karte nutzt dieselbe Gitterspalte, Breite, `position: sticky` und Kartengestalt (`.card`: heller Hintergrund, 14 px Rundung, Schatten, Innenabstand) wie das Archiv. Unter 64 em: eine Spalte, Filterkarte zwischen Suchfeld und Treffern, damit die Filter nicht am Ende der Liste versteckt sind (die Reihenfolge im Quelltext bleibt Inhalt, dann Seitenleiste; die Anzeige ordnet per `order` um). Treffer: Trennlinie gestrichelt wie bei Beitragslisten, Vorschaubild rund und 5 em groß wie bei `entry`, Titel kursiv in Linkfarbe, Ausschnitt in normaler Textfarbe, Suchwort mit `mark` in `--accent-soft` und fetter Schrift (ohne grelles Gelb) (Treffer tragen kein Kategorie-Etikett mehr; Kategorien gibt es nicht mehr, der Filter heißt „Thema“).

### 4. Tests

Playwright: genau ein sichtbares Suchfeld im Inhalt plus das Feld der Topbar; berechnete Schriftgrößen (Titel, Ausschnitt, Filter mindestens 16 px, Etikett mindestens 14 px), Schriftfamilie Lora; Hauptspalte breiter als 1000 px bei 1440; Filterkarte rechts neben den Treffern bei 1440 und über den Treffern bei 360; runde Bilder; kein waagerechtes Scrollen; bestehende Such- und Filtertests bleiben grün. Snapshot der Suchseite im Rust-Test.

## Risks / Trade-offs

- [Pagefind-Klassen enthalten Hashes (`svelte-xxxx`) und können sich bei einem Update ändern] → Selektoren nur über die stabilen `.pagefind-ui__*`-Klassen, Browser-Test schlägt bei Abweichung an.
- [Gitter der Oberfläche ist nicht vorgesehen] → Das Gitter hängt am `drawer`; fällt es bei einem Update weg, bleibt die Seite einspaltig und lesbar.
- [Skalierung 1,25 verändert Abstände überall] → Feinabstimmung im Screenshot-Vergleich, Abnahme an Staging.
