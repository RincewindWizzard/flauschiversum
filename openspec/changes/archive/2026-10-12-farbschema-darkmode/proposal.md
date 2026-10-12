## Why

Die Seite hat nur ein helles Salbei-Schema. Besucher, die abends lesen oder empfindlich auf Helligkeit reagieren, haben keinen Dunkelmodus. Außerdem ist das Schema bewusst nüchtern; ein Blog mit kreativem Inhalt darf farbiger sein. Der Name „Flauschiversum“ und das Maskottchen (ein flauschiger Planet) laden zu einem Universum-Motiv ein: Sterne, Nebel und Galaxien als Stilelement, ohne die Lesbarkeit zu gefährden. Gerade für Menschen mit Sehschwäche und Rot-Grün-Blindheit müssen Kontraste und Farbunterscheidung geprüft und belegt sein.

## What Changes

- **Dunkelmodus** zusätzlich zum hellen Modus: automatisch nach Systemeinstellung (`prefers-color-scheme`), von Besuchern über einen Umschalter in der Topbar (System, Hell, Dunkel) überschreibbar, ohne Aufblitzen beim Laden.
- **Neues, farbigeres Schema „Flauschi-Nebel“** (Violett und Indigo wie das Maskottchen, Cyan und Gold als Sternenlicht) mit hellem und dunklem Modus. Entschieden von der Autorin aus vier Vorschlägen (Flauschi-Nebel, Mitternachtsblau, Orchideen-Galaxie, Salbei-Kosmos; Vorschau: `themen-vorschau.html`). Die übrigen drei werden nicht gebaut.
- **Universum als Stilelement:** dezenter Sternenhimmel und Nebel im Hintergrund (nur Dekoration, hinter einfarbigen Karten), im Dunkelmodus sichtbar, im Hellmodus nur als zarte Andeutung. Abschaltbar per `prefers-reduced-motion`/`prefers-contrast`/erzwungene Farben; keine Animation nötig.
- **Barrierefreiheit:** Text mindestens 4,5 : 1, große Schrift und Bedienelemente mindestens 3 : 1, nichts allein über Farbe (Links unterstrichen, aktive Elemente mit Form/Gewicht), geprüft auch in Simulationen von Protanopie, Deuteranopie, Tritanopie und Graustufen.
- Alle Farben werden zu Variablen an einer Stelle (heute stehen einzelne Werte noch fest in einzelnen Stylesheets).
- Das Maskottchen und die Suchoberfläche (Pagefind) bekommen passende Dunkelmodus-Varianten.

Nicht Teil dieses Changes: neue Inhalte, Layoutänderungen, Änderung der Schriften.

## Capabilities

### New Capabilities
- `color-themes`: Hell/Dunkel, Umschalter, Barrierefreiheit der Farben, Universum-Dekoration.

### Modified Capabilities
- `site-layout`: Anforderung „Farbschema“ gilt jetzt für die gewählte Themenfamilie in Hell und Dunkel statt für das feste Salbeigrün.

## Impact

- `content/style/theme.css` (Tokens für beide Modi), alle Stylesheets mit festen Farben (`sidebar.css`, `topics.css`, `search.css`, `topbar.css`, `archive.css`, `related.css`, `footer.css`, `media.css`, `page.css` …), neue `starfield.css`.
- `templates/layout.html` (Umschalter, Skript im `<head>` gegen Aufblitzen, `color-scheme`-Meta), `assets/js/topbar.js` bzw. neues `assets/js/theme.js`.
- Browser-Tests und ein Kontrast-Test über alle Farbpaare beider Modi.
- Entscheidung gefallen: Flauschi-Nebel.
