# site-layout Specification

## Purpose

Beschreibt den gemeinsamen Rahmen aller HTML-Seiten des Flauschiversums: Kopfbereich mit Logo und Navigation, Fußzeile, Seitentitel und eingebundene Ressourcen.

## Requirements

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

### Requirement: Fußzeile
Jede Seite SHALL am Ende eine Fußzeile haben, die genau dem Layout der Fußzeile der Segelgruppe Kiel (segelgruppe-kiel.de) folgt: ein Band über die volle Breite mit Innenabstand 2 em, gedämpfter heller Schrift (`#afafaf`) auf dunklem Grund und zwei Spalten in einer Zeile, mittig angeordnet. Die linke Spalte ist ein Block, in dem eine Navigation mit den Icons der Kontaktwege nach der Spec `contact-links` (eine Zeile, gleichmäßig verteilt, Höhe 3 em) über dem Hinweis „© <Jahr> Ermeline Wollknoll“ steht. Die rechte Spalte ist eine Navigation mit den Links „About me“ (`/about/`) und „Impressum“ (`/impressum/`) als Liste untereinander, ohne Aufzählungszeichen und ohne Unterstreichung, jeweils mit 0,2 em Innenabstand. Links behalten ihre Farbe auch nach dem Besuch. Das Jahr ist das Jahr des Builds. Auf dem Handy steht das Copyright immer als letztes Element der Fußzeile.

#### Scenario: Jahr im Copyright
- **WHEN** die Seite im Jahr 2021 gebaut wurde
- **THEN** steht in der Fußzeile „© 2021 Ermeline Wollknoll“

#### Scenario: Seitenlinks in der Fußzeile
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Fußzeile die Links „About me“ und „Impressum“ mit den genannten Zielen untereinander, aber keine Links „Abonnieren“ und „Tipps und Tricks“ (der steht in der Topbar)

#### Scenario: Anordnung wie das Vorbild
- **WHEN** eine Seite bei 1280 px Breite geöffnet wird
- **THEN** spannt die Fußzeile die volle Breite mit dunklem Hintergrund und Schriftfarbe `#afafaf`, links stehen die Icons in einer Zeile mit dem Copyright linksbündig darunter, und rechts neben diesem Block stehen die zwei Links untereinander

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

### Requirement: Keine Ressourcen von Drittanbietern
Seiten SHALL keine Skripte, Stylesheets, Schriften oder Bilder von fremden Servern laden. Schriften (Lobster, Lora, Merriweather) und Skripte werden von der eigenen Domain ausgeliefert. Links auf fremde Seiten im Inhalt sind erlaubt.

#### Scenario: Seitenaufruf ohne Drittanfragen
- **WHEN** eine beliebige Seite vollständig geladen wird
- **THEN** wurden alle Ressourcen von `flauschiversum.de` geladen

### Requirement: Inhalte ohne JavaScript nutzbar
Alle Texte, Bilder, Links und die Navigation SHALL auch ohne JavaScript sichtbar und nutzbar sein. JavaScript erweitert nur die Bedienung (z.B. Vollbildansicht), und kein Skript benötigt eine Bibliothek wie jQuery.

#### Scenario: JavaScript deaktiviert
- **WHEN** eine Post-Seite mit Karussell bei deaktiviertem JavaScript geöffnet wird
- **THEN** sind alle Bilder des Karussells durch Wischen oder Scrollen erreichbar, und der Text ist vollständig lesbar

### Requirement: Staging-Leiste mit Branchname
Im Staging-Modus SHALL jede HTML-Seite ganz oben, noch vor dem Seitenkopf, eine deutlich sichtbare Leiste mit dem Text „Staging“ und dem Namen des Branches zeigen, aus dem gebaut wurde. Die Leiste ist auch ohne JavaScript sichtbar, scrollt mit der Seite oder bleibt oben stehen, ohne Inhalt zu verdecken, und ist bei jeder Fensterbreite lesbar. Ohne Staging-Modus gibt es sie nicht. Weiterleitungsseiten sind davon ausgenommen.

#### Scenario: Leiste auf der Startseite
- **WHEN** aus dem Branch `claude/beautiful-darwin-66rwab` im Staging-Modus gebaut wird
- **THEN** steht ganz oben auf `/` „Staging: claude/beautiful-darwin-66rwab“

#### Scenario: Leiste auf jeder Seite
- **WHEN** im Staging-Modus gebaut wird
- **THEN** zeigen Beitrag, Thema, Suche und Einzelseite dieselbe Leiste

#### Scenario: Lange Branchnamen am Handy
- **WHEN** der Branchname länger ist als die Zeile
- **THEN** wird er umgebrochen, und die Seite scrollt nicht waagerecht

#### Scenario: Produktion
- **WHEN** ohne Staging-Modus gebaut wird
- **THEN** enthält keine Seite die Leiste

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

### Requirement: Typografie
Die Schrift „Lobster“ SHALL nur für das Logo und für Titel (Überschriften der Seiten, Abschnitte und Karten) verwendet werden. Navigationslinks, Tag-Cloud, Themenlisten, Knöpfe und Feldbeschriftungen SHALL in der Textschrift „Lora“ stehen, mindestens 0,9 em groß und, wo sie Navigation sind, halbfett statt in Zierschrift. Es gibt eine gemeinsame Skala mit festen Größen für Titel, Zwischentitel, Fließtext und Kleintext.

#### Scenario: Navigation in Textschrift
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** sind die Links in Topbar und Fußzeile nicht in Lobster gesetzt

#### Scenario: Titel in Lobster
- **WHEN** die Startseite geöffnet wird
- **THEN** sind Logo und die Abschnittstitel der Karten in Lobster gesetzt

#### Scenario: Lesbarkeit im Footer
- **WHEN** die Fußzeile angezeigt wird
- **THEN** haben ihre Links mindestens 0,9 em Schriftgröße und die Textschrift

### Requirement: Farbschema
Die Seite SHALL das Farbschema „Flauschi-Nebel“ (Violett und Indigo, Variablen in `content/style/theme.css`) in einem hellen und einem dunklen Modus verwenden: Hintergrund, weiße bzw. dunkle Karten, Text, graue Schrift, Links, Akzent für Knöpfe und aktive Elemente, Rand und Tönung für Hover. Alle Farben sind als Variablen an einer Stelle definiert, Text und Links erreichen mindestens 4,5 : 1 Kontrast, das frühere Neongrün entfällt.

#### Scenario: Hintergrund und Karten
- **WHEN** eine beliebige Seite in einem der beiden Modi geöffnet wird
- **THEN** sind Karten deutlich von ihrem Seitenhintergrund unterscheidbar, und es gibt keine grellen Neonflächen

#### Scenario: Kontrast
- **WHEN** Fließtext, Links, Knöpfe und Tags in beiden Modi auf ihren Hintergründen geprüft werden
- **THEN** beträgt das Kontrastverhältnis mindestens 4,5 : 1

#### Scenario: Staging-Leiste bleibt auffällig
- **WHEN** eine Staging-Seite in einem der beiden Modi geöffnet wird
- **THEN** hebt sich die Staging-Leiste mit ihrer roten Farbe weiterhin deutlich vom Schema ab

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

### Requirement: Karten für ergänzende Blöcke
Tag-Cloud und Archiv SHALL auf Karten liegen: heller Hintergrund, abgerundete Ecken, feiner Rand oder Schatten und Innenabstand.

#### Scenario: Karten sichtbar
- **WHEN** die Startseite bei 1440 px Breite geöffnet wird
- **THEN** liegt die Tag-Cloud auf einer Karte mit abgerundeten Ecken und hellem Hintergrund

### Requirement: Titel der Seiten
Jede HTML-Seite SHALL einen nicht leeren `<title>` mit korrekt maskierten Sonderzeichen tragen. Post-Seiten, Einzelseiten und Schlagwortseiten heißen „<Titel> – Flauschiversum“, die Startseite „Flauschiversum – Mehr als nur eine Masche“, die Liste „Alle Beiträge – Flauschiversum“ mit Folgeseiten „Alle Beiträge – Seite <N> – Flauschiversum“, die Übersicht „Themen – Flauschiversum“ und die Suche „Suche – Flauschiversum“.

#### Scenario: Titel eines Posts
- **WHEN** die Seite des Posts „Alraune“ geöffnet wird
- **THEN** lautet der Dokumenttitel „Alraune – Flauschiversum“

#### Scenario: Titel der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Flauschiversum – Mehr als nur eine Masche“

#### Scenario: Titel einer Folgeseite
- **WHEN** `/page/1/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Alle Beiträge – Seite 2 – Flauschiversum“

#### Scenario: Titel eines Themas
- **WHEN** `/tag/filzen/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Filzen – Flauschiversum“
