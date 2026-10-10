# Tasks

## 1. Inhalte vorbereiten

- [x] 1.1 `src/` per `git mv` nach `content/` verschieben (eigener Commit, LESS-Dateien vorerst mit); prüfen: `git show --stat` zeigt nur Umbenennungen
- [x] 1.2 Liste der 194 Live-URLs als `tests/fixtures/live-urls.txt` ablegen; prüfen: 194 Zeilen, alle im Format `/JJJJ/MM/slug/`
- [x] 1.3 Migrationsskript ausführen, das jedem Post seinen `permalink` aus der Liste als einzelne Zeile nach `title:` einfügt; prüfen: `git diff --numstat` zeigt 194 Dateien mit je `1 0`, jede Live-URL kommt genau einmal vor
- [x] 1.4 `category: naehen` in 5 Posts auf `nähen` ändern; prüfen: `grep -r "category: naehen" content/` liefert nichts
- [x] 1.5 Bei den 82 Originalen mit veraltetem Orientation-Tag (3, 6, 8) den Wert per Byte-Patch auf 1 setzen; prüfen: dekodierte Pixel vorher und nachher identisch, keine Datei mit Orientation ≠ 1 mehr unter `content/posts`

## 2. Projektgerüst und Inhaltsmodell

- [x] 2.1 Cargo-Projekt `flauschiversum` mit `clap`-CLI (`build`, `check`, `serve`, Optionen `--root`, `--out`, `--cache`) anlegen; prüfen: `cargo run -- --help` listet die drei Befehle
- [x] 2.2 Laden von Posts, Seiten und Medien aus `content/` mit Frontmatter-Parsing, Datum in Europe/Berlin, „veröffentlicht“-Logik und Medienklassifizierung (Bild/Animation/Video/ignoriert) umsetzen; prüfen: Unit-Tests für beide Datumsformate, Zeitzone, geplanten Post und alle Dateiendungen
- [x] 2.3 Validierung (Pflichtfelder, Format, existierendes `image`, doppelte URLs) mit deutschen Fehlermeldungen samt Pfad und Feld umsetzen; prüfen: Unit-Tests für jeden Fehlerfall prüfen den Meldungstext
- [x] 2.4 URL-Bildung (Permalink, Slug aus Verzeichnisname mit `ue`-Transliteration, Medien-URLs, Prozentkodierung) umsetzen; prüfen: Unit-Tests mit `grüne-mütze`, `Schwacher Moment.jpg`, `kürbis_thumb.jpg`
- [x] 2.5 `flauschiversum check` auf `content/` ausführbar machen; prüfen: Exit-Code 0, 194 Posts und 3 Seiten erkannt

## 3. Bildverarbeitung

- [x] 3.1 Badge-Vorlage `assets/badge.png` nach den Parametern aus dem Design erzeugen und einchecken; prüfen: Vergleichsbild auf hellem und dunklem Foto entspricht dem Entwurf V3
- [x] 3.2 Dekodieren, EXIF-Orientierung, kaskadiertes Skalieren (1000/700/200, nie vergrößern) und Badge-Komposition umsetzen; prüfen: Unit-Tests für Abmessungen (562×750, 4592×3448, Orientation 6), Badge-Breite ≈ 1/5, kein Badge bei `200x`
- [x] 3.3 Kodieren im Originalformat (JPEG, PNG, WebP, statisches GIF) mit 1-MB-Grenze für `1000x` umsetzen; prüfen: Unit-Test mit großem Testbild hält die Grenze ein, Ausgabe-Magic-Bytes passen zur Endung
- [x] 3.4 Rechte-Metadaten (EXIF Artist/Copyright, XMP, IPTC bei JPEG; EXIF+XMP bei PNG/WebP) erzeugen und einsetzen; prüfen: Test liest die Felder mit `kamadak-exif` zurück, Test bestätigt, dass GPS eines Originals nicht in der Ausgabe steht
- [x] 3.5 Animierte GIFs und Videos unverändert durchreichen, Warnung über 20 MB ausgeben; prüfen: Unit-Test für Frame-Erkennung, Integrationstest auf Byte-Gleichheit
- [x] 3.6 Cache mit vollständigem Schlüssel und parallele Verarbeitung (rayon) umsetzen; prüfen: Test „zweiter Lauf dekodiert nichts“, Test „geänderte Badge-Vorlage invalidiert nur 700/1000“; Kaltstart aller Bilder auf 4 Kernen gemessen < 2 min

## 4. Markdown und Seiten

- [x] 4.1 Markdown-Rendering mit Erkennung von Mediengruppen umsetzen (Einzelbild → `figure`, Gruppe → Karussell, Video → `video`, GIF → `img`, Text vor der Gruppe bleibt als Absatz, `srcset`, Breite/Höhe, `loading="lazy"` ab dem zweiten Bild, Warnung bei unbekannter Datei); prüfen: Snapshot-Tests für jeden Fall
- [x] 4.2 Templates nach Askama portieren (je Seitentyp ein Struct) (Layout mit SEO-Kopf, Index, Post mit Vor/Zurück-Navigation, Seite, Paginierung) und `h1`-Struktur umstellen; prüfen: Snapshot-Tests der Mini-Site, genau eine `h1` pro Post- und Einzelseite, ein absichtlich falscher Feldname in einem Template lässt `cargo build` scheitern
- [x] 4.3 Indexseiten mit Paginierung (nur veröffentlichte Posts, Fenster ±2, kein Phantom, Titel je Seite) erzeugen; prüfen: Unit-Tests für Seite 1, 10 und 49 von 49 sowie für den Fall nur einer Seite
- [x] 4.4 RSS-Feed (20 neueste, `pubDate`, `dc:creator`, `content:encoded` mit absoluten URLs, `enclosure`, `atom:link`, stabiles `lastBuildDate`, GUID unverändert) erzeugen; prüfen: Snapshot-Test sowie Prüfung des Feeds der Gesamtseite mit einem RSS-Validator
- [x] 4.5 SEO-Ausgaben (Sitemap, robots.txt, Meta-Description mit Kürzung, Canonical, Open Graph, Twitter-Card, JSON-LD `BlogPosting`, Feed-Link im Kopf) erzeugen; prüfen: Snapshot-Tests, Unit-Test der Kürzung an Wortgrenzen, JSON-LD parst als gültiges JSON
- [x] 4.6 Prüfung interner Links in `check` und `build` umsetzen; prüfen: Integrationstest mit kaputtem Link liefert Warnung mit Post und Ziel

## 5. Frontend

- [x] 5.1 LESS nach Standard-CSS unter `content/style/` überführen (Custom Properties, natives Nesting, relative Farbsyntax statt `lighten()`) und LESS-Dateien entfernen; Bündeln und Minimieren mit `lightningcss`, kein Präprozessor; prüfen: Screenshot-Vergleich Start-, Post- und About-Seite gegen die Live-Seite
- [x] 5.2 CSS für `figure.media`, Karussell (Scroll-Snap) und Video ergänzen; prüfen: Karussell ist ohne JavaScript per Wischen und Scrollen bedienbar (Browser-Test mit deaktiviertem JS)
- [x] 5.3 `assets/js/carousel.js` (Pfeile, Punkte, Tastatur, Position) schreiben; prüfen: Browser-Test in Handy- und Desktopgröße mit Playwright
- [x] 5.4 PhotoSwipe 5 unter `assets/vendor/photoswipe/` einbinden (Lizenzdatei mitliefern), Galerie je Gruppe, Bildunterschriften, Zurück-Taste schließt; prüfen: Playwright-Test öffnet das Vollbild, blättert und schließt es mit Escape und der Zurück-Navigation
- [x] 5.5 Behandlung von `prefers-reduced-motion` für Videos und Cache-Busting-Parameter für CSS und JS umsetzen; prüfen: Playwright-Test mit emulierter reduzierter Bewegung (Video pausiert), Snapshot zeigt `?v=` mit Inhalts-Hash

## 6. Ausgabe und Vorschau

- [x] 6.1 Writer mit Manifest, „nur geänderte Dateien schreiben“, atomarem Schreiben, Löschen verwaister Dateien und leerer Verzeichnisse sowie Schutz vor fremdem `--out` umsetzen; prüfen: Integrationstests „zweiter Build: 0 geschrieben, 0 gelöscht“, „gelöschter Post verschwindet“, „fremdes Verzeichnis wird nicht angefasst“
- [x] 6.2 Zusammenfassung am Ende des Builds (neu, geändert, unverändert, gelöscht, Warnungen, Dauer) ausgeben; prüfen: Integrationstest liest die Zahlen aus der Ausgabe
- [x] 6.3 `flauschiversum serve` mit Auslieferung, Dateibeobachtung und Live-Reload umsetzen; prüfen: manueller Test, bei dem eine Änderung an `index.md` innerhalb weniger Sekunden im Browser erscheint

## 7. Gesamtabnahme und Aufräumen

- [x] 7.1 Gesamtseiten-Test (`cargo test -- --ignored`): alle 194 Live-URLs existieren, alle Medien-Referenzen lösen auf, `check` ohne Fehler; prüfen: Test grün
- [ ] 7.2 Vergleichsgrafik Wasserzeichen alt/neu und Screenshots alt/neu zur Abnahme vorlegen; prüfen: Freigabe im Gespräch
- [x] 7.3 `blogcompile/`, `main.py`, `settings.py`, `requirements.txt`, `test/`, `docker/`, `docker-compose.yml`, `.dockerignore` und alte Skripte entfernen, `.gitignore` um `dist/` und `.cache/` ergänzen; prüfen: `cargo build --release` und `cargo test` grün, `git ls-files '*.py'` leer
- [x] 7.4 `README.md` (Aufruf, Verzeichnisse, Inhalte schreiben, Medienformate) und `openspec/config.yaml` (Projektkontext) auf den neuen Stack aktualisieren; prüfen: die Befehle aus der README laufen wie beschrieben

## Workflow follow-up

- Nach der Abnahme `/opsx:archive rust-neubau`, damit die Specs aktualisiert werden.
- Danach einen eigenen Change für das Deployment als GitHub Action mit SSH-Push planen.
