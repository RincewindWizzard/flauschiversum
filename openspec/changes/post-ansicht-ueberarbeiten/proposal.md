## Why

Die Beitragsseite hat beim Review mehrere Schwächen gezeigt: Der Hinweis „Schreibe Ihr eine Nachricht!“ steht ganz oben und drängt sich vor den Beitrag, statt als Abschluss zu dienen. Das Wasserzeichen „Flauschiversum“ in den Bildern ist noch im alten grünen Look (limonengrüne Fläche, dunkles Graugrün) und passt nicht zum Farbschema „Flauschi-Nebel“. Die Karte „Ähnliche Beiträge“ sieht anders aus als die Beitragsliste der Startseite, obwohl beide dasselbe zeigen. Und das Datum eines Beitrags fehlt auf der Beitragsseite ganz.

## What Changes

- **Kontakt-Hinweis ans Ende:** „Schreibe Ihr eine Nachricht!“ (Wortlaut und `mailto:`-Link unverändert) steht nach dem Beitragstext und der Zeile „Von <Autor>“, vor „Ähnliche Beiträge“, nicht mehr im Kopf.
- **Datum über dem Beitrag:** Der Kopf zeigt unter dem Titel das Veröffentlichungsdatum („31. Oktober 2020“) gut lesbar (normale Textgröße, Kontrast mindestens 4,5 : 1 in Hell und Dunkel) als `<time datetime>`, vor Vorschaubild und Text.
- **Wasserzeichen im neuen Farbthema:** Das Badge in den Bildvarianten `700x` und `1000x` wird neu gestaltet: Lobster-Schriftzug „Flauschiversum“ in Nebel-Indigo auf halbtransparenter, abgerundeter Fläche in hellem Lavendel statt Limonengrün/Graugrün. Es muss auf hellen und dunklen Fotos erkennbar bleiben. Weil Bilder dauerhaft das Badge tragen, ist es für Hell- und Dunkelmodus gleich. Alle Bildvarianten werden einmal neu berechnet (der Cache-Schlüssel enthält den Inhalt des Badges).
- **Ähnliche Beiträge wie die Liste der Startseite:** Die Karte zeigt die ähnlichen Beiträge im selben Aufbau wie die Beitragsliste (`ul.entries`/`entry.html`): rundes Vorschaubild, Titel kursiv, Datum, Kurztext. Die Chips mit den gemeinsamen Schlagworten entfallen, da die Liste sie nicht zeigt (Annahme, siehe design.md).
- **Nicht Teil dieses Changes:** Auswahl und Reihenfolge der ähnlichen Beiträge, Wortlaut des Kontakt-Hinweises, Layout der Seite.

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `post-page`: Datum im Kopf, Kontakt-Hinweis am Ende, Karte „Ähnliche Beiträge“ im Listenaufbau.
- `related-posts`: Anzeige wie die Beitragsliste statt Titel mit Schlagwort-Chips.
- `image-processing`: Wasserzeichen im Farbthema „Flauschi-Nebel“.

## Impact

- `templates/post.html`, `src/render.rs` (`PostTemplate`, `RelatedLink` mit Entry-Daten), `src/site.rs` (Daten der ähnlichen Beiträge), `content/style/related.css`, `header.css`/`topics.css` (Datum), `assets/badge.png` (neu erzeugt), Snapshots, Browser-Tests, README.
- Alle Bildvarianten ändern sich (neue Cache-Schlüssel); der erste Build danach dauert länger, die ausgelieferten Bild-URLs bleiben gleich.
