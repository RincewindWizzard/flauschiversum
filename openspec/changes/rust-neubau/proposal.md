# Proposal

## Why

Der Python-Generator des Flauschiversums läuft mit aktuellen Bibliotheken nicht mehr, und das Deployment ist kaputt. Der Blog kann also weder gebaut noch aktualisiert werden. Ein Neubau in Rust soll ihn wieder benutzbar machen: als einzelnes Binary ohne Python- oder Node-Abhängigkeiten, parallel und inkrementell, damit ein Build schnell ist. Gleichzeitig werden die bekannten Fehler aus dem Change `status-quo-dokumentieren` behoben. Oberste Regel ist, dass sich keine bestehende URL ändert.

## What Changes

- **Neuer Generator in Rust** (`cargo run -- build`). Er ersetzt `blogcompile/`, `main.py`, `settings.py`, `requirements.txt` und die LESS-Toolchain. Diese werden entfernt, ebenso das kaputte `docker/`-Deployment. **BREAKING** für den Aufruf: `python3 main.py …` wird ersetzt durch `flauschiversum build …`.
- **Inhalte ziehen von `src/` nach `content/`.** Der Rust-Code liegt in `src/`, die Templates bleiben in `templates/`. Die URLs sind davon nicht betroffen.
- **Permalinks:** Jeder der 194 Posts bekommt im Frontmatter ein Feld `permalink` mit seiner bisherigen Live-URL. Die URL kommt künftig aus diesem Feld. Neue Posts ohne `permalink` erhalten eine URL aus Datum und Verzeichnisname.
- **Mehr Medienformate:** Als Bilder werden `.jpg`, `.jpeg`, `.png` und `.webp` verarbeitet (in allen Schreibweisen). Animierte `.gif` sowie Kurzvideos `.mp4` und `.webm` werden unverändert ausgeliefert. Im Post erscheinen Videos als stumm laufende Schleife, wie früher GIFs.
- **Neue Bilddarstellung:** Ein einzelnes Bild wird als Preview gezeigt, ein Klick öffnet Full im Vollbild. Mehrere Bilder hintereinander bilden ein wischbares Karussell, ebenfalls mit Vollbild. Das funktioniert auf dem Handy und ohne jQuery.
- **Neues Wasserzeichen:** Ein dezentes Badge „Flauschiversum“ (Lobster-Schrift, Themengrün, halbtransparent), das sich nach der Ausgabegröße richtet. Das Vorschaubild `200x` bleibt ohne Badge.
- **Metadaten:** EXIF, XMP und IPTC nennen pauschal „Flauschiversum / Ermeline Wollknoll“ als Urheber. GPS- und Kameradaten werden nie ausgeliefert.
- **Inkrementeller Build:** Bildvarianten werden mit einem Schlüssel aus allen Eingaben gecacht. Geschrieben werden nur geänderte Dateien. Was nicht mehr erzeugt wird, wird entfernt (behebt das Liegenbleiben alter Dateien).
- **SEO:** `sitemap.xml`, `robots.txt`, aussagekräftige Seitentitel, genau eine `<h1>` pro Seite, Links zum vorherigen und nächsten Post, Meta-Description, Canonical-URL, Open-Graph-Vorschauen und strukturierte Daten (`BlogPosting`). Interne Links werden beim Build geprüft.
- **RSS-Feed:** Neueste zuerst, mit `pubDate`, Autor, Vorschaubild und vollem Inhalt. Nicht veröffentlichte Posts erscheinen nicht. Die bisherigen GUIDs bleiben erhalten.
- **Bugfixes:** Zukünftig datierte Posts werden gar nicht erzeugt. Die Paginierung zeigt keine Phantomseite mehr. Indexseiten haben einen Titel. URLs werden korrekt kodiert. `category` bleibt ohne Wirkung, wird aber vereinheitlicht (`naehen` → `nähen`).
- **Stylesheet:** Die LESS-Dateien werden einmalig in normales CSS überführt (CSS-Variablen, natives Nesting). Gestaltung und Farben bleiben gleich.
- **Vorschau-Server:** `flauschiversum serve` baut bei jeder Änderung neu und zeigt die Seite lokal an.

Nicht Teil dieses Changes: das neue Deployment als GitHub Action (eigener Change), Kategorieseiten, Suche, Kommentare sowie das Umkodieren von Videos.

## Capabilities

### New Capabilities

- `seo`: Maschinenlesbare Informationen für Suchmaschinen und Vorschauen beim Teilen (Sitemap, robots.txt, Meta-Angaben, Open Graph, strukturierte Daten, Linkprüfung).

### Modified Capabilities

- `url-scheme`: Die Post-URL kommt aus `permalink`. Regel für neue Posts. URLs für neue Medienformate. `sitemap.xml` und `robots.txt`. Korrekte Kodierung.
- `content-format`: Verzeichnis `content/` statt `src/`, Feld `permalink`, weitere Bild- und Videoformate, `category` vereinheitlicht.
- `site-layout`: Seitentitel mit Blogname, auch auf Indexseiten. Kein jQuery. Statische Ressourcen mit Cache-Busting.
- `post-index`: Nicht veröffentlichte Posts beeinflussen die Paginierung nicht mehr. Die Paginierung endet an der letzten Seite.
- `post-page`: Zukünftig datierte Posts werden nicht erzeugt.
- `post-images`: Einzelbild mit Vollbild, Karussell statt Slideshow, Videos und GIFs, `srcset`.
- `image-processing`: Badge-Wasserzeichen nach dem Skalieren, Rechte-Metadaten, Ausgabe im Format des Originals, Größengrenze für Full, vollständiger Cache-Schlüssel.
- `rss-feed`: Sortierung, `pubDate`, Autor, Inhalt, nur veröffentlichte Posts.
- `build`: Neuer Aufruf, paralleler und inkrementeller Build, Entfernen verwaister Dateien, Prüfung der Inhalte, Vorschau-Server.

## Impact

- **Code:** `blogcompile/`, `main.py`, `settings.py`, `requirements.txt`, `test/`, `src/style/` (LESS) und `docker/` sowie `docker-compose.yml` entfallen. Neu sind `Cargo.toml` und `src/` (Rust), dazu `templates/` (an MiniJinja angepasst) und `content/` (alle Inhalte, verschoben).
- **Inhalte:** 194 `index.md` erhalten je eine Zeile `permalink:`, sonst bleiben sie unverändert. 5 Posts: `category: naehen` → `nähen`. Bei 82 Originalbildern ist der EXIF-Drehungs-Tag veraltet: Die Pixel sind schon aufrecht, aber der Tag verlangt eine Drehung. Er wird verlustfrei auf „normal“ zurückgesetzt, damit die neue Pipeline Drehungen korrekt anwenden kann, ohne diese Bilder schief darzustellen.
- **Abhängigkeiten:** Rust-Toolchain (stable) statt Python 3, Pillow und Node/lessc. Zur Laufzeit gibt es keine externen Programme.
- **Ausgabe:** Alle bestehenden URLs bleiben erhalten. Bildvarianten bekommen unter denselben URLs neuen Inhalt (neues Wasserzeichen, bessere Kompression). Entfernt werden `/script/jquery.js`, `/script/responsiveslides.min.js`, `/script/slideshow.js` und `/images/nav_btn.gif`. Das sind keine Inhalts-URLs. Neu hinzu kommen `/sitemap.xml`, `/robots.txt`, die Originale animierter GIFs und Videos sowie die Skripte für Karussell und Vollbild.
- **Projektkontext:** `openspec/config.yaml` und `README.md` werden auf den neuen Stack aktualisiert.
