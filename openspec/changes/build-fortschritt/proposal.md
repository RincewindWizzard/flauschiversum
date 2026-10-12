## Why

Ein Kaltstart von `build` und `serve` (leerer Cache) braucht bei über 3000 Bildern und dem Suchindex lange, und bisher gibt der Generator währenddessen nichts aus. Man sieht nicht, ob er arbeitet oder hängt, und woran er gerade arbeitet.

## What Changes

- `build`, `serve` (bei jedem Neubau) und `check` melden auf der Konsole (stderr), woran sie gerade arbeiten: Inhalte lesen, statische Dateien, Stylesheet, Bilder und Medien, Beiträge rendern, Listen und Archive, Suchindex, Feed und Sitemap, Linkprüfung, Dateien schreiben.
- Jede Meldung trägt die verstrichene Zeit; beim nächsten Schritt steht die Dauer des vorherigen dabei, wenn sie länger als eine halbe Sekunde war.
- Bei den Bildern gibt es alle zwei Sekunden einen Zwischenstand mit Zähler, Prozent und Aufschlüsselung (neu berechnet / aus dem Cache). Beim Suchindex steht, ob er aus dem Cache kommt oder neu berechnet wird und dass das dauern kann.
- Die Abschlusszeilen auf stdout bleiben unverändert; Meldungen und Warnungen stehen auf stderr, sodass Ausgaben weiterverwendbar bleiben. Tests und Bibliotheksaufrufe bleiben still.

Nicht Teil dieses Changes: Fortschrittsbalken mit Terminalsteuerzeichen, Farben, Parallelitätsänderungen.

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `build`: Fortschrittsmeldungen.

## Impact

- Neues Modul `src/progress.rs`; Aufrufe in `src/main.rs`, `src/site.rs`, `src/media/mod.rs`, `src/search.rs`; README.
