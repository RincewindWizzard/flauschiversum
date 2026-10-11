# Spec Delta

## Purpose

Beschreibt die Kontaktwege des Flauschiversums auf jeder Seite: die E-Mail-Adresse mit einem Betreff, der Spamfilter passiert, und die Social-Media-Profile als Icons.

## ADDED Requirements

### Requirement: E-Mail-Kontakt mit Pflichtwort im Betreff
Jede Seite SHALL in der Fußzeile einen `mailto:`-Link an `ermeline@flauschiversum.de` zeigen. Jeder `mailto:`-Link der Seite, auch in Beiträgen und Einzelseiten, SHALL einen Betreff haben, der das Wort „Flauschiversum“ enthält. Der Link in der Fußzeile hat den Betreff „Flauschiversum“.

#### Scenario: Kontakt aus der Fußzeile
- **WHEN** in der Fußzeile der E-Mail-Link angeklickt wird
- **THEN** öffnet sich eine E-Mail an `ermeline@flauschiversum.de` mit dem Betreff „Flauschiversum“

#### Scenario: Betreff bei Beiträgen
- **WHEN** auf der Seite des Posts „Alraune“ „Nachricht“ angeklickt wird
- **THEN** lautet der Betreff „[Flauschiversum] Alraune“ und enthält „Flauschiversum“

#### Scenario: Kein Link ohne Betreff
- **WHEN** der gebaute Auftritt nach `mailto:`-Links durchsucht wird
- **THEN** hat jeder einen Betreff mit „Flauschiversum“

### Requirement: Social-Media-Profile als Icons
Jede Seite SHALL in der Fußzeile die Profile auf Instagram (`https://www.instagram.com/ermelinewollknoll/`) und Mastodon (`https://chaos.social/@ErmelineWollknoll`) als Icons verlinken, nicht als Text. Jedes Icon ist ein Link mit einem zugänglichen Namen („Instagram“, „Mastodon“), per Tastatur erreichbar und mit sichtbarem Fokus. Der Mastodon-Link trägt `rel="me"`; externe Links tragen `noopener`.

#### Scenario: Icons sichtbar
- **WHEN** eine beliebige Seite geöffnet wird
- **THEN** zeigt die Fußzeile ein Instagram- und ein Mastodon-Icon als Links auf die genannten Adressen

#### Scenario: Zugänglicher Name
- **WHEN** ein Screenreader das Instagram-Icon vorliest
- **THEN** nennt er „Instagram“, und der Link ist per Tab-Taste fokussierbar

#### Scenario: Ohne JavaScript
- **WHEN** die Seite ohne JavaScript geöffnet wird
- **THEN** sind die Icons sichtbar und die Links funktionieren

### Requirement: Icons ohne Dienste von Dritten
Die Icons SHALL Teil des ausgelieferten HTML sein (inline SVG). Beim Laden einer Seite werden keine Schriften, Skripte, Bilder oder Stylesheets von fremden Servern geladen; erst der Klick auf das Icon verlässt die Seite.

#### Scenario: Keine fremden Anfragen
- **WHEN** eine Seite geladen wird
- **THEN** gehen alle Anfragen an `flauschiversum.de` (bzw. die Staging-Domain)

#### Scenario: Farbe der Icons
- **WHEN** die Seite im hellen Standarddesign angezeigt wird
- **THEN** haben Icons und Beschriftungen ausreichenden Kontrast zum Hintergrund und passen zu den Farben der Seite
