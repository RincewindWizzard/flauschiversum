# Spec Delta

## MODIFIED Requirements

### Requirement: Grundgerüst jeder Seite
Jede HTML-Seite SHALL als deutschsprachiges Dokument (`lang="de"`, UTF-8) mit Viewport-Angabe für Mobilgeräte ausgeliefert werden. Sie bindet das Stylesheet `/style.css` und das Favicon `/favicon.ico` ein. Verweise auf Stylesheet und Skripte tragen einen Versionsparameter aus dem Inhalts-Hash (z.B. `/style.css?v=3f2a91c0`), damit Browser nach einer Änderung keine veraltete Fassung verwenden.

#### Scenario: Seite auf dem Handy
- **WHEN** eine beliebige Seite auf einem Mobilgerät geöffnet wird
- **THEN** wird sie in Gerätebreite dargestellt, nicht als verkleinerte Desktop-Ansicht

#### Scenario: Geändertes Stylesheet
- **WHEN** sich der Inhalt von `/style.css` ändert und neu gebaut wird
- **THEN** verweisen alle Seiten mit einem neuen Wert für `v` auf `/style.css`

### Requirement: Seitentitel
Jede HTML-Seite SHALL einen nicht leeren `<title>` mit korrekt maskierten Sonderzeichen tragen. Post-Seiten und Einzelseiten heißen „<Titel> – Flauschiversum“, die erste Indexseite „Flauschiversum – Mehr als nur eine Masche“, und Folgeseiten heißen „Flauschiversum – Seite <N>“, wobei N die angezeigte Seitennummer ist.

#### Scenario: Titel eines Posts
- **WHEN** die Seite des Posts „Alraune“ geöffnet wird
- **THEN** lautet der Dokumenttitel „Alraune – Flauschiversum“

#### Scenario: Titel der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Flauschiversum – Mehr als nur eine Masche“

#### Scenario: Titel einer Folgeseite
- **WHEN** `/page/1/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Flauschiversum – Seite 2“

## ADDED Requirements

### Requirement: Inhalte ohne JavaScript nutzbar
Alle Texte, Bilder, Links und die Navigation SHALL auch ohne JavaScript sichtbar und nutzbar sein. JavaScript erweitert nur die Bedienung (z.B. Vollbildansicht), und kein Skript benötigt eine Bibliothek wie jQuery.

#### Scenario: JavaScript deaktiviert
- **WHEN** eine Post-Seite mit Karussell bei deaktiviertem JavaScript geöffnet wird
- **THEN** sind alle Bilder des Karussells durch Wischen oder Scrollen erreichbar, und der Text ist vollständig lesbar
