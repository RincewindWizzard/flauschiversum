# Design

## Context

`templates/layout.html` rendert Kopf (Logo, `<nav class="center">` mit sieben Links, Suchfeld) und eine einzeilige Fußzeile (`<footer class="center">` mit Copyright und Impressum-Link; CSS in `layout.css`). Alle Seiten erben das Layout, auch die Suchseite und die Staging-Leiste. Die Navigationslinks sind `float: left` mit der Schrift „Lobster“ (`header.css`). Die E-Mail-Adresse steht in `post.html` (Betreff „[Flauschiversum] <Titel>“, per `mail_subject` kodiert) und im Impressum (Betreff „[Flauschiversum]“); beide enthalten das Wort bereits. Die Seite lädt nichts von Dritten (Spec `site-layout`), also kommen keine Icon-Schriften oder CDN-Skripte in Frage. Siehe proposal.md für Motivation und Umfang.

Das Seitengerüst ist ein einspaltiger `div.container-fluid` mit `max-width: 1000px` (`layout.css`), Beitragstext und Listen stehen in schmalen Blöcken (`--contentwidth: 34em`, `main#index article` 31 em), links und rechts bleibt auf dem Desktop viel Leerraum. Die Schlagwortliste kommt aus `Topics` (`src/topics.rs`) und wird nur für die Startseite (12 häufigste) und `/themen/` gerendert. Der Autor steht in `post.html` im Kopfblock `post-header`, vor Einordnung und Kontakt.

## Goals / Non-Goals

**Goals:**
- Ein gestalteter Footer auf jeder Seite mit Seitenlinks, Kontakt und Social-Icons.
- Eine kleinere, ruhigere Kopfnavigation (Start, Themen, Beiträge).
- Kontaktlinks, die durch Spamfilter kommen (Betreff mit „Flauschiversum“).
- Bessere Platznutzung auf dem Desktop: dreispaltig mit Navigationsspalte links und Sidebar rechts, auf mittleren Fenstern zweispaltig; lesbare Zeilenlänge bleibt.
- Der Autor am Ende des Beitrags.

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

### 4. Drei Spalten, Navigationsspalte und Sidebar

`layout.html` bekommt zwischen Kopf und Footer ein `div.page-layout` mit `{% block content %}` und, außer auf der Suchseite und `/themen/`, zwei Seitenelemente nach dem Inhalt (Steuerung über `head.sidebar: bool`, wie `search_form`):

- `<nav class="sitenav" aria-label="Navigation">` (Suchfeld, Links Start/Themen/Beiträge, Kategorienliste mit Anzahl) und
- `<aside class="sidebar" aria-label="Themen">` (Kategorienliste, Tag-Cloud, „Alle Themen“).

Die Quellreihenfolge ist Inhalt, `sitenav`, `sidebar`, damit Screenreader den Inhalt zuerst erreichen und der Handy-Aufbau ohne CSS-Umordnung stimmt. Per CSS-Grid und `grid-template-areas` stehen die Elemente ab `80em` als `"nav content side"` (Spalten `13em minmax(0, 1fr) 16em`, Lücke `2em`, Rahmen `--maxwidth: 1400px`), zwischen `64em` und `80em` als `"content side"` (`minmax(0, 1fr) 17em`, Rahmen 1200 px) und darunter einspaltig. Beitragstext (`main.page`) bleibt auf `--contentwidth` (40 em) begrenzt und mittig in der Mittelspalte; Listen und Kacheln füllen sie. Die Navigationsspalte ist `position: sticky; top: 1em` mit `max-height` und eigenem Scrollen, die Sidebar bleibt normal.

**Doppelte Markup-Teile statt Umbauen per JavaScript:** Navigationsleiste im Kopf (`header nav`) und Navigationsspalte (`nav.sitenav`) bzw. Suchfeld im Kopf und in der Spalte sowie Kategorienliste in Spalte und Sidebar kommen beide im HTML vor und werden per Media Query mit `display: none` ausgeblendet. Ausgeblendete Elemente sind auch für Screenreader unsichtbar, es gibt also nie zwei sichtbare Fassungen. Ein Test prüft das bei 1440, 1100 und 360 px. Die Suchfelder erhalten verschiedene `id`s (`q`, `q-side`).

Die Sidebar-Daten (Kategorien mit Anzahl, Tags mit Anzahl und Größenstufe) berechnet `Topics` einmal je Build (`topics.sidebar()`), werden in `Common` abgelegt und von jedem Template gerendert; Partials `partials/sidebar.html` und `partials/sitenav.html`. Größenstufe: `level = 1 + round(4 * (ln(n) - ln(min)) / (ln(max) - ln(min)))`, bei `max == min` Stufe 3; CSS-Klassen `cloud-1` … `cloud-5` mit 0.9 bis 1.1 em und schwacher Tönung. Nach der Sichtprüfung wirkten große Größenunterschiede und grüne Schrift unruhig; die Cloud besteht deshalb aus Chips wie die Schlagwortlisten (neutrale Textfarbe, Zahl der Beiträge klein im Chip).

Die Themenübersicht `/themen/` und die Suchseite bleiben einspaltig mit Navigationsleiste und Suchfeld im Kopf. Die Startseite verliert den Abschnitt „Häufige Themen“ und die Konstante `HOME_TOPICS`.

*Alternativen:* (a) Eine einzige Navigation, die per CSS-Grid zwischen Kopf und Spalte wandert (`display: contents`, Umordnung): weniger Markup, aber die Navigation im Kopf und die Spalte stehen an verschiedenen Stellen im DOM; Umordnung per Grid auf ein Element, das nicht im Gridcontainer liegt, ist nicht möglich. (b) Navigationsspalte links und Sidebar rechts per JavaScript verschieben: ohne JavaScript wäre das Layout kaputt. (c) Tag-Cloud links, Navigation rechts: Navigation steht üblicherweise links. Gewählt ist doppeltes Markup, weil es ohne JavaScript, ohne Layoutsprünge und mit einfachen Tests auskommt.

### 4a. Karten, Suchfeld und Typografie (Überarbeitung nach der ersten Sichtprüfung)

- **Karten:** Eine Klasse `.card` (Hintergrund `rgb(255 255 255 / 0.55)`, Rand `1px solid rgb(0 0 0 / 0.06)`, Radius `14px`, `box-shadow` fein, Innenabstand `1em 1.1em`) liegt unter der Navigationsspalte, der Kategorien-Karte und der Themen-Karte der Sidebar. Das Hintergrund-Grün der Seite bleibt, die Karten heben die Spalten ab. Kontrast Text (`#333`) auf dem hellen Kartenhintergrund liegt über 7:1.
- **Suchfeld:** Ein Wrapper `.searchbox` (Flex, `position: relative`, abgerundet) enthält das Eingabefeld (`padding-right` für die Lupe) und den Knopf, der absolut am rechten Rand sitzt; er besteht aus einem Lupen-Icon (inline SVG, `partials/icon-search.html`) mit `aria-label="Suchen"`. Das sichtbare „Suchen“ als Beschriftung bleibt als `label` mit der Klasse für Screenreader (visuell verborgen). Es gibt keinen Umbruch, weil das Feld die ganze Breite füllt und die Lupe nicht daneben steht. Die Pagefind-Oberfläche auf der Suchseite behält ihre eigene Lupe.
- **Typografie:** Variablen `--font-display: "Lobster"`, `--font-text: "Lora", serif`; Skala `--fs-title: 2em`, `--fs-section: 1.5em`, `--fs-ui: 1em`, `--fs-small: 0.875em`. Lobster bleibt für Logo, h1/h2 und Kachel- bzw. Kartentitel. Navigationslinks, Kategorien, Cloud, Footer, Knöpfe und Beschriftungen stehen in Lora, Links der Navigation halbfett. Die Schriftdatei „Lora-Bold“ ist bereits eingebunden; für „halbfett“ wird `font-weight: 700` genutzt, damit keine neue Schrift geladen werden muss.
- **Farbschema:** `theme.css` definiert die Variablen neu: `--bgcolor: #faf7f0`, `--card-bg: #fff`, `--acolor` (Text) `#2b2b2b`, `--link: #2f5d3a`, `--accent: #6f9a5b`, `--accent-soft: #e4eedc` (Hover, Tags, Pagination), `--bordercolor: #e2dccf`. Knöpfe und aktive Elemente nutzen den Akzent mit dunklem Text `#121a12` (Kontrast über 5 : 1; weißer Text auf `#6f9a5b` käme nur auf etwa 3 : 1). Die alten Namen `--theme-*`, `--pagination-*` werden umgestellt oder entfernt, die Leuchteffekte (Textschatten am Logo, grüner Schein um Bilder in `autoren.css` und `media.css`) werden durch dezente Schatten ersetzt. Die Staging-Leiste bleibt rot.
- **E-Mail im Footer:** nur das Briefumschlag-Icon mit `aria-label="E-Mail schreiben"` und `title` mit der Adresse.

### 5. Autor am Ende

`post.html`: Die Zeile „Von <Autor>“ wandert aus dem Kopf (`post-header`) hinter `{{ content|safe }}` und vor `nav.post-nav`, als `<p class="author">` in `main.page`. Die Einordnung (Kategorie, Schlagworte) und der E-Mail-Hinweis bleiben im Kopf. Das Markup-Fragment und die Klasse `author` bleiben, damit `autoren.css` weiter greift (zu prüfen).

### 6. Gestaltung

Neue Dateien `content/style/footer.css` und `content/style/sidebar.css` (in `main.css` importiert): oberer gestrichelter Rand wie die Beitragslisten (`1px dashed var(--bordercolor)`), „Lobster“ nur für die Linkgruppe wie in der Kopfnavigation, Link-Hover mit `var(--pagination-hover)` wie bei der Paginierung, Icons in `var(--acolor)` und beim Hover in einem dunkleren Ton. Abstände in `em`, damit es mit der Schriftgröße skaliert. Die Staging-Leiste bleibt oberhalb der Seite und ist vom Footer unabhängig.

Kopfnavigation: Mit drei Einträgen bleibt `float: left` mit zentriertem Block möglich; die vorhandene Mobil-Regel (kleinere Ränder unter 40 em) bleibt.

### 5. E-Mail-Betreff

Eine kleine Funktion `mailto(subject)` in `render.rs` erzeugt alle `mailto:`-Adressen aus Konstante und kodiertem Betreff; sie lässt keinen Aufruf ohne „Flauschiversum“ im Betreff zu (Debug-Assertion und Test). Die Links in Beiträgen („[Flauschiversum] <Titel>“) und im Impressum (Markdown, „[Flauschiversum]“) bleiben unverändert und erfüllen die Regel bereits; ein Test über die gesamte gebaute Seite sichert das.

## Risks / Trade-offs

- [Wichtige Seiten (About, Impressum) sind nicht mehr im Kopf sichtbar] → Gewollt; der Footer steht auf jeder Seite, das Impressum bleibt dort Pflichtlink.
- [Markenlogos und deren Nutzungsregeln] → Nur als Verweis auf die eigenen Profile, in einfarbiger Standardform; kein Hinweis auf Zugehörigkeit. Pfade aus einem CC0-Satz.
- [Eigene Pfade der Icons weichen optisch von den Originalen ab] → Pfade unverändert aus dem Satz übernehmen und im Test (Snapshot) festhalten.
- [Fester Betreff „Flauschiversum“ ist schon in Beiträgen enthalten, aber ein künftiger Link ohne Wort fällt nicht auf] → Test über alle gebauten Seiten schlägt dann fehl.
- [Drei Spalten wirken auf einem 1440-px-Bildschirm eng] → Rahmen 1400 px, Spalten 13 und 16 em, Lesetext höchstens 40 em; Vergleichsbilder zur Abnahme, Schwelle von 80 em bleibt als Konstante einstellbar.
- [Doppeltes Markup (Navigation, Suchfeld, Kategorien) driftet auseinander] → Beide Fassungen entstehen aus denselben Partials und Daten; ein Test vergleicht Ziele und Zahlen und prüft, dass bei jeder Breite genau eine Fassung sichtbar ist.
- [Sidebar macht den Seitenaufbau am Desktop unruhig oder zu hoch] → `sticky` nur bei passender Höhe, Sidebar kompakt (Kategorien klein, Cloud in einer Zeile umbrechend); Vergleichsbilder zur Abnahme.
- [Tag-Cloud wird als Spielerei empfunden oder ist schwer lesbar] → Größenbereich begrenzt (0,85–1,6 em), alphabetisch, hoher Kontrast; Nutzerwunsch ist ausdrücklich eine Cloud.
- [Platz bleibt auf sehr breiten Bildschirmen ungenutzt] → Rahmen auf 1200 px begrenzt; breitere Layouts sind Lesbarkeit zuliebe nicht vorgesehen.
- [Autor nach dem Text wird bei kurzen Beiträgen übersehen] → Gewollt; er steht unmittelbar vor den Links zum älteren und neueren Beitrag.
- [Footer bricht am Handy unschön um] → Browser-Test bei 360 px mit Blick auf waagerechtes Scrollen und Mindestgröße der Klickflächen; Vergleichsbilder zur Abnahme.

## Migration Plan

1. Templates, Icons, Konstanten und CSS umsetzen; Snapshots und Browser-Tests anpassen.
2. Push auf einen Claude-Branch: Der Staging-Lauf veröffentlicht den Footer unter https://staging.flauschiversum.de/ zur Abnahme (Desktop und Handy).
3. Nach der Abnahme archivieren. Rollback: Commit zurücknehmen; es gibt keine Datenänderung.
