# post-index Specification

## Purpose

Beschreibt die Startseite und ihre Folgeseiten: die chronologische Liste aller veröffentlichten Posts mit Vorschau und die Paginierung zum Blättern.

## Requirements

### Requirement: Reihenfolge und Seitengröße
Die Seite `/beitraege/` SHALL die zehn neuesten veröffentlichten Beiträge nach Datum absteigend zeigen, ohne Paginierung, und daneben das Archiv aller Beiträge (Spec `post-archive`). Die Folgeseiten `/page/<N>/` bleiben unverändert erreichbar und zeigen weiter je 4 Beiträge in derselben chronologischen Reihenfolge; sie werden aus der Navigation nicht mehr verlinkt.

#### Scenario: Neuester Post zuerst
- **WHEN** der neueste Post „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag auf `/beitraege/`

#### Scenario: Letzte Seite unvollständig
- **WHEN** es 194 veröffentlichte Posts gibt
- **THEN** gibt es unter `/page/<N>/` weiterhin die Seiten bis `/page/48/`, und die letzte enthält 2 Posts

#### Scenario: Zehn Einträge auf Beiträge
- **WHEN** `/beitraege/` geöffnet wird
- **THEN** zeigt die Liste genau zehn Beiträge und keine Seitennummern

### Requirement: Veröffentlichungsdatum
Ein Post mit einem Datum in der Zukunft (bezogen auf den Zeitpunkt des Builds) SHALL weder auf den Indexseiten erscheinen noch bei Seitenzahl und Verteilung der Posts auf die Seiten mitzählen.

#### Scenario: Geplanter Post
- **WHEN** ein Post das Datum von morgen hat
- **THEN** erscheint er nicht in der Liste auf den Indexseiten

#### Scenario: Volle Seiten trotz geplantem Post
- **WHEN** es 8 veröffentlichte Posts und einen geplanten Post gibt
- **THEN** gibt es genau 2 Indexseiten mit je 4 Posts

### Requirement: Eintrag eines Posts
Jeder Eintrag SHALL Folgendes zeigen: das Datum im Format „<Tag>. <Monatsname> <Jahr>“ mit deutschem Monatsnamen (maschinenlesbar als `<time datetime>`), den Titel als Link auf den Post, das Vorschaubild (Größe `200x`, mit Breite, Höhe und dem Titel als Alt-Text) als Link auf den Post (falls `image` gesetzt ist) und den `excerpt` gefolgt von einem Link „weiterlesen“ auf den Post.

#### Scenario: Eintrag mit Bild
- **WHEN** der Post „Kürbis“ vom 2020-10-31 mit `image: "kürbis_thumb.jpg"` gelistet wird
- **THEN** zeigt der Eintrag „31. Oktober 2020“, den Titel „Kürbis“ mit Link auf `/2020/10/kurbis/`, das Bild `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg` mit Alt-Text „Kürbis“, den Excerpt und „weiterlesen“

#### Scenario: Eintrag ohne Bild
- **WHEN** ein Post kein `image` im Frontmatter hat
- **THEN** zeigt der Eintrag kein Vorschaubild, aber Datum, Titel, Excerpt und „weiterlesen“

### Requirement: Paginierung
Auf den Folgeseiten `/page/<N>/` SHALL über und unter der Liste eine Paginierung stehen: „«“ zur vorherigen Seite (auf `/page/1/` zu `/beitraege/`), die Seitennummern im Bereich von zwei Seiten vor bis zwei nach der aktuellen, begrenzt auf existierende Seiten, die aktuelle hervorgehoben, und „»“ zur nächsten Seite (nicht auf der letzten). Auf `/beitraege/` gibt es keine Paginierung.

#### Scenario: Erste Seite
- **WHEN** die erste Folgeseite `/page/1/` angezeigt wird
- **THEN** zeigt die Paginierung „«“ mit Link auf `/beitraege/`, „1“ (Link auf `/beitraege/`), „2“ (hervorgehoben), „3“, „4“ und „»“ mit Link auf `/page/2/`

#### Scenario: Mittlere Seite
- **WHEN** die zehnte Indexseite (`/page/9/`) angezeigt wird
- **THEN** zeigt die Paginierung „«“, „8“, „9“, „10“ (hervorgehoben), „11“, „12“ und „»“

#### Scenario: Letzte Seite
- **WHEN** es 49 Indexseiten gibt und die letzte (`/page/48/`) angezeigt wird
- **THEN** zeigt die Paginierung „«“, „47“, „48“, „49“ (hervorgehoben), aber weder „50“ noch „»“

#### Scenario: Nur eine Seite
- **WHEN** es höchstens 4 veröffentlichte Posts gibt
- **THEN** gibt es keine Folgeseiten und keine Paginierung
