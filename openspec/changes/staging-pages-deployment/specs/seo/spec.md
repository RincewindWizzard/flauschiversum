# Spec Delta

## ADDED Requirements

### Requirement: Staging-Modus ohne Indexierung
Im Staging-Modus SHALL jede HTML-Seite `<meta name="robots" content="noindex">` tragen, `robots.txt` SHALL `Disallow: /` ohne Verweis auf eine Sitemap enthalten, und es SHALL keine Sitemap ausgeliefert werden. Kanonische URLs, Open Graph und Feed richten sich nach der Basis-URL des Stagings, damit sie nicht auf die Produktivseite zeigen.

#### Scenario: Seite im Staging
- **WHEN** im Staging-Modus gebaut wird
- **THEN** enthält jede HTML-Seite `noindex`

#### Scenario: robots.txt im Staging
- **WHEN** im Staging-Modus gebaut wird
- **THEN** lautet `robots.txt` „User-agent: *“ und „Disallow: /“, und `/sitemap.xml` existiert nicht

#### Scenario: Kanonische URL im Staging
- **WHEN** mit der Staging-Basis `https://rincewindwizzard.github.io/flauschiversum` gebaut wird
- **THEN** zeigt `canonical` einer Seite auf `https://rincewindwizzard.github.io/flauschiversum/…` und nicht auf `flauschiversum.de`

#### Scenario: Produktiv unverändert
- **WHEN** ohne Staging-Modus gebaut wird
- **THEN** gibt es kein `noindex` außer auf der Suchseite, und `robots.txt` und Sitemap bleiben wie bisher
