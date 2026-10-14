## Why

Im Text der Beiträge sind Absätze kaum zu erkennen: Das globale Zurücksetzen in `content/style/main.css` setzt `margin` und `padding` von `p`, `ul`, `ol`, `blockquote`, `pre` und `figure` auf 0, und im Hauptbereich (`main.page`) gibt es keine Gegenregel. Aufeinanderfolgende Absätze stoßen deshalb ohne Abstand aneinander und lesen sich wie ein einziger Textblock. Dasselbe gilt für Listen, Zitate und Code-Blöcke innerhalb des Textes.

## What Changes

- Im Textbereich von Beiträgen und Einzelseiten (`main.page`) bekommen Absätze einen deutlichen vertikalen Abstand (ein Zeilenabstand-Vielfaches, in `em`, damit er mit der Schriftgröße skaliert).
- Listen, Zitate und vorformatierter Text im selben Bereich erhalten passende Abstände nach oben und unten, Überschriften einen größeren Abstand davor als danach.
- Der Abstand gilt nur im Fließtext von `main.page`; Kopf, Sidebar, Übersichten, Themenseiten, Karten und Bildergruppen behalten ihr Aussehen.
- Kein Eingriff in Markdown, HTML-Ausgabe oder URLs; nur das Stylesheet ändert sich.

## Capabilities

### New Capabilities

(keine)

### Modified Capabilities

- `post-page`: neue Anforderung „Lesbare Absätze“ für den Beitragstext (gilt wegen gemeinsamer Vorlage auch für Einzelseiten).

## Impact

- `content/style/page.css` (oder ein neuer Abschnitt dort), ggf. kleine Anpassungen in `content/style/media.css` falls Bilder/Figuren im Text doppelte Abstände bekämen.
- Alle Seiten mit `main.page`: Beiträge (`templates/post.html`) und Einzelseiten (`templates/page.html`); dort können sich Snapshots/Tests, die das Stylesheet prüfen, ändern.
- Browser-Tests (Playwright) und Smoke-Test bleiben unverändert, soweit sie keine Abstände messen; ein neuer Test misst den Absatzabstand.
- Keine Änderung an Inhalten, Medien, Suchindex oder Deployment.
