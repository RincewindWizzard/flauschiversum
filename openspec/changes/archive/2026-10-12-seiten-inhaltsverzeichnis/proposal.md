## Why

Einzelseiten wie „Tipps und Tricks“, „About“ oder das Impressum sind lange Texte mit mehreren Abschnitten, aber Besucher sehen nicht, was auf der Seite steht, und können nicht gezielt zu einem Abschnitt springen. Ein Inhaltsverzeichnis in der Seitenleiste löst das, und es soll sich ohne Pflege aus den Überschriften der Seite von selbst ergeben, damit es nicht veraltet.

## What Changes

- **Inhaltsverzeichnis als Seitenleiste:** Einzelseiten (`content/pages/*.md`) mit mindestens zwei Überschriften zeigen eine Karte „Inhalt“ mit Links auf alle Überschriften, verschachtelt nach Überschriftenebene. Ab 64 em steht sie rechts neben dem Text (wie Archiv und Tag-Cloud, sticky), auf schmalen Fenstern über dem Text, zunächst zugeklappt.
- **Automatisch aus den Überschriften:** Der Build liest die Überschriften des gerenderten Markdown und erzeugt das Verzeichnis. Es gibt keine Pflege von Hand und keine Angabe im Frontmatter.
- **Anker für Überschriften:** Jede Überschrift bekommt ein `id` aus ihrem Text (deutsche Umschreibung wie bei den URLs, Doppelte werden nummeriert), damit Verweise wie `/tipps-und-tricks/#ufos` und das Verzeichnis funktionieren. Bestehende Seiteninhalte und URLs bleiben unverändert.
- **Aktueller Abschnitt:** Mit JavaScript hebt das Verzeichnis den Abschnitt hervor, der gerade im Bild ist (ohne JavaScript bleibt es eine einfache Linkliste).
- **Nicht Teil dieses Changes:** Beiträge (Posts) bekommen kein Inhaltsverzeichnis; die Überschriftenebenen im Markdown der Seiten werden nicht umgeschrieben (mehrere Seiten nutzen `#` statt `##`, das bleibt).

## Capabilities

### New Capabilities

- `page-toc`: Automatisches Inhaltsverzeichnis in der Seitenleiste von Einzelseiten, Überschrift-Anker und Hervorhebung des aktuellen Abschnitts.

### Modified Capabilities

- `site-layout`: Die ergänzenden Karten des Seitenrahmens umfassen auch das Inhaltsverzeichnis, das auf schmalen Fenstern über dem Text steht.

## Impact

- `src/markdown.rs` (Überschriften mit `id` und Liste der Überschriften), `src/render.rs`/`src/site.rs` (Daten an `PageTemplate`), `templates/page.html` (Seitenleiste, Body-Klasse `has-aside aside-early`), neue `content/style/toc.css`, kleines Skript `assets/js/toc.js` (Hervorhebung), Snapshots, Rust- und Browser-Tests, README.
- Alle Seiten-URLs bleiben unverändert; in den Seiten kommen nur `id`-Attribute an den Überschriften und die Karte hinzu.
