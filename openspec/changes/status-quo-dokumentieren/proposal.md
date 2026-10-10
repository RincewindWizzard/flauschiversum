# Proposal

## Why

Das Flauschiversum wird seit Jahren nicht mehr gepflegt, und der Python-Generator läuft mit aktuellen Bibliotheken nicht mehr (`markdown.util.etree` entfernt, `PIL.ANTIALIAS` entfernt, `htmlmin` nicht mehr baubar). Bevor die Technik in Rust neu umgesetzt wird, muss das heutige Verhalten präzise festgehalten werden, damit der Neubau dagegen geprüft werden kann und keine bestehende URL verloren geht („Cool URIs don't change“).

## What Changes

- Dokumentiert das **heutige, beobachtbare Verhalten** des Generators und der Live-Seite als Specs. Grundlage sind der Code in `blogcompile/`, `templates/`, `src/` und ein Abgleich mit https://flauschiversum.de/ (Stand: Build vom 13.12.2021).
- Am Code, an den Inhalten und am Deployment ändert sich **nichts**. Nach dem Archivieren bilden die Specs die Grundlage unter `openspec/specs/`.
- Fest spezifiziert wird nur, worauf Leser, Suchmaschinen oder Inhalte sich verlassen. Erkennbare Fehler werden **nicht** als Anforderung festgeschrieben, sondern unten aufgelistet.

### Abgleich mit der Live-Seite

- Alle 194 Post-URLs sind live erreichbar. 193 davon lassen sich aus Datum und Titel mit dem heutigen `python-slugify` nachrechnen. `Gib´s mir!… - Audrey II` liegt live unter `/2014/05/gibs-mir-audrey-ii/`, eine aktuelle Bibliothek ergibt aber `gib-s-mir-audrey-ii`. **Maßgeblich sind daher die Live-URLs, nicht der Algorithmus.**
- 2857 Bild- und Script-Referenzen aus allen Posts wurden geprüft, 2856 sind erreichbar.
- 49 Indexseiten mit 4 Posts pro Seite, die letzte Seite enthält 2 Posts.

### Bekannte Fehler und Lücken (bewusst nicht spezifiziert)

1. Zukünftig datierte Posts blendet nur die Indexseite aus. Post-Seite und RSS-Eintrag werden trotzdem erzeugt, und in der Paginierung zählen sie mit.
2. Der RSS-Feed ist unsortiert (Dateisystem-Reihenfolge) und hat kein `pubDate`.
3. Auf der letzten Indexseite zeigt die Paginierung eine nicht existierende Seite „50“ und einen »-Link mit leerem `href`.
4. Der `<title>` der Indexseiten ist leer.
5. Nur `.jpg`/`.JPG` werden verarbeitet. `20170212_141939.jpeg` im Post „für Blumenkinder“ ist deshalb live ein kaputtes Bild.
6. Bild-URLs werden nicht kodiert, Leerzeichen und Umlaute stehen roh im `src`-Attribut.
7. Ein Klick auf ein Bild bewirkt nichts (`href="#"`). Die Größe `1000x/` wird erzeugt, aber nirgends verlinkt. Auf dem Handy kann man nicht wischen.
8. Die Größe des Wasserzeichens hängt von der Auflösung des Originals ab: von ca. 50 % der Bildbreite bis praktisch unsichtbar. Auf dunklem Grund ist es nicht lesbar.
9. Der Bild-Cache ist nur nach dem Inhalt des Originals geschlüsselt. Ändert man Wasserzeichen oder Größen, wird der Cache nicht ungültig.
10. `--clean` erwartet ein Argument. Der Build muss aus dem Repo-Wurzelverzeichnis gestartet werden.
11. `category` wird gepflegt (inkl. Doppel `nähen`/`naehen`), aber nicht ausgewertet.
12. `docker-compose.yml` verweist auf das fehlende Verzeichnis `docker/flauschiversum-build`. Das Basis-Image `python:alpine3.7` ist veraltet.

### Bereits getroffene Entscheidungen für den Folge-Change (nicht Teil dieses Changes)

Festgehalten, damit sie nicht verloren gehen; umgesetzt werden sie im Rust-Neubau:

- Jeder Post erhält im Frontmatter ein `permalink`-Feld mit seiner **Live-URL**.
- Drei Bildformate unter den bestehenden URLs: Thumbnail `200x/`, Preview `700x/`, Full `1000x/`. Full wird nie mehrere MB groß.
- Ein einzelnes Bild wird als Preview gezeigt, ein Klick öffnet Full im Vollbild. Mehrere aufeinanderfolgende Bilder bilden ein Karussell, das auch auf dem Handy funktioniert (Wischen), mit Vollbildansicht.
- Neues Wasserzeichen „Badge“ (Lobster-Schrift, Themengrün, halbtransparent). Es wird nach dem Skalieren aufgebracht, und seine Größe richtet sich nach der Ausgabegröße.
- Rechte-Metadaten (EXIF, XMP, IPTC) lauten pauschal „Flauschiversum / Ermeline Wollknoll“. GPS- und Kameradaten werden nie ausgeliefert. Ein unsichtbares Wasserzeichen ist nicht vorgesehen.

## Capabilities

### New Capabilities

- `content-format`: Aufbau der Inhaltsquellen (Post-Verzeichnisse, Frontmatter, Markdown-Konventionen, Seiten), die unverändert weiter funktionieren müssen.
- `url-scheme`: Alle öffentlich erreichbaren URLs der Seite (Posts, Seiten, Bilder, Index, Feed, statische Dateien).
- `site-layout`: Gemeinsamer Rahmen jeder HTML-Seite (Kopf, Navigation, Fuß, Titel, Schriften).
- `post-index`: Startseite und Folgeseiten mit der Liste der Posts und der Paginierung.
- `post-page`: Darstellung eines einzelnen Posts.
- `static-pages`: Darstellung der Einzelseiten (About, Impressum, Tipps und Tricks).
- `post-images`: Darstellung der im Markdown eines Posts eingebundenen Bilder (Größe, Slideshow, Bildunterschrift).
- `image-processing`: Erzeugung der Bildvarianten aus den Originalen (Größen, Wasserzeichen, Format, Metadaten, Cache).
- `rss-feed`: Der RSS-Feed zum Abonnieren.
- `build`: Aufruf und Ergebnis des Generators (CLI, Ausgabeverzeichnis, statische Dateien, Stylesheet).
- `deployment`: Wie die gebaute Seite aktualisiert und ausgeliefert wird.

### Modified Capabilities

<!-- keine: es existieren noch keine Specs -->

## Impact

- Nur Dokumentation unter `openspec/`. Keine Änderungen an Code, Templates, Inhalten oder am Deployment.
- Die Specs sind die Abnahmegrundlage für den Rust-Neubau. Dieser ändert sie gezielt über eigene MODIFIED-Deltas (z.B. `post-images`, `image-processing`, `url-scheme`).
