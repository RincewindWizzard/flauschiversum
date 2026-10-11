# Spec Delta

## MODIFIED Requirements

### Requirement: Bildgröße im Post
Jedes im Markdown eines Posts eingebundene Bild SHALL in der Größe `700x` (Preview) angezeigt werden. Zusätzlich wird `1000x` per `srcset` für hochauflösende Bildschirme angeboten. Das Bild trägt Breite und Höhe, damit die Seite beim Laden nicht springt, und wird außer beim ersten Bild der Seite erst bei Bedarf geladen (`loading="lazy"`).

#### Scenario: Bild im Text
- **WHEN** der Post unter `/2014/01/alraune/` `![Ohrenschützer](DSCF0531.jpg)` enthält
- **THEN** zeigt die Seite das Bild `/2014/01/alraune/700x/DSCF0531.jpg` und bietet `/2014/01/alraune/1000x/DSCF0531.jpg` im `srcset` an

#### Scenario: Späteres Bild lädt verzögert
- **WHEN** ein Post zehn Bilder enthält
- **THEN** tragen alle Bilder außer dem ersten `loading="lazy"`

### Requirement: Bildbeschreibung
Der Alt-Text eines Bildes SHALL als `alt`-Attribut erhalten bleiben und als sichtbare Bildunterschrift erscheinen, auch in der Vollbildansicht. Ein leerer Alt-Text erzeugt keine Bildunterschrift.

#### Scenario: Beschreibung sichtbar
- **WHEN** das Bild `![Alraune aus Topf gezogen](DSCF0529.jpg)` angezeigt wird
- **THEN** steht „Alraune aus Topf gezogen“ als Bildunterschrift am Bild und in dessen Vollbildansicht

### Requirement: Text neben Bildern
Steht Text im selben Absatz wie Bilder, SHALL der Text als Absatz erhalten bleiben. Das Bild bzw. Karussell folgt direkt nach diesem Absatz.

#### Scenario: Gemischter Absatz
- **WHEN** ein Absatz aus einer Textzeile und zwei direkt folgenden Bildzeilen besteht
- **THEN** erscheint zuerst der Text und direkt danach ein Karussell mit beiden Bildern

## ADDED Requirements

### Requirement: Einzelbild
Eine Bildgruppe mit genau einem Bild SHALL als einzelnes Bild mit Bildunterschrift dargestellt werden, ohne Karussell-Bedienelemente.

#### Scenario: Einzelnes Bild
- **WHEN** ein Bild allein in einem Absatz steht
- **THEN** wird es als einzelnes Bild ohne Pfeile oder Punkte angezeigt

### Requirement: Karussell je Bildgruppe
Eine Bildgruppe mit mehreren Bildern SHALL als horizontales Karussell in der Reihenfolge aus dem Markdown dargestellt werden. Auf Touch-Geräten blättert man durch Wischen, sonst über Pfeile. Punkte zeigen Anzahl und Position und erlauben den direkten Sprung. Es wechselt nicht automatisch.

#### Scenario: Wischen auf dem Handy
- **WHEN** auf dem Handy in einem Karussell mit vier Bildern nach links gewischt wird
- **THEN** wird das zweite Bild angezeigt, und der zweite Punkt ist markiert

#### Scenario: Pfeile am Desktop
- **WHEN** am Desktop im Karussell auf den Pfeil nach rechts geklickt wird
- **THEN** wird das nächste Bild angezeigt

#### Scenario: Kein automatischer Wechsel
- **WHEN** ein Karussell ohne Interaktion angezeigt wird
- **THEN** bleibt das erste Bild sichtbar

### Requirement: Vollbildansicht
Ein Klick oder Tippen auf ein Bild (einzeln oder im Karussell) SHALL dieses Bild in der Größe `1000x` als Vollbild öffnen. Ist es Teil einer Gruppe, kann man im Vollbild durch die ganze Gruppe blättern (Wischen, Pfeile, Pfeiltasten). Auf Touch-Geräten ist Zoomen mit zwei Fingern möglich. Schließen geht über Schaltfläche, Escape-Taste, Wischen nach unten oder die Zurück-Taste des Browsers.

#### Scenario: Vollbild aus dem Karussell
- **WHEN** im Karussell des Posts „Alraune“ auf das zweite Bild getippt wird
- **THEN** öffnet sich `1000x/DSCF0528.jpg` im Vollbild mit der Bildunterschrift „Alraune nah oben“, und durch Wischen erreicht man die übrigen drei Bilder

#### Scenario: Zurück-Taste schließt
- **WHEN** im Vollbild die Zurück-Taste des Handys gedrückt wird
- **THEN** schließt sich das Vollbild, und die Post-Seite bleibt an derselben Stelle

#### Scenario: Ohne JavaScript
- **WHEN** JavaScript deaktiviert ist und auf ein Bild geklickt wird
- **THEN** öffnet der Browser die Datei `1000x/<Dateiname>` direkt

### Requirement: Animationen und Videos im Post
Eine eingebundene Animation (GIF) SHALL im Original als Bild angezeigt werden. Ein eingebundenes Video SHALL als stumm und in Schleife automatisch abspielendes Inline-Video ohne Ton-Bedienelemente erscheinen, das auch auf dem Handy im Seitenfluss läuft statt im Vollbild. Wer reduzierte Bewegung eingestellt hat, startet es selbst.

#### Scenario: Video als Schleife
- **WHEN** ein Post `![Karteikartenfresser in Aktion](VID_20130115_184912.mp4)` enthält
- **THEN** läuft das Video an dieser Stelle stumm in Schleife, mit „Karteikartenfresser in Aktion“ als Beschreibung

#### Scenario: Reduzierte Bewegung
- **WHEN** im Betriebssystem „Bewegung reduzieren“ aktiviert ist
- **THEN** startet das Video nicht automatisch und lässt sich per Tippen abspielen

#### Scenario: Video in einer Bildgruppe
- **WHEN** ein Video direkt zwischen zwei Bildern ohne Leerzeile steht
- **THEN** ist es eine Station im Karussell dieser Gruppe

## REMOVED Requirements

### Requirement: Slideshow je Bildgruppe
**Reason**: Ersetzt durch „Einzelbild“ und „Karussell je Bildgruppe“. Ein einzelnes Bild wird nicht mehr als Slideshow mit einer Folie dargestellt.
**Migration**: Keine Änderung an Inhalten nötig. Bildgruppen im Markdown werden automatisch als Einzelbild oder Karussell dargestellt.

### Requirement: Bedienung der Slideshow
**Reason**: Ersetzt durch „Karussell je Bildgruppe“ (Wischen, Pfeile, Punkte) und „Vollbildansicht“. Die bisherige jQuery-Slideshow entfällt.
**Migration**: Keine Änderung an Inhalten nötig.
