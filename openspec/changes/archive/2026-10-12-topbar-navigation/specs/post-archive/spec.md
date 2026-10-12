# Spec Delta

## Purpose

Beschreibt das Archiv der Beiträge nach Jahr und Monat: eine einklappbare Liste, mit der Besucher jeden Beitrag über sein Datum finden, auf der Seite Beiträge für alle Beiträge und auf Themenseiten für die Beiträge des Themas.

## ADDED Requirements

### Requirement: Archiv nach Jahr und Monat
Das Archiv SHALL die veröffentlichten Beiträge nach Jahr (neuestes zuerst) und darin nach Monat (neuester zuerst) gruppieren. Jede Jahres- und Monatsgruppe zeigt ihre Beitragszahl und lässt sich ein- und ausklappen. Unter jedem Monat stehen die Beiträge, neueste zuerst, als Links mit Titel und Tag des Monats. Geplante Beiträge fehlen, leere Gruppen gibt es nicht.

#### Scenario: Gruppierung
- **WHEN** es Beiträge aus Oktober 2020 und März 2019 gibt
- **THEN** steht „2020“ vor „2019“, darunter „Oktober (n)“ bzw. „März (n)“ mit den Titeln als Links

#### Scenario: Ein- und Ausklappen
- **WHEN** auf „2018“ geklickt wird, während die Gruppe zugeklappt ist
- **THEN** klappt sie auf und zeigt ihre Monate, ein zweiter Klick klappt sie wieder zu

#### Scenario: Beitrag anklicken
- **WHEN** im Archiv ein Titel angeklickt wird
- **THEN** öffnet sich die Seite dieses Beitrags

### Requirement: Voreinstellung und Bedienung des Archivs
Beim Laden SHALL nur die Gruppe des neuesten Jahres und darin des neuesten Monats aufgeklappt sein, alle anderen sind zugeklappt. Das Archiv ist ohne JavaScript bedienbar (`details`/`summary`), per Tastatur erreichbar und für Screenreader als Liste mit Überschrift ausgezeichnet. Ein Knopf „Alle aufklappen“ bzw. „Alle zuklappen“ ist nur mit JavaScript sichtbar.

#### Scenario: Zustand beim Laden
- **WHEN** die Seite Beiträge geöffnet wird
- **THEN** ist das neueste Jahr mit seinem neuesten Monat aufgeklappt, und alle anderen Gruppen sind zugeklappt

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet und eine Jahresgruppe angeklickt wird
- **THEN** klappt sie auf

#### Scenario: Alle aufklappen
- **WHEN** mit JavaScript „Alle aufklappen“ gedrückt wird
- **THEN** sind alle Jahres- und Monatsgruppen offen

### Requirement: Archiv eines Themas
Auf einer Themenseite SHALL das Archiv nur die veröffentlichten Beiträge dieses Schlagworts enthalten, mit denselben Gruppen und derselben Bedienung. Beiträge ohne das Schlagwort fehlen.

#### Scenario: Nur ein Thema
- **WHEN** `/tag/filzen/` geöffnet wird
- **THEN** enthält das Archiv genau die Beiträge mit dem Schlagwort „Filzen“, und die Gesamtzahl im Archiv stimmt mit der Zahl der Beiträge des Themas überein

#### Scenario: Leere Monate
- **WHEN** ein Thema in einem Monat keinen Beitrag hat
- **THEN** erscheint dieser Monat nicht

### Requirement: Platz des Archivs
Das Archiv SHALL auf der Seite Beiträge und auf Themenseiten als Karte neben der Beitragsliste stehen (ab 64 em rechts), auf schmaleren Fenstern auf der Seite Beiträge unter der Liste und auf Themenseiten zwischen Titel und Liste, und beim Scrollen am Desktop sichtbar bleiben, solange es ins Fenster passt.

#### Scenario: Desktop
- **WHEN** `/beitraege/` bei 1440 px Breite geöffnet wird
- **THEN** steht das Archiv als Karte rechts neben den zehn neuesten Beiträgen

#### Scenario: Handy
- **WHEN** `/beitraege/` bei 360 px Breite geöffnet wird
- **THEN** steht das Archiv unter der Beitragsliste, ohne waagerechtes Scrollen
