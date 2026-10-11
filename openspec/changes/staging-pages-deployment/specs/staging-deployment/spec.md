# Spec Delta

## Purpose

Beschreibt, wie das Flauschiversum nach jeder Änderung am Branch `dev` automatisch gebaut, geprüft und als Staging-Seite auf GitHub Pages veröffentlicht wird, damit Änderungen vor dem Produktivgang öffentlich ansehbar sind.

## ADDED Requirements

### Requirement: Veröffentlichung bei Änderungen an dev und an Claude-Branches
Jeder Push auf den Branch `dev`, auch durch einen Merge, und jeder Push auf einen Branch, dessen Name mit `claude/` beginnt, SHALL die Seite neu bauen und auf GitHub Pages veröffentlichen. Pushes auf andere Branches, auch `main` und `master`, veröffentlichen nichts. Der Lauf kann zusätzlich von Hand gestartet werden. Es gibt eine einzige Staging-Seite; sie zeigt den Stand des zuletzt veröffentlichten Laufs.

#### Scenario: Merge nach dev
- **WHEN** ein Pull Request nach `dev` gemergt wird
- **THEN** startet ein Lauf, und die Staging-Seite zeigt danach den Stand von `dev`

#### Scenario: Push auf einen Claude-Branch
- **WHEN** auf `claude/beautiful-darwin-66rwab` gepusht wird
- **THEN** startet ein Lauf, und die Staging-Seite zeigt danach den Stand dieses Branches

#### Scenario: Anderer Branch
- **WHEN** auf einen Branch außer `dev` und `claude/**` gepusht wird
- **THEN** wird die Staging-Seite nicht verändert

#### Scenario: Zwei Pushes kurz nacheinander
- **WHEN** während eines laufenden Deployments ein weiterer Push auf `dev` oder einen Claude-Branch folgt
- **THEN** wird der ältere Lauf abgebrochen oder übersprungen, und am Ende ist der neueste Stand veröffentlicht

#### Scenario: Herkunft sichtbar
- **WHEN** die Staging-Seite veröffentlicht wurde
- **THEN** nennt der Lauf Branch und Commit im Protokoll, und die Datei `/staging.txt` der Seite enthält beides, damit erkennbar ist, welcher Stand gerade online ist

### Requirement: Prüfung vor der Veröffentlichung
Vor der Veröffentlichung SHALL die Pipeline die Inhalte prüfen und die Tests ausführen. Schlägt eine Prüfung fehl, wird nichts veröffentlicht, und die zuletzt veröffentlichte Staging-Seite bleibt unverändert.

#### Scenario: Fehler im Frontmatter
- **WHEN** ein Post auf dem gepushten Branch ein unbekanntes Schlagwort trägt
- **THEN** bricht der Lauf mit der Meldung des Builds ab, und die Staging-Seite zeigt weiter den vorigen Stand

#### Scenario: Fehlschlagender Test
- **WHEN** ein Test fehlschlägt
- **THEN** wird nicht veröffentlicht

### Requirement: Staging unter dem Pages-Unterpfad
Die Staging-Seite SHALL unter `https://rincewindwizzard.github.io/flauschiversum/` erreichbar sein, und alle internen Verweise SHALL dort funktionieren, einschließlich Bilder, Stylesheet, Schriften, Karussell, Vollbild und Suche.

#### Scenario: Navigation im Unterpfad
- **WHEN** auf der Staging-Startseite „Themen“ angeklickt wird
- **THEN** öffnet sich `https://rincewindwizzard.github.io/flauschiversum/themen/`

#### Scenario: Suche im Unterpfad
- **WHEN** auf der Staging-Seite nach „Katze“ gesucht wird
- **THEN** erscheinen Treffer, und ihre Links führen auf Seiten unter `/flauschiversum/`

### Requirement: Smoke-Test nach dem Deployment
Nach der Veröffentlichung SHALL die Pipeline die veröffentlichte Seite abrufen und prüfen, dass die Startseite, die Suchseite, der Suchindex, das Stylesheet und ein Beitrag mit Status 200 antworten und die Startseite `noindex` enthält. Schlägt das fehl, ist der Lauf rot.

#### Scenario: Seite erreichbar
- **WHEN** das Deployment abgeschlossen ist
- **THEN** antworten die geprüften URLs mit 200

#### Scenario: Seite fehlt
- **WHEN** eine der geprüften URLs 404 liefert
- **THEN** schlägt der Lauf fehl und nennt die URL

### Requirement: Bild-Cache zwischen Läufen
Die Pipeline SHALL den Bild-Cache des Builds zwischen Läufen wiederverwenden, sodass unveränderte Bilder nicht neu berechnet werden. Ein fehlender oder verworfener Cache verlangsamt den Lauf, macht ihn aber nicht fehlerhaft.

#### Scenario: Zweiter Lauf ohne neue Bilder
- **WHEN** ein Lauf nur Text ändert
- **THEN** meldet der Build 0 neu berechnete Bilder

#### Scenario: Cache verworfen
- **WHEN** der Cache nicht mehr vorhanden ist
- **THEN** werden alle Bilder neu berechnet, und die Seite wird trotzdem korrekt veröffentlicht

### Requirement: Nur Berechtigungen für Pages
Der Workflow SHALL nur die Berechtigungen anfordern, die er braucht: Lesen des Repositorys und Schreiben der Pages-Veröffentlichung. Er braucht keine Zugangsdaten außer dem automatisch bereitgestellten Token.

#### Scenario: Keine eigenen Geheimnisse
- **WHEN** der Workflow ausgeführt wird
- **THEN** nutzt er keine selbst hinterlegten Secrets
