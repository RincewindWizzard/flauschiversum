## ADDED Requirements

### Requirement: Lesbare Absätze
Im Text eines Beitrags und einer Einzelseite SHALL zwischen aufeinanderfolgenden Absätzen ein sichtbarer vertikaler Abstand von mindestens einer Zeilenhöhe liegen. Listen, Zitate und vorformatierte Blöcke SHALL nach unten einen vergleichbaren Abstand haben, Überschriften SHALL einen größeren Abstand davor als danach haben. Der Abstand SHALL mit der Schriftgröße skalieren und gilt nur im Textbereich der Seite, nicht in Kopf, Sidebar, Übersichten und Karten.

#### Scenario: Zwei Absätze nacheinander
- **WHEN** ein Beitrag zwei aufeinanderfolgende Absätze enthält
- **THEN** liegt zwischen dem Ende des ersten und dem Anfang des zweiten Absatzes ein Abstand, der mindestens eine Zeilenhöhe beträgt

#### Scenario: Liste und Überschrift im Text
- **WHEN** ein Beitrag auf einen Absatz eine Liste und danach eine Überschrift folgen lässt
- **THEN** ist die Liste vom Absatz davor und vom Text danach sichtbar abgesetzt, und die Überschrift hat mehr Abstand nach oben als nach unten

#### Scenario: Bereiche außerhalb des Textes bleiben unverändert
- **WHEN** eine Übersichts- oder Themenseite, die Sidebar oder der Autorenhinweis betrachtet wird
- **THEN** ändern sich deren Abstände durch diese Anforderung nicht

#### Scenario: Dunkles Thema
- **WHEN** das dunkle Farbschema aktiv ist
- **THEN** sind die Absatzabstände dieselben wie im hellen Schema
