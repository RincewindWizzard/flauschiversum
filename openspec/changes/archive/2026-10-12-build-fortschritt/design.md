## Context

`site::generate` durchläuft feste Schritte; der Löwenanteil eines Kaltstarts entfällt auf `media::build` (Rayon-Schleife über rund 3200 Medien) und den Pagefind-Suchindex (`search::build`, mit Zwischenspeicher in `.cache`). Gemessen auf dem Entwicklungsrechner dauert ein Kaltstart rund 18 s (Bilder) plus Suchindex; auf langsameren Rechnern (CI, Laptop) ein Vielfaches.

## Decisions

### 1. Eigenes kleines Modul statt Logging-Bibliothek

`progress.rs` hält globale Zustände (Startzeit, Zeit des letzten Schritts, Schalter) und bietet `step(text)`, `note(text)` und `count(label, fertig, gesamt, detail)`. Aktiviert wird es nur von den Kommandos (`enable()`, `begin()` pro Lauf), deshalb sind Tests und Bibliotheksnutzung still. Keine neue Abhängigkeit.

### 2. Format

`[  12.3 s] Schritt …   (vorheriger Schritt: 4.1 s)`; Zwischenstände eingerückt: `    Bilder und Medien: 1298/3197 (40 %) · 1297 neu berechnet, 1 aus dem Cache`. Ausgabe auf stderr, damit die zwei Zusammenfassungszeilen auf stdout unverändert bleiben.

### 3. Zwischenstand ohne eigenen Thread

Der Zähler wird in der Rayon-Schleife nach jedem Medium erhöht; ausgegeben wird höchstens alle 2 s (atomarer Zeitstempel, `compare_exchange`, damit nur ein Thread druckt) und immer beim letzten Medium.

## Risks / Trade-offs

- [Mehr Ausgabe bei `serve`] → nur bei Neubau; Meldungen sind kurz.
- [Globale Zustände] → nur Anzeige, ohne Einfluss auf das Ergebnis.
