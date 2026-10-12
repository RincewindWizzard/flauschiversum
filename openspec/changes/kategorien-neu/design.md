## Context

Ursprünglich sollte dieser Change zehn Kategorien einführen. Nach der Prüfung (Überschneidungen, Pflicht zur Ausschließlichkeit) wurde entschieden, Kategorien ganz zu streichen und alles über Schlagworte abzubilden. Die Zuordnung der Beiträge (`zuordnung.md`) bleibt als Grundlage der neuen Schlagworte erhalten.

## Goals / Non-Goals

**Goals:** jeder Beitrag mit passenden Schlagworten; deterministische „Ähnliche Beiträge“ allein über Schlagworte.
**Non-Goals:** neue Beitragsinhalte, Änderungen an Beitrags-URLs, Kategorien jeder Art.

## Decisions

### 1. Zuordnung per Skript
Ein einmaliges Skript hat `category` aus dem Frontmatter aller Beiträge gestrichen und den zugehörigen Wert (laut `zuordnung.md`) als Schlagwort ergänzt, ohne den übrigen Text zu verändern. `content/tags.yaml` führt alle Schlagworte alphabetisch.

### 2. Ähnliche Beiträge
Für einen Beitrag P und jeden anderen veröffentlichten Beitrag Q: `score = Σ_{t ∈ Tags(P)∩Tags(Q)} 1/count(t)`. Q ohne gemeinsames Schlagwort werden ausgelassen. Sortierung: Score absteigend, Datum absteigend, URL aufsteigend; höchstens 6 Treffer. Anzeige als Karte „Ähnliche Beiträge“ nach dem Autor und vor der Navigation zum älteren/neueren Beitrag: Titel als Link, darunter die gemeinsamen Schlagworte. Berechnung zur Build-Zeit in `topics.rs`; deterministisch.

## Risks / Trade-offs

- [Zu wenige Schlagworte an einzelnen Beiträgen] → keine Karte statt Zufallstreffern; Schlagworte lassen sich im Frontmatter ergänzen.
- [Sehr häufige Schlagworte dominieren] → Gewicht 1/Anzahl lässt seltene Schlagworte stärker zählen.
