# Spec Delta

## ADDED Requirements

### Requirement: Konfigurierbare Basis-URL
Der Build SHALL eine Basis-URL (Standard `https://flauschiversum.de`) annehmen können, die nur aus Schema und Host besteht (z.B. `https://staging.flauschiversum.de`), ohne Pfadanteil. Alle Absolut-URLs in kanonischen Verweisen, Open Graph, strukturierten Daten, Sitemap und Feed richten sich nach dieser Basis. Interne Verweise bleiben wurzelbezogen (`/themen/`), und die Dateien im Ausgabeverzeichnis liegen unabhängig von der Basis an denselben Stellen.

#### Scenario: Staging-Basis
- **WHEN** mit Basis `https://staging.flauschiversum.de` gebaut wird
- **THEN** zeigt `canonical` der Seite „Kürbis“ auf `https://staging.flauschiversum.de/2020/10/kurbis/`, und der Feed verweist auf `https://staging.flauschiversum.de/rss.xml`

#### Scenario: Interne Links unverändert
- **WHEN** mit Basis `https://staging.flauschiversum.de` gebaut wird
- **THEN** zeigt der Navigationslink „Themen“ weiter auf `/themen/`

#### Scenario: Standardbasis
- **WHEN** ohne Basis-URL gebaut wird
- **THEN** ist die Ausgabe gleich der bisherigen mit `https://flauschiversum.de`

#### Scenario: Ungültige Basis
- **WHEN** die Basis-URL nicht mit `http://` oder `https://` beginnt, einen Pfadanteil hat oder auf `/` endet
- **THEN** bricht der Build ab und nennt den Wert und das erwartete Format
