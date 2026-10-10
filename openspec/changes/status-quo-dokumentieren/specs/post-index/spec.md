# Spec Delta

## Purpose

Beschreibt die Startseite und ihre Folgeseiten: die chronologische Liste aller veröffentlichten Posts mit Vorschau und die Paginierung zum Blättern.

## ADDED Requirements

### Requirement: Reihenfolge und Seitengröße
Die Indexseiten SHALL alle veröffentlichten Posts nach Datum absteigend (neueste zuerst) zeigen, mit 4 Posts pro Seite. Die erste Seite enthält die neuesten Posts.

#### Scenario: Neuester Post zuerst
- **WHEN** der neueste Post „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag auf `/`

#### Scenario: Letzte Seite unvollständig
- **WHEN** es 194 veröffentlichte Posts gibt
- **THEN** gibt es 49 Indexseiten, und die letzte enthält 2 Posts

### Requirement: Veröffentlichungsdatum
Ein Post mit einem Datum in der Zukunft (bezogen auf den Zeitpunkt des Builds) SHALL nicht auf den Indexseiten erscheinen.

#### Scenario: Geplanter Post
- **WHEN** ein Post das Datum von morgen hat
- **THEN** erscheint er nicht in der Liste auf den Indexseiten

### Requirement: Eintrag eines Posts
Jeder Eintrag SHALL Folgendes zeigen: das Datum im Format „<Tag>. <Monatsname> <Jahr>“ mit deutschem Monatsnamen, den Titel als Link auf den Post, das Vorschaubild (Größe `200x`) als Link auf den Post (falls `image` gesetzt ist) und den `excerpt` gefolgt von einem Link „weiterlesen“ auf den Post.

#### Scenario: Eintrag mit Bild
- **WHEN** der Post „Kürbis“ vom 2020-10-31 mit `image: "kürbis_thumb.jpg"` gelistet wird
- **THEN** zeigt der Eintrag „31. Oktober 2020“, den Titel „Kürbis“ mit Link auf `/2020/10/kurbis/`, das Bild `/2020/10/kurbis/200x/kürbis_thumb.jpg`, den Excerpt und „weiterlesen“

#### Scenario: Eintrag ohne Bild
- **WHEN** ein Post kein `image` im Frontmatter hat
- **THEN** zeigt der Eintrag kein Vorschaubild, aber Datum, Titel, Excerpt und „weiterlesen“

### Requirement: Paginierung
Über und unter der Liste SHALL eine Paginierung stehen. Sie enthält: „«“ zur vorherigen Seite (nicht auf der ersten Seite), die Seitennummern (beginnend bei 1) im Bereich von zwei Seiten vor bis zwei Seiten nach der aktuellen, wobei die aktuelle hervorgehoben ist, und „»“ zur nächsten Seite.

#### Scenario: Erste Seite
- **WHEN** die erste Indexseite angezeigt wird
- **THEN** zeigt die Paginierung „1“ (hervorgehoben), „2“, „3“ und „»“ mit Link auf `/page/1/index.html`, aber kein „«“

#### Scenario: Mittlere Seite
- **WHEN** die zehnte Indexseite (`/page/9/index.html`) angezeigt wird
- **THEN** zeigt die Paginierung „«“, „8“, „9“, „10“ (hervorgehoben), „11“, „12“ und „»“
