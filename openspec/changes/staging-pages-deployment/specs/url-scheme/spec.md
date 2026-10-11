# Spec Delta

## ADDED Requirements

### Requirement: Konfigurierbare Basis-URL
Der Build SHALL eine Basis-URL (Standard `https://flauschiversum.de`) annehmen können, die auch einen Pfadanteil haben darf (z.B. `https://rincewindwizzard.github.io/flauschiversum`). Alle internen Verweise im HTML, Stylesheet und in den Skripten sowie alle Absolut-URLs in kanonischen Verweisen, Open Graph, strukturierten Daten, Sitemap und Feed richten sich nach dieser Basis. Die Dateien im Ausgabeverzeichnis liegen unabhängig davon an denselben Stellen.

#### Scenario: Link im Unterpfad
- **WHEN** mit Basis `https://rincewindwizzard.github.io/flauschiversum` gebaut wird
- **THEN** zeigt der Navigationslink „Themen“ auf `/flauschiversum/themen/` und das Stylesheet liegt unter `/flauschiversum/style.css`

#### Scenario: Link im Beitragstext
- **WHEN** ein Beitrag im Text auf `/2020/10/kurbis/` verweist und mit der Unterpfad-Basis gebaut wird
- **THEN** zeigt der Link auf `/flauschiversum/2020/10/kurbis/`

#### Scenario: Schriften und Skripte
- **WHEN** mit Unterpfad-Basis gebaut wird
- **THEN** laden Schriften, Karussell, Vollbild und Suchskript ihre Dateien aus `/flauschiversum/`

#### Scenario: Standardbasis
- **WHEN** ohne Basis-URL gebaut wird
- **THEN** ist die Ausgabe gleich der bisherigen mit Wurzelpfad `/` und `https://flauschiversum.de`

#### Scenario: Ungültige Basis
- **WHEN** die Basis-URL nicht mit `http://` oder `https://` beginnt oder auf `/` endet
- **THEN** bricht der Build ab und nennt den Wert und das erwartete Format
