# Spec Delta

## MODIFIED Requirements

### Requirement: Feed-Format und Kanal
Unter `/rss.xml` SHALL ein gültiger RSS-2.0-Feed in UTF-8 ausgeliefert werden. Der Kanal hat den Titel „Flauschiversum“, den Link `https://flauschiversum.de/`, die Beschreibung „Mehr als nur eine Masche“, die Sprache `de-DE`, einen Selbstverweis (`atom:link rel="self"`) auf `https://flauschiversum.de/rss.xml` und als `lastBuildDate` das Datum des neuesten veröffentlichten Posts. Ohne neuen Post bleibt der Feed bei einem erneuten Build also byte-gleich.

#### Scenario: Feed im Feedreader
- **WHEN** `https://flauschiversum.de/rss.xml` in einem Feedreader abonniert wird
- **THEN** erscheint der Kanal als „Flauschiversum“ mit der Beschreibung „Mehr als nur eine Masche“

#### Scenario: Feed-Validator
- **WHEN** der Feed mit dem W3C-Feed-Validator geprüft wird
- **THEN** meldet dieser keine Fehler

#### Scenario: Erneuter Build ohne neuen Post
- **WHEN** die Seite ohne Änderung an Posts zweimal gebaut wird
- **THEN** ist `rss.xml` nach beiden Builds byte-gleich

### Requirement: Feed-Einträge
Der Feed SHALL die 20 neuesten veröffentlichten Posts enthalten, neueste zuerst. Jeder Eintrag hat: den Titel, die absolute Post-URL als `link`, den `excerpt` als `description`, den vollständigen Post-Inhalt als HTML mit absoluten URLs in `content:encoded`, das Veröffentlichungsdatum als `pubDate`, den Autor als `dc:creator` und, falls `image` gesetzt ist, die `700x`-Variante als `enclosure` mit Typ und Größe.

#### Scenario: Eintrag eines Posts
- **WHEN** der Post „Kollegenkarten“ vom März 2018 im Feed steht
- **THEN** hat sein Eintrag den Titel „Kollegenkarten“, den Link `https://flauschiversum.de/2018/03/kollegenkarten/`, den Excerpt als Beschreibung, sein Datum als `pubDate` und den vollständigen Text mit Bildern in `content:encoded`

#### Scenario: Reihenfolge
- **WHEN** der Feed abgerufen wird
- **THEN** ist der erste Eintrag der neueste veröffentlichte Post und die `pubDate`-Werte fallen von oben nach unten

#### Scenario: Geplanter Post
- **WHEN** ein Post ein Datum in der Zukunft hat
- **THEN** erscheint er nicht im Feed

#### Scenario: Bilder im Feedreader
- **WHEN** ein Feedreader den Inhalt eines Eintrags mit Bildern anzeigt
- **THEN** werden die Bilder geladen, weil ihre URLs absolut mit `https://flauschiversum.de/` beginnen
