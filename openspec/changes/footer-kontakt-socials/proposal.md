# Proposal

## Why

Der Seitenkopf trägt heute sieben Navigationseinträge, von denen vier (Tipps und Tricks, About, Impressum, Abonnieren) selten gebraucht werden, aber dieselbe Aufmerksamkeit bekommen wie Start, Themen und Beiträge; die Fußzeile besteht nur aus Copyright und einem Impressum-Link. Kontakt und Social-Media-Profile fehlen auf der Seite ganz, bis auf den E-Mail-Hinweis in Beiträgen. Ein richtiger Footer räumt den Kopf auf, macht Kontaktwege auf jeder Seite auffindbar und ist ein erster Schritt zu einem besseren Design.

## What Changes

- **Footer:** Jede Seite bekommt eine gestaltete Fußzeile mit den Links „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“, dem Kontakt per E-Mail, den Social-Media-Profilen und dem Copyright.
- **Kopfnavigation schrumpft:** Im Kopf bleiben nur „Start“, „Themen“ und „Beiträge“. Die vier anderen Einträge wandern in den Footer; ihre URLs ändern sich nicht.
- **E-Mail:** Der Kontakt `ermeline@flauschiversum.de` steht im Footer als `mailto:`-Link. Der Betreff enthält immer „Flauschiversum“ (Spamfilter); der Link im Footer nutzt den Betreff „Flauschiversum“, die Links in Beiträgen und im Impressum behalten ihren Betreff, der das Wort bereits enthält.
- **Social-Media-Icons:** Instagram (https://www.instagram.com/ermelinewollknoll/) und Mastodon (https://chaos.social/@ErmelineWollknoll) erscheinen als Icons, nicht als Text. Die Icons liegen als SVG im HTML (keine Dateien oder Dienste von Dritten), sind für Screenreader benannt und per Tastatur erreichbar. Der Mastodon-Link trägt `rel="me"`.
- **Gestaltung:** Der Footer passt zu Farben und Schriften der Seite, bricht am Handy sauber um und bleibt ohne JavaScript nutzbar. Weitere Design-Verbesserungen (Kopf, Typografie, Beitragsseiten) sind nicht Teil dieses Changes.

## Capabilities

### New Capabilities

- `contact-links`: Die Kontaktwege der Seite: E-Mail mit Pflichtwort im Betreff und die Social-Media-Profile als Icons, einheitlich auf jeder Seite.

### Modified Capabilities

- `site-layout`: Hauptnavigation im Kopf mit drei Einträgen; neue Fußzeile mit Seitenlinks, Kontakt, Socials und Copyright.

## Impact

- **Code:** `templates/layout.html` (Navigation, Footer), neues Partial für die Icons, `content/style/layout.css` bzw. neue `footer.css` (in `main.css` importiert), Anpassung von Snapshots und Tests; die Konstanten für Adressen in `src/config.rs`.
- **Inhalte und URLs:** Keine Änderung an Beiträgen, Einzelseiten oder URLs.
- **Abhängigkeiten:** Keine; die Icons sind inline SVG.
- **Betrieb:** Der Staging-Lauf veröffentlicht den Footer zur Abnahme.
