## Purpose

Beschreibt das automatische Inhaltsverzeichnis in der Seitenleiste von Einzelseiten: Besucher sehen die Abschnitte einer langen Seite und springen gezielt dorthin, ohne dass das Verzeichnis gepflegt werden muss.

## ADDED Requirements

### Requirement: Inhaltsverzeichnis aus den Überschriften
Jede Einzelseite SHALL eine Karte „Inhalt“ mit Links auf die Überschriften ihres Inhalts zeigen, wenn der Inhalt mindestens zwei Überschriften hat. Das Verzeichnis entsteht beim Build automatisch aus den Überschriften des Markdown, ohne Angabe im Frontmatter; der Seitentitel gehört nicht dazu. Hat die Seite weniger als zwei Überschriften, gibt es keine Karte.

#### Scenario: Tipps und Tricks
- **WHEN** `/tipps/` geöffnet wird und der Inhalt die Überschriften „Das kreative schwarze Loch“, „Ufo's“ und „Good-2-Know“ enthält
- **THEN** zeigt die Karte „Inhalt“ drei Links in dieser Reihenfolge, jeweils auf den Anker der Überschrift

#### Scenario: Überschrift ändern
- **WHEN** eine Überschrift im Markdown umbenannt und neu gebaut wird
- **THEN** zeigt das Verzeichnis den neuen Text, ohne dass etwas anderes gepflegt wird

#### Scenario: Zu wenige Überschriften
- **WHEN** eine Einzelseite nur eine oder keine Überschrift im Inhalt hat
- **THEN** zeigt sie keine Karte „Inhalt“

#### Scenario: Beiträge
- **WHEN** die Seite eines Beitrags geöffnet wird
- **THEN** gibt es dort kein Inhaltsverzeichnis

### Requirement: Verschachtelung nach Überschriftenebene
Das Verzeichnis SHALL die Überschriften nach ihrer Ebene relativ zur kleinsten vorkommenden Ebene der Seite verschachteln, höchstens drei Stufen tief; tiefere Überschriften erscheinen auf der dritten Stufe. Die Reihenfolge ist die des Textes. Uneinheitliche Ebenen im Markdown (`#` und `##` gemischt) ergeben dennoch eine sinnvolle Gliederung.

#### Scenario: Abschnitte mit Unterabschnitten
- **WHEN** eine Seite die Überschriften „Die heimlichen Stars“ (Ebene 1), „Shubia“ und „Jemima“ (Ebene 2) enthält
- **THEN** stehen „Shubia“ und „Jemima“ eingerückt unter „Die heimlichen Stars“

#### Scenario: Nur Ebene zwei
- **WHEN** alle Überschriften einer Seite die Ebene 2 haben (wie im Impressum)
- **THEN** stehen sie alle auf der ersten Stufe

### Requirement: Anker an den Überschriften
Jede Überschrift im Inhalt einer Einzelseite SHALL ein `id` tragen, das aus dem Text der Überschrift entsteht (Kleinschreibung, `ä`→`ae`, `ö`→`oe`, `ü`→`ue`, `ß`→`ss`, Sonderzeichen als Bindestrich). Gleiche Überschriften werden nummeriert (`-2`, `-3`). Jede `id` kommt auf der Seite genau einmal vor, auch gegenüber den Elementen des Seitenrahmens. Ein Sprung auf den Anker zeigt die Überschrift unterhalb der Topbar, nicht unter ihr.

#### Scenario: Anker aus dem Text
- **WHEN** eine Überschrift „Ufo's“ lautet
- **THEN** hat sie ein `id`, und `#<id>` in der Adresse führt zu ihr

#### Scenario: Doppelte Überschriften
- **WHEN** zwei Überschriften denselben Text haben
- **THEN** haben sie verschiedene `id`

#### Scenario: Nicht unter der Topbar
- **WHEN** ein Link im Verzeichnis angeklickt wird
- **THEN** steht die Überschrift sichtbar unter der Topbar

### Requirement: Platz des Verzeichnisses
Das Verzeichnis SHALL eine Karte der Seitenleiste sein: ab 64 em rechts neben dem Text (beim Scrollen sichtbar bleibend, solange es ins Fenster passt), auf schmaleren Fenstern über dem Text und zunächst zugeklappt, aufklappbar per Klick und Tastatur. Es ist ohne JavaScript benutzbar.

#### Scenario: Desktop
- **WHEN** `/tipps/` bei 1440 px Breite geöffnet wird
- **THEN** steht die Karte „Inhalt“ rechts neben dem Text und bleibt beim Scrollen im Bild

#### Scenario: Handy
- **WHEN** `/tipps/` bei 360 px Breite geöffnet wird
- **THEN** steht die zugeklappte Karte „Inhalt“ über dem Text, lässt sich aufklappen, und die Seite scrollt nicht waagerecht

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet wird
- **THEN** sind alle Links des Verzeichnisses sichtbar und führen zu den Abschnitten

### Requirement: Aktueller Abschnitt
Mit JavaScript SHALL das Verzeichnis den Abschnitt hervorheben (`aria-current`), der beim Lesen gerade im Bild ist. Ohne JavaScript bleibt das Verzeichnis eine einfache Linkliste.

#### Scenario: Scrollen
- **WHEN** auf `/tipps/` bis zur zweiten Überschrift gescrollt wird
- **THEN** ist deren Link im Verzeichnis hervorgehoben und der vorherige nicht mehr
