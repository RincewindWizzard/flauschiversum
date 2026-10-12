# color-themes Specification

## Purpose

Beschreibt das Farbsystem der Seite: einen hellen und einen dunklen Modus des Schemas „Flauschi-Nebel“, deren Umschaltung, die Barrierefreiheit der Farben und das Universum als Stilelement.

## Requirements

### Requirement: Hell und Dunkel
Die Seite SHALL einen hellen und einen dunklen Modus bieten. Standard ist die Systemeinstellung des Besuchers (`prefers-color-scheme`); ohne JavaScript gilt ausschließlich sie. Alle Farben stammen aus Variablen, die für beide Modi an einer Stelle definiert sind. Im Dunkelmodus haben Seite, Karten, Menüs, Suche, Archiv, Tag-Cloud, Fußzeile und Lightbox dunkle Hintergründe und helle Schrift; Fotos bleiben unverändert.

#### Scenario: System dunkel
- **WHEN** ein Besucher mit Systemeinstellung „dunkel“ eine beliebige Seite öffnet
- **THEN** erscheint die Seite im Dunkelmodus, ohne dass ein Hintergrund kurz hell aufblitzt

#### Scenario: System hell
- **WHEN** ein Besucher mit Systemeinstellung „hell“ oder ohne Angabe eine Seite öffnet
- **THEN** erscheint der helle Modus

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet wird
- **THEN** folgt sie der Systemeinstellung, und alle Inhalte sind lesbar

### Requirement: Umschalter in der Topbar
Die Topbar SHALL einen Umschalter mit den Stufen „Automatisch“, „Hell“ und „Dunkel“ enthalten. Die Wahl gilt sofort, wird im Browser gespeichert und bei späteren Besuchen angewendet. Der Umschalter ist per Tastatur bedienbar, hat einen zugänglichen Namen, zeigt seinen Zustand (nicht nur über die Farbe) und eine Klickfläche von mindestens 44 px. Ohne JavaScript entfällt er.

#### Scenario: Dunkel wählen
- **WHEN** ein Besucher mit heller Systemeinstellung „Dunkel“ wählt und die Seite neu lädt
- **THEN** bleibt der Dunkelmodus aktiv

#### Scenario: Automatisch
- **WHEN** der Besucher „Automatisch“ wählt
- **THEN** folgt die Seite wieder der Systemeinstellung

#### Scenario: Speicher nicht verfügbar
- **WHEN** der Browser das Speichern verweigert
- **THEN** wirkt die Wahl auf der aktuellen Seite, und es gibt keinen Fehler

### Requirement: Kontrast in beiden Modi
In beiden Modi SHALL Text auf seinem Hintergrund mindestens 4,5 : 1 Kontrast haben (Fließtext, Links, grauer Text, Knopftext, Tags), große Schrift und Bedienelemente (Rand von Eingabefeldern, Akzentflächen) mindestens 3 : 1.

#### Scenario: Kontrast beider Modi
- **WHEN** die Farbpaare Text/Hintergrund, Link/Karte, Knopftext/Akzent, grauer Text/Karte beider Modi geprüft werden
- **THEN** erreicht jedes mindestens 4,5 : 1

#### Scenario: Bedienelemente
- **WHEN** Rand und Akzent gegen die Karte geprüft werden
- **THEN** erreichen sie mindestens 3 : 1

### Requirement: Nichts allein über Farbe
Bedeutung SHALL nie allein über den Farbton vermittelt werden: Links sind unterstrichen oder durch Form erkennbar, aktive Zustände unterscheiden sich durch Füllung, Gewicht oder Symbol. Es gibt keine Rot-Grün-Paare als Bedeutungsträger. Der Link-Kontrast auf Karten bleibt bei Simulation von Protanopie, Deuteranopie und Tritanopie mindestens 4,5 : 1.

#### Scenario: Links ohne Farbe erkennbar
- **WHEN** die Seite in Graustufen angezeigt wird
- **THEN** sind Links im Text anhand von Unterstreichung und Helligkeit vom übrigen Text unterscheidbar

#### Scenario: Farbfehlsichtigkeit
- **WHEN** die Seite mit Simulation von Deuteranopie angesehen wird
- **THEN** lassen sich Link, Text, aktive Elemente und Knöpfe weiterhin unterscheiden

### Requirement: Universum als Stilelement
Die Seite SHALL einen dezenten Sternenhimmel mit Nebelglühen als Hintergrunddekoration zeigen, im Dunkelmodus deutlich, im Hellmodus nur zart. Die Dekoration ist rein dekorativ, liegt hinter allen Inhalten, fängt keine Mausereignisse ab, bewegt sich nicht und berührt keine Textfläche: Text steht immer auf einfarbigem Hintergrund oder auf einer einfarbigen Karte. Bei `prefers-contrast: more` und erzwungenen Farben entfällt sie.

#### Scenario: Sterne im Dunkelmodus
- **WHEN** eine Seite im Dunkelmodus geöffnet wird
- **THEN** sind Sterne und Nebel im Hintergrund außerhalb der Karten sichtbar

#### Scenario: Text auf ruhigem Grund
- **WHEN** der Text eines Beitrags im Dunkelmodus gelesen wird
- **THEN** liegt er auf einer einfarbigen Fläche ohne Sterne

#### Scenario: Erhöhter Kontrast
- **WHEN** das System `prefers-contrast: more` meldet
- **THEN** zeigt die Seite keine Sterne und keinen Nebel

### Requirement: Alle Farben an einer Stelle
Farbwerte SHALL nur in der Datei mit den Farbvariablen stehen; andere Stylesheets verwenden ausschließlich diese Variablen oder Ableitungen davon.

#### Scenario: Keine festen Farben
- **WHEN** alle Stylesheets außer der Farbdatei nach Hex-Farben durchsucht werden
- **THEN** findet sich keine (ausgenommen die Staging-Leiste)

### Requirement: Suche und Lightbox im Dunkelmodus
Die Suchoberfläche (Pagefind) und die Vollbildansicht SHALL im Dunkelmodus ebenfalls dunkel sein und die Kontrastanforderungen erfüllen.

#### Scenario: Suchergebnisse dunkel
- **WHEN** im Dunkelmodus nach „Mütze“ gesucht wird
- **THEN** stehen Treffer, Filter und Eingabefeld auf dunklem Grund mit hellem Text
