## MODIFIED Requirements

### Requirement: Suchfeld in der Topbar
Die Topbar SHALL ein Suchfeld mit eingebauter Lupe enthalten (Spezifikation „Suchfeld mit eingebauter Lupe“). Der Platzhalter lautet „Beitrag suchen …“, und das Feld trägt für Screenreader die Beschriftung „Beitrag suchen“. Auf der Startseite und der Suchseite entfällt es nicht; es gibt je Seite höchstens ein sichtbares Suchfeld in der Topbar.

#### Scenario: Suche aus der Topbar
- **WHEN** in der Topbar „Mütze“ eingegeben und Enter gedrückt wird
- **THEN** öffnet sich `/suche/?q=M%C3%BCtze`

#### Scenario: Platzhalter
- **WHEN** eine beliebige Seite geöffnet wird und das Suchfeld der Topbar leer ist
- **THEN** zeigt es den Platzhalter „Beitrag suchen …“, und sein zugänglicher Name lautet „Beitrag suchen“
