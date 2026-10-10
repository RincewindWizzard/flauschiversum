# Spec Delta

## Purpose

Beschreibt, wie die im Markdown eines Posts eingebundenen Bilder auf der Post-Seite dargestellt werden: Bildgröße, Gruppierung zu Slideshows und Bildbeschreibungen.

## ADDED Requirements

### Requirement: Bildgröße im Post
Jedes im Markdown eines Posts eingebundene Bild SHALL in der Größe `700x` angezeigt werden, nicht im Original.

#### Scenario: Bild im Text
- **WHEN** der Post unter `/2014/01/alraune/` `![Ohrenschützer](DSCF0531.jpg)` enthält
- **THEN** zeigt die Seite das Bild `/2014/01/alraune/700x/DSCF0531.jpg`

### Requirement: Slideshow je Bildgruppe
Jede Bildgruppe (siehe `content-format`) SHALL als eine Slideshow dargestellt werden, in der jeweils ein Bild sichtbar ist. Das gilt auch für eine Gruppe mit nur einem Bild. Die Bilder erscheinen in der Reihenfolge aus dem Markdown.

#### Scenario: Gruppe mit vier Bildern
- **WHEN** ein Post vier Bilder ohne Leerzeile hintereinander enthält
- **THEN** zeigt die Seite an dieser Stelle eine Slideshow mit vier Bildern, von denen das erste sichtbar ist

#### Scenario: Einzelnes Bild
- **WHEN** ein Bild allein in einem Absatz steht
- **THEN** wird es als Slideshow mit einem Bild dargestellt

### Requirement: Bedienung der Slideshow
Eine Slideshow SHALL Schaltflächen für das vorherige und das nächste Bild sowie eine Leiste zur direkten Bildauswahl haben. Sie wechselt nicht automatisch und ohne Überblendung.

#### Scenario: Weiterblättern
- **WHEN** in einer Slideshow mit vier Bildern auf „weiter“ geklickt wird
- **THEN** wird sofort das zweite Bild angezeigt

#### Scenario: Kein automatischer Wechsel
- **WHEN** eine Slideshow ohne Interaktion angezeigt wird
- **THEN** bleibt das erste Bild sichtbar

### Requirement: Bildbeschreibung
Der Alt-Text eines Bildes SHALL als Beschreibung zum jeweiligen Bild der Slideshow angezeigt werden und als `alt`-Attribut des Bildes erhalten bleiben.

#### Scenario: Beschreibung sichtbar
- **WHEN** das Bild `![Alraune aus Topf gezogen](DSCF0529.jpg)` in der Slideshow angezeigt wird
- **THEN** steht „Alraune aus Topf gezogen“ als Beschreibung am Bild

### Requirement: Text neben Bildern
Steht Text im selben Absatz wie Bilder, SHALL der Text als Absatz erhalten bleiben und die Slideshow direkt nach diesem Absatz folgen.

#### Scenario: Gemischter Absatz
- **WHEN** ein Absatz aus einer Textzeile und zwei direkt folgenden Bildzeilen besteht
- **THEN** erscheint zuerst der Text und direkt danach eine Slideshow mit beiden Bildern
