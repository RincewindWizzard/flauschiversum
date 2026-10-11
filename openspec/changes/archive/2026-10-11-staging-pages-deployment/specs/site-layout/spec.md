# Spec Delta

## ADDED Requirements

### Requirement: Staging-Leiste mit Branchname
Im Staging-Modus SHALL jede HTML-Seite ganz oben, noch vor dem Seitenkopf, eine deutlich sichtbare Leiste mit dem Text „Staging“ und dem Namen des Branches zeigen, aus dem gebaut wurde. Die Leiste ist auch ohne JavaScript sichtbar, scrollt mit der Seite oder bleibt oben stehen, ohne Inhalt zu verdecken, und ist bei jeder Fensterbreite lesbar. Ohne Staging-Modus gibt es sie nicht.

#### Scenario: Leiste auf der Startseite
- **WHEN** aus dem Branch `claude/beautiful-darwin-66rwab` im Staging-Modus gebaut wird
- **THEN** steht ganz oben auf `/` „Staging: claude/beautiful-darwin-66rwab“

#### Scenario: Leiste auf jeder Seite
- **WHEN** im Staging-Modus gebaut wird
- **THEN** zeigen Beitrag, Kategorie, Suche und Einzelseite dieselbe Leiste

#### Scenario: Lange Branchnamen am Handy
- **WHEN** der Branchname länger ist als die Zeile
- **THEN** wird er umgebrochen, und die Seite scrollt nicht waagerecht

#### Scenario: Produktion
- **WHEN** ohne Staging-Modus gebaut wird
- **THEN** enthält keine Seite die Leiste
