# Spec Delta

## MODIFIED Requirements

### Requirement: Hauptnavigation
Jede Seite SHALL eine Navigation mit diesen Einträgen in dieser Reihenfolge haben: „Start“ (`/`), „Themen“ (`/themen/`), „Beiträge“ (`/beitraege/`), „Tipps und Tricks“ (`/tipps-und-tricks/`), „About“ (`/about/`), „Impressum“ (`/impressum/`), „Abonnieren“ (`/rss.xml`).

#### Scenario: Feed abonnieren
- **WHEN** „Abonnieren“ angeklickt wird
- **THEN** wird `/rss.xml` geöffnet

#### Scenario: Themen finden
- **WHEN** „Themen“ angeklickt wird
- **THEN** wird `/themen/` geöffnet

### Requirement: Seitentitel
Jede HTML-Seite SHALL einen nicht leeren `<title>` mit korrekt maskierten Sonderzeichen tragen. Post-Seiten, Einzelseiten, Kategorie- und Schlagwortseiten heißen „<Titel> – Flauschiversum“, die Startseite „Flauschiversum – Mehr als nur eine Masche“, die Liste „Alle Beiträge – Flauschiversum“ mit Folgeseiten „Alle Beiträge – Seite <N> – Flauschiversum“, die Übersicht „Themen – Flauschiversum“ und die Suche „Suche – Flauschiversum“.

#### Scenario: Titel eines Posts
- **WHEN** die Seite des Posts „Alraune“ geöffnet wird
- **THEN** lautet der Dokumenttitel „Alraune – Flauschiversum“

#### Scenario: Titel der Startseite
- **WHEN** `/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Flauschiversum – Mehr als nur eine Masche“

#### Scenario: Titel einer Folgeseite
- **WHEN** `/page/1/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Alle Beiträge – Seite 2 – Flauschiversum“

#### Scenario: Titel einer Kategorie
- **WHEN** `/kategorie/filzen/` geöffnet wird
- **THEN** lautet der Dokumenttitel „Filzen – Flauschiversum“

## ADDED Requirements

### Requirement: Suchfeld im Kopf
Jede Seite außer der Startseite und der Suchseite SHALL im Kopf ein Suchfeld mit Beschriftung „Suchen“ haben. Das Abschicken öffnet `/suche/?q=<Begriff>`.

#### Scenario: Suche aus einem Beitrag
- **WHEN** auf einer Post-Seite „Mütze“ eingegeben und abgeschickt wird
- **THEN** öffnet sich `/suche/?q=M%C3%BCtze`
