# Proposal

## Why

Der RSS-Feed des Flauschiversums (`/rss.xml`) ist nur als Textlink „Abonnieren“ im Footer zu finden. Besucher suchen für Feeds meist das bekannte RSS-Symbol, und neben den Social-Icons fehlt der Weg, dem Blog ohne Konto auf einer Plattform zu folgen. Ein RSS-Icon neben den Socials macht den Feed auf jeder Seite auffindbar.

## What Changes

- **RSS-Icon im Footer:** Neben E-Mail, Instagram und Mastodon erscheint als letztes Icon das RSS-Symbol (Punkt und zwei Bögen) als Link auf `/rss.xml`. Es hat den zugänglichen Namen „RSS-Feed abonnieren“, einen Tooltip, den gleichen Stil, dieselbe Klickfläche (mindestens 44 × 44 px) und denselben sichtbaren Fokus wie die anderen Icons.
- **Textlink bleibt:** „Abonnieren“ bleibt in der Linkgruppe der Fußzeile bestehen; das Icon ergänzt ihn für Besucher, die das Symbol suchen.
- **Feed-Erkennung:** Der Link trägt `rel="alternate"` und `type="application/rss+xml"`, damit Feedreader und Browser-Erweiterungen ihn erkennen; der Hinweis im `<head>` bleibt unverändert.
- **Icon:** Eigenes einfaches SVG (inline, einfarbig wie die anderen), keine Datei und kein Dienst von Dritten.

## Capabilities

### New Capabilities

Keine.

### Modified Capabilities

- `contact-links`: Das RSS-Icon als weiterer Kontaktweg neben E-Mail und Social-Profilen; gleiche Anforderungen an Name, Fokus, Klickfläche und fremdfreies Laden.

## Impact

- **Code:** `templates/layout.html` (Footer), neues Partial `templates/partials/icon-rss.html`, ggf. kleine Anpassung in `content/style/footer.css`; Tests (`site.rs`, `tests/browser/layout.mjs`) und Snapshots.
- **Inhalte und URLs:** Keine Änderung; `/rss.xml` bleibt, wie er ist.
- **Staging:** Das Icon zeigt dort wie überall auf `/rss.xml` der jeweiligen Domain (wurzelbezogen).
