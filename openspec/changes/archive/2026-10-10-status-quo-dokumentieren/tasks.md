# Tasks

## 1. Ist-Zustand erheben

- [x] 1.1 Generator-Code (`blogcompile/`, `templates/`, `src/style/`, `main.py`, `settings.py`) vollständig lesen; Prüfung: jede Capability im Proposal ist einer Code-Stelle zugeordnet
- [x] 1.2 Lauffähigkeit des alten Generators mit aktuellen Bibliotheken prüfen; Ergebnis: Abbruch beim Import (`markdown.util.etree`), festgehalten im Proposal
- [x] 1.3 Inhalte auswerten (194 Posts, 3 Seiten, 3195 Bilder, Dateitypen, Frontmatter-Felder, Bildgrößen, EXIF/GPS); Prüfung: Zahlen stehen in Proposal bzw. Specs

## 2. Abgleich mit der Live-Seite

- [x] 2.1 Alle 49 Indexseiten von https://flauschiversum.de/ abrufen und die Post-URLs mit den aus Datum und Titel berechneten vergleichen; Ergebnis: 194/194 live, 1 Abweichung (`gibs-mir-audrey-ii`) in `url-scheme` festgehalten
- [x] 2.2 Alle 2857 Bild- und Script-Referenzen der Post-Seiten abrufen; Ergebnis: 2856 erreichbar, 1 kaputtes `.jpeg` in der Fehlerliste
- [x] 2.3 Paginierung, Seitentitel, RSS-Feed, 301-Weiterleitung und 304-Antwort live prüfen; Prüfung: Szenarien in `post-index`, `site-layout`, `rss-feed`, `url-scheme` und `deployment` stimmen mit den Antworten überein

## 3. Specs schreiben und abstimmen

- [x] 3.1 Proposal mit Fehlerliste und Entscheidungen für den Folge-Change schreiben; Prüfung: Review und Freigabe im Gespräch
- [x] 3.2 Elf Specs schreiben; Prüfung: `openspec validate status-quo-dokumentieren --strict` meldet „valid“
- [x] 3.3 Rückmeldungen einarbeiten (Liegenbleiben alter Dateien als Fehler, Deployment-Spec auf Auslieferung beschränkt); Prüfung: erneute strikte Validierung erfolgreich

## Workflow follow-up

- Change archivieren, damit die Specs unter `openspec/specs/` als Grundlage gelten.
- Danach den Rust-Neubau und das neue Deployment als eigene Changes planen.
