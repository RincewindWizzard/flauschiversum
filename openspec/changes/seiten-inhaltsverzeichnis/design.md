## Context

`templates/page.html` rendert Titel (`.post-header h1`) und `<main class="page">` mit dem Markdown-Inhalt (`markdown::render_page`, pulldown-cmark, `src/markdown.rs`). Überschriften erhalten dort keine `id`. Die Seiten in `content/pages` nutzen die Ebenen uneinheitlich: `tipps.md` und `autoren.md` schreiben Abschnitte als `#`, `impressum.md` als `##`, `autoren.md` mischt `#` und `##`; der Seitentitel selbst ist ebenfalls eine `h1`. Der Seitenrahmen kennt bereits eine Seitenleiste (`body.has-aside`, `aside` im Block `aside` des Layouts, Gitterspalte `--aside-width`, sticky ab 64 em) und `aside-early` für Karten, die auf schmalen Fenstern vor dem Inhalt stehen (Suche). Karten sind `.card`.

## Goals / Non-Goals

**Goals:** ein automatisches, ohne Pflege aktuelles Inhaltsverzeichnis für Einzelseiten; stabile Anker; passend zu Archiv und Filterkarte.
**Non-Goals:** Verzeichnis für Beiträge; Umschreiben der Überschriftenebenen im Markdown; Nummerierung der Abschnitte.

## Decisions

### 1. Überschriften beim Rendern erfassen

`render_page` bleibt, bekommt aber eine Variante, die die Ereignisse von pulldown-cmark durchläuft: Für jedes `Start(Heading)` wird der Text bis `End(Heading)` gesammelt (nur Text und Code, ohne Auszeichnung), daraus ein Anker erzeugt (`urls::slugify`, also Kleinbuchstaben und `ä`→`ae`), bei Doppelten mit `-2`, `-3` nummeriert, und die Überschrift mit `id` ausgegeben (`Tag::Heading { id, .. }` setzen, damit der Standard-HTML-Renderer das Attribut schreibt). Die Funktion liefert `(html, Vec<TocEntry { level, id, text }>)`. Heading-Attribute im Markdown (`{#id}`) werden nicht aktiviert, damit bestehender Text mit Klammern unverändert bleibt. Anker, die mit festen IDs des Seitenrahmens kollidieren würden (`search-form`, `q` usw.), erhalten ein Suffix.

*Alternative:* HTML nachträglich mit Regeln durchsuchen. Verworfen: fehleranfällig bei eingebettetem HTML und Code.

### 2. Verschachtelung nach relativer Ebene

Die Ebenen der Seiten sind uneinheitlich. Die Liste wird daher nach der *relativen* Ebene verschachtelt: die kleinste vorkommende Ebene ist die erste Stufe, jede größere Ebene eine Stufe tiefer (höchstens drei Stufen, tiefere Überschriften erscheinen auf der dritten). Das Verzeichnis zeigt nur Überschriften im Inhalt, nicht den Seitentitel. Weniger als zwei Überschriften: keine Karte, die Seite bleibt einspaltig wie heute.

### 3. Seitenleiste

`page.html` füllt den `aside`-Block mit `<aside class="toc card" aria-labelledby="toc-titel"><details id="toc-details" open><summary><h2 class="side-title" id="toc-titel">Inhalt</h2></summary><nav aria-label="Inhalt dieser Seite"><ol>…</ol></nav></details></aside>`; Body-Klasse `has-aside aside-early` (vorhandene Mechanik: ab 64 em rechts und sticky, darunter vor dem Text). Am Desktop ist `details` offen, am Handy zunächst zu (`toc.js` setzt `open` nach der Fensterbreite, ohne JavaScript bleibt es offen und damit auf Handys einfach ein Block über dem Text). Eigenes `content/style/toc.css` (Karte wie `aside.filters`/Archiv, Einrückung je Stufe, Fokus sichtbar, Titel in Lobster). Ankersprünge brauchen `scroll-margin-top` an den Überschriften, damit die sticky Topbar sie nicht verdeckt.

### 4. Aktueller Abschnitt

`assets/js/toc.js` (nur auf Seiten mit Verzeichnis geladen, Hash im Dateinamen wie die anderen Skripte) beobachtet die Überschriften mit `IntersectionObserver` und setzt `aria-current="true"` am Link des obersten sichtbaren Abschnitts (Hervorhebung in `--accent-soft`). Fehlt `IntersectionObserver` oder JavaScript, bleibt die Linkliste unverändert nutzbar.

### 5. Annahmen

- „Pages“ meint alle Einzelseiten aus `content/pages`; Beiträge sind ausgenommen.
- Die Schwelle von zwei Überschriften verhindert Ein-Eintrag-Verzeichnisse (z. B. beim kurzen Impressum mit zwei Überschriften erscheint es, bei einer nicht).
- Der Text „Inhalt“ als Titel der Karte.

## Risks / Trade-offs

- [Zwei `h1` auf der Seite (Titel und Abschnitte) sind schlechte Struktur] → bleibt wie heute (Non-Goal); das Verzeichnis zeigt nur die Abschnitte.
- [Anker ändern sich, wenn die Überschrift umformuliert wird] → Anker folgen dem Text; Verweise von außen auf Anker gibt es bisher nicht.
- [Sticky Topbar verdeckt Ziele] → `scroll-margin-top` in Höhe der Topbar.
- [Kollision der Anker mit IDs im Seitenrahmen] → Reserveliste mit Suffix, Test über alle Seiten: jede `id` kommt genau einmal vor.
