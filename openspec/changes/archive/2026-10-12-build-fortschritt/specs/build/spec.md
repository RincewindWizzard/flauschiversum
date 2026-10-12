## ADDED Requirements

### Requirement: Fortschrittsmeldungen
`flauschiversum build`, `serve` (bei jedem Neubau) und `check` SHALL auf der Fehlerausgabe zu jedem Arbeitsschritt melden, woran sie gerade arbeiten (Inhalte lesen, Bilder verarbeiten, Beiträge rendern, Suchindex bauen, Dateien schreiben und so weiter). Jede Meldung nennt die seit dem Start verstrichene Zeit, und die Meldung des nächsten Schrittes nennt die Dauer des vorherigen, wenn sie länger als 0,5 Sekunden war.

#### Scenario: Kaltstart
- **WHEN** `flauschiversum build` mit leerem Cache gestartet wird
- **THEN** erscheint sofort die erste Meldung, und jeder weitere Schritt meldet sich beim Beginn mit der verstrichenen Zeit

#### Scenario: Zusammenfassung bleibt
- **WHEN** ein Build endet
- **THEN** stehen auf der Standardausgabe weiterhin die beiden Zusammenfassungszeilen mit Zahl der Beiträge, Bilder und Dateien

### Requirement: Zwischenstand bei Bildern und Suchindex
Bei der Verarbeitung der Medien SHALL der Generator höchstens alle 2 Sekunden einen Zwischenstand mit verarbeiteter Anzahl, Gesamtzahl, Prozent und der Aufteilung in neu berechnet und aus dem Cache melden, und beim letzten Medium immer. Beim Suchindex SHALL er melden, ob er aus dem Cache übernommen oder neu berechnet wird, und im zweiten Fall darauf hinweisen, dass es dauern kann.

#### Scenario: Zwischenstand
- **WHEN** mehr als zwei Sekunden lang Bilder verarbeitet werden
- **THEN** erscheint etwa alle zwei Sekunden eine Zeile wie `Bilder und Medien: 1298/3197 (40 %) · 1297 neu berechnet, 1 aus dem Cache`

#### Scenario: Suchindex aus dem Cache
- **WHEN** der Suchindex unverändert im Cache liegt
- **THEN** meldet der Generator, dass er aus dem Cache übernommen wurde

#### Scenario: Suchindex neu
- **WHEN** der Suchindex neu berechnet werden muss
- **THEN** meldet der Generator das vor dem Start der Berechnung mit dem Hinweis, dass es einige Minuten dauern kann

### Requirement: Tests und Bibliothek bleiben still
Die Fortschrittsmeldungen SHALL nur von den Kommandos ausgelöst werden; Tests und Aufrufe von `generate` ohne Kommando erzeugen keine Ausgabe.

#### Scenario: Test-Build
- **WHEN** ein Test die Seite über `generate` baut
- **THEN** werden keine Fortschrittsmeldungen ausgegeben
