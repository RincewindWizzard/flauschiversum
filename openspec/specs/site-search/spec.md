# site-search Specification

## Purpose

Beschreibt die Volltextsuche des Flauschiversums: Besucher finden Beiträge über Begriffe aus Titel, Text und Schlagworten, direkt im Browser und ohne Dienste von Dritten.

## Requirements

### Requirement: Suchseite
Unter `/suche/` SHALL die Suchseite stehen. Mit dem Parameter `q` (z.B. `/suche/?q=katze`) führt sie die Suche beim Laden aus und zeigt zu jedem Treffer den Titel als Link, einen Textausschnitt mit hervorgehobenen Suchwörtern sowie die Anzahl der Treffer. Auf der Seite gibt es im Inhalt genau ein Suchfeld (das der Suchoberfläche); das Suchfeld der Topbar bleibt zusätzlich bestehen.

#### Scenario: Suche über die Adresse
- **WHEN** `/suche/?q=katze` geöffnet wird
- **THEN** zeigt die Seite die Trefferzahl und für jeden Treffer Titel und Ausschnitt mit hervorgehobenem Suchwort

#### Scenario: Ohne Suchbegriff
- **WHEN** `/suche/` ohne `q` geöffnet wird
- **THEN** zeigt die Seite das Suchfeld und Links auf `/themen/` und `/beitraege/`

#### Scenario: Keine Treffer
- **WHEN** nach einem Begriff gesucht wird, der in keinem Beitrag vorkommt
- **THEN** meldet die Seite „Keine Treffer“ und verweist auf `/themen/`

#### Scenario: Ein Suchfeld im Inhalt
- **WHEN** `/suche/?q=mütze` mit JavaScript geöffnet wird
- **THEN** gibt es im Inhalt genau ein sichtbares Suchfeld, gefüllt mit „mütze“, und zusätzlich das Suchfeld der Topbar

### Requirement: Durchsuchter Bestand
Die Suche SHALL die Texte aller veröffentlichten Beiträge und Einzelseiten sowie die Schlagworte der Beiträge berücksichtigen. Startseite, Beitragslisten, Themenseiten und die Suchseite selbst sind nicht Teil des Bestands.

#### Scenario: Begriff aus dem Text
- **WHEN** ein Begriff nur im Fließtext eines Beitrags von 2014 vorkommt
- **THEN** erscheint dieser Beitrag unter den Treffern

#### Scenario: Begriff aus einem Schlagwort
- **WHEN** nach „Halloween“ gesucht wird
- **THEN** erscheinen auch Beiträge mit dem Schlagwort „Halloween“, in deren Text das Wort nicht steht

#### Scenario: Geplanter Beitrag
- **WHEN** ein Beitrag ein Datum in der Zukunft hat
- **THEN** erscheint er in keinem Suchergebnis

### Requirement: Tolerante Suche
Die Suche SHALL Groß- und Kleinschreibung ignorieren, Umlaute und ihre Umschreibungen gleich behandeln, deutsche Wortformen zusammenführen und bereits den Anfang eines Wortes als Treffer gelten lassen.

#### Scenario: Wortform
- **WHEN** nach „Katze“ gesucht wird
- **THEN** erscheinen auch Beiträge, in denen nur „Katzen“ steht

#### Scenario: Umlaut
- **WHEN** nach „Kuerbis“ oder „kürbis“ gesucht wird
- **THEN** erscheint der Beitrag „Kürbis“

#### Scenario: Wortanfang
- **WHEN** nach „filz“ gesucht wird
- **THEN** erscheinen Beiträge, die „Filz“, „Filzen“ oder „Filzstift“ enthalten

### Requirement: Suchfeld auf der Startseite
Die Startseite SHALL über das Suchfeld der Topbar erreichbar suchen lassen (im Inhalt gibt es kein eigenes Suchfeld). Eine abgeschickte Suche öffnet `/suche/?q=<Begriff>`.

#### Scenario: Suche von der Startseite
- **WHEN** auf `/` „Stirnband“ eingegeben und abgeschickt wird
- **THEN** öffnet sich `/suche/?q=Stirnband` mit den Treffern

### Requirement: Index als statische Dateien
Der Suchindex SHALL beim Build erzeugt und als statische Dateien unter `/pagefind/` ausgeliefert werden. Die Suche läuft vollständig im Browser, ohne Serveranfragen außer dem Laden dieser Dateien und ohne Dienste von Dritten. Die Seite lädt nur die für den Suchbegriff nötigen Teile des Index.

#### Scenario: Keine Drittanbieter
- **WHEN** eine Suche ausgeführt wird
- **THEN** werden alle Dateien von `flauschiversum.de` geladen

#### Scenario: Index im Ausgabeverzeichnis
- **WHEN** gebaut wird
- **THEN** liegen die Suchdateien unter `<out>/pagefind/` und sind im Manifest des Builds aufgeführt

### Requirement: Reproduzierbarer Index
Der Suchindex SHALL bei unveränderten Inhalten byte-gleich sein, damit ein erneuter Build keine Datei schreibt. Ändert sich ein Beitrag, ändern sich nur die davon betroffenen Dateien.

#### Scenario: Zweiter Build
- **WHEN** ohne Änderung an den Inhalten ein zweites Mal gebaut wird
- **THEN** meldet der Build 0 neue und 0 geänderte Dateien, auch unter `pagefind/`

### Requirement: Ohne JavaScript
Ohne JavaScript SHALL die Suchseite einen Hinweis zeigen, dass die Suche JavaScript braucht, und auf `/themen/` und `/beitraege/` verweisen. Das Suchfeld der Topbar bleibt sichtbar.

#### Scenario: JavaScript deaktiviert
- **WHEN** `/suche/?q=katze` bei deaktiviertem JavaScript geöffnet wird
- **THEN** zeigt die Seite den Hinweis und die Links auf `/themen/` und `/beitraege/`

### Requirement: Filter nach Thema
Die Suchseite SHALL die Treffer nach Thema (Schlagwort) einschränken können. Die Auswahl heißt „Thema“ und zeigt jedes Schlagwort mit der Zahl der Treffer.

#### Scenario: Auf Thema einschränken
- **WHEN** nach „Mütze“ gesucht und das Thema „Wolle“ gewählt wird
- **THEN** bleiben nur Treffer mit dem Schlagwort „Wolle“ übrig

#### Scenario: Beitrag ohne Schlagworte
- **WHEN** ein Beitrag keine Schlagworte hat
- **THEN** erscheint er in der Suche ohne Filter, aber in keiner Themenauswahl

### Requirement: Konzentrische Lupe
Der Suchknopf SHALL als Kreis im Suchfeld sitzen, dessen Mittelpunkt auf dem Mittelpunkt der Rundung am rechten Rand des Feldes liegt (konzentrisch): oben, unten und rechts hat er denselben Abstand zum Rand des Feldes. Das gilt für jedes Suchfeld (Topbar, Suchseite) in jeder Schriftgröße.

#### Scenario: Gleiche Abstände
- **WHEN** ein Suchfeld angezeigt wird
- **THEN** sind die Abstände der Lupe zum Feldrand oben, unten und rechts gleich (Abweichung höchstens 1,5 px), und der Knopf ist rund

#### Scenario: Größeres Suchfeld
- **WHEN** das Suchfeld der Suchseite, das größer als das der Topbar ist, angezeigt wird
- **THEN** sind die Abstände ebenfalls gleich

### Requirement: Darstellung im Stil der Seite
Die Suchseite SHALL in der Schrift der Seite (Lora) und in einer Größe dargestellt werden, die zum übrigen Text passt: Titel der Treffer mindestens 16 px, Textausschnitte und Filter mindestens 16 px (bei Desktop-Schriftgröße). Treffer entsprechen im Aussehen den Einträgen der Beitragslisten (runde Vorschaubilder, gestrichelte Trenner, Titel in Linkfarbe), Suchwörter sind farblich zurückhaltend hervorgehoben, Bedienelemente (Löschen, Filter, Mehr laden) haben die Gestalt der übrigen Bedienelemente der Seite.

#### Scenario: Schrift und Größe
- **WHEN** `/suche/?q=mütze` bei 1440 px Breite geöffnet wird
- **THEN** sind Titel, Ausschnitte und Filter mindestens 16 px groß, und die Schrift ist Lora

#### Scenario: Runde Vorschaubilder
- **WHEN** Treffer mit Vorschaubild angezeigt werden
- **THEN** sind die Bilder rund und einheitlich groß

### Requirement: Seitenaufbau der Suchseite
Die Suchseite SHALL den Rahmen der Seiten mit Karte (bis 1200 px) verwenden: Treffer in der Hauptspalte (`main`), der Filter „Thema“ als echte Seitenleiste (`aside` des Seitenrahmens, wie bei Archiv und Tag-Cloud, nicht Teil des `main`) als Karte rechts daneben (ab 64 em). Auf schmaleren Fenstern steht die Filterkarte über den Treffern. Die Seite scrollt nie waagerecht.

#### Scenario: Desktop
- **WHEN** `/suche/?q=mütze` bei 1440 px Breite geöffnet wird
- **THEN** stehen die Treffer links und der Filter „Thema“ als Karte rechts, und die Hauptspalte ist breiter als 1000 px

#### Scenario: Filter ist eine Seitenleiste
- **WHEN** `/suche/?q=mütze` mit JavaScript geöffnet wird
- **THEN** liegt der Filter in einem `aside` neben dem Inhalt und nicht innerhalb von `main`

#### Scenario: Handy
- **WHEN** `/suche/?q=mütze` bei 360 px Breite geöffnet wird
- **THEN** steht die Filterkarte über den Treffern, und es gibt kein waagerechtes Scrollen
