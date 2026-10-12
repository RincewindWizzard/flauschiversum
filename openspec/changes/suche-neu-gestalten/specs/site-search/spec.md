## MODIFIED Requirements

### Requirement: Suchseite
Unter `/suche/` SHALL die Suchseite stehen. Mit dem Parameter `q` (z.B. `/suche/?q=katze`) führt sie die Suche beim Laden aus und zeigt zu jedem Treffer den Titel als Link, einen Textausschnitt mit hervorgehobenen Suchwörtern und die Kategorie sowie die Anzahl der Treffer. Auf der Seite gibt es im Inhalt genau ein Suchfeld (das der Suchoberfläche); das Suchfeld der Topbar bleibt zusätzlich bestehen.

#### Scenario: Suche über die Adresse
- **WHEN** `/suche/?q=katze` geöffnet wird
- **THEN** zeigt die Seite die Trefferzahl und für jeden Treffer Titel, Ausschnitt mit hervorgehobenem Suchwort und Kategorie

#### Scenario: Ohne Suchbegriff
- **WHEN** `/suche/` ohne `q` geöffnet wird
- **THEN** zeigt die Seite das Suchfeld und Links auf `/themen/` und `/beitraege/`

#### Scenario: Keine Treffer
- **WHEN** nach einem Begriff gesucht wird, der in keinem Beitrag vorkommt
- **THEN** meldet die Seite „Keine Treffer“ und verweist auf `/themen/`

#### Scenario: Ein Suchfeld im Inhalt
- **WHEN** `/suche/?q=mütze` mit JavaScript geöffnet wird
- **THEN** gibt es im Inhalt genau ein sichtbares Suchfeld, gefüllt mit „mütze“, und zusätzlich das Suchfeld der Topbar

### Requirement: Ohne JavaScript
Ohne JavaScript SHALL die Suchseite einen Hinweis zeigen, dass die Suche JavaScript braucht, und auf `/themen/` und `/beitraege/` verweisen. Das Suchfeld der Topbar bleibt sichtbar.

#### Scenario: JavaScript deaktiviert
- **WHEN** `/suche/?q=katze` bei deaktiviertem JavaScript geöffnet wird
- **THEN** zeigt die Seite den Hinweis und die Links auf `/themen/` und `/beitraege/`

## ADDED Requirements

### Requirement: Darstellung im Stil der Seite
Die Suchseite SHALL in der Schrift der Seite (Lora) und in einer Größe dargestellt werden, die zum übrigen Text passt: Titel der Treffer mindestens 16 px, Textausschnitte und Filter mindestens 16 px, Kategorie-Etiketten mindestens 14 px (bei Desktop-Schriftgröße). Treffer entsprechen im Aussehen den Einträgen der Beitragslisten (runde Vorschaubilder, gestrichelte Trenner, Titel in Linkfarbe), Suchwörter sind farblich zurückhaltend hervorgehoben, Bedienelemente (Löschen, Filter, Mehr laden) haben die Gestalt der übrigen Bedienelemente der Seite.

#### Scenario: Schrift und Größe
- **WHEN** `/suche/?q=mütze` bei 1440 px Breite geöffnet wird
- **THEN** sind Titel, Ausschnitte und Filter mindestens 16 px groß, Etiketten mindestens 14 px, und die Schrift ist Lora

#### Scenario: Runde Vorschaubilder
- **WHEN** Treffer mit Vorschaubild angezeigt werden
- **THEN** sind die Bilder rund und einheitlich groß

### Requirement: Seitenaufbau der Suchseite
Die Suchseite SHALL den Rahmen der Seiten mit Karte (bis 1200 px) verwenden: Treffer in der Hauptspalte, der Kategorie-Filter als Karte rechts daneben (ab 64 em). Auf schmaleren Fenstern steht die Filterkarte über den Treffern. Die Seite scrollt nie waagerecht.

#### Scenario: Desktop
- **WHEN** `/suche/?q=mütze` bei 1440 px Breite geöffnet wird
- **THEN** stehen die Treffer links und der Kategorie-Filter als Karte rechts, und die Hauptspalte ist breiter als 1000 px

#### Scenario: Handy
- **WHEN** `/suche/?q=mütze` bei 360 px Breite geöffnet wird
- **THEN** steht die Filterkarte über den Treffern, und es gibt kein waagerechtes Scrollen
