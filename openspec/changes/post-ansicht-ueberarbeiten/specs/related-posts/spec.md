## ADDED Requirements

### Requirement: Anzeige wie in der Beitragsliste
Jeder Eintrag der Auswahl SHALL im Aufbau der Einträge der Beitragsliste dargestellt werden: Vorschaubild (falls vorhanden), Titel als Link, Datum und Kurztext. Gemeinsame Schlagworte werden nicht eigens angezeigt.

#### Scenario: Eintrag mit Bild
- **WHEN** der ähnliche Beitrag „Kürbis“ ein Vorschaubild hat
- **THEN** zeigt der Eintrag das runde Bild, den Titel mit Link, „31. Oktober 2020“ und den Kurztext wie in der Liste auf der Startseite

#### Scenario: Ohne Bild und Kurztext
- **WHEN** ein ähnlicher Beitrag weder Vorschaubild noch Kurztext hat
- **THEN** zeigt der Eintrag Titel und Datum, ohne leere Flächen

## REMOVED Requirements

### Requirement: Anzeige der gemeinsamen Schlagworte
**Reason**: Die ähnlichen Beiträge sollen genauso aussehen wie die Liste der Startseite, die keine Schlagworte zeigt.
**Migration**: Siehe „Anzeige wie in der Beitragsliste“.
