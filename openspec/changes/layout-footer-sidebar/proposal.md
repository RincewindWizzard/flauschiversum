# Proposal

## Why

Das Layout nutzt den Platz auf dem Desktop schlecht: Alles steht in einer schmalen mittleren Spalte, links und rechts bleibt Leerraum, und die Schlagworte erscheinen als kleine Liste mitten auf der Startseite. Der Seitenkopf trägt sieben Navigationseinträge, von denen vier (Tipps und Tricks, About, Impressum, Abonnieren) selten gebraucht werden; die Fußzeile besteht nur aus Copyright und einem Impressum-Link. Kontakt und Social-Media-Profile fehlen auf der Seite ganz, bis auf den E-Mail-Hinweis in Beiträgen. Und der Autor steht am Anfang jedes Beitrags, wo er vom eigentlichen Inhalt ablenkt. Ein besseres Layout mit Sidebar und Footer macht das Handbuch auf großen Bildschirmen übersichtlicher und die Kontaktwege auffindbar.

## What Changes

- **Sidebar mit Tag-Cloud:** Auf dem Desktop steht rechts neben dem Inhalt eine Sidebar mit der Tag-Cloud (alle Schlagworte alphabetisch, Schriftgröße nach Zahl der Beiträge) und darüber den Kategorien mit Anzahl. Am Handy steht sie unter dem Inhalt. Sie erscheint auf allen Seiten außer der Suche und der Themenübersicht (dort steht dasselbe im Inhalt). Der Abschnitt „Häufige Themen“ auf der Startseite entfällt, die Cloud ersetzt ihn.
- **Platz auf dem Desktop:** Die Seite wird breiter und zweispaltig (Inhalt und Sidebar); der Lesetext in Beiträgen behält eine angenehme Zeilenlänge, Kacheln und Listen nutzen die gewonnene Breite.
- **Autor am Ende:** Auf Beitragsseiten steht „Von <Autor>“ nach dem Beitragstext statt im Kopf. Titel, Vorschaubild, Kategorie und Schlagworte bleiben oben.
- **Footer:** Jede Seite bekommt eine gestaltete Fußzeile mit den Links „Tipps und Tricks“, „About“, „Impressum“ und „Abonnieren“, dem Kontakt per E-Mail, den Social-Media-Profilen und dem Copyright.
- **Kopfnavigation schrumpft:** Im Kopf bleiben nur „Start“, „Themen“ und „Beiträge“. Die vier anderen Einträge wandern in den Footer; ihre URLs ändern sich nicht.
- **E-Mail:** Der Kontakt `ermeline@flauschiversum.de` steht im Footer als `mailto:`-Link. Der Betreff enthält immer „Flauschiversum“ (Spamfilter); der Link im Footer nutzt den Betreff „Flauschiversum“, die Links in Beiträgen und im Impressum behalten ihren Betreff, der das Wort bereits enthält.
- **Social-Media-Icons:** Instagram (https://www.instagram.com/ermelinewollknoll/) und Mastodon (https://chaos.social/@ErmelineWollknoll) erscheinen als Icons, nicht als Text. Die Icons liegen als SVG im HTML (keine Dateien oder Dienste von Dritten), sind für Screenreader benannt und per Tastatur erreichbar. Der Mastodon-Link trägt `rel="me"`.
- **Gestaltung:** Footer und Sidebar passen zu Farben und Schriften der Seite, brechen am Handy sauber um und bleiben ohne JavaScript nutzbar. Weitere Design-Verbesserungen (Typografie, Farben, Beitragsseiten im Einzelnen) sind nicht Teil dieses Changes.

## Capabilities

### New Capabilities

- `contact-links`: Die Kontaktwege der Seite: E-Mail mit Pflichtwort im Betreff und die Social-Media-Profile als Icons, einheitlich auf jeder Seite.
- `sidebar`: Die Sidebar mit Kategorienliste und Tag-Cloud (Inhalt, Skalierung, Reihenfolge, Zählung).

### Modified Capabilities

- `site-layout`: Hauptnavigation im Kopf mit drei Einträgen; neue Fußzeile mit Seitenlinks, Kontakt, Socials und Copyright; breiteres zweispaltiges Layout mit Sidebar.
- `home-page`: Aufbau der Startseite ohne „Häufige Themen“ (die Sidebar übernimmt).
- `post-page`: Autor am Ende des Beitrags statt im Kopf; Einordnung steht unter dem Vorschaubild.

## Impact

- **Code:** `templates/layout.html` (Navigation, Spalten, Sidebar, Footer), `home.html` und `post.html`, neue Partials für Icons und Sidebar, `src/topics.rs`/`src/render.rs`/`src/site.rs` (Sidebar-Daten für jede Seite), `content/style/` (Breite, Sidebar, Footer; neue `sidebar.css` und `footer.css`), `src/config.rs` (Adressen; `HOME_TOPICS` entfällt), Anpassung von Snapshots und Tests.
- **Inhalte und URLs:** Keine Änderung an Beiträgen, Einzelseiten oder URLs. Der Autorenname bleibt in `author:` und wird nur an anderer Stelle angezeigt.
- **Abhängigkeiten:** Keine; die Icons sind inline SVG.
- **Betrieb:** Der Staging-Lauf veröffentlicht Layout, Sidebar und Footer zur Abnahme.
