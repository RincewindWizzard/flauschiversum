## REMOVED Requirements

### Requirement: RSS-Feed-Icon im Footer
**Reason**: Der Textlink „Abonnieren“ in der Fußzeile entfällt, weil das RSS-Icon ihn abdeckt; ersetzt durch „RSS-Feed-Icon ohne Textlink“.
**Migration**: Siehe ADDED Requirement „RSS-Feed-Icon ohne Textlink“.

## ADDED Requirements

### Requirement: RSS-Feed-Icon ohne Textlink
Jede Seite SHALL in der Fußzeile neben den Icons für E-Mail, Instagram und Mastodon als letztes ein RSS-Icon zeigen, das auf `/rss.xml` verlinkt. Der Link hat den zugänglichen Namen „RSS-Feed abonnieren“, einen Tooltip, `rel="alternate"` und `type="application/rss+xml"`, ist per Tastatur erreichbar, hat einen sichtbaren Fokus und eine Klickfläche von mindestens 44 × 44 px. Das Icon liegt inline als SVG im HTML. Einen zusätzlichen Textlink „Abonnieren“ gibt es nicht.

#### Scenario: Icon sichtbar
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** zeigt die Fußzeile nach dem Mastodon-Icon das RSS-Icon als Link auf `/rss.xml`

#### Scenario: Feed öffnen
- **WHEN** das RSS-Icon angeklickt wird
- **THEN** wird `/rss.xml` geöffnet

#### Scenario: Zugänglicher Name
- **WHEN** ein Screenreader das RSS-Icon vorliest
- **THEN** nennt er „RSS-Feed abonnieren“, und der Link ist per Tab-Taste erreichbar

#### Scenario: Reihenfolge der Icons
- **WHEN** die Fußzeile angezeigt wird
- **THEN** stehen die Icons in der Reihenfolge E-Mail, Instagram, Mastodon, RSS

#### Scenario: Kein Textlink
- **WHEN** die Fußzeile angezeigt wird
- **THEN** gibt es keinen Textlink „Abonnieren“; `/rss.xml` ist nur über das Icon verlinkt

#### Scenario: Keine Anfragen an Dritte
- **WHEN** eine Seite geladen wird
- **THEN** werden für das Icon keine Dateien von fremden Servern geladen
