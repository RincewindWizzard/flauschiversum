# Design

## Context

`templates/layout.html` rendert heute einen Kopf mit Logo, Untertitel, Kopfnavigation und Suchfeld, dazu bei den meisten Seiten eine linke Navigationsspalte (`nav.sitenav`) und eine rechte Sidebar (`aside.sidebar` mit Kategorienliste und Tag-Cloud) in einem Grid (`layout.css`, `sidebar.css`), und die Fußzeile. `Common.sidebar` (Kategorien und Cloud-Daten) wird in `site.rs` aus `Topics` gebaut und an jede Seite gegeben. Die Startseite (`home.html`) hat ein großes Suchfeld, Kategoriekacheln und „Neu im Flauschiversum“ (4 Einträge); `/beitraege/` und `/page/<N>/` sind eine Blätterliste mit 4 Beiträgen je Seite (`index.html`, `render_index`); Kategorieseiten (`topic.html`) zeigen Name, Reim, eine Schlagwortliste und alle Beiträge. Inhalte: `content/categories.yaml` (id, name, image, motto), keine Startseitentexte. Die Seite hat 194 Beiträge von 2013 bis 2020. Siehe proposal.md für Motivation.

## Goals / Non-Goals

**Goals:**
- Eine einzige Navigation: Topbar mit Titel links und den Menüs Beiträge und Kategorien (rechtsbündig) und Suche, alles vertikal auf einer Linie.
- Beiträge finden über die Beiträge-Seite mit Archiv nach Jahr und Monat und Kategorieseiten mit Archiv der Kategorie.
- Startseite als Begrüßung mit direkt folgenden Beiträgen; Cloud nur dort.
- Alle Menüs und Archive ohne JavaScript brauchbar, mit JavaScript komfortabler.

**Non-Goals:**
- Neue Inhalte über Begrüßungstext und Kategorie-Einleitungen hinaus; Änderungen an Suche, Feed, Deployment, Farbschema und Typografie-Regeln.
- Entfernen oder Umleiten der Blätterseiten `/page/<N>/` (Cool URIs).

## Decisions

### 1. Topbar-Markup und Verhalten

```html
<header class="topbar">
  <a class="brand" href="/">Flauschiversum</a>
  <details class="menu-toggle"> … Menü-Knopf für das Handy … </details>
  <nav class="topnav" aria-label="Hauptnavigation">
    <ul>
      <li><a href="/beitraege/">Beiträge</a></li>
      <li class="has-menu"><a href="/themen/">Kategorien</a> … ein Link je Kategorie …</li>
    </ul>
  </nav>
  <form class="search">…</form>
</header>
```

Titel und Menüs sind in einer Flex-Leiste (`position: sticky; top: 0`, heller Hintergrund mit leichtem Schatten, Höhe etwa 3,5 em unter der Staging-Leiste). Rechtsbündig durch `margin-left: auto` am `nav`. Die Untermenüs sind absolut positionierte Karten unter dem Eintrag. **Ohne JavaScript** öffnen sie sich per `:hover` und `:focus-within` (CSS). **Mit JavaScript** (`assets/js/topbar.js`, ca. 60 Zeilen, wie `media.js` nur auf Seiten geladen) schalten die Knöpfe `aria-expanded` um, öffnet höchstens ein Menü, schließen Escape und ein Klick außerhalb, und Pfeiltasten wechseln zwischen den Einträgen; damit funktionieren auch Touch-Geräte mit großem Bildschirm. Die Menüdaten (10 neueste Beiträge, Kategorien) berechnet der Build einmal und rendert sie in jede Seite (rund 1,5 KB je Seite).

*Alternativen:* (a) Nur `<details>` für Dropdowns: der Eintrag „Beiträge“ ist zugleich Link, das verträgt sich nicht mit `summary`. (b) Menüs nur mit JavaScript: ohne JS wäre „Kategorien“ nicht erreichbar. (c) Mega-Menü mit Vorschaubildern: für zehn Titel unnötig.

### 1a. Maskottchen im Markenzeichen

`<a class="brand" href="/">` enthält zuerst ein Feld `span.mascot` mit zwei `<img>` (`/images/flauschi_normal.svg`, `/images/flauschi_surprised.svg`, beide `alt=""`, weil der Link durch den Titeltext benannt ist) und danach den Titel. Beide Bilder liegen im selben Grid-Feld übereinander; das Feld hat feste Maße (Höhe 2,5 em, Breite aus dem Seitenverhältnis 478 : 425 = 1,125, also 2,8 em), jedes Bild füllt es mit `object-fit: contain; object-position: center bottom`. Dadurch ändert der Wechsel nichts am Layout, auch wenn die Quellen leicht unterschiedlich hoch sind (419 und 425 px). Der Wechsel passiert nur über `opacity` (`.brand:hover` und `.brand:focus-visible`), ohne Verschiebung; bei `prefers-reduced-motion` ohne Übergang. Beide Bilder sind immer im DOM, damit beim ersten Mouse-over nichts nachlädt. Die SVG-Dateien kommen bereinigt (ohne Inkscape-Metadaten und `sodipodi`-Attribute, mit `viewBox="0 0 478.19 424.96"` und gleicher Zeichenfläche) nach `content/static/images/`; die Originale liegen unter `assets/` in diesem Change. Auf dem Handy bleibt das Maskottchen sichtbar (Höhe 2 em), der Titel kürzt sich nicht.

*Alternativen:* (a) Inline-SVG: verdoppelt Markup in jeder Seite, nicht cachebar. (b) CSS-`background-image`-Wechsel: lädt das zweite Bild erst beim Hover, Flackern.

### 2. Handy-Menü

Unter 48 em zeigt die Topbar Titel, Suche (Lupe als Knopf, klappt das Feld auf bzw. führt auf `/suche/`) und einen Menü-Knopf. Der Knopf ist das `summary` eines `details`; dessen Inhalt ist dieselbe Liste wie am Desktop, aber mit allen Untermenüs offen und in Spalte. Ohne JavaScript bleibt die Bedienung durch `details`; Desktop-Dropdown-CSS und Handy-Liste teilen sich Markup per Media Query (kein doppeltes HTML).

### 3. Archiv nach Jahr und Monat

`Topics` (oder ein neues `archive.rs`) liefert `Vec<Year { year, count, months: Vec<Month { number, name, count, posts: Vec<(title, url, day)> }> }>` aus einer Liste Posts (alle oder die einer Kategorie), neueste zuerst; leere Gruppen entfallen. `partials/archive.html` rendert verschachtelte `<details class="year">` und `<details class="month">` mit `summary` („2020 (12)“, „Oktober (3)“), geöffnet nur beim neuesten Jahr und dessen neuestem Monat (`open`-Attribut, vom Build gesetzt). Ein Knopf „Alle aufklappen/zuklappen“ wird nur von `topbar.js` bzw. `archive.js` eingefügt. Das Archiv liegt als Karte rechts (ab 64 em, `position: sticky` mit eigener Scrollhöhe) und sonst unter der Liste.

*Alternative:* Archiv als eigene Seite je Jahr (`/2020/`): neue URLs, mehr Klicks. Verworfen: Gewünscht ist ein einklappbares Archiv direkt bei der Liste.

### 4. Seiten

- **Startseite:** `home.html` ohne eigene Überschrift (der Titel steht in der Topbar; Entscheidung der Autorin), Begrüßung aus `content/start.md` (gerendert wie Einzelseiten), die zehn neuesten Einträge (Partial `entry.html`), Link „Alle Beiträge“, rechts die Karte mit der Tag-Cloud (aus `Sidebar`-Daten, nur noch hier; `HOME_NEWS = 10`).
- **Beiträge:** `/beitraege/` mit `<h1>Beiträge</h1>`, den zehn neuesten Einträgen und dem Archiv. Die Seiten `/page/<N>/` bleiben mit dem bisherigen Inhalt (4 je Seite, Paginierung), „«“ auf `/page/1/` führt nach `/beitraege/`. Sie sind nicht mehr verlinkt, haben aber weiter Titel „Alle Beiträge – Seite N“.
- **Kategorie:** `topic.html` für Kategorien: Name, Reim und `intro` (Markdown) oben, darunter alle Beiträge (Einträge), rechts das Archiv der Kategorie. Schlagwortseiten und `/themen/` bleiben wie bisher (ohne Archiv, einspaltig).
- **Entfall:** `nav.sitenav`, `aside.sidebar` (außer der Cloud-Karte auf der Startseite), Spalten-Grid, `HOME_TOPICS` (bereits entfernt), Kategoriekacheln, großes Suchfeld auf der Startseite (die Suchseite behält ihr großes Feld).

### 5. Inhalte

`content/start.md` enthält einen kurzen Begrüßungstext (Entwurf aus „About“ abgeleitet, zur Prüfung durch die Autorin). `content/categories.yaml` bekommt je Kategorie ein optionales `intro` (zwei bis drei Sätze). Claude schlägt die Texte vor; sie werden in der Abnahme geprüft. Fehlt `start.md`, warnt der Build und die Startseite lässt die Begrüßung weg; fehlt `intro`, zeigt die Kategorie nur Reim und Zahl.

### 6. Annahmen aus dem Auftrag

- „Kategorien“ führt beim Klick auf `/themen/` (Übersicht), die Einträge im Untermenü auf die Kategorieseiten.
- Das Suchfeld bleibt, jetzt in der Topbar (im Auftrag nicht genannt, für die Auffindbarkeit nötig).
- Footer: Verbindliche Vorlage ist der Footer der Segelgruppe Kiel (HTML und SCSS der Autorin, siehe `footer.css`): `footer` mit Innenabstand 2 em, Hintergrund dunkel, Text `#afafaf`, `a` und `a:visited` in Textfarbe; `.footer-inner` als Flex-Zeile mittig; `.footer-left` als Flex-Spalte mit `justify-content: space-evenly` (Icon-Navigation 3 em hoch als Flex-Zeile `space-evenly`, darunter das Copyright); rechts die Linkliste ohne Aufzählungszeichen, Einträge mit 0,2 em Innenabstand ohne Unterstreichung. Handy: einspaltig, `.footer-left` wird mit `display: contents` aufgelöst und die Reihenfolge per `order` auf Icons, Links, Copyright gesetzt, damit das Copyright immer zuletzt steht. Die Hintergrundfarbe ist eine Variable am Footer (`#263329`) und folgt später dem Farbschema.
- `/themen/` und die Schlagwortseiten bleiben erhalten und sind über die Cloud auf der Startseite und über „Kategorien“ erreichbar.
- Startseite zeigt die zehn neuesten Beiträge wie `/beitraege/`; Archiv: neuestes Jahr und neuester Monat offen, Rest zu.

### 7. Überarbeitung nach der ersten Sichtung

- Alle Elemente der Topbar liegen auf einer Mittellinie (`align-items: center`); das Suchfeld verliert den Außenabstand unten (`form.search` setzte `margin: 0 1em 1em`, deshalb sitzt es höher), geprüft durch einen Test auf die Mittelpunkte.
- Nur „Kategorien“ hat ein Untermenü; „Beiträge“ ist ein einfacher Link. Damit entfallen die Menüdaten der zehn neuesten Beiträge.
- Handy: Hamburger-Symbol als quadratischer Knopf (mindestens 44 px, `aria-label="Menü"`) in der Zeile der Marke, die Suche in einer eigenen Zeile darunter; das Menü klappt unter der Leiste auf.

## Risks / Trade-offs

- [Dropdowns sind schwer zugänglich oder auf Touch unbrauchbar] → Knopf mit `aria-expanded`, Tastatur und Escape, Touch über Knopf; Browser-Tests für Maus, Tastatur und Handy.
- [Sticky-Topbar verdeckt Anker und Inhalt] → `scroll-margin-top` für Überschriften, Höhe als Variable; Staging-Leiste scrollt weg (nicht sticky).
- [Kategorie-Text fehlt und die Seite wirkt leer] → Entwürfe für alle Kategorien liefern; ohne `intro` bleibt Reim.
- [Der Auftrag macht frühere Entscheidungen rückgängig (Footer-Navigation, linke Spalte)] → Footer bleibt, die Spalten entfallen; die Spezifikation führt die Änderungen als REMOVED mit Begründung.
- [Blätterseiten werden zu verwaisten Seiten] → Sie bleiben erreichbar und indexierbar wie bisher; Verweis „Ältere Beiträge“ am Ende von `/beitraege/` auf `/page/1/` wäre möglich, ist aber nicht gewünscht (Archiv ersetzt es).
- [Seiten werden größer durch Menüdaten] → ca. 1,5 KB je Seite, bei 194 Beiträgen vernachlässigbar.

## Migration Plan

1. Datenmodell und Seiten umbauen, Tests und Snapshots anpassen.
2. Inhalte (`start.md`, `intro`) als Entwurf hinzufügen.
3. Push auf den Claude-Branch, Staging-Lauf, Abnahme am veröffentlichten Staging (Desktop, Tablet, Handy).
4. Rollback: Commit zurücknehmen; keine Datenmigration, alle URLs bleiben.
