# Spec Delta

## Purpose

Sorgt dafür, dass Suchmaschinen, Feedreader und soziale Netzwerke die Inhalte des Flauschiversums vollständig finden, richtig einordnen und beim Teilen mit Titel, Beschreibung und Bild anzeigen.

## ADDED Requirements

### Requirement: Sitemap
Unter `/sitemap.xml` SHALL eine XML-Sitemap nach sitemaps.org stehen. Sie enthält die absoluten URLs der Startseite, aller veröffentlichten Posts und aller Einzelseiten. Bei Posts ist `lastmod` das Datum des Posts. Folgeseiten des Index und Medien-URLs gehören nicht hinein.

#### Scenario: Post in der Sitemap
- **WHEN** der Post „Kürbis“ vom 2020-10-31 veröffentlicht ist
- **THEN** enthält die Sitemap `https://flauschiversum.de/2020/10/kurbis/` mit `lastmod` 2020-10-31

#### Scenario: Geplanter Post
- **WHEN** ein Post ein Datum in der Zukunft hat
- **THEN** fehlt er in der Sitemap

### Requirement: robots.txt
Unter `/robots.txt` SHALL eine Datei stehen, die allen Crawlern alles erlaubt und auf `https://flauschiversum.de/sitemap.xml` verweist.

#### Scenario: Sitemap-Hinweis
- **WHEN** `/robots.txt` abgerufen wird
- **THEN** enthält sie die Zeile `Sitemap: https://flauschiversum.de/sitemap.xml`

### Requirement: Meta-Beschreibung
Jede HTML-Seite SHALL eine `<meta name="description">` mit höchstens 160 Zeichen tragen. Bei Posts ist das der `excerpt`, bei Einzelseiten der Anfang des ersten Textabsatzes und bei Indexseiten „Mehr als nur eine Masche“. Zu lange Texte werden an einer Wortgrenze mit „…“ gekürzt.

#### Scenario: Post mit Excerpt
- **WHEN** die Seite des Posts „Kürbis“ erzeugt wird
- **THEN** lautet die Meta-Beschreibung „Es ist wieder die gruselige Jahreszeit und da muss man einen Kürbis haben.“

### Requirement: Kanonische URL
Jede HTML-Seite SHALL per `<link rel="canonical">` auf ihre absolute URL mit `https://flauschiversum.de` und abschließendem `/` verweisen. Für die Startseite ist das `https://flauschiversum.de/`, für Folgeseiten `https://flauschiversum.de/page/<N>/`.

#### Scenario: Aufruf über index.html
- **WHEN** die Startseite über `/index.html` aufgerufen wird
- **THEN** verweist sie als kanonische URL auf `https://flauschiversum.de/`

### Requirement: Vorschau beim Teilen
Jede HTML-Seite SHALL Open-Graph-Angaben tragen: `og:title`, `og:description`, `og:url` (kanonisch), `og:site_name` „Flauschiversum“, `og:locale` `de_DE` und `og:type` (`article` für Posts, sonst `website`). Posts mit `image` geben dessen `1000x`-Variante als `og:image` mit Breite, Höhe und Alt-Text an, dazu `article:published_time`. Außerdem steht `twitter:card` auf `summary_large_image`.

#### Scenario: Post in einem Messenger teilen
- **WHEN** der Link `https://flauschiversum.de/2014/01/alraune/` in einem Messenger geteilt wird
- **THEN** zeigt die Vorschau den Titel „Alraune“, den Excerpt und das Bild `https://flauschiversum.de/2014/01/alraune/1000x/DSCF0527.jpg`

### Requirement: Strukturierte Daten
Jede Post-Seite SHALL ein JSON-LD-Objekt vom Typ `BlogPosting` enthalten, mit `headline`, `datePublished` (ISO 8601 mit Zeitzone), `author` (Person mit dem Namen aus `author`), `image` (falls vorhanden), `mainEntityOfPage` (kanonische URL) und `publisher` (Organisation „Flauschiversum“).

#### Scenario: Rich-Results-Test
- **WHEN** eine Post-Seite mit dem Google-Test für Rich-Suchergebnisse geprüft wird
- **THEN** wird ein gültiges `BlogPosting` ohne Fehler erkannt

### Requirement: Überschriftenstruktur
Jede Post-Seite und Einzelseite SHALL genau eine `<h1>` mit ihrem Titel haben. Auf den Indexseiten ist das Logo „Flauschiversum“ die `<h1>`, und die Post-Titel sind `<h2>`. Das Erscheinungsbild bleibt dabei gleich.

#### Scenario: Post-Seite
- **WHEN** die Seite des Posts „Alraune“ geöffnet wird
- **THEN** gibt es genau eine `<h1>`, und sie lautet „Alraune“

### Requirement: Feed-Hinweis im Kopf
Jede HTML-Seite SHALL per `<link rel="alternate" type="application/rss+xml">` auf `/rss.xml` verweisen, damit Browser und Feedreader den Feed automatisch finden.

#### Scenario: Feed automatisch finden
- **WHEN** ein Feedreader die Startseite als Adresse bekommt
- **THEN** findet er den Feed `https://flauschiversum.de/rss.xml`
