# Tasks

## 1. RSS-Icon im Footer

- [x] 1.1 Partial `templates/partials/icon-rss.html` mit eigenem SVG (Punkt und zwei Bögen, `currentColor`, `aria-hidden`, `focusable="false"`) und in `layout.html` als vierten Eintrag von `ul.socials` einsetzen (`href="/rss.xml"`, `aria-label` und `title` „RSS-Feed abonnieren“, `rel="alternate"`, `type="application/rss+xml"`); prüfen: Fixture-Test über alle Seiten: der Footer enthält den Link genau einmal nach dem Mastodon-Icon, mit Namen und Attributen, vier inline SVG in der Icon-Liste, und weiterhin den Textlink „Abonnieren“
- [x] 1.2 Darstellung am Desktop und Handy prüfen (Abstände, Umbruch, Fokus); prüfen: Playwright-Test in `tests/browser/layout.mjs` bei 1280 und 360 px: Icon sichtbar, Klickfläche mindestens 44 × 44 px, per Tab erreichbar mit sichtbarem Fokusrahmen, Klick öffnet `/rss.xml` (Antwort mit Typ `application/rss+xml` bzw. XML), kein waagerechtes Scrollen, keine Anfragen an fremde Server
- [ ] 1.3 Snapshots und bestehende Tests anpassen; prüfen: `cargo test` und `tests/browser/layout.mjs` grün, Vergleichsbild des Footers (Desktop und Handy) zur Abnahme

## Workflow follow-up

- Nach der Abnahme `/opsx:archive footer-rss-icon` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
