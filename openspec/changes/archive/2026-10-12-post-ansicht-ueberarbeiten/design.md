## Context

`templates/post.html` rendert im Block `header` Titel, Vorschaubild, Schlagworte und die Zeile „Schreibe Ihr eine Nachricht!“; im Inhalt Text, Autorenzeile, die Karte `aside.related` (`ul` mit Titel-Link und Chips der gemeinsamen Schlagworte) und die Navigation. Die Beitragslisten nutzen `partials/entry.html` in `ul.entries` (`topics.css`: rundes Bild 5 em, Titel kursiv 1,1 em, Datum 0,8 em mit 0,8 Deckkraft, Kurztext 0,9 em). Das Badge (`assets/badge.png`, 800 × 133, RGBA) ist limonengrün (189, 242, 57, Alpha 110) mit graugrünem Text (69, 76, 52) und wird von `media/process.rs` aufgebracht; der Cache-Schlüssel enthält den Badge-Inhalt.

## Goals / Non-Goals

**Goals:** Abschluss unten, Datum oben, ein stimmiges Wasserzeichen, eine einheitliche Darstellung von Beitragseinträgen.
**Non-Goals:** neue Auswahlregeln für ähnliche Beiträge, Umformulierung des Hinweises, Änderungen am Farbschema.

## Decisions

### 1. Kontakt-Hinweis
Der Absatz wandert aus `.post-header` in den Inhalt nach der Autorenzeile und vor die Karte „Ähnliche Beiträge“ (`<p class="contact">`, zentriert, Kleintext wie bisher). Wortlaut, Link und Betreff bleiben (Spec `contact-links`).

### 2. Datum
Im Kopf direkt unter dem Titel: `<p class="post-date"><time datetime="…">31. Oktober 2020</time></p>`, mittig wie der Titel, in Textgröße (`--fs-ui`, nicht kleiner als der Fließtext), Farbe `--muted` (Kontrast 7 : 1 laut Theme). Die Daten (`date_iso`, `date_human`) gibt es bereits (`render.rs`); `PostTemplate` bekommt sie. Das Datum steht damit „über dem Beitrag“, vor Bild und Text.

### 3. Ähnliche Beiträge wie die Liste
`RelatedLink` wird durch die vorhandene `Entry` ersetzt (Bild, Titel, Datum, Kurztext), die Karte rendert `<ul class="entries">` mit `partials/entry.html`. `related.css` verliert die eigene Listengestaltung und behält nur die Karte (Überschrift in Lobster, Breite wie der Beitragstext). Die gemeinsamen Schlagworte entfallen: Die Liste der Startseite zeigt keine, und die Anforderung war „genau so“. *Annahme:* Die Chips sind nicht gewünscht; sonst ließen sie sich als Zusatz ergänzen.

*Alternative:* Chips beibehalten und nur das Layout angleichen. Verworfen, weil „genau so“ gefordert ist.

### 4. Wasserzeichen
Neues `assets/badge.png` mit Pillow aus `Lobster.otf` erzeugt (800 × 133 wie bisher, damit Größenlogik und Spec-Szenarien gleich bleiben): Fläche helles Lavendel in Nebel-Farbe (`#e9e4fa`, wie `--accent-soft`) mit Alpha etwa 0,45 (110/255 wie bisher; Dunkles Foto: auf Schwarz bleibt die Fläche als hellvioletter Schleier sichtbar), Schriftzug Nebel-Indigo (`#1d1930`, Textfarbe des hellen Themas) mit Alpha 0,85, abgerundet wie bisher. Zusätzlich ein feiner Rand in `#6d4fe0` (Akzent) mit geringer Deckkraft, damit das Badge auf hellen Fotos nicht verschwimmt. Das Badge ist für beide Modi gleich, weil es im Bild steckt. Das Skript liegt als `openspec/changes/post-ansicht-ueberarbeiten/assets/make_badge.py` (zur Nachvollziehbarkeit). Die Bildpipeline bleibt unverändert; der Cache-Schlüssel enthält den Badge-Inhalt, daher werden alle `700x`/`1000x`-Varianten neu berechnet (einmalig, bei 3200 Bildern mehrere Minuten, danach wieder aus dem Cache). *Hinweis:* Die Farbregel „Farben nur in `theme.css`“ gilt für CSS, nicht für das Bild.

## Risks / Trade-offs

- [Lavendel-Fläche auf hellen Fotos schwach sichtbar] → Rand in Akzentfarbe und kräftiger Text; Kontrast Text/Fläche über 7 : 1, Test mit einem hellen und einem dunklen Foto.
- [Neuberechnung aller Bilder dauert] → einmalig, der Build zeigt Fortschritt; der Cache im Staging-Workflow wird danach erneuert.
- [Datum doppelt in der Liste und auf der Seite] → gewollt: Die Liste verweist, die Seite zeigt es im Kopf.
