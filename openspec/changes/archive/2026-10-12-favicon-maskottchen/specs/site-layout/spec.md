## MODIFIED Requirements

### Requirement: Grundgerüst jeder Seite
Jede HTML-Seite SHALL als deutschsprachiges Dokument (`lang="de"`, UTF-8) mit Viewport-Angabe für Mobilgeräte ausgeliefert werden. Sie bindet das Stylesheet `/style.css` und die Favicon-Dateien (`/favicon.ico`, `/favicon.svg`, `/apple-touch-icon.png`, Spec `favicon`) ein. Verweise auf Stylesheet und Skripte tragen einen Versionsparameter aus dem Inhalts-Hash (z.B. `/style.css?v=3f2a91c0`), damit Browser nach einer Änderung keine veraltete Fassung verwenden.

#### Scenario: Seite auf dem Handy
- **WHEN** eine beliebige Seite auf einem Mobilgerät geöffnet wird
- **THEN** wird sie in Gerätebreite dargestellt, nicht als verkleinerte Desktop-Ansicht

#### Scenario: Geändertes Stylesheet
- **WHEN** sich der Inhalt von `/style.css` ändert und neu gebaut wird
- **THEN** verweisen alle Seiten mit einem neuen Wert für `v` auf `/style.css`

#### Scenario: Icons verlinkt
- **WHEN** der Kopf einer beliebigen Seite untersucht wird
- **THEN** enthält er Verweise auf `/favicon.ico`, `/favicon.svg` und `/apple-touch-icon.png`
