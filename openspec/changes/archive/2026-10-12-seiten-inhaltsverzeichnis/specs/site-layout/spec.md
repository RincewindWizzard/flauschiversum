## MODIFIED Requirements

### Requirement: Seitenaufbau mit Topbar
Jede Seite SHALL aus Topbar, Inhalt und Fußzeile bestehen. Es gibt keine linke Navigationsspalte und keine allgemeine Sidebar mehr. Ergänzende Karten (Tag-Cloud auf der Startseite, Archiv auf Beiträge- und Themenseiten, Inhaltsverzeichnis auf Einzelseiten) stehen ab 64 em rechts neben dem Inhalt, auf schmaleren Fenstern darunter; Karten, die der Orientierung im Inhalt dienen (Inhaltsverzeichnis der Einzelseiten, Themenfilter der Suche), stehen dort stattdessen über dem Inhalt. Der Seitenrahmen ist bis etwa 1200 px breit, der Lesetext höchstens etwa 40 em.

#### Scenario: Beitragsseite
- **WHEN** eine Beitragsseite bei 1440 px Breite geöffnet wird
- **THEN** steht der Text mittig in einer Spalte, ohne Spalten rechts oder links

#### Scenario: Startseite
- **WHEN** die Startseite bei 1440 px Breite geöffnet wird
- **THEN** steht rechts neben den Beiträgen die Karte mit der Tag-Cloud

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** stehen ergänzende Karten unter dem Inhalt (Inhaltsverzeichnis und Suchfilter über dem Inhalt), und die Seite scrollt nicht waagerecht
