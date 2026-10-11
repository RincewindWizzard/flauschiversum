# Spec Delta

## Purpose

Beschreibt die Topbar des Flauschiversums: die einzige Navigation oben auf jeder Seite mit Titel, den Menüs About me, Beiträge und Kategorien sowie der Suche.

## ADDED Requirements

### Requirement: Topbar auf jeder Seite
Jede Seite SHALL oben eine Topbar zeigen. Links steht der Titel „Flauschiversum“ in Lobster als Link auf `/`. Rechtsbündig stehen in dieser Reihenfolge die Menüs „About me“, „Beiträge“ und „Kategorien“ und danach das Suchfeld. Auf Beitragsseiten wächst oder verdeckt die Topbar den Inhalt nicht, und sie bleibt beim Scrollen oben stehen.

#### Scenario: Aufbau am Desktop
- **WHEN** eine beliebige Seite bei 1280 px Breite geöffnet wird
- **THEN** steht links „Flauschiversum“, rechts „About me“, „Beiträge“, „Kategorien“ und das Suchfeld

#### Scenario: Titel führt zur Startseite
- **WHEN** auf einer Beitragsseite „Flauschiversum“ angeklickt wird
- **THEN** öffnet sich die Startseite `/`

#### Scenario: Leiste bleibt oben
- **WHEN** auf einer langen Seite gescrollt wird
- **THEN** bleibt die Topbar am oberen Rand sichtbar

### Requirement: About me
„About me“ SHALL ein einfacher Link auf `/about/` sein, ohne Untermenü. Die Seite enthält nur die Informationen über die Autorin.

#### Scenario: About me öffnen
- **WHEN** „About me“ angeklickt wird
- **THEN** öffnet sich `/about/`

#### Scenario: Kein Aufklappen
- **WHEN** der Mauszeiger über „About me“ steht oder es den Fokus erhält
- **THEN** erscheint kein Untermenü

### Requirement: Menü Beiträge
„Beiträge“ SHALL ein Link auf `/beitraege/` sein, der beim Überfahren, bei Fokus oder über einen Knopf ein Untermenü mit den zehn neuesten veröffentlichten Beiträgen (neueste zuerst, Titel als Links) und am Ende „Alle Beiträge“ auf `/beitraege/` aufklappt. Geplante Beiträge fehlen.

#### Scenario: Zehn neueste im Untermenü
- **WHEN** es 194 veröffentlichte Beiträge gibt und das Menü „Beiträge“ aufgeklappt wird
- **THEN** zeigt es genau die zehn neuesten Beiträge und „Alle Beiträge“

#### Scenario: Klick auf Beiträge
- **WHEN** der Eintrag „Beiträge“ selbst angeklickt wird
- **THEN** öffnet sich `/beitraege/`

#### Scenario: Beitrag aus dem Untermenü
- **WHEN** im Untermenü „Kürbis“ angeklickt wird
- **THEN** öffnet sich `/2020/10/kurbis/`

### Requirement: Menü Kategorien
„Kategorien“ SHALL ein Untermenü mit den Kategorien aus `content/categories.yaml` in der Reihenfolge Filzen, Basteln, Malen, Wolle, Brandings, Nähen aufklappen, jede mit Link auf `/kategorie/<name>/`. Kategorien ohne veröffentlichten Beitrag fehlen. Ein Klick auf „Kategorien“ selbst öffnet `/themen/`.

#### Scenario: Sechs Kategorien
- **WHEN** das Menü „Kategorien“ aufgeklappt wird
- **THEN** zeigt es die sechs Kategorien als Links auf ihre Kategorieseiten

#### Scenario: Reihenfolge
- **WHEN** `content/categories.yaml` die Reihenfolge Filzen, Basteln, Malen, Wolle, Brandings, Nähen vorgibt
- **THEN** erscheinen die Einträge in dieser Reihenfolge

#### Scenario: Klick auf Kategorien
- **WHEN** der Eintrag „Kategorien“ selbst angeklickt wird
- **THEN** öffnet sich die Themenübersicht `/themen/`

### Requirement: Bedienung der Untermenüs
Die Untermenüs SHALL per Maus (Überfahren, Klick auf den Knopf), per Tastatur (Tab, Enter/Leertaste auf dem Knopf, Pfeiltasten, Escape zum Schließen) und per Touch bedienbar sein. Der Knopf trägt `aria-expanded` und `aria-controls`. Ein Klick außerhalb oder Escape schließt ein offenes Untermenü, und es ist höchstens eines offen. Ohne JavaScript öffnen sich die Untermenüs bei Überfahren und Fokus (CSS), und die Links sind im Quelltext vorhanden.

#### Scenario: Tastatur
- **WHEN** mit Tab auf den Knopf von „Kategorien“ gewechselt und Enter gedrückt wird
- **THEN** öffnet sich das Untermenü, der Knopf meldet `aria-expanded="true"`, und Escape schließt es wieder

#### Scenario: Nur ein Menü offen
- **WHEN** „Kategorien“ geöffnet wird, während „Beiträge“ offen ist
- **THEN** schließt sich „Beiträge“

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet und der Mauszeiger über „Beiträge“ bewegt wird
- **THEN** erscheint das Untermenü mit den zehn Beiträgen

### Requirement: Topbar auf dem Handy
Auf Fenstern unter 48 em SHALL die Topbar den Titel, das Suchfeld bzw. einen Such-Knopf und einen Menü-Knopf zeigen. Der Menü-Knopf klappt eine Liste mit „About me“, „Beiträge“ und „Kategorien“ samt Unterpunkten auf; die Liste ist ohne JavaScript über ein `details`-Element bedienbar. Die Seite scrollt nie waagerecht, und alle Ziele haben eine Klickfläche von mindestens 44 px Höhe.

#### Scenario: Menü am Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet und der Menü-Knopf gedrückt wird
- **THEN** erscheinen „About me“, „Beiträge“ und „Kategorien“ mit ihren Unterpunkten untereinander

#### Scenario: Kein waagerechtes Scrollen
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** gibt es kein waagerechtes Scrollen, auch nicht mit geöffnetem Menü

### Requirement: Suchfeld in der Topbar
Die Topbar SHALL ein Suchfeld mit eingebauter Lupe enthalten (Spezifikation „Suchfeld mit eingebauter Lupe“). Auf der Startseite und der Suchseite entfällt es nicht; es gibt je Seite höchstens ein sichtbares Suchfeld in der Topbar.

#### Scenario: Suche aus der Topbar
- **WHEN** in der Topbar „Mütze“ eingegeben und Enter gedrückt wird
- **THEN** öffnet sich `/suche/?q=M%C3%BCtze`
