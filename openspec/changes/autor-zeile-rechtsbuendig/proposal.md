## Why

Die Zeile „Von Ermeline“ am Ende eines Beitrags steht linksbündig und ohne Abstand direkt unter dem letzten Absatz (das globale Zurücksetzen in `content/style/main.css` nimmt `p` jeden Rand, `p.author` hat keine eigene Regel). Sie liest sich dadurch wie ein weiterer Absatz des Textes statt wie eine Unterschrift.

## What Changes

- `p.author` im Beitrag wird rechtsbündig ausgerichtet.
- Zwischen dem Beitragstext und der Zeile liegt ein Abstand nach oben (rund 1,5 Zeilen); nach unten bleibt der Abstand zu „Schreibe Ihr eine Nachricht!“ bzw. zum Seitenende angemessen klein.
- Die Zeile bleibt in Wortlaut, Verlinkung (`/about/`) und Reihenfolge unverändert; nur die Darstellung ändert sich. Hat ein Beitrag keinen Autor, gibt es die Zeile weiterhin nicht.

## Capabilities

### New Capabilities

(keine)

### Modified Capabilities

- `post-page`: Anforderung „Autor am Ende des Beitrags“ um rechtsbündige Darstellung mit Abstand zum Text erweitert.

## Impact

- `content/style/page.css` (Regel für `main.page p.author`), keine Änderung an Vorlagen, HTML oder URLs.
- Abgrenzung zum Change `absatzabstand-beitraege`: Dessen Absatzregel gilt nur für Absätze ohne Klasse und lässt `p.author` unberührt; die Regeln überschneiden sich nicht.
- Neuer Playwright-Test für Ausrichtung und Abstand.
