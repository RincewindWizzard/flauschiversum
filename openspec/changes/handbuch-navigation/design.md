# Design

## Context

Motivation und Umfang stehen in `proposal.md`, das gewünschte Verhalten in den Delta-Specs. Für die Umsetzung zählt der heutige Stand:

- Jeder Post hat schon `category` (189 von 194; sechs Werte: basteln 54, wolle 49, filzen 37, nähen 27, brandings 16, malen 6), aber kein Feld `tags`. Das Frontmatter wird in `src/content.rs` nur als Text-Felder gelesen, Listen werden verworfen.
- Der Generator rendert alle Seiten aus einer Datenstruktur `Site` (veröffentlichte Posts, Seiten, statische Dateien), parallel und in den Speicher (`Outputs`); der Writer schreibt nur Geändertes und führt ein Manifest. Neue Seitentypen und der Suchindex müssen sich in dieses Muster einfügen.
- `templates/index.html` rendert die chronologische Liste mit Paginierung; die ersten Seite liegt unter `/`. Eine auskommentierte Kategorie-Navigation mit Reimen ist der ursprüngliche Plan; die sechs Bilder (300×225 px) liegen in `content/static/images/`.
- Das Verhalten der Seite ist in `openspec/specs/` beschrieben; dieser Change ändert dort neun Capabilities und fügt drei hinzu.
- Alle bestehenden URLs müssen erhalten bleiben („Cool URIs don't change“). Hier betrifft das vor allem `/` und `/index.html`, deren Inhalt sich ändert, und `/page/<N>/`.

## Goals / Non-Goals

**Goals:**
- Jeder der 194 Beiträge ist über mindestens zwei Wege auffindbar: Kategorie und Schlagwort, dazu Suche.
- Eine Startseite, die in zwei Klicks zu einem Thema führt, ohne dass JavaScript nötig ist (außer für die Suche selbst).
- Alles weiterhin statisch, aus dem Build, ohne Server und ohne Dienste von Dritten.
- Ein Build bleibt schnell und inkrementell: ein zweiter Build ohne Änderung schreibt keine Datei, auch nicht im Suchindex.

**Non-Goals:**
- Verwandte Beiträge, Kennzeichnung „Anleitung“ gegenüber „Werkstück“, ein zurückgenommenes Datum, Jahres- oder Alphabetsarchiv, Kommentare, Mehrfach-Kategorien.
- Änderungen an den Texten der Beiträge oder an den Beitrags-URLs.
- Das neue Deployment.

## Decisions

### 1. Kategorien und Schlagworte aus festen Listen

`content/categories.yaml` und `content/tags.yaml` bilden den erlaubten Wertevorrat. Ein Beitrag nennt `category: filzen` (die `id`) und `tags: [Katzen, Hüte]` (Namen, Groß-/Kleinschreibung egal). Der Build bricht bei unbekannten Werten ab und schlägt das ähnlichste vor (Levenshtein-Abstand auf dem Slug).

```yaml
# content/categories.yaml
- id: basteln
  name: Basteln
  image: /images/Basteln.jpg
  motto: |-
    Mit Klebe und Schere bestückt,
    wird mit Liebe geschmückt.
```

*Alternative:* freie Schlagworte direkt im Frontmatter, Seiten daraus ableiten. Verworfen: Bei 194 Posts von mehreren Autorinnen entstehen sofort „Katze“, „Katzen“ und „Kätzchen“, und niemand merkt es. Die feste Liste kostet eine Zeile pro neuem Schlagwort und verhindert das.

*Mottos:* Die Reime stammen aus der alten, auskommentierten Navigation (`templates/index.html`) und werden unverändert in `categories.yaml` übernommen. Die frühere Schreibweise „nahen“ (ohne Umlaut) wird nicht übernommen; die `id` heißt wie im Frontmatter `nähen`.

### 2. Datenmodell und Themenberechnung

`Post` erhält `category: Option<String>` (die `id`) und `tags: Vec<String>` (kanonische Namen aus der Liste). `Site` erhält `categories` und `tags` (Listeneinträge). Neues Modul `src/topics.rs` berechnet aus den **veröffentlichten** Posts die Zuordnungen, Zählungen und Sortierungen (neueste zuerst; Schlagworte alphabetisch mit deutscher Sortierung, `ä` wie `a`) und liefert reine Datenstrukturen für die Templates. Dadurch ist alles ohne Dateisystem testbar, und geplante Posts können nirgends durchrutschen.

Slugs entstehen mit der vorhandenen Funktion `urls::slugify` (`ä`→`ae`). Zwei Namen mit demselben Slug sind ein Fehler. Seiten für leere Kategorien oder Schlagworte entfallen.

### 3. URLs und Startseite

| URL | Inhalt | Template |
|---|---|---|
| `/`, `/index.html` | Handbuch-Startseite | `home.html` (neu) |
| `/beitraege/` | erste Seite der chronologischen Liste | `index.html` (bestehend) |
| `/page/<N>/` | Folgeseiten der Liste (unverändert) | `index.html` |
| `/themen/` | Übersicht Kategorien und Schlagworte | `topics.html` (neu) |
| `/kategorie/<name>/` | Kategorieseite | `topic.html` (neu) |
| `/tag/<name>/` | Schlagwortseite | `topic.html` |
| `/suche/` | Suchseite | `search.html` (neu) |

`render::index_url(0)` wird `/beitraege/`; die Paginierung verweist von `/page/1/` zurück dorthin. Reservierte Pfade (`themen`, `suche`, `beitraege`, `kategorie`, `tag`, `pagefind`) werden gegen Einzelseiten und Posts geprüft, bevor etwas geschrieben wird.

*Alternative:* die chronologische Liste auf `/` lassen und die Handbuch-Startseite unter einer neuen URL anlegen. Verworfen: Besucher und Suchmaschinen landen auf `/`; dort muss das Handbuch stehen. Die Liste verliert nichts, weil `/page/<N>/` bleibt und `/beitraege/` neu ist.

Die Seiten der Liste sind ohnehin nicht stabil (jeder neue Beitrag verschiebt alle Seiten), deshalb zählen sie nicht als schützenswerte URLs.

### 4. Aufbau der Seiten

- **Startseite:** Suchfeld (`<form action="/suche/" method="get">`, funktioniert ohne eigenes JavaScript), Kachelgitter der Kategorien (Bild 300×225, Name, Reim, Anzahl; ganze Kachel ist ein Link), 12 häufigste Schlagworte als Liste, „Neu im Flauschiversum“ mit 4 Einträgen im vorhandenen Kartenstil.
- **Kategorie- und Schlagwortseite:** Überschrift, Anzahl, bei Kategorien Reim und Schlagworte der Kategorie, danach die Beitragsliste. Alle Beiträge stehen auf **einer** Seite (größte Kategorie: 54): Besucher überfliegen und scrollen lieber, als zu blättern, und der Seitenumfang ist mit Vorschaubildern (200 px, `loading="lazy"`) unkritisch. Die Einträge sind kompakter als auf der chronologischen Liste (kleines rundes Vorschaubild links, Titel, Kurztext, Datum).
- **Beitragsseite:** In `post-header` steht unter dem Autor eine Zeile mit der Kategorie und den Schlagworten.
- **Navigation:** Start, Themen, Beiträge, Tipps und Tricks, About, Impressum, Abonnieren. Das Suchfeld sitzt im Kopf unter der Navigation (auf Start und Suche entfällt es, dort gibt es ein eigenes).

Die Templates sind Askama-Strukturen wie bisher; Listeneinträge teilen sich ein Partial `partials/entry.html`. Das Styling kommt in eine neue Datei `content/style/topics.css`, eingebunden in `main.css`.

### 5. Suche mit Pagefind als Bibliothek

Der Suchindex entsteht mit dem Rust-Crate `pagefind` (MIT, v1.5.2) als Bibliothek, nicht als externes Programm: `PagefindIndex::add_html_file` nimmt die bereits gerenderten Seiten entgegen, `build_indexes()` und `get_files()` liefern alle Dateien (Index, `pagefind.js`, WebAssembly, `pagefind-ui.js`/`.css`) als Bytes im Speicher. Diese werden wie alle anderen Ausgaben in `Outputs` eingefügt, erhalten deshalb Manifest-Eintrag, Diff und Löschung verwaister Dateien, und der Build braucht kein zusätzliches Programm.

- **Was indexiert wird:** die Seiten der veröffentlichten Posts und die Einzelseiten, in sortierter Reihenfolge (für einen reproduzierbaren Index). Die Templates markieren den Inhalt mit `data-pagefind-body`; Kategorie und Schlagworte stehen sichtbar im Beitragskopf und werden zusätzlich als `data-pagefind-filter="Kategorie"` bzw. als Suchtext im Index geführt.
- **Sprache:** `lang="de"` mit deutscher Wortstammbildung (Pagefind bringt sie mit); Treffer auf Wortanfänge (Präfixe) sind eingebaut.
- **Oberfläche:** Die mitgelieferte Pagefind-Oberfläche liefert Trefferliste, Textausschnitt mit Hervorhebung und den Kategoriefilter. Ein kleines eigenes Skript `assets/js/search.js` startet sie auf `/suche/`, mit deutschen Texten, setzt die Suche aus `?q=` und hält die Adresse aktuell. Die Dateien liegen unter `/pagefind/`, werden von der eigenen Domain geladen, und es gibt keine Anfragen an Dritte.
- **Asynchronität:** Die Bibliothek ist asynchron; ein kleiner Tokio-Runtime-Aufruf (`block_on`) in `src/search.rs` kapselt das, der Rest des Generators bleibt synchron.
- **Ohne JavaScript:** `<noscript>`-Hinweis mit Links auf Themen und Beiträge.

*Alternativen:* (a) Pagefind als Kommandozeilenprogramm nach dem Build aufrufen: braucht ein zusätzliches Programm in jeder Umgebung und umgeht Manifest und Diff. (b) Eigener kleiner Suchindex (JSON, Wortindex, ca. 150 Zeilen JS): volle Kontrolle, aber Wortstämme, Hervorhebung und Filter müssten wir selbst bauen. (c) Lunr/MiniSearch: Index in JavaScript, aber ohne Rust-Integration und mit größerem Datentransfer, weil alles auf einmal geladen wird. Gewählt ist die Bibliothek, weil sie zu den Zielen (statisch, Rust, inkrementell) passt.

### 6. Zuordnung der 194 Beiträge: Vorschlag, Prüfung, Eintragen

1. Claude liest für jeden Beitrag Titel, Kurztext und Textanfang und schlägt vor: `category` (nur für die 5 Beiträge ohne Kategorie, und nur wenn eine klar passt; sonst bleibt sie leer) und 1–4 Schlagworte aus einer **vorgeschlagenen Liste** von etwa 20 Schlagworten. Startpunkt dafür, abgeleitet aus Titeln und Texten: *Techniken* Häkeln, Stricken, Spinnen, Quilten, Amigurumi, Papier; *Gegenstände* Hüte, Mützen, Taschen, Kissen, Schals; *Anlässe und Themen* Katzen, Geschenke, LARP, Halloween, Weihnachten, Einhörner, Recycling und Blog (für Beiträge über das Flauschiversum selbst, die in keine Kategorie passen). Die endgültige Liste entsteht in der Prüfung.
2. Das Ergebnis steht als Tabelle (`Beitrag | Kategorie | Schlagworte`, nach Kategorie sortiert) in `openspec/changes/handbuch-navigation/zuordnung.md`, zusammen mit der Schlagwortliste und der Zahl der Beiträge je Schlagwort. Du korrigierst direkt in dieser Datei.
3. Erst nach deiner Freigabe schreibt ein einmaliges Skript `category:` und `tags:` in die `index.md` jedes Beitrags, **nur als eingefügte Zeilen** (keine Neuserialisierung des YAML; Prüfung: `git diff --numstat` zeigt je Beitrag nur eingefügte Zeilen) und legt `content/tags.yaml` an.

*Beiträge ohne Kategorie sind erlaubt:* Passt ein Beitrag in keine Kategorie, bleibt `category` leer; er ist dann über seine Schlagworte, die Suche und `/beitraege/` zu finden. Das gilt ausdrücklich für „Relaunch aus der Hackermatte“, einen Blog-internen Beitrag. Für die übrigen vier Beiträge ohne Kategorie schlägt Claude eine vor, wenn sie klar passt (z.B. „Amigurumi Häkeldrache“ → wolle, „gefilzte Kapuzenweste“ → filzen), sonst bleibt das Feld leer. Die Kategorieliste bleibt bei den sechs vorhandenen.

### 7. SEO und Feed

Sitemap, Titel, Meta-Beschreibung und kanonische URL der neuen Seiten entstehen im selben Muster wie bei Posts (`src/seo.rs`, `render.rs`). Der JSON-LD-Datensatz der Beiträge erhält `articleSection` und `keywords`. Die Suchseite bekommt `noindex`, ist aber nicht in `robots.txt` gesperrt (Crawler müssen das Meta-Tag lesen können). Der RSS-Feed bleibt unverändert.

### 8. Tests

- **Einheit:** `topics.rs` (Zählungen, Sortierung, geplante Posts, Slug-Kollisionen, deutsche Sortierung), Listen laden, Prüfmeldungen (inkl. Vorschlag „Meintest du …“), Tags als YAML-Liste und als Text.
- **Integration:** Die Test-Site bekommt Kategorien, Schlagworte und einen geplanten Beitrag. Snapshots für Startseite, Kategorieseite, Schlagwortseite, Themenübersicht und Beitragskopf. Ein zweiter Build schreibt 0 Dateien (auch unter `pagefind/`). Gesamtseitentest: jeder Beitrag mit Kategorie erscheint auf genau einer Kategorieseite, jede Schlagwortseite ist verlinkt, alle Live-URLs existieren weiter.
- **Browser (Playwright):** Suche von der Startseite aus, Treffer für „katze“ (Wortform), „kuerbis“ (Umlaut) und „filz“ (Wortanfang), Kategoriefilter, Hinweis ohne JavaScript, Kachelklick führt auf die Kategorieseite, Handy-Layout ohne waagerechtes Scrollen.

## Risks / Trade-offs

- [Die Pagefind-Bibliothek ist schwer einzubinden (Kompilierzeit, asynchrone API, Größe)] → Als erste Aufgabe der Suche ein kleiner Probelauf mit drei Beiträgen im Browser. Scheitert er, greift die Alternative (b) mit einem kleinen eigenen Index. Die Specs beschreiben nur das Verhalten und bleiben gleich.
- [Pagefind erzeugt nicht byte-gleiche Dateien] → Tests prüfen das; Eingaben werden sortiert; notfalls wird der Index nur neu geschrieben, wenn sich die Seitentexte ändern (Hash der Eingaben).
- [Die Zuordnung der Schlagworte ist subjektiv] → Vorschlag und Prüfung vor dem Eintragen; die Liste ist klein und später leicht erweiterbar. Schlagworte, die weniger als zwei Beiträge tragen, meldet der Build als Warnung.
- [Die Startseite ändert sich stark] → Die chronologische Liste und ihre URLs bleiben; die Navigation führt weiter dorthin. Vergleichsbilder (alt/neu) und Browser-Test vor dem Abschluss.
- [Kategorieseiten mit 54 Einträgen werden lang] → Vorschaubilder laden verzögert; die Einträge sind kompakt. Falls es sich zäh anfühlt, lässt sich später eine Gruppierung nach Jahr ergänzen, ohne URLs zu ändern.
- [Die Navigation wird breit (7 Einträge)] → sie bricht wie bisher um; im Handy-Test geprüft.

## Migration Plan

1. Schlagwortliste und Zuordnung als Vorschlag erzeugen, von dir prüfen lassen (`zuordnung.md`).
2. Nach Freigabe `content/categories.yaml`, `content/tags.yaml` anlegen und `category`/`tags` in die Beiträge eintragen (eigener Commit).
3. Generator, Templates, Styles und Suche umsetzen; Gesamtseitentest und Browser-Tests.
4. Vergleichsbilder (Startseite, Kategorieseite, Beitrag, Suche, Handy) zur Abnahme vorlegen.
5. **Rollback:** Bis zum Deployment ist die Live-Seite unberührt; ein Zurück ist ein Git-Revert. Die neuen Frontmatter-Felder stören den alten Stand nicht.
