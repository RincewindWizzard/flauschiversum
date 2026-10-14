## Context

`content/style/main.css` beginnt mit einem Reset (`h1…h6, p, body, a, img, ul, ol, blockquote, pre, figure { margin: 0; padding: 0; border: 0 }`). `content/style/page.css` stylt `main.page` (Breite `--contentwidth`, Seitenabstand 1em) und gibt nur `h2` ein `padding-top: 1em` und direkten `ul`-Kindern `margin-left/-bottom: 1em`; Absätze haben keinen Abstand. Die Schrift (`fonts.css`) hat `line-height: 1.52`. `main.page` wird von `templates/post.html` (Beitrag) und `templates/page.html` (Einzelseite) verwendet; Übersichten nutzen `main.topic`, `#home`, `#index`. Bildergruppen und Figuren (`media.css`) haben eigene Abstände (`margin: 1em 0`).

## Goals / Non-Goals

**Goals:** Absätze im Beitragstext sind sofort als getrennt erkennbar; Listen, Zitate, Code und Überschriften haben stimmige Abstände; keine Auswirkung außerhalb des Textbereichs.
**Non-Goals:** neue Schrift, Zeilenabstand oder Textbreite ändern; Einrückung statt Abstand; Änderung an Markdown oder HTML.

## Decisions

### 1. Abstände im Textbereich per Regel auf `main.page`

In `page.css`:

```css
main.page {
  & p, & ul, & ol, & blockquote, & pre { margin-block: 0 1em; }
  & p + p { margin-top: 0; }
  & h2, & h3, & h4 { margin-block: 1.6em 0.5em; }
}
```

Abstand **nach** jedem Block (`1em`, entspricht rund 1,5 Zeilen bei `line-height: 1.52`) statt Abstand oben und unten, damit sich Abstände nicht addieren; `h2` behält seine Wirkung, das bisherige `padding-top: 1em` wird durch den Überschriften-Abstand ersetzt, damit es nicht doppelt zählt. Der Abstand ist in `em`, wächst also mit der Schriftgröße.

*Alternative:* ein Wert in `rem` oder `px` — verworfen, weil er bei geänderter Schriftgröße (Zoom, Nutzer-Einstellung) nicht mitwächst. *Alternative:* Absatzeinzug statt Abstand — verworfen, weil im Web Abstand üblicher und für kurze Absätze besser lesbar ist.

### 2. Was nicht getroffen wird

Die Regel hängt an `main.page` (Beitrag und Einzelseite). Übersichten, Themenseiten, Suche, Startseite, Sidebar-Karten, Kopf und Fuß bleiben unberührt, weil dort eigene Abstände gelten. Der Autorenhinweis (`p.author`), Kontakt (`p.contact`) und andere `p` mit Klasse behalten ihr Aussehen, soweit sie eigene Abstände setzen; die Regel gilt als `main.page p:not([class])`, damit Klassen-Absätze (Autor, Kontakt, Datum, Schlagworte) unverändert bleiben.

### 3. Bilder und Figuren

`figure`/Bildergruppen setzen `margin: 1em 0` selbst; sie bleiben außerhalb der neuen Regel (`figure` ist nicht aufgeführt), damit Abstände nicht doppelt wirken. Ein Absatz, der direkt auf eine Bildergruppe folgt, bekommt keinen zusätzlichen oberen Abstand.

### 4. Test

Ein Playwright-Test lädt einen Beitrag mit mindestens zwei aufeinanderfolgenden Absätzen und prüft per `getComputedStyle`, dass `margin-bottom` des ersten Absatzes mindestens eine Zeilenhöhe beträgt und der Abstand zum folgenden Absatz im gerenderten Layout (Differenz der `getBoundingClientRect`-Werte) größer als die Zeilenhöhe ist, im hellen und im dunklen Thema. Dazu ein Blick auf einen Screenshot (vorher/nachher).

## Risks / Trade-offs

- [Seiten werden länger] → gewollt; der Abstand ist mit rund 1,5 Zeilen moderat, in `page.css` an einer Stelle einstellbar.
- [Doppelte Abstände mit bestehenden Regeln (`h2` padding, `ul` margin, Figuren)] → die bisherigen Einzelregeln werden mit der neuen zusammengeführt; Sichtprüfung auf Beiträgen mit Bildern, Listen und Überschriften.
- [Eingebettetes HTML in Beiträgen hat eigene Abstände] → `p:not([class])` und Regeln auf Blockelementen können dort greifen; stichprobenartige Prüfung der Beiträge mit viel eingebettetem HTML.
