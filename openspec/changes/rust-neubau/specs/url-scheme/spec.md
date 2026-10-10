# Spec Delta

## MODIFIED Requirements

### Requirement: Post-URL
Jeder Post SHALL unter der URL erreichbar sein, die im Frontmatter-Feld `permalink` steht. Fehlt `permalink`, ist die URL `/<JJJJ>/<MM>/<slug>/`. Jahr und Monat kommen dann aus `date`, der Slug aus dem Namen des Post-Verzeichnisses (kleingeschrieben, `ä`→`ae`, `ö`→`oe`, `ü`→`ue`, `ß`→`ss`, alle anderen Zeichen außer `a-z0-9` durch `-` ersetzt).

#### Scenario: Umlaut im Titel
- **WHEN** ein Post den Titel „Kürbis“ und `permalink: /2020/10/kurbis/` hat
- **THEN** ist er unter `/2020/10/kurbis/` erreichbar

#### Scenario: ß im Titel
- **WHEN** ein Post den Titel „ein weißer Magierhut“ und `permalink: /2018/02/ein-weisser-magierhut/` hat
- **THEN** ist er unter `/2018/02/ein-weisser-magierhut/` erreichbar

#### Scenario: Sonderzeichen im Titel
- **WHEN** ein Post den Titel „Stricklies'ln & Häkelpetra's“ und `permalink: /2013/07/stricklies-ln-hakelpetra-s/` hat
- **THEN** ist er unter `/2013/07/stricklies-ln-hakelpetra-s/` erreichbar

#### Scenario: Neuer Post ohne Permalink
- **WHEN** das Verzeichnis `content/posts/grüne-mütze/` einen Post vom 2026-11-01 ohne `permalink` enthält
- **THEN** ist er unter `/2026/11/gruene-muetze/` erreichbar

#### Scenario: Titeländerung
- **WHEN** der Titel eines Posts mit `permalink` geändert wird
- **THEN** bleibt seine URL unverändert

### Requirement: Live-URLs sind maßgeblich
Die Menge der Post-URLs SHALL alle 194 URLs enthalten, unter denen die Posts auf https://flauschiversum.de/ vor dem Neubau erreichbar waren. Dazu trägt jeder dieser Posts seine Live-URL als `permalink`. Zwei Posts mit derselben URL sind ein Fehler, und der Build bricht ab.

#### Scenario: Abweichender Slug
- **WHEN** der Post „Gib´s mir!… - Audrey II“ vom Mai 2014 veröffentlicht wird
- **THEN** ist er unter `/2014/05/gibs-mir-audrey-ii/` erreichbar (nicht unter `/2014/05/gib-s-mir-audrey-ii/`)

#### Scenario: Doppelte URL
- **WHEN** zwei Posts denselben `permalink` haben
- **THEN** bricht der Build mit einer Fehlermeldung ab, die beide Post-Verzeichnisse nennt

### Requirement: URLs der Post-Bilder
Jedes Bild eines Posts SHALL in drei Größen unter `<Post-URL>200x/<Dateiname>`, `<Post-URL>700x/<Dateiname>` und `<Post-URL>1000x/<Dateiname>` erreichbar sein. Der Dateiname bleibt unverändert, einschließlich Groß-/Kleinschreibung, Endung, Umlauten und Leerzeichen.

#### Scenario: Bild in allen Größen
- **WHEN** der Post unter `/2014/01/alraune/` das Bild `DSCF0527.jpg` enthält
- **THEN** sind `/2014/01/alraune/200x/DSCF0527.jpg`, `/2014/01/alraune/700x/DSCF0527.jpg` und `/2014/01/alraune/1000x/DSCF0527.jpg` erreichbar

#### Scenario: Dateiname mit Umlaut
- **WHEN** der Post unter `/2020/10/kurbis/` das Bild `kürbis_thumb.jpg` enthält
- **THEN** ist `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg` erreichbar

#### Scenario: PNG-Bild
- **WHEN** ein Post das Bild `muster.png` enthält
- **THEN** sind `200x/muster.png`, `700x/muster.png` und `1000x/muster.png` unter der Post-URL erreichbar

### Requirement: Feste URLs
Der RSS-Feed SHALL unter `/rss.xml`, das Stylesheet unter `/style.css`, die Sitemap unter `/sitemap.xml` und die Crawler-Regeln unter `/robots.txt` erreichbar sein.

#### Scenario: Feed abrufen
- **WHEN** `/rss.xml` abgerufen wird
- **THEN** wird der RSS-Feed ausgeliefert

#### Scenario: Sitemap abrufen
- **WHEN** `/sitemap.xml` abgerufen wird
- **THEN** wird die XML-Sitemap ausgeliefert

### Requirement: URLs statischer Dateien
Jede Datei unter `content/static/` SHALL unter ihrem Pfad relativ zu `content/static/` erreichbar sein.

#### Scenario: Favicon und Schrift
- **WHEN** `content/static/favicon.ico` und `content/static/fonts/Lobster.otf` existieren
- **THEN** sind sie unter `/favicon.ico` und `/fonts/Lobster.otf` erreichbar

## ADDED Requirements

### Requirement: URLs von Animationen und Videos
Jede animierte GIF-Datei und jedes Video eines Posts SHALL unverändert unter `<Post-URL><Dateiname>` erreichbar sein, ohne Größenvarianten.

#### Scenario: Video im Post
- **WHEN** der Post unter `/2014/05/karteikartenfresser/` das Video `VID_20130115_184912.mp4` enthält
- **THEN** ist `/2014/05/karteikartenfresser/VID_20130115_184912.mp4` erreichbar

### Requirement: Kodierte URLs im HTML
Alle vom Generator in HTML, Feed und Sitemap geschriebenen URLs SHALL nach RFC 3986 prozentkodiert sein. Leerzeichen, Umlaute und andere Zeichen außerhalb von ASCII stehen nie roh in einem Attribut.

#### Scenario: Leerzeichen im Dateinamen
- **WHEN** ein Post das Bild `Schwacher Moment.jpg` einbindet
- **THEN** endet die URL im `src`-Attribut auf `700x/Schwacher%20Moment.jpg`
