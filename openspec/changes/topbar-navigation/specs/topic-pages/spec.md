# Spec Delta

## MODIFIED Requirements

### Requirement: Kategorieseite
Für jede Kategorie mit mindestens einem veröffentlichten Beitrag SHALL es unter `/kategorie/<name>/` eine Seite geben. Sie zeigt oben den Namen, den Reim und einen kurzen Einleitungstext zur Kategorie (`intro` aus `content/categories.yaml`, fehlt er, nur Reim und Beitragszahl), darunter alle veröffentlichten Beiträge der Kategorie, neueste zuerst, auf einer einzigen Seite ohne Paginierung, und daneben das Archiv der Kategorie nach Jahr und Monat (Spec `post-archive`).

#### Scenario: Alle Beiträge auf einer Seite
- **WHEN** die Kategorie „Basteln“ 54 veröffentlichte Beiträge hat
- **THEN** zeigt `/kategorie/basteln/` alle 54 Beiträge, ohne Seitennummern

#### Scenario: Ältere Beiträge sind erreichbar
- **WHEN** ein Beitrag von 2013 in der Kategorie „Wolle“ liegt
- **THEN** steht er auf `/kategorie/wolle/` unten in der Liste und im Archiv unter 2013

#### Scenario: Text vor den Beiträgen
- **WHEN** `/kategorie/filzen/` geöffnet wird
- **THEN** steht der Einleitungstext über der Beitragsliste, und die Beiträge folgen darunter

#### Scenario: Archiv der Kategorie
- **WHEN** `/kategorie/filzen/` bei 1440 px Breite geöffnet wird
- **THEN** steht rechts das Archiv mit nur den Beiträgen der Kategorie „Filzen“

## REMOVED Requirements

### Requirement: Schlagwörter innerhalb einer Kategorie
**Reason**: Die Kategorieseite zeigt statt der Schlagwortliste das Archiv nach Monat und Jahr; die Schlagwörter stehen weiter an den Beiträgen und als Tag-Cloud auf der Startseite.
**Migration**: Schlagworte über die Cloud auf der Startseite, `/themen/` oder die Beiträge erreichen.
