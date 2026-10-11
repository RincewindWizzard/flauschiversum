# Design

## Context

Der Footer (`templates/layout.html`, Stil `content/style/footer.css`) zeigt in `ul.socials` bereits drei Icon-Links (E-Mail, Instagram, Mastodon) mit inline SVG aus `templates/partials/icon-*.html`, `aria-label`, `title`, Klickfläche 44 × 44 px und sichtbarem Fokus. „Abonnieren“ steht als Text in `.footer-links`. Der Feed liegt unter `/rss.xml` (Spec `rss-feed`), der `<head>` trägt `link rel="alternate"`. Siehe proposal.md für Motivation.

## Goals / Non-Goals

**Goals:**
- Ein RSS-Icon neben den Socials, im selben Stil und mit denselben Zugänglichkeits-Eigenschaften.

**Non-Goals:**
- Änderungen am Feed, an der Feed-URL oder am Textlink „Abonnieren“.
- Weitere Feed-Formate (Atom, JSON Feed) oder Abo-Dienste von Dritten.

## Decisions

### 1. Eigenes SVG statt Marken-Icon

Das RSS-Symbol (Punkt unten links, zwei Viertelbögen) ist ein schlichtes, allgemein bekanntes Zeichen. Es wird als einfarbiges SVG (`fill="currentColor"` bzw. Strich mit `currentColor`, 24 × 24, `aria-hidden`, `focusable="false"`) in `templates/partials/icon-rss.html` selbst gezeichnet; so gibt es keine Lizenzfragen und es passt zu den anderen Icons.

*Alternative:* Icon aus dem Satz „Simple Icons“ (wie Instagram und Mastodon). Verworfen: Dort ist es ein Markenzeichen-Eintrag mit anderem Strich und der Pfad lässt sich nicht aus dem Gedächtnis sicher angeben; ein eigenes dreiteiliges Symbol ist ohnehin einfach.

### 2. Einreihung in `ul.socials`

Das Icon wird als vierter `li` hinter Mastodon in dieselbe Liste gesetzt. Dadurch gelten Klickfläche, Fokusrahmen und Hover der vorhandenen Regeln ohne neue CSS-Regel; der Link bekommt `href="/rss.xml"`, `aria-label="RSS-Feed abonnieren"`, `title="RSS-Feed abonnieren"`, `rel="alternate"` und `type="application/rss+xml"`. Der Feed-Link ist wurzelbezogen wie die anderen internen Links; die Absolut-URLs im Feed folgen `--base-url`.

### 3. Textlink bleibt

„Abonnieren“ in `.footer-links` bleibt: Wer kein Icon erkennt, findet den Text; die Hauptspec `site-layout` fordert ihn. Die doppelte Auffindbarkeit kostet nichts.

## Risks / Trade-offs

- [Vier Icons in einer Zeile werden am Handy eng] → Die Icons haben je 44 px plus Lücke (zusammen unter 200 px) und die Gruppe bricht bei Bedarf um; Browser-Test bei 360 px prüft, dass nichts überläuft.
- [Das Icon wird als „Wi-Fi“ missverstanden] → Name und Tooltip „RSS-Feed abonnieren“ und die übliche Lage (Punkt unten links, Bögen nach rechts oben) machen es erkennbar.

## Migration Plan

1. Partial, Footer und Tests ändern; Push auf den Claude-Branch, der Staging-Lauf veröffentlicht die Änderung zur Abnahme.
2. Rollback: Commit zurücknehmen; es gibt keine Datenänderung.
