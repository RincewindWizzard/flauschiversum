# Spec Delta

## MODIFIED Requirements

### Requirement: Sitemap
Unter `/sitemap.xml` SHALL eine XML-Sitemap nach sitemaps.org stehen. Sie enthält die absoluten URLs der Startseite, von `/themen/`, aller Kategorie- und Schlagwortseiten, aller veröffentlichten Posts und aller Einzelseiten. Bei Posts ist `lastmod` das Datum des Posts. Folgeseiten der Liste, die Suchseite und Medien-URLs gehören nicht hinein.

#### Scenario: Post in der Sitemap
- **WHEN** der Post „Kürbis“ vom 2020-10-31 veröffentlicht ist
- **THEN** enthält die Sitemap `https://flauschiversum.de/2020/10/kurbis/` mit `lastmod` 2020-10-31

#### Scenario: Geplanter Post
- **WHEN** ein Post ein Datum in der Zukunft hat
- **THEN** fehlt er in der Sitemap

#### Scenario: Themenseiten
- **WHEN** die Kategorie „Filzen“ und das Schlagwort „Hüte“ Beiträge haben
- **THEN** enthält die Sitemap `https://flauschiversum.de/kategorie/filzen/` und `https://flauschiversum.de/tag/huete/`

### Requirement: Strukturierte Daten
Jede Post-Seite SHALL ein JSON-LD-Objekt vom Typ `BlogPosting` enthalten, mit `headline`, `datePublished` (ISO 8601 mit Zeitzone), `author` (Person mit dem Namen aus `author`), `image` (falls vorhanden), `mainEntityOfPage` (kanonische URL), `articleSection` (Name der Kategorie), `keywords` (Namen der Schlagworte, falls vorhanden) und `publisher` (Organisation „Flauschiversum“).

#### Scenario: Rich-Results-Test
- **WHEN** eine Post-Seite mit dem Google-Test für Rich-Suchergebnisse geprüft wird
- **THEN** wird ein gültiges `BlogPosting` ohne Fehler erkannt

#### Scenario: Einordnung im Datensatz
- **WHEN** der Post „Alraune“ in der Kategorie „Basteln“ liegt und das Schlagwort „LARP“ trägt
- **THEN** enthält der Datensatz `articleSection` „Basteln“ und `keywords` mit „LARP“

## ADDED Requirements

### Requirement: Meta-Angaben der Themenseiten
Kategorie-, Schlagwort- und Übersichtsseiten SHALL eine eigene Meta-Beschreibung, eine kanonische URL und Open-Graph-Angaben tragen. Die Beschreibung lautet „<Anzahl> Beiträge zu <Name> im Flauschiversum“, bei Kategorien ergänzt um den Reim.

#### Scenario: Beschreibung einer Kategorie
- **WHEN** `/kategorie/filzen/` mit 37 Beiträgen erzeugt wird
- **THEN** beginnt die Meta-Beschreibung mit „37 Beiträge zu Filzen im Flauschiversum“

### Requirement: Suchseite nicht indexieren
Die Suchseite SHALL mit `<meta name="robots" content="noindex">` ausgeliefert werden und in `robots.txt` nicht gesperrt sein.

#### Scenario: Suchseite
- **WHEN** `/suche/` erzeugt wird
- **THEN** enthält der Kopf `noindex`
