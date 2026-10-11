# Spec Delta

## MODIFIED Requirements

### Requirement: Eintrag eines Posts
Jeder Eintrag SHALL Folgendes zeigen: das Datum im Format „<Tag>. <Monatsname> <Jahr>“ mit deutschem Monatsnamen (maschinenlesbar als `<time datetime>`), den Titel als Link auf den Post, das Vorschaubild (Größe `200x`, mit Breite, Höhe und dem Titel als Alt-Text) als Link auf den Post (falls `image` gesetzt ist) und den `excerpt` gefolgt von einem Link „weiterlesen“ auf den Post.

#### Scenario: Eintrag mit Bild
- **WHEN** der Post „Kürbis“ vom 2020-10-31 mit `image: "kürbis_thumb.jpg"` gelistet wird
- **THEN** zeigt der Eintrag „31. Oktober 2020“, den Titel „Kürbis“ mit Link auf `/2020/10/kurbis/`, das Bild `/2020/10/kurbis/200x/k%C3%BCrbis_thumb.jpg` mit Alt-Text „Kürbis“, den Excerpt und „weiterlesen“

#### Scenario: Eintrag ohne Bild
- **WHEN** ein Post kein `image` im Frontmatter hat
- **THEN** zeigt der Eintrag kein Vorschaubild, aber Datum, Titel, Excerpt und „weiterlesen“

### Requirement: Paginierung
Über und unter der Liste SHALL eine Paginierung stehen. Sie enthält: „«“ zur vorherigen Seite (nicht auf der ersten Seite), die Seitennummern (beginnend bei 1) im Bereich von zwei Seiten vor bis zwei Seiten nach der aktuellen, begrenzt auf existierende Seiten, wobei die aktuelle hervorgehoben ist, und „»“ zur nächsten Seite (nicht auf der letzten Seite). Jeder Link führt auf eine existierende Indexseite.

#### Scenario: Erste Seite
- **WHEN** die erste Indexseite angezeigt wird
- **THEN** zeigt die Paginierung „1“ (hervorgehoben), „2“, „3“ und „»“ mit Link auf `/page/1/`, aber kein „«“

#### Scenario: Mittlere Seite
- **WHEN** die zehnte Indexseite (`/page/9/`) angezeigt wird
- **THEN** zeigt die Paginierung „«“, „8“, „9“, „10“ (hervorgehoben), „11“, „12“ und „»“

#### Scenario: Letzte Seite
- **WHEN** es 49 Indexseiten gibt und die letzte (`/page/48/`) angezeigt wird
- **THEN** zeigt die Paginierung „«“, „47“, „48“, „49“ (hervorgehoben), aber weder „50“ noch „»“

#### Scenario: Nur eine Seite
- **WHEN** es höchstens 4 veröffentlichte Posts gibt
- **THEN** zeigt die Paginierung nur „1“ (hervorgehoben), ohne „«“ und „»“

### Requirement: Veröffentlichungsdatum
Ein Post mit einem Datum in der Zukunft (bezogen auf den Zeitpunkt des Builds) SHALL weder auf den Indexseiten erscheinen noch bei Seitenzahl und Verteilung der Posts auf die Seiten mitzählen.

#### Scenario: Geplanter Post
- **WHEN** ein Post das Datum von morgen hat
- **THEN** erscheint er nicht in der Liste auf den Indexseiten

#### Scenario: Volle Seiten trotz geplantem Post
- **WHEN** es 8 veröffentlichte Posts und einen geplanten Post gibt
- **THEN** gibt es genau 2 Indexseiten mit je 4 Posts
