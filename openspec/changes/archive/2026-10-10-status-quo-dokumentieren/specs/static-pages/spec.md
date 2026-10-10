# Spec Delta

## Purpose

Beschreibt die Einzelseiten außerhalb des Blog-Stroms (About, Impressum, Tipps und Tricks): wie sie dargestellt werden und worin sie sich von Posts unterscheiden.

## ADDED Requirements

### Requirement: Darstellung einer Einzelseite
Eine Einzelseite SHALL im gemeinsamen Rahmen (siehe `site-layout`) ihren Titel als Überschrift zeigen, darunter den aus Markdown erzeugten Inhalt. Datum, Autor, Vorschaubild und Kontaktlink wie bei Posts gibt es nicht.

#### Scenario: Impressum
- **WHEN** `/impressum/` geöffnet wird
- **THEN** zeigt die Seite die Überschrift „Impressum“ und den Inhalt aus `src/pages/impressum.md`

### Requirement: Bilder auf Einzelseiten
Bilder auf Einzelseiten SHALL unverändert mit ihrer angegebenen URL eingebunden werden: ohne Größenvarianten, ohne Wasserzeichen und ohne Slideshow.

#### Scenario: Bild auf der About-Seite
- **WHEN** die About-Seite `![Ermeline](/images/Ermeline.jpg)` enthält
- **THEN** zeigt sie das Bild direkt von `/images/Ermeline.jpg`

### Requirement: Einzelseiten nicht im Blog-Strom
Einzelseiten SHALL weder auf den Indexseiten noch im RSS-Feed erscheinen. Erreichbar sind sie über die Hauptnavigation.

#### Scenario: Feed ohne Einzelseiten
- **WHEN** der RSS-Feed erzeugt wird
- **THEN** enthält er keinen Eintrag für „Impressum“, „About“ oder „Tipps und Tricks“
