## Why

Der Platzhalter im Suchfeld der Topbar lautet „Anleitung suchen …“. Die Seite besteht aber nicht nur aus Anleitungen, sondern aus Beiträgen aller Art (Ideen, Einkaufstipps, Persönliches, Blog-Interna), und die Suche findet alle. Der Text soll das allgemeiner ausdrücken.

## What Changes

- **Neuer Platzhalter:** Das Suchfeld der Topbar zeigt „Beitrag suchen …“ statt „Anleitung suchen …“.
- **Passende Beschriftung für Screenreader:** Die verborgene Beschriftung des Feldes lautet ebenfalls „Beitrag suchen“ (bisher „Suchen“), damit sie zum sichtbaren Platzhalter passt. Der Lupen-Knopf behält den Namen „Suchen“.
- **Nicht Teil dieses Changes:** Das Feld auf der Suchseite (Platzhalter „z. B. Mütze, Katze, Filz …“, dort sind es Beispiele), Suchlogik und Gestaltung.

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `topbar`: „Suchfeld in der Topbar“ nennt jetzt den Platzhalter „Beitrag suchen …“ und die zugehörige Beschriftung.

## Impact

- `templates/layout.html` (Platzhalter und Beschriftung), die Snapshots aller Seiten (neun Dateien), ein Test, der den Platzhalter prüft. Keine Änderung an URLs, Styles oder Skripten.
