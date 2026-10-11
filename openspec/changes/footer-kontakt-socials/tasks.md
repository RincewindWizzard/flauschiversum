# Tasks

## 1. Footer und Navigation

- [ ] 1.1 Konstanten `CONTACT_EMAIL`, `INSTAGRAM_URL`, `MASTODON_URL` in `src/config.rs` und eine Funktion `mailto(subject)` in `render.rs`, die den Betreff kodiert und ohne „Flauschiversum“ im Betreff nicht zulässt; `post.html` nutzt sie für den bestehenden Betreff; prüfen: Unit-Tests für Kodierung (Umlaute, Leerzeichen, Klammern) und für die Ablehnung eines Betreffs ohne das Wort
- [ ] 1.2 Partial `templates/partials/icons.html` mit den inline SVG für Instagram, Mastodon und Briefumschlag (`aria-hidden`, `focusable="false"`, `fill="currentColor"`); prüfen: Snapshot-Test der Footer-Ausgabe enthält die drei SVG
- [ ] 1.3 `layout.html`: Kopfnavigation auf Start, Themen, Beiträge kürzen; Fußzeile mit Seitenlinks (Tipps und Tricks, About, Impressum, Abonnieren), E-Mail-Link (Betreff „Flauschiversum“), Instagram- und Mastodon-Icon-Links (zugängliche Namen, `rel="me"` beim Mastodon-Link, `rel="noopener"`) und Copyright; prüfen: Test über alle Seiten der Fixture-Site: Navigation im Kopf hat genau drei Links, Footer enthält die vier Seitenlinks, beide Profil-URLs und den `mailto:`-Link, Copyright mit Jahr
- [ ] 1.4 `content/style/footer.css` (in `main.css` importiert): Flex-Aufbau, gestrichelter Rand, Hover und Fokus, Klickflächen mindestens 44 × 44 px, Umbruch am Handy; Kopfnavigation an drei Einträge anpassen; prüfen: Playwright-Test bei 1280 und 360 px: Footer ist das letzte sichtbare Element, alle Links sichtbar, kein waagerechtes Scrollen, Tab-Fokus ist auf den Icons sichtbar

## 2. Absicherung

- [ ] 2.1 Test über die gesamte gebaute Seite: jeder `mailto:`-Link (Footer, Beiträge, Einzelseiten) hat einen Betreff mit „Flauschiversum“, und beim Laden einer Seite gehen alle Anfragen an die eigene Domain; prüfen: `cargo test --release -- --ignored` grün und Playwright-Test zu den Anfragen
- [ ] 2.2 Snapshots und bestehende Tests an die neue Navigation anpassen (Seitenstruktur, Staging-Leiste, Suchseite); prüfen: `cargo test` und alle Playwright-Skripte in `tests/browser/` grün
- [ ] 2.3 Vergleichsbilder (Desktop und Handy, mit und ohne Staging-Leiste) zur Abnahme vorlegen; prüfen: Freigabe im Gespräch am veröffentlichten Staging

## 3. Dokumentation

- [ ] 3.1 `README.md` (Kontaktdaten und Social-Links ändern: nur `src/config.rs`) aktualisieren; prüfen: der beschriebene Weg funktioniert (Änderung der URL erscheint im Footer)

## Workflow follow-up

- Nach der Abnahme `/opsx:archive footer-kontakt-socials` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
