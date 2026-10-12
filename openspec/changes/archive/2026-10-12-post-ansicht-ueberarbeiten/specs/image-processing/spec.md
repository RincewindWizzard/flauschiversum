## MODIFIED Requirements

### Requirement: Sichtbares Wasserzeichen
Die Varianten `700x` und `1000x` SHALL unten rechts ein Badge tragen: der Schriftzug „Flauschiversum“ in Lobster, in Nebel-Indigo (Textfarbe des Farbschemas „Flauschi-Nebel“) auf einer halbtransparenten, abgerundeten Fläche in hellem Lavendel (Farbe `accent-soft`) mit feinem Rand in der Akzentfarbe. Das Badge ist für Hell- und Dunkelmodus gleich, etwa ein Fünftel so breit wie die Variante, hat zu den Rändern etwa 2,5 % Abstand und wird nach dem Skalieren aufgebracht. Die Variante `200x` trägt kein Badge. Grün- oder Graugrüntöne des früheren Themas kommen im Badge nicht mehr vor.

#### Scenario: Wasserzeichen auf kleinem Original
- **WHEN** von einem 562 Pixel breiten Original die `700x`-Variante (562 Pixel breit) erzeugt wird
- **THEN** ist das Badge etwa 112 Pixel breit, also ein Fünftel der Bildbreite

#### Scenario: Wasserzeichen auf großem Original
- **WHEN** von einem 4592 Pixel breiten Original die `700x`-Variante erzeugt wird
- **THEN** ist das Badge etwa 140 Pixel breit, also ebenfalls ein Fünftel der Bildbreite

#### Scenario: Dunkles Foto
- **WHEN** die rechte untere Ecke eines Fotos fast schwarz ist
- **THEN** ist das Badge durch seine helle lavendelfarbene Fläche dennoch erkennbar

#### Scenario: Vorschaubild
- **WHEN** die `200x`-Variante erzeugt wird
- **THEN** enthält sie kein Badge

#### Scenario: Farben des Badges
- **WHEN** das Badge-Bild geprüft wird
- **THEN** besteht es aus Lavendel (Fläche), Indigo (Schrift) und Akzentviolett (Rand), und der Kontrast zwischen Schrift und Fläche beträgt mindestens 7 : 1

#### Scenario: Helles Foto
- **WHEN** die rechte untere Ecke eines Fotos fast weiß ist
- **THEN** bleibt das Badge durch Schrift und Rand lesbar
