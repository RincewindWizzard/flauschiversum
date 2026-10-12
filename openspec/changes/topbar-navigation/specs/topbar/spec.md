# Spec Delta

## Purpose

Beschreibt die Topbar des Flauschiversums: die einzige Navigation oben auf jeder Seite mit Titel, den Menüs About me, Beiträge und Kategorien sowie der Suche.

## ADDED Requirements

### Requirement: Topbar auf jeder Seite
Jede Seite SHALL oben eine Topbar zeigen. Links steht der Titel „Flauschiversum“ in Lobster als Link auf `/`. Rechtsbündig stehen in dieser Reihenfolge die Menüs „Beiträge“ und „Kategorien“ und danach das Suchfeld. Alle Elemente der Topbar (Maskottchen, Titel, Menüs, Suchfeld samt Lupe) sind vertikal auf einer Linie zentriert. Auf Beitragsseiten wächst oder verdeckt die Topbar den Inhalt nicht, und sie bleibt beim Scrollen oben stehen.

#### Scenario: Aufbau am Desktop
- **WHEN** eine beliebige Seite bei 1280 px Breite geöffnet wird
- **THEN** steht links „Flauschiversum“, rechts „Beiträge“, „Kategorien“ und das Suchfeld

#### Scenario: Vertikal zentriert
- **WHEN** eine beliebige Seite bei 1280 px Breite geöffnet wird
- **THEN** liegen die Mittelpunkte von Maskottchen, Titel, Menüeinträgen und Suchfeld auf derselben Linie (Abweichung höchstens 2 px)

#### Scenario: Titel führt zur Startseite
- **WHEN** auf einer Beitragsseite „Flauschiversum“ angeklickt wird
- **THEN** öffnet sich die Startseite `/`

#### Scenario: Leiste bleibt oben
- **WHEN** auf einer langen Seite gescrollt wird
- **THEN** bleibt die Topbar am oberen Rand sichtbar

### Requirement: Maskottchen im Markenzeichen
Links neben dem Titel in der Topbar SHALL das Maskottchen Flauschi stehen und zusammen mit dem Titel ein einziger Link auf `/` sein. Normal zeigt es `flauschi_normal.svg`, bei Mouse-over und Tastaturfokus `flauschi_surprised.svg`. Beide Bilder SHALL denselben Platz mit gleichen Maßen einnehmen, sodass der Wechsel nichts im Layout verschiebt. Das Maskottchen ist dekorativ (`alt=""`).

#### Scenario: Normal und Mouse-over
- **WHEN** die Maus über Maskottchen oder Titel fährt
- **THEN** erscheint das überraschte Bild an derselben Stelle und in derselben Größe, und Titel, Menüs und Suchfeld bleiben an ihrem Platz

#### Scenario: Position
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** steht das Maskottchen ganz links in der Topbar vor dem Titel „Flauschiversum“

#### Scenario: Tastaturfokus
- **WHEN** der Titel per Tab fokussiert wird
- **THEN** zeigt die Topbar das überraschte Bild

#### Scenario: Handy
- **WHEN** die Seite bei 360 px Breite geöffnet wird
- **THEN** sind Maskottchen und Titel sichtbar, und es entsteht kein waagerechtes Scrollen

### Requirement: Beiträge als einfacher Link
„Beiträge“ SHALL ein einfacher Link auf `/beitraege/` ohne Untermenü sein. „About me“ steht nicht in der Topbar, sondern in der Fußzeile.

#### Scenario: Klick auf Beiträge
- **WHEN** der Eintrag „Beiträge“ angeklickt wird
- **THEN** öffnet sich `/beitraege/`

#### Scenario: Kein Aufklappen
- **WHEN** der Mauszeiger über „Beiträge“ steht oder es den Fokus erhält
- **THEN** erscheint kein Untermenü

#### Scenario: About me nicht in der Topbar
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** enthält die Topbar keinen Link auf `/about/`

### Requirement: Menü Kategorien
„Kategorien“ SHALL ein Untermenü mit den Kategorien aus `content/categories.yaml` in der Reihenfolge der Datei aufklappen, jede mit Link auf `/kategorie/<name>/`. Kategorien ohne veröffentlichten Beitrag fehlen. Ein Klick auf „Kategorien“ selbst öffnet `/themen/`.

#### Scenario: Alle Kategorien
- **WHEN** das Menü „Kategorien“ aufgeklappt wird
- **THEN** zeigt es alle Kategorien als Links auf ihre Kategorieseiten

#### Scenario: Reihenfolge
- **WHEN** `content/categories.yaml` eine Reihenfolge vorgibt
- **THEN** erscheinen die Einträge in dieser Reihenfolge

#### Scenario: Klick auf Kategorien
- **WHEN** der Eintrag „Kategorien“ selbst angeklickt wird
- **THEN** öffnet sich die Themenübersicht `/themen/`

### Requirement: Bedienung der Untermenüs
Das Untermenü „Kategorien“ SHALL per Maus (Überfahren, Klick auf den Knopf), per Tastatur (Tab, Enter/Leertaste auf dem Knopf, Pfeiltasten, Escape zum Schließen) und per Touch bedienbar sein. Der Knopf trägt `aria-expanded` und `aria-controls`. Ein Klick außerhalb oder Escape schließt ein geöffnetes Untermenü; hat der Mauszeiger es geöffnet, hält ein Klick auf den Knopf es offen. Ohne JavaScript öffnet es sich bei Überfahren und Fokus (CSS), und die Links sind im Quelltext vorhanden.

#### Scenario: Tastatur
- **WHEN** mit Tab auf den Knopf von „Kategorien“ gewechselt und Enter gedrückt wird
- **THEN** öffnet sich das Untermenü, der Knopf meldet `aria-expanded="true"`, und Escape schließt es wieder

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet und der Mauszeiger über „Kategorien“ bewegt wird
- **THEN** erscheint das Untermenü mit den Kategorien

### Requirement: Topbar auf dem Handy
Auf Fenstern unter 48 em SHALL die Topbar den Titel, das Suchfeld bzw. einen Such-Knopf und einen Hamburger-Knopf (Symbol mit drei Strichen, zugänglicher Name „Menü“, mindestens 44 × 44 px) zeigen, Titel und Knopf in einer Zeile, das Suchfeld darunter. Der Knopf klappt eine Liste mit „Beiträge“ und „Kategorien“ samt Unterpunkten auf und beim zweiten Antippen wieder zu; die Liste ist ohne JavaScript über ein `details`-Element bedienbar.

#### Scenario: Menü am Handy
- **WHEN** eine Seite bei 360 px Breite geöffnet und der Menü-Knopf gedrückt wird
- **THEN** erscheinen „Beiträge“ und „Kategorien“ mit den Kategorien als Unterpunkten untereinander

#### Scenario: Hamburger-Knopf
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** steht rechts neben dem Titel ein Knopf mit drei Strichen auf einer Linie mit dem Titel, und das Menü ist zugeklappt

### Requirement: Handy ohne Überlauf
Auf dem Handy SHALL die Seite nie waagerecht scrollen, und alle Ziele des Menüs haben eine Klickfläche von mindestens 44 px Höhe.

#### Scenario: Kein waagerechtes Scrollen
- **WHEN** eine Seite bei 360 px Breite geöffnet wird
- **THEN** gibt es kein waagerechtes Scrollen, auch nicht mit geöffnetem Menü

### Requirement: Suchfeld in der Topbar
Die Topbar SHALL ein Suchfeld mit eingebauter Lupe enthalten (Spezifikation „Suchfeld mit eingebauter Lupe“). Auf der Startseite und der Suchseite entfällt es nicht; es gibt je Seite höchstens ein sichtbares Suchfeld in der Topbar.

#### Scenario: Suche aus der Topbar
- **WHEN** in der Topbar „Mütze“ eingegeben und Enter gedrückt wird
- **THEN** öffnet sich `/suche/?q=M%C3%BCtze`
