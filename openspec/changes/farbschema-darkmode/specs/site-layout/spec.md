## MODIFIED Requirements

### Requirement: Farbschema
Die Seite SHALL ein Farbschema der gewählten Themenfamilie (siehe `design.md` dieses Changes) in einem hellen und einem dunklen Modus verwenden: Hintergrund, weiße bzw. dunkle Karten, Text, graue Schrift, Links, Akzent für Knöpfe und aktive Elemente, Rand und Tönung für Hover. Alle Farben sind als Variablen an einer Stelle definiert, Text und Links erreichen mindestens 4,5 : 1 Kontrast, das frühere Neongrün entfällt.

#### Scenario: Hintergrund und Karten
- **WHEN** eine beliebige Seite in einem der beiden Modi geöffnet wird
- **THEN** sind Karten deutlich von ihrem Seitenhintergrund unterscheidbar, und es gibt keine grellen Neonflächen

#### Scenario: Kontrast
- **WHEN** Fließtext, Links, Knöpfe und Tags in beiden Modi auf ihren Hintergründen geprüft werden
- **THEN** beträgt das Kontrastverhältnis mindestens 4,5 : 1

#### Scenario: Staging-Leiste bleibt auffällig
- **WHEN** eine Staging-Seite in einem der beiden Modi geöffnet wird
- **THEN** hebt sich die Staging-Leiste mit ihrer roten Farbe weiterhin deutlich vom Schema ab
