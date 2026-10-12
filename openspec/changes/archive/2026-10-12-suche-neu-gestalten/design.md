## Context

Die Suchseite (`templates/search.html`) enthält ein `<main class="topic search-page">` mit Überschrift „Suche“, einem großen Formular (`#search-form`, no-JS-Fallback), einem Container `#search` für die Pagefind-Oberfläche und einem Hinweis für Besucher ohne JavaScript. `assets/js/search.js` startet `PagefindUI`, blendet das Formular aus (`form.hidden = true`), liest `?q=` und hält die Adresse aktuell. Gemessen am Stand mit Topbar (Desktop, 1440 px): Seitentext 20,8 px; Treffertitel 16,8 px, Ausschnitt, Filter und Meldung 12,8 px, Kategorie-Etikett 11,2 px, alles in Systemschrift; Hauptspalte 1000 px; Filterfeld 243 px breit mit eckigem grauem Rahmen links neben den Treffern; Vorschaubilder quadratisch in unterschiedlichen Größen; Löschen-Knopf mit schwarzem Rahmen. Ein Screenshot des Zustands liegt in `screenshots/vorher.png`.

## Goals / Non-Goals

**Goals:** ein Suchfeld im Inhalt; Schrift und Größe wie der Rest der Seite; Treffer in der Hauptspalte, der Filter als echte Seitenleiste (`aside` des Seitenrahmens wie Archiv und Tag-Cloud, nicht Teil des `main`); Bedienelemente im Seitenstil.
**Non-Goals:** Suchlogik, Index, Filterinhalte, Dunkelmodus (Farben aus Variablen).

## Decisions

### 1. Modulare Pagefind-Oberfläche statt Überschreiben der fertigen Oberfläche

*Geändert beim Umsetzen:* Die fertige Oberfläche (`PagefindUI`) rendert Eingabe, Filter und Treffer in einem einzigen Element; eine Seitenleiste außerhalb von `main` ließe sich nur durch Verschieben ihrer Knoten erzwingen (Svelte hält Verweise darauf, das ist brüchig), und ihr Aussehen hängt an Klassen mit Hashes. Stattdessen nutzt die Seite die modulare Oberfläche von Pagefind (`pagefind-modular-ui.js`, gleiche Index-Dateien, wird vom Build mit ausgeliefert; die fertige Oberfläche und ihr CSS entfallen aus dem Auslieferungsumfang). Sie liefert Suche, Filter und Ereignisse (`search`, `loading`, `results`, `filters`) und rendert nur das, was die Seite ihr vorgibt. Damit sind Schrift (Lora), Größen, Farben und Aufbau ganz die der Seite, ohne Variablenüberschreibung.

*Alternative:* `PagefindUI` behalten und den Filterbereich per DOM verschieben. Verworfen wegen der Brüchigkeit.

### 2. Ein Suchfeld

Das große Formular wird gelöscht. Für Besucher ohne JavaScript bleibt der Hinweis mit Links auf `/themen/` und `/beitraege/` (das Suchfeld der Topbar bleibt sichtbar und schickt ein GET nach `/suche/?q=`; es zeigt dann den Hinweis). Das Feld (`.searchbox` mit konzentrischer Lupe wie in der Topbar) steht im Kopf der Seite und ist ohne JavaScript verborgen; `search.js` zeigt es, fokussiert es, wenn kein `q` gesetzt ist, und füllt es sonst. Enter und Lupe lösen die Suche aus, Tippen mit 250 ms Verzögerung. Die Überschrift „Suche“ bleibt (h1, Lobster).

### 3. Seitenaufbau wie Archiv und Cloud

Die Suchseite bekommt dieselbe Body-Klasse wie die Seiten mit Karte (`has-aside`, Rahmen 1200 px). Der Filter gehört nicht in den Hauptinhalt, sondern ist wie das Archiv und die Tag-Cloud eine echte Seitenleiste des Seitenrahmens: `templates/search.html` füllt den `aside`-Block des Layouts mit `<aside class="filters card" id="search-filters">` neben `.page-content` (nicht innerhalb von `<main>`). Titel, Suchfeld und Trefferzahl stehen im Kopfbereich (`header`-Block), die Treffer als `ul.entries` im `main`. `search.js` füllt die Filterliste aus dem Ereignis `filters` (Themen mit Treffern, alphabetisch, Häkchen und Anzahl; mehrere Häkchen schränken weiter ein) und blendet die Karte aus, solange es keine Treffer gibt. Die Karte nutzt dieselbe Gitterspalte, Breite, `position: sticky` und Kartengestalt wie das Archiv; die Body-Klasse `has-aside aside-early` stellt sie unter 64 em zwischen Kopf (Suchfeld) und Treffer. Dort ist sie zunächst zugeklappt (`details`), am Desktop offen, damit lange Themenlisten die Treffer nicht verdrängen. Treffer: Treffer verwenden das Markup der Beitragslisten (`li.entry` mit `entry-image`, `entry-title`, `excerpt`): Trennlinie gestrichelt, Vorschaubild rund und 5 em groß, Titel kursiv in Linkfarbe, Ausschnitt in normaler Textfarbe, Suchwort mit `mark` in `--accent-soft` und fetter Schrift (ohne grelles Gelb) (Treffer tragen kein Kategorie-Etikett mehr; Kategorien gibt es nicht mehr, der Filter heißt „Thema“).

### 4. Tests

Playwright: genau ein sichtbares Suchfeld im Inhalt plus das Feld der Topbar; berechnete Schriftgrößen (Titel, Ausschnitt, Filter mindestens 16 px, Etikett mindestens 14 px), Schriftfamilie Lora; Hauptspalte breiter als 1000 px bei 1440; Filterkarte rechts neben den Treffern bei 1440 und über den Treffern bei 360; runde Bilder; kein waagerechtes Scrollen; bestehende Such- und Filtertests bleiben grün. Snapshot der Suchseite im Rust-Test.

## Risks / Trade-offs

- [Die modulare Oberfläche hat in 1.5.2 nur Eingabe, Trefferliste, Zusammenfassung und Filter-Pillen; Texte sind englisch] → Die Seite rendert Treffer, Meldungen und Filter selbst auf Deutsch und nutzt nur Instanz und Ereignisse.
- [Änderung der Dateien unter `/pagefind/`] → Der Cache-Schlüssel des Suchindex wurde angehoben, damit alte Zwischenstände nicht ohne `pagefind-modular-ui.js` ausgeliefert werden.
