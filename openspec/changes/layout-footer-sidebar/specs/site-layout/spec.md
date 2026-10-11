# Spec Delta

## MODIFIED Requirements

### Requirement: Hauptnavigation
Jede Seite SHALL eine Navigation mit genau diesen Einträgen in dieser Reihenfolge haben: „Start“ (`/`), „Themen“ (`/themen/`) und „Beiträge“ (`/beitraege/`). Auf Fenstern unter 80 em steht sie als Leiste im Kopf; ab 80 em steht sie in der linken Spalte (siehe „Dreispaltiges Layout auf breiten Fenstern“) und der Kopf zeigt nur das Logo. „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“ stehen in der Fußzeile.

#### Scenario: Feed abonnieren
- **WHEN** in der Fußzeile „Abonnieren“ angeklickt wird
- **THEN** wird `/rss.xml` geöffnet

#### Scenario: Themen finden
- **WHEN** „Themen“ angeklickt wird
- **THEN** wird `/themen/` geöffnet

#### Scenario: Kopf ohne seltene Seiten
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält keine der Navigationen im Kopf oder in der linken Spalte ein „About“, „Impressum“, „Tipps und Tricks“ oder „Abonnieren“

#### Scenario: Navigation als Leiste im Kopf
- **WHEN** eine Seite bei 1100 px Breite geöffnet wird
- **THEN** stehen „Start“, „Themen“ und „Beiträge“ als Leiste im Kopf

### Requirement: Fußzeile
Jede Seite SHALL am Ende eine Fußzeile haben. Sie enthält die Links „Tipps und Tricks“ (`/tipps-und-tricks/`), „About“ (`/about/`), „Impressum“ (`/impressum/`) und „Abonnieren“ (`/rss.xml`), die Kontaktwege nach der Spec `contact-links` und den Hinweis „© <Jahr> Ermeline Wollknoll“. Das Jahr ist das Jahr des Builds. Die Fußzeile ist auch ohne JavaScript vollständig und bricht bei schmalen Fenstern um, ohne waagerechtes Scrollen zu verursachen.

#### Scenario: Jahr im Copyright
- **WHEN** die Seite im Jahr 2021 gebaut wurde
- **THEN** steht in der Fußzeile „© 2021 Ermeline Wollknoll“

#### Scenario: Seitenlinks in der Fußzeile
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Fußzeile die Links „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“ mit den genannten Zielen

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** sind alle Links der Fußzeile sichtbar und erreichbar, und die Seite scrollt nicht waagerecht

#### Scenario: Reihenfolge am Seitenende
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** steht die Fußzeile nach dem Hauptinhalt als letztes sichtbares Element

## ADDED Requirements

### Requirement: Dreispaltiges Layout auf breiten Fenstern
Ab 80 em Fensterbreite SHALL jede Seite außer der Suchseite und der Themenübersicht drei Spalten in einem Seitenrahmen von bis zu etwa 1400 px zeigen: links die Navigationsspalte (etwa 13 em), in der Mitte den Inhalt, rechts die Sidebar (etwa 16 em) mit der Tag-Cloud. Der Lesetext von Beiträgen und Einzelseiten hat höchstens etwa 40 em Breite und steht in der Mittelspalte; Kacheln und Listen füllen sie. Der Kopf zeigt dann nur das Logo mit Untertitel.

#### Scenario: Desktop
- **WHEN** eine Beitragsseite bei 1440 px Breite geöffnet wird
- **THEN** stehen links die Navigation mit Suchfeld und Kategorien, in der Mitte der Text mit höchstens etwa 40 em Breite und rechts die Tag-Cloud

#### Scenario: Liste auf dem Desktop
- **WHEN** eine Kategorieseite bei 1440 px Breite geöffnet wird
- **THEN** füllt die Liste der Beiträge die Mittelspalte zwischen den beiden Seitenspalten

#### Scenario: Kopf nur mit Logo
- **WHEN** eine beliebige Seite bei 1440 px Breite geöffnet wird
- **THEN** zeigt der Kopf das Logo mit Untertitel, aber keine Navigationsleiste und kein Suchfeld

### Requirement: Navigationsspalte wie ein Inhaltsverzeichnis
Die linke Spalte SHALL von oben nach unten das Suchfeld, die Links „Start“, „Themen“ und „Beiträge“ und darunter die Kategorien mit der Zahl der Beiträge zeigen. Sie bleibt beim Scrollen sichtbar, solange sie ins Fenster passt.

#### Scenario: Reihenfolge der linken Spalte
- **WHEN** eine beliebige Seite bei 1440 px Breite geöffnet wird
- **THEN** stehen in der linken Spalte „Start“, „Themen“ und „Beiträge“ und darunter die Kategorien mit der Zahl der Beiträge, jeweils als Link

#### Scenario: Spalte bleibt sichtbar
- **WHEN** auf einer langen Seite bei 1440 px Breite und 900 px Höhe gescrollt wird
- **THEN** bleibt die linke Spalte sichtbar

### Requirement: Zweispaltiges Layout auf mittleren Fenstern
Zwischen 64 em und 80 em Fensterbreite SHALL jede Seite außer der Suchseite und der Themenübersicht den Inhalt links und die Sidebar rechts zeigen (Rahmen bis etwa 1200 px); die Navigation steht als Leiste im Kopf, und die Sidebar enthält zusätzlich die Kategorien vor der Tag-Cloud.

#### Scenario: Tablet quer
- **WHEN** eine Seite bei 1100 px Breite geöffnet wird
- **THEN** stehen Inhalt links und Sidebar rechts, die Navigation steht im Kopf, und die Sidebar zeigt Kategorien und Tag-Cloud

### Requirement: Sidebar auf schmalen Fenstern und Ausnahmen
Auf Fenstern unter 64 em SHALL die Sidebar mit Kategorien und Tag-Cloud nach dem Inhalt und vor der Fußzeile stehen. Navigationsspalte und Sidebar stehen in der Quelle nach dem Hauptinhalt und sind als `aside` bzw. `nav` mit Beschriftung ausgezeichnet. Die Suchseite und die Themenübersicht haben weder Navigationsspalte noch Sidebar und sind einspaltig; Navigationsleiste und Suchfeld stehen dort im Kopf.

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** steht die Sidebar nach dem Inhalt und vor der Fußzeile, und die Seite scrollt nicht waagerecht

#### Scenario: Suche einspaltig
- **WHEN** die Suchseite bei 1440 px Breite geöffnet wird
- **THEN** gibt es keine Seitenspalten, und Filter und Trefferliste nutzen die Breite der Inhaltsspalte

#### Scenario: Themenübersicht
- **WHEN** `/themen/` bei 1440 px Breite geöffnet wird
- **THEN** gibt es keine Seitenspalten, weil Kategorien und Schlagworte im Inhalt stehen, und die Navigation steht im Kopf

### Requirement: Suchfeld auf breiten Fenstern
Ab 80 em SHALL das Suchfeld in der linken Spalte stehen statt im Kopf (außer auf Seiten ohne Seitenspalten); es gibt immer genau ein sichtbares Suchfeld je Seite.

#### Scenario: Ein Suchfeld
- **WHEN** eine Beitragsseite bei 1440 px oder bei 360 px Breite geöffnet wird
- **THEN** ist genau ein Suchfeld sichtbar und eine Suche öffnet `/suche/?q=<Begriff>`

### Requirement: Seitenspalten auf Karten
Die Navigationsspalte und die Blöcke der Sidebar (Kategorien, Tag-Cloud) SHALL auf Karten liegen: ein heller, leicht durchscheinender Hintergrund, abgerundete Ecken, ein feiner Rand oder Schatten und Innenabstand, sodass sie sich vom grünen Seitenhintergrund abheben. Der Kontrast von Text und Kartenhintergrund bleibt ausreichend.

#### Scenario: Karten sichtbar
- **WHEN** eine Beitragsseite bei 1440 px Breite geöffnet wird
- **THEN** liegen die Navigationsspalte links und die Tag-Cloud rechts jeweils auf einer Karte mit abgerundeten Ecken und hellem Hintergrund

#### Scenario: Karten auf dem Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** liegen die Blöcke der Sidebar unter dem Inhalt auf Karten, und die Seite scrollt nicht waagerecht

### Requirement: Suchfeld mit eingebauter Lupe
Jedes Suchfeld der Seite (Kopf, Navigationsspalte, Startseite, Suchseite ohne JavaScript) SHALL aus einem einzigen Eingabefeld bestehen, in dem der Suchknopf mit Lupen-Icon am rechten Rand integriert ist. Eingabefeld und Knopf werden nie getrennt umgebrochen. Der Knopf hat den zugänglichen Namen „Suchen“, und Enter schickt die Suche ab.

#### Scenario: Lupe im Feld
- **WHEN** ein Suchfeld bei beliebiger Breite angezeigt wird
- **THEN** sitzt die Lupe innerhalb des abgerundeten Feldes am rechten Rand, und es gibt keinen separaten Knopf daneben oder darunter

#### Scenario: Schmale Spalte
- **WHEN** das Suchfeld in der linken Spalte (13 em) oder auf einem Handy mit 360 px Breite angezeigt wird
- **THEN** bleiben Feld und Lupe in einer Zeile

#### Scenario: Suche abschicken
- **WHEN** Text eingegeben und Enter gedrückt oder die Lupe angeklickt wird
- **THEN** öffnet sich `/suche/?q=<Begriff>`

### Requirement: Typografie
Die Schrift „Lobster“ SHALL nur für das Logo und für Titel (Überschriften der Seiten, Abschnitte, Kacheln und Karten) verwendet werden. Navigationslinks, Tag-Cloud, Kategorienlisten, Knöpfe und Feldbeschriftungen SHALL in der Textschrift „Lora“ stehen, mindestens 0,9 em groß und, wo sie Navigation sind, halbfett statt in Zierschrift. Es gibt eine gemeinsame Skala mit festen Größen für Titel, Zwischentitel, Fließtext und Kleintext.

#### Scenario: Navigation in Textschrift
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** sind die Links in Kopfnavigation, linker Spalte und Fußzeile nicht in Lobster gesetzt

#### Scenario: Titel in Lobster
- **WHEN** die Startseite geöffnet wird
- **THEN** sind Logo und die Abschnittstitel „Kategorien“ und „Neu im Flauschiversum“ in Lobster gesetzt

#### Scenario: Lesbarkeit im Footer
- **WHEN** die Fußzeile angezeigt wird
- **THEN** haben ihre Links mindestens 0,9 em Schriftgröße und die Textschrift

### Requirement: Farbschema
Die Seite SHALL ein gedämpftes Schema mit Salbeigrün als Akzent verwenden: Hintergrund Cremeweiß (`#faf7f0`), Karten weiß, Text `#2b2b2b`, Links dunkelgrün (`#2f5d3a`), Akzent `#6f9a5b` für Knöpfe und aktive Elemente, in heller Tönung für Hover. Alle Farben sind als Variablen an einer Stelle definiert, Text und Links erreichen mindestens 4,5 : 1 Kontrast, das frühere Neongrün entfällt.

#### Scenario: Hintergrund und Karten
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** ist der Seitenhintergrund Cremeweiß, Karten sind heller als der Hintergrund, und es gibt keinen leuchtend grünen Hintergrund

#### Scenario: Kontrast
- **WHEN** Fließtext, Links, Knöpfe und Tags auf ihren Hintergründen geprüft werden
- **THEN** beträgt das Kontrastverhältnis mindestens 4,5 : 1

#### Scenario: Staging-Leiste bleibt auffällig
- **WHEN** eine Staging-Seite geöffnet wird
- **THEN** hebt sich die Staging-Leiste mit ihrer roten Farbe weiterhin deutlich vom neuen Schema ab
