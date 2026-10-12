## Purpose

Beschreibt das Favicon des Flauschiversums: das Maskottchen Flauschi in einer für kleine Größen vereinfachten Variante, in den Formaten, die Browser und Mobilgeräte verlangen.

## ADDED Requirements

### Requirement: Favicon aus dem Maskottchen
Das Favicon SHALL das normale Maskottchen Flauschi (nicht die überraschte Variante) in einer vereinfachten Fassung zeigen: Körper mit dem Farbverlauf des Maskottchens, zwei große Augen mit Pupillen, ohne Fransen, Mund und Kleindetails, auf transparentem Grund. Es ist bei 16 px als Maskottchen erkennbar (die Augen sind mindestens 3 px groß) und auf hellen wie dunklen Tab-Leisten sichtbar.

#### Scenario: Erkennbar bei 16 px
- **WHEN** das Favicon bei 16 × 16 px gerendert wird
- **THEN** sind Körper und beide Augen als getrennte Flächen erkennbar, und der Körper hat die violett-indigoblauen Töne des Maskottchens

#### Scenario: Auf dunklem Grund
- **WHEN** das Favicon auf einen fast schwarzen Grund gelegt wird
- **THEN** bleibt die Silhouette durch den hellen Rand und die hellen Augen erkennbar

#### Scenario: Normale Variante
- **WHEN** Favicon und Maskottchen in der Topbar verglichen werden
- **THEN** entspricht das Favicon der normalen, nicht der überraschten Variante

### Requirement: Dateien und Formate
Der Auftritt SHALL das Favicon als `favicon.svg` (skalierbar), als `favicon.ico` (Bilder mit 16, 32 und 48 px) und als `apple-touch-icon.png` (180 × 180 px, mit deckendem hellem Hintergrund) ausliefern. `favicon.ico` bleibt unter seiner bisherigen Adresse erreichbar.

#### Scenario: ICO-Datei
- **WHEN** `/favicon.ico` abgerufen wird
- **THEN** liefert der Server eine gültige ICO-Datei mit Bildern in 16, 32 und 48 px

#### Scenario: Apple-Touch-Icon
- **WHEN** `/apple-touch-icon.png` abgerufen wird
- **THEN** liefert der Server ein PNG mit 180 × 180 px ohne transparente Pixel

#### Scenario: SVG
- **WHEN** `/favicon.svg` abgerufen wird
- **THEN** liefert der Server eine SVG-Datei mit `viewBox`, kleiner als 10 KB
