## Context

Das Suchfeld der Topbar steht in `templates/layout.html` (`#q-top`) mit dem Platzhalter „Anleitung suchen …“ und einem Label „Suchen“ (`sr-only`). Der Platzhalter steht in keiner Spec; er steht in den Snapshots aller Seiten. Das Feld auf der Suchseite (`#q`) hat Beispiele als Platzhalter und bleibt.

## Decisions

### 1. Text

Platzhalter „Beitrag suchen …“ (mit Leerzeichen vor den drei Punkten wie bisher, typografisch einheitlich mit der übrigen Seite). Das `label` für `#q-top` wird zu „Beitrag suchen“, damit der zugängliche Name des Feldes dem sichtbaren Text entspricht (WCAG „Label in Name“). Der Knopf behält „Suchen“.

### 2. Spec und Test

Der Platzhalter wird in der Spec `topbar` festgehalten (bisher ungeregelt), damit er nicht unbemerkt wieder zu einem engeren Begriff wird. Ein Test im Browser prüft Platzhalter und zugänglichen Namen auf einer Seite; die Snapshots ziehen den neuen Text nach.

## Risks / Trade-offs

- [Platzhalter wird in schmalen Feldern abgeschnitten] → „Beitrag suchen …“ ist kürzer als „Anleitung suchen …“; die Topbar-Tests bei 360 und 770 px bleiben maßgeblich.
