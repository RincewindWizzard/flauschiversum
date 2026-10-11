# Spec Delta

## Purpose

Beschreibt die Volltextsuche des Flauschiversums: Besucher finden Beiträge über Begriffe aus Titel, Text, Kategorie und Schlagworten, direkt im Browser und ohne Dienste von Dritten.

## ADDED Requirements

### Requirement: Suchseite
Unter `/suche/` SHALL die Suchseite stehen. Mit dem Parameter `q` (z.B. `/suche/?q=katze`) führt sie die Suche beim Laden aus und zeigt zu jedem Treffer den Titel als Link, einen Textausschnitt mit hervorgehobenen Suchwörtern und die Kategorie sowie die Anzahl der Treffer.

#### Scenario: Suche über die Adresse
- **WHEN** `/suche/?q=katze` geöffnet wird
- **THEN** zeigt die Seite die Trefferzahl und für jeden Treffer Titel, Ausschnitt mit hervorgehobenem Suchwort und Kategorie

#### Scenario: Ohne Suchbegriff
- **WHEN** `/suche/` ohne `q` geöffnet wird
- **THEN** zeigt die Seite das Suchfeld und Links auf `/themen/` und `/beitraege/`

#### Scenario: Keine Treffer
- **WHEN** nach einem Begriff gesucht wird, der in keinem Beitrag vorkommt
- **THEN** meldet die Seite „Keine Treffer“ und verweist auf `/themen/`

### Requirement: Durchsuchter Bestand
Die Suche SHALL die Texte aller veröffentlichten Beiträge und Einzelseiten sowie Kategorien und Schlagworte der Beiträge berücksichtigen. Startseite, Beitragslisten, Themenseiten und die Suchseite selbst sind nicht Teil des Bestands.

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
- **THEN** erscheinen Beiträge, die „Filzen“ oder „gefilzt“ enthalten

### Requirement: Filter nach Kategorie
Die Suchseite SHALL die Treffer nach Kategorie einschränken können. Die Auswahl zeigt jede Kategorie mit der Zahl der Treffer.

#### Scenario: Auf Kategorie einschränken
- **WHEN** nach „Mütze“ gesucht und die Kategorie „Wolle“ gewählt wird
- **THEN** bleiben nur Treffer der Kategorie „Wolle“ übrig

### Requirement: Suchfeld auf der Startseite
Die Startseite SHALL das Suchfeld deutlich sichtbar zeigen. Eine abgeschickte Suche öffnet `/suche/?q=<Begriff>`.

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
Ohne JavaScript SHALL die Suchseite einen Hinweis zeigen, dass die Suche JavaScript braucht, und auf `/themen/` und `/beitraege/` verweisen. Das Suchfeld bleibt sichtbar.

#### Scenario: JavaScript deaktiviert
- **WHEN** `/suche/?q=katze` bei deaktiviertem JavaScript geöffnet wird
- **THEN** zeigt die Seite den Hinweis und die Links auf `/themen/` und `/beitraege/`
