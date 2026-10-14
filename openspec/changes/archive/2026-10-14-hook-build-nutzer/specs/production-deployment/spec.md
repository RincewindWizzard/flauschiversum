## MODIFIED Requirements

### Requirement: Deployment per Git-Hook
Für Umgebungen ohne GitHub Actions SHALL das Repository einen `post-receive`-Hook (`scripts/git-hooks/post-receive`) und ein Installationsskript (`scripts/install-git-hook.sh`) liefern. Bei einem Push auf den Hauptbranch des Repositorys (den Branch, auf den `HEAD` zeigt) lässt der Hook den gepushten Stand von einem eigenen Build-Benutzer bauen und mit `flauschiversum deploy` übertragen; Aufbau, Rechte und Einrichtung dieses Benutzers beschreibt die Spec `build-user`. Pushes auf andere Branches lösen nichts aus. Der Hook meldet Erfolg oder Fehler an den Pusher, das Protokoll liegt im Home des Build-Benutzers.

#### Scenario: Push auf master
- **WHEN** auf `master` (den Hauptbranch) des Git-Servers gepusht wird
- **THEN** wird die Seite gebaut und nach `/httpdocs/` übertragen, und der Pusher sieht das Ergebnis

#### Scenario: Push auf einen anderen Branch
- **WHEN** auf `dev` gepusht wird
- **THEN** passiert nichts

#### Scenario: Fehler im Build
- **WHEN** der Build mit einem Fehler im Inhalt abbricht
- **THEN** wird nichts übertragen, und der Pusher erhält die Fehlermeldung
