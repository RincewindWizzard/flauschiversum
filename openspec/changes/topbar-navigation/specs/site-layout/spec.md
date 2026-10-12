# Spec Delta

## ADDED Requirements

### Requirement: Seitenaufbau mit Topbar
Jede Seite SHALL aus Topbar, Inhalt und Fußzeile bestehen. Es gibt keine linke Navigationsspalte und keine allgemeine Sidebar mehr. Ergänzende Karten (Tag-Cloud auf der Startseite, Archiv auf Beiträge- und Kategorieseiten) stehen ab 64 em rechts neben dem Inhalt, auf schmaleren Fenstern darunter. Der Seitenrahmen ist bis etwa 1200 px breit, der Lesetext höchstens etwa 40 em.

#### Scenario: Beitragsseite
- **WHEN** eine Beitragsseite bei 1440 px Breite geöffnet wird
- **THEN** steht der Text mittig in einer Spalte, ohne Spalten rechts oder links

#### Scenario: Startseite
- **WHEN** die Startseite bei 1440 px Breite geöffnet wird
- **THEN** steht rechts neben den Beiträgen die Karte mit der Tag-Cloud

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** stehen ergänzende Karten unter dem Inhalt, und die Seite scrollt nicht waagerecht

### Requirement: Karten für ergänzende Blöcke
Tag-Cloud und Archiv SHALL auf Karten liegen: heller Hintergrund, abgerundete Ecken, feiner Rand oder Schatten und Innenabstand. Die Untermenüs der Topbar sind ebenfalls Karten mit Schatten über dem Inhalt.

#### Scenario: Karten sichtbar
- **WHEN** die Startseite bei 1440 px Breite geöffnet wird
- **THEN** liegt die Tag-Cloud auf einer Karte mit abgerundeten Ecken und hellem Hintergrund

## MODIFIED Requirements

### Requirement: Fußzeile
Jede Seite SHALL am Ende eine Fußzeile haben, die genau dem Layout der Fußzeile der Segelgruppe Kiel (segelgruppe-kiel.de) folgt: ein Band über die volle Breite mit Innenabstand 2 em, gedämpfter heller Schrift (`#afafaf`) auf dunklem Grund und zwei Spalten in einer Zeile, mittig angeordnet. Die linke Spalte ist ein Block, in dem eine Navigation mit den Icons der Kontaktwege nach der Spec `contact-links` (eine Zeile, gleichmäßig verteilt, Höhe 3 em) über dem Hinweis „© <Jahr> Ermeline Wollknoll“ steht. Die rechte Spalte ist eine Navigation mit den Links „Tipps und Tricks“ (`/tipps-und-tricks/`), „About me“ (`/about/`) und „Impressum“ (`/impressum/`) als Liste untereinander, ohne Aufzählungszeichen und ohne Unterstreichung, jeweils mit 0,2 em Innenabstand. Links behalten ihre Farbe auch nach dem Besuch. Das Jahr ist das Jahr des Builds. Auf dem Handy steht das Copyright immer als letztes Element der Fußzeile.

#### Scenario: Jahr im Copyright
- **WHEN** die Seite im Jahr 2021 gebaut wurde
- **THEN** steht in der Fußzeile „© 2021 Ermeline Wollknoll“

#### Scenario: Seitenlinks in der Fußzeile
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Fußzeile die Links „Tipps und Tricks“, „About me“ und „Impressum“ mit den genannten Zielen untereinander, aber keinen Link „Abonnieren“

#### Scenario: Anordnung wie das Vorbild
- **WHEN** eine Seite bei 1280 px Breite geöffnet wird
- **THEN** spannt die Fußzeile die volle Breite mit dunklem Hintergrund und Schriftfarbe `#afafaf`, links stehen die Icons in einer Zeile mit dem Copyright linksbündig darunter, und rechts neben diesem Block stehen die drei Links untereinander

#### Scenario: Größe der Icons
- **WHEN** die Fußzeile angezeigt wird
- **THEN** sind die Icons 3 em hoch

#### Scenario: Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** sind alle Links der Fußzeile sichtbar und erreichbar, und die Seite scrollt nicht waagerecht

#### Scenario: Copyright zuletzt auf dem Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** stehen in der Fußzeile von oben nach unten die Icons, die Links und zuletzt das Copyright

#### Scenario: Reihenfolge am Seitenende
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** steht die Fußzeile nach dem Hauptinhalt als letztes sichtbares Element

### Requirement: Suchfeld mit eingebauter Lupe
Jedes Suchfeld der Seite (Topbar, Startseite, Suchseite ohne JavaScript) SHALL aus einem einzigen Eingabefeld bestehen, in dem der Suchknopf mit Lupen-Icon am rechten Rand integriert ist. Eingabefeld und Knopf werden nie getrennt umgebrochen. Der Knopf hat den zugänglichen Namen „Suchen“, und Enter schickt die Suche ab.

#### Scenario: Lupe im Feld
- **WHEN** ein Suchfeld bei beliebiger Breite angezeigt wird
- **THEN** sitzt die Lupe innerhalb des abgerundeten Feldes am rechten Rand, und es gibt keinen separaten Knopf daneben oder darunter

#### Scenario: Schmale Spalte
- **WHEN** das Suchfeld in der Topbar bei 1024 px oder auf einem Handy mit 360 px Breite angezeigt wird
- **THEN** bleiben Feld und Lupe in einer Zeile

#### Scenario: Suche abschicken
- **WHEN** Text eingegeben und Enter gedrückt oder die Lupe angeklickt wird
- **THEN** öffnet sich `/suche/?q=<Begriff>`

## REMOVED Requirements

### Requirement: Kopfbereich
**Reason**: Logo und Untertitel gehen in der Topbar (Spec `topbar`) auf; der große Kopf entfällt.
**Migration**: Der Titel „Flauschiversum“ links in der Topbar verlinkt auf `/`.

### Requirement: Hauptnavigation
**Reason**: Die Navigation besteht jetzt aus den Menüs der Topbar (Spec `topbar`); „Tipps und Tricks“, „Impressum“ und „Abonnieren“ stehen weiter in der Fußzeile.
**Migration**: Start über den Titel, Beiträge und Kategorien über die Menüs, About über „About me“.

### Requirement: Suchfeld im Kopf
**Reason**: Das Suchfeld steht in der Topbar (Spec `topbar`, „Suchfeld in der Topbar“).
**Migration**: Kein Handlungsbedarf.

### Requirement: Dreispaltiges Layout auf breiten Fenstern
**Reason**: Die Seitenspalten entfallen zugunsten der Topbar.
**Migration**: Siehe „Seitenaufbau mit Topbar“.

### Requirement: Navigationsspalte wie ein Inhaltsverzeichnis
**Reason**: Die Navigation steht in der Topbar.
**Migration**: Kategorien im Menü „Kategorien“, Seiten über die übrigen Menüs.

### Requirement: Zweispaltiges Layout auf mittleren Fenstern
**Reason**: Ersetzt durch „Seitenaufbau mit Topbar“.
**Migration**: Keine.

### Requirement: Sidebar auf schmalen Fenstern und Ausnahmen
**Reason**: Es gibt keine allgemeine Sidebar mehr; ergänzende Karten stehen nach „Seitenaufbau mit Topbar“.
**Migration**: Keine.

### Requirement: Suchfeld auf breiten Fenstern
**Reason**: Das Suchfeld steht immer in der Topbar.
**Migration**: Keine.

### Requirement: Seitenspalten auf Karten
**Reason**: Ersetzt durch „Karten für ergänzende Blöcke“.
**Migration**: Keine.
