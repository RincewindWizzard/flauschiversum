# Spec Delta

## ADDED Requirements

### Requirement: Meta-Angaben der Themen- und Übersichtsseiten
Schlagwort- und Übersichtsseiten SHALL eine eigene Meta-Beschreibung, eine kanonische URL und Open-Graph-Angaben tragen. Die Beschreibung lautet „<Anzahl> Beiträge zu <Name> im Flauschiversum“.

#### Scenario: Beschreibung eines Themas
- **WHEN** `/tag/filzen/` mit 37 Beiträgen erzeugt wird
- **THEN** beginnt die Meta-Beschreibung mit „37 Beiträge zu Filzen im Flauschiversum“

## MODIFIED Requirements

### Requirement: Sitemap
Unter `/sitemap.xml` SHALL eine XML-Sitemap nach sitemaps.org stehen. Sie enthält die absoluten URLs der Startseite, von `/themen/`, aller Schlagwortseiten, aller veröffentlichten Posts und aller Einzelseiten. Bei Posts ist `lastmod` das Datum des Posts. Folgeseiten der Liste, Weiterleitungsseiten, die Suchseite und Medien-URLs gehören nicht hinein.

#### Scenario: Post in der Sitemap
- **WHEN** der Post „Kürbis“ vom 2020-10-31 veröffentlicht ist
- **THEN** enthält die Sitemap `https://flauschiversum.de/2020/10/kurbis/` mit `lastmod` 2020-10-31

#### Scenario: Geplanter Post
- **WHEN** ein Post ein Datum in der Zukunft hat
- **THEN** fehlt er in der Sitemap

#### Scenario: Themenseiten
- **WHEN** das Schlagwort „Hüte“ Beiträge hat
- **THEN** enthält die Sitemap `https://flauschiversum.de/tag/huete/`

#### Scenario: Weiterleitung
- **WHEN** `/kategorie/basteln/` auf `/tag/basteln/` weiterleitet
- **THEN** enthält die Sitemap `/kategorie/basteln/` nicht

### Requirement: Strukturierte Daten
Jede Post-Seite SHALL ein JSON-LD-Objekt vom Typ `BlogPosting` enthalten, mit `headline`, `datePublished` (ISO 8601 mit Zeitzone), `author` (Person mit dem Namen aus `author`), `image` (falls vorhanden), `mainEntityOfPage` (kanonische URL), `keywords` (Namen der Schlagworte, falls vorhanden) und `publisher` (Organisation „Flauschiversum“).

#### Scenario: Rich-Results-Test
- **WHEN** eine Post-Seite mit dem Google-Test für Rich-Suchergebnisse geprüft wird
- **THEN** wird ein gültiges `BlogPosting` ohne Fehler erkannt

#### Scenario: Einordnung im Datensatz
- **WHEN** der Post „Alraune“ das Schlagwort „LARP“ trägt
- **THEN** enthält der Datensatz `keywords` mit „LARP“

## REMOVED Requirements

### Requirement: Meta-Angaben der Themenseiten
**Reason**: Kategorien entfallen, Themen sind Schlagworte; die Anforderung wird unter dem Namen „Meta-Angaben der Themen- und Übersichtsseiten“ ohne Kategorien neu gefasst.
**Migration**: Siehe „Meta-Angaben der Themen- und Übersichtsseiten“.
