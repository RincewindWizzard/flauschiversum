# Design

## Context

`templates/layout.html` rendert Kopf (Logo, `<nav class="center">` mit sieben Links, Suchfeld) und eine einzeilige Fußzeile (`<footer class="center">` mit Copyright und Impressum-Link; CSS in `layout.css`). Alle Seiten erben das Layout, auch die Suchseite und die Staging-Leiste. Die Navigationslinks sind `float: left` mit der Schrift „Lobster“ (`header.css`). Die E-Mail-Adresse steht in `post.html` (Betreff „[Flauschiversum] <Titel>“, per `mail_subject` kodiert) und im Impressum (Betreff „[Flauschiversum]“); beide enthalten das Wort bereits. Die Seite lädt nichts von Dritten (Spec `site-layout`), also kommen keine Icon-Schriften oder CDN-Skripte in Frage. Siehe proposal.md für Motivation und Umfang.

## Goals / Non-Goals

**Goals:**
- Ein gestalteter Footer auf jeder Seite mit Seitenlinks, Kontakt und Social-Icons.
- Eine kleinere, ruhigere Kopfnavigation (Start, Themen, Beiträge).
- Kontaktlinks, die durch Spamfilter kommen (Betreff mit „Flauschiversum“).

**Non-Goals:**
- Allgemeine Neugestaltung von Kopf, Typografie, Farben oder Beitragsseiten.
- Weitere soziale Netzwerke, Newsletter, Kontaktformular.
- Cookie-Banner oder Tracking (gibt es nicht).

## Decisions

### 1. Footer-Aufbau

Die Fußzeile wird zu einem `<footer id="blogfooter">` mit drei Gruppen, die auf breiten Fenstern nebeneinander und am Handy untereinander stehen (Flex mit `flex-wrap`, kein Raster, das überläuft):

1. **Seiten:** `<nav aria-label="Weitere Seiten">` mit „Tipps und Tricks“, „About“, „Impressum“, „Abonnieren“.
2. **Kontakt:** E-Mail-Link (sichtbarer Text mit der Adresse, Briefumschlag als Icon davor) und die zwei Social-Icons als Liste.
3. **Copyright:** „© <Jahr> Ermeline Wollknoll“ in kleiner Schrift.

Die Navigation im Kopf behält ihr Markup, verliert vier Links.

*Alternative:* Footer mit den Spalten Navigation/Kontakt/Social in einem CSS-Raster. Verworfen: Bei nur drei kurzen Gruppen reicht Flex-Umbruch, und es gibt keine Überlauf-Fälle auf schmalen Geräten.

### 2. Icons als inline SVG aus einem Partial

Instagram und Mastodon (und der Briefumschlag) liegen als SVG-Pfade in `templates/partials/icons.html` und werden mit `{% include %}` bzw. Askama-Makros eingebunden. Die Pfade stammen aus dem Icon-Satz „Simple Icons“ (CC0); die Marken bleiben Eigentum ihrer Inhaber, die Logos werden nur als Verweis auf die Profile genutzt. Jedes SVG hat `aria-hidden="true"`, `focusable="false"` und `fill="currentColor"`, damit Farbe und Fokus vom Link kommen; der Link trägt `aria-label` („Instagram“, „Mastodon“) und `title`. Größe 1.6 em, Klickfläche mindestens 44 × 44 px (Padding), sichtbarer Fokusrahmen.

*Alternativen:* (a) SVG-Dateien unter `content/static/` und `<img>`: Farbe und Hover lassen sich nicht über CSS steuern. (b) Icon-Schrift (Font Awesome): ein weiterer Font und Lizenzfragen, bei drei Icons unverhältnismäßig. (c) Emojis: je nach System unterschiedlich und keine Markenzeichen. Gewählt ist inline SVG, weil es ohne zusätzliche Anfragen funktioniert und mit `currentColor` zur Seite passt.

### 3. Adressen als Konstanten

Die Profil-URLs und die Kontaktadresse stehen in `src/config.rs` (`CONTACT_EMAIL`, `INSTAGRAM_URL`, `MASTODON_URL`) und gehen über `Common` an die Templates. So gibt es genau eine Stelle für Änderungen, und ein Test kann prüfen, dass jeder `mailto:`-Link der gebauten Seite im Betreff „Flauschiversum“ enthält. Der Betreff im Footer ist „Flauschiversum“ (URL-kodiert über die vorhandene Funktion für `mail_subject`).

*Alternative:* Adressen in `content/` als Daten (YAML). Verworfen: Es sind drei feste Werte; eine Datei mehr und ein Ladeschritt bringen nichts.

### 4. Gestaltung

Neue Datei `content/style/footer.css` (in `main.css` importiert): oberer gestrichelter Rand wie die Beitragslisten (`1px dashed var(--bordercolor)`), „Lobster“ nur für die Linkgruppe wie in der Kopfnavigation, Link-Hover mit `var(--pagination-hover)` wie bei der Paginierung, Icons in `var(--acolor)` und beim Hover in einem dunkleren Ton. Abstände in `em`, damit es mit der Schriftgröße skaliert. Die Staging-Leiste bleibt oberhalb der Seite und ist vom Footer unabhängig.

Kopfnavigation: Mit drei Einträgen bleibt `float: left` mit zentriertem Block möglich; die vorhandene Mobil-Regel (kleinere Ränder unter 40 em) bleibt.

### 5. E-Mail-Betreff

Eine kleine Funktion `mailto(subject)` in `render.rs` erzeugt alle `mailto:`-Adressen aus Konstante und kodiertem Betreff; sie lässt keinen Aufruf ohne „Flauschiversum“ im Betreff zu (Debug-Assertion und Test). Die Links in Beiträgen („[Flauschiversum] <Titel>“) und im Impressum (Markdown, „[Flauschiversum]“) bleiben unverändert und erfüllen die Regel bereits; ein Test über die gesamte gebaute Seite sichert das.

## Risks / Trade-offs

- [Wichtige Seiten (About, Impressum) sind nicht mehr im Kopf sichtbar] → Gewollt; der Footer steht auf jeder Seite, das Impressum bleibt dort Pflichtlink.
- [Markenlogos und deren Nutzungsregeln] → Nur als Verweis auf die eigenen Profile, in einfarbiger Standardform; kein Hinweis auf Zugehörigkeit. Pfade aus einem CC0-Satz.
- [Eigene Pfade der Icons weichen optisch von den Originalen ab] → Pfade unverändert aus dem Satz übernehmen und im Test (Snapshot) festhalten.
- [Fester Betreff „Flauschiversum“ ist schon in Beiträgen enthalten, aber ein künftiger Link ohne Wort fällt nicht auf] → Test über alle gebauten Seiten schlägt dann fehl.
- [Footer bricht am Handy unschön um] → Browser-Test bei 360 px mit Blick auf waagerechtes Scrollen und Mindestgröße der Klickflächen; Vergleichsbilder zur Abnahme.

## Migration Plan

1. Templates, Icons, Konstanten und CSS umsetzen; Snapshots und Browser-Tests anpassen.
2. Push auf einen Claude-Branch: Der Staging-Lauf veröffentlicht den Footer unter https://staging.flauschiversum.de/ zur Abnahme (Desktop und Handy).
3. Nach der Abnahme archivieren. Rollback: Commit zurücknehmen; es gibt keine Datenänderung.
