# Spec Delta

## Purpose

Legt alle öffentlich erreichbaren URLs des Flauschiversums fest. Sie sind von außen verlinkt (Suchmaschinen, Pinterest, Lesezeichen) und dürfen sich nicht ändern („Cool URIs don't change“).

## ADDED Requirements

### Requirement: Post-URL
Jeder Post SHALL unter `/<JJJJ>/<MM>/<slug>/` erreichbar sein. Jahr und Monat kommen aus dem `date` des Posts. Der Slug wurde beim ursprünglichen Build aus dem Titel gebildet: kleingeschrieben, Umlaute ohne „e“ (ä→a, ö→o, ü→u), ß→ss, andere Zeichen durch `-` ersetzt.

#### Scenario: Umlaut im Titel
- **WHEN** ein Post den Titel „Kürbis“ und das Datum 2020-10-31 hat
- **THEN** ist er unter `/2020/10/kurbis/` erreichbar

#### Scenario: ß im Titel
- **WHEN** ein Post den Titel „ein weißer Magierhut“ und ein Datum im Februar 2018 hat
- **THEN** ist er unter `/2018/02/ein-weisser-magierhut/` erreichbar

#### Scenario: Sonderzeichen im Titel
- **WHEN** ein Post den Titel „Stricklies'ln & Häkelpetra's“ und ein Datum im Juli 2013 hat
- **THEN** ist er unter `/2013/07/stricklies-ln-hakelpetra-s/` erreichbar

### Requirement: Live-URLs sind maßgeblich
Die Menge der Post-URLs SHALL genau den 194 URLs entsprechen, unter denen die Posts auf https://flauschiversum.de/ erreichbar sind. Weicht ein nachgerechneter Slug von der Live-URL ab, gilt die Live-URL.

#### Scenario: Abweichender Slug
- **WHEN** der Post „Gib´s mir!… - Audrey II“ vom Mai 2014 veröffentlicht wird
- **THEN** ist er unter `/2014/05/gibs-mir-audrey-ii/` erreichbar (nicht unter `/2014/05/gib-s-mir-audrey-ii/`)

### Requirement: URL der Einzelseiten
Jede Einzelseite SHALL unter `/<slug>/` erreichbar sein, wobei der Slug aus ihrem Titel gebildet wird. Die bestehenden Seiten sind `/about/`, `/impressum/` und `/tipps-und-tricks/`.

#### Scenario: Seite „Tipps und Tricks“
- **WHEN** die Seite mit dem Titel „Tipps und Tricks“ veröffentlicht wird
- **THEN** ist sie unter `/tipps-und-tricks/` erreichbar

### Requirement: URLs der Post-Bilder
Jedes Bild eines Posts SHALL in drei Größen unter `<Post-URL>200x/<Dateiname>`, `<Post-URL>700x/<Dateiname>` und `<Post-URL>1000x/<Dateiname>` erreichbar sein. Der Dateiname bleibt unverändert, einschließlich Groß-/Kleinschreibung, Umlauten und Leerzeichen.

#### Scenario: Bild in allen Größen
- **WHEN** der Post unter `/2014/01/alraune/` das Bild `DSCF0527.jpg` enthält
- **THEN** sind `/2014/01/alraune/200x/DSCF0527.jpg`, `/2014/01/alraune/700x/DSCF0527.jpg` und `/2014/01/alraune/1000x/DSCF0527.jpg` erreichbar

#### Scenario: Dateiname mit Umlaut
- **WHEN** der Post unter `/2020/10/kurbis/` das Bild `kürbis_thumb.jpg` enthält
- **THEN** ist `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg` erreichbar

### Requirement: URLs der Indexseiten
Die erste Indexseite SHALL unter `/` und `/index.html` erreichbar sein. Die Folgeseiten SHALL unter `/page/<N>/` und `/page/<N>/index.html` erreichbar sein, mit N ab 1 für die zweite Seite.

#### Scenario: Zweite Indexseite
- **WHEN** es mehr als 4 veröffentlichte Posts gibt
- **THEN** ist die zweite Indexseite unter `/page/1/index.html` und `/page/1/` erreichbar

### Requirement: Feste URLs
Der RSS-Feed SHALL unter `/rss.xml` und das Stylesheet unter `/style.css` erreichbar sein.

#### Scenario: Feed abrufen
- **WHEN** `/rss.xml` abgerufen wird
- **THEN** wird der RSS-Feed ausgeliefert

### Requirement: URLs statischer Dateien
Jede Datei unter `src/static/` SHALL unter ihrem Pfad relativ zu `src/static/` erreichbar sein.

#### Scenario: Favicon und Schrift
- **WHEN** `src/static/favicon.ico` und `src/static/fonts/Lobster.otf` existieren
- **THEN** sind sie unter `/favicon.ico` und `/fonts/Lobster.otf` erreichbar

### Requirement: Verzeichnis-URLs ohne Schrägstrich
Wird eine Post- oder Seiten-URL ohne abschließenden `/` aufgerufen, SHALL dauerhaft (HTTP 301) auf die URL mit `/` weitergeleitet werden.

#### Scenario: Fehlender Schrägstrich
- **WHEN** `/about` aufgerufen wird
- **THEN** antwortet der Server mit 301 und `Location: https://flauschiversum.de/about/`
