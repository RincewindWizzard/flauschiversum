# Spec Delta

## MODIFIED Requirements

### Requirement: Reihenfolge und Seitengröße
Die chronologische Liste unter `/beitraege/` und `/page/<N>/` SHALL alle veröffentlichten Posts nach Datum absteigend (neueste zuerst) zeigen, mit 4 Posts pro Seite. Die erste Seite `/beitraege/` enthält die neuesten Posts.

#### Scenario: Neuester Post zuerst
- **WHEN** der neueste Post „Kürbis“ vom 31. Oktober 2020 ist
- **THEN** steht er als erster Eintrag auf `/beitraege/`

#### Scenario: Letzte Seite unvollständig
- **WHEN** es 194 veröffentlichte Posts gibt
- **THEN** gibt es 49 Seiten der Liste, und die letzte enthält 2 Posts
