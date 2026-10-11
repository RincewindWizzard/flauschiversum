# Spec Delta

## MODIFIED Requirements

### Requirement: Drei Größenvarianten
Für jedes Bild eines Posts SHALL je eine Variante mit höchstens 200 (Thumbnail), 700 (Preview) und 1000 (Full) Pixel Breite erzeugt werden. Das Seitenverhältnis bleibt erhalten. Kleinere Originale werden nicht vergrößert. Varianten entstehen auch für Bilder, die im Markdown nicht eingebunden sind. Die EXIF-Drehung (Orientation) des Originals wird vor dem Skalieren angewendet.

#### Scenario: Hochformat-Original
- **WHEN** das Original 562×750 Pixel groß ist
- **THEN** ist die `200x`-Variante 200×267 Pixel groß, und die `700x`- und `1000x`-Varianten sind 562×750 Pixel groß

#### Scenario: Großes Original
- **WHEN** das Original 4592×3448 Pixel groß ist
- **THEN** ist die `1000x`-Variante 1000 Pixel breit

#### Scenario: Handyfoto mit Drehung
- **WHEN** ein Original quer gespeichert ist und EXIF-Orientation 6 (90° im Uhrzeigersinn) trägt
- **THEN** sind alle Varianten aufrecht im Hochformat

### Requirement: Sichtbares Wasserzeichen
Die Varianten `700x` und `1000x` SHALL unten rechts ein Badge tragen: der Schriftzug „Flauschiversum“ in Lobster, dunkelgrau auf einer halbtransparenten, abgerundeten Fläche im Themengrün. Das Badge ist etwa ein Fünftel so breit wie die Variante, hat zu den Rändern etwa 2,5 % Abstand und wird nach dem Skalieren aufgebracht. Die Variante `200x` trägt kein Badge.

#### Scenario: Wasserzeichen auf kleinem Original
- **WHEN** von einem 562 Pixel breiten Original die `700x`-Variante (562 Pixel breit) erzeugt wird
- **THEN** ist das Badge etwa 112 Pixel breit, also ein Fünftel der Bildbreite

#### Scenario: Wasserzeichen auf großem Original
- **WHEN** von einem 4592 Pixel breiten Original die `700x`-Variante erzeugt wird
- **THEN** ist das Badge etwa 140 Pixel breit, also ebenfalls ein Fünftel der Bildbreite

#### Scenario: Dunkles Foto
- **WHEN** die rechte untere Ecke eines Fotos fast schwarz ist
- **THEN** ist das Badge durch seine helle grüne Fläche dennoch erkennbar

#### Scenario: Vorschaubild
- **WHEN** die `200x`-Variante erzeugt wird
- **THEN** enthält sie kein Badge

### Requirement: Ausgabeformat
Jede Variante SHALL im Format des Originals gespeichert werden (JPEG bleibt JPEG, PNG bleibt PNG, WebP bleibt WebP, GIF bleibt GIF), damit Dateiendung und Inhalt zusammenpassen. Eine `1000x`-Variante darf höchstens 1 MB groß sein. Verlustbehaftete Formate werden mit einer Qualität gespeichert, die das einhält. Lässt sich das bei verlustfreien Formaten nicht einhalten, warnt der Build.

#### Scenario: Großgeschriebene Endung
- **WHEN** das Original `IMG_4568.JPG` heißt
- **THEN** ist die Variante `700x/IMG_4568.JPG` ein JPEG-Bild

#### Scenario: Großes Handyfoto
- **WHEN** ein Original 4000×3000 Pixel groß und 6 MB schwer ist
- **THEN** ist seine `1000x`-Variante höchstens 1 MB groß

### Requirement: Keine Kamera- und Standortdaten
Ausgelieferte Varianten SHALL keine Metadaten der Originale enthalten, insbesondere keine GPS-Koordinaten, kein Kameramodell, keine Seriennummer, keine Aufnahmezeit und keine eingebetteten Vorschaubilder. Animationen und Videos werden unverändert ausgeliefert und sind davon ausgenommen.

#### Scenario: Original mit GPS-Daten
- **WHEN** ein Original EXIF-Daten mit GPS-Koordinaten enthält
- **THEN** enthält keine seiner Varianten GPS-Koordinaten oder andere Metadaten des Originals

### Requirement: Wiederverwendung berechneter Varianten
Varianten SHALL in einem Cache-Verzeichnis abgelegt werden. Der Schlüssel ergibt sich aus dem Inhalt des Originals, dem Inhalt der Badge-Vorlage, Zielgröße, Qualitätsstufe, Metadaten-Text und einer Versionsnummer des Verfahrens. Unveränderte Varianten werden wiederverwendet. Ändert sich eine dieser Eingaben, wird die Variante neu erzeugt.

#### Scenario: Unverändertes Bild
- **WHEN** ein Build mit gefülltem Cache läuft und sich nichts geändert hat
- **THEN** werden alle Varianten aus dem Cache übernommen, ohne ein Bild zu dekodieren

#### Scenario: Geändertes Bild
- **WHEN** ein Original durch eine bearbeitete Fassung mit gleichem Dateinamen ersetzt wird
- **THEN** werden beim nächsten Build seine drei Varianten neu erzeugt

#### Scenario: Geändertes Wasserzeichen
- **WHEN** die Badge-Vorlage ersetzt wird
- **THEN** werden beim nächsten Build alle `700x`- und `1000x`-Varianten neu erzeugt, die `200x`-Varianten aber nicht

## ADDED Requirements

### Requirement: Rechteangaben in den Metadaten
Jede Variante SHALL Rechteangaben enthalten: als Urheber und Rechteinhaber „Flauschiversum / Ermeline Wollknoll“ und als Lizenzhinweis die URL `https://flauschiversum.de/impressum/`. In JPEG-Dateien stehen sie in EXIF (Artist, Copyright), XMP (`dc:creator`, `dc:rights`, `xmpRights:WebStatement`) und IPTC (By-line, Copyright Notice, Credit). In PNG- und WebP-Dateien stehen sie in EXIF und XMP.

#### Scenario: Urheber im Bildbetrachter
- **WHEN** die Datei `700x/DSCF0527.jpg` in einem Programm geöffnet wird, das Metadaten anzeigt
- **THEN** zeigt es als Urheber „Flauschiversum / Ermeline Wollknoll“ und das Copyright an

### Requirement: Parallele Verarbeitung
Die Varianten verschiedener Bilder SHALL parallel über alle verfügbaren Prozessorkerne erzeugt werden. Jedes Original wird pro Build höchstens einmal dekodiert.

#### Scenario: Kaltstart
- **WHEN** alle 3195 Bilder ohne Cache auf einem Rechner mit 4 Kernen verarbeitet werden
- **THEN** ist die Bildverarbeitung in weniger als 2 Minuten abgeschlossen
