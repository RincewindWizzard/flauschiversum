## Why

Die Suchseite wirkt wie ein Fremdkörper: Der Text der Treffer ist mit rund 13 px (Ausschnitt, Filter) und 11 px (Kategorie-Etikett) deutlich kleiner als der Seitentext mit rund 21 px, und er steht in einer fremden Systemschrift statt in der Lora des Blogs. Die Seite zeigt außerdem gleichzeitig drei Suchfelder (Topbar, ein großes Formular und das Suchfeld der Suchoberfläche), weil das große Formular bei aktivem JavaScript nur mit dem Attribut `hidden` verborgen wird, das eine CSS-Regel überstimmt. Und der Kategorie-Filter sitzt als eckiger Kasten links in der schmalen Mitte: Alles drängt sich in die 1000 px breite Inhaltsspalte, während die übrigen Seiten mit Karte rechts den 1200-px-Rahmen nutzen.

Ursache für die falsche Schrift und Größe: Die Suchoberfläche (Pagefind) setzt ihre Variablen (`--pagefind-ui-font`, `--pagefind-ui-scale: .8`, Bildradius 8 px) auf ihrem eigenen Wurzelelement. Die Werte, die `topics.css` auf `main.search-page` setzt, werden dadurch nie wirksam.

## What Changes

- **Ein Suchfeld auf der Seite:** Das zusätzliche große Formular entfällt. Auf der Suchseite gibt es das Suchfeld der Topbar und das der Suchoberfläche, gestaltet wie das Suchfeld der Seite (rund, Lupe im Feld). Die Topbar-Suche bleibt, aber eine Suche dort lädt die Seite mit dem Begriff und füllt das Feld der Oberfläche.
- **Lesbare, passende Schrift:** Treffer, Ausschnitt, Filter und Etiketten in Lora und in einer Größe, die zum Rest passt (Titel wie Beitragslisten, Ausschnitt und Filter mindestens 16 px, Etiketten mindestens 14 px bei Desktop-Schrift).
- **Eigener Seitenaufbau wie die Karten-Seiten:** Rahmen bis 1200 px, Treffer in der Hauptspalte, der Kategorie-Filter als Karte rechts (wie das Archiv), auf schmalen Fenstern über den Treffern. Die Treffer sehen aus wie die Einträge der Beitragslisten (runde Vorschaubilder, Titel kursiv, Datum nicht nötig).
- **Bedienelemente im Seitenstil:** Löschen-Knopf, Filter-Häkchen, „Mehr laden“ und Hervorhebung der Suchwörter in den Farben der Seite, ohne harte schwarze Rahmen.
- Das Verhalten der Suche (Index, Filter, Parameter `q`, Adresse aktualisieren) bleibt unverändert.

Nicht Teil dieses Changes: Änderungen am Suchindex, an der Suchlogik oder am Dunkelmodus (die Farben kommen weiter aus den Variablen der Seite und sind für `farbschema-darkmode` vorbereitet).

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `site-search`: Darstellung der Suchseite (ein Suchfeld, Schrift und Größe, Seitenaufbau mit Filterkarte); „Suchseite“ und „Ohne JavaScript“ werden angepasst.

## Impact

- `templates/search.html` (großes Formular nur noch im `noscript`, Rahmenklasse für den breiten Aufbau), `src/render.rs` (Body-Klasse für die Suchseite), `assets/js/search.js` (Formular nicht mehr nötig, Fokus und Adresse bleiben).
- `content/style/search.css`/`topics.css` (Suchseitenstil, Pagefind-Variablen auf `.pagefind-ui`, Gitter für Treffer und Filter), `layout.css` (Rahmen für die Suchseite).
- Tests: Schrift- und Größen-Prüfung, genau ein sichtbares Suchfeld im Inhalt, Aufbau bei 1440, 1100 und 360 px; Snapshot der Suchseite.
