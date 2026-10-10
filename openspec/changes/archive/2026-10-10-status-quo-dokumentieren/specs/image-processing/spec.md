# Spec Delta

## Purpose

Beschreibt, wie aus den Originalbildern der Posts die ausgelieferten Bildvarianten entstehen: Größen, Wasserzeichen, Dateiformat, Umgang mit Metadaten und Wiederverwendung bereits berechneter Varianten.

## ADDED Requirements

### Requirement: Drei Größenvarianten
Für jedes Bild eines Posts SHALL je eine Variante mit höchstens 200, 700 und 1000 Pixel Breite erzeugt werden. Das Seitenverhältnis bleibt erhalten. Kleinere Originale werden nicht vergrößert. Varianten entstehen auch für Bilder, die im Markdown nicht eingebunden sind.

#### Scenario: Hochformat-Original
- **WHEN** das Original 562×750 Pixel groß ist
- **THEN** ist die `200x`-Variante 200×267 Pixel groß, und die `700x`- und `1000x`-Varianten sind 562×750 Pixel groß

#### Scenario: Großes Original
- **WHEN** das Original 4592×3448 Pixel groß ist
- **THEN** ist die `1000x`-Variante 1000 Pixel breit

### Requirement: Sichtbares Wasserzeichen
Jede Variante SHALL das Wasserzeichen `src/static/images/Wasserzeichen.png` tragen (diagonaler Schriftzug „Flauschiversum.de“, dunkelgrau, transparenter Hintergrund). Es wird bündig in die rechte untere Ecke des Originals gesetzt und nur verkleinert, wenn es größer als das Original ist. Erst danach wird skaliert.

#### Scenario: Wasserzeichen auf kleinem Original
- **WHEN** das Original 562×750 Pixel groß ist und das Wasserzeichen 279×277 Pixel
- **THEN** bedeckt das Wasserzeichen in jeder Variante etwa die Hälfte der Bildbreite in der rechten unteren Ecke

#### Scenario: Wasserzeichen auf großem Original
- **WHEN** das Original 4592 Pixel breit ist
- **THEN** nimmt das Wasserzeichen in jeder Variante nur etwa 6 % der Bildbreite ein

### Requirement: Ausgabeformat
Alle Varianten SHALL als JPEG ausgeliefert werden, unabhängig von der Dateiendung `.jpg` oder `.JPG` des Originals. Der Dateiname bleibt dabei gleich.

#### Scenario: Großgeschriebene Endung
- **WHEN** das Original `IMG_4568.JPG` heißt
- **THEN** ist die Variante `700x/IMG_4568.JPG` ein JPEG-Bild

### Requirement: Keine Kamera- und Standortdaten
Ausgelieferte Varianten SHALL keine EXIF-Metadaten der Originale enthalten, insbesondere keine GPS-Koordinaten, kein Kameramodell und keine Aufnahmezeit.

#### Scenario: Original mit GPS-Daten
- **WHEN** ein Original EXIF-Daten mit GPS-Koordinaten enthält
- **THEN** enthält keine seiner Varianten GPS-Koordinaten oder andere EXIF-Daten

### Requirement: Wiederverwendung berechneter Varianten
Varianten SHALL in einem Cache-Verzeichnis abgelegt werden, geschlüsselt nach dem Inhalt des Originals. Beim nächsten Build werden sie für unveränderte Originale wiederverwendet statt neu berechnet. Ändert sich der Inhalt eines Originals, werden seine Varianten neu erzeugt.

#### Scenario: Unverändertes Bild
- **WHEN** ein Build mit gefülltem Cache läuft und sich ein Original nicht geändert hat
- **THEN** werden seine Varianten aus dem Cache übernommen, ohne das Bild neu zu skalieren

#### Scenario: Geändertes Bild
- **WHEN** ein Original durch eine bearbeitete Fassung mit gleichem Dateinamen ersetzt wird
- **THEN** werden beim nächsten Build alle drei Varianten neu erzeugt
