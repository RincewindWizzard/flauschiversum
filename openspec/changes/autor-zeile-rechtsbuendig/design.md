## Context

`templates/post.html` gibt nach dem Inhalt `<p class="author">Von <a href="/about/" class="author">…</a></p>` und danach `<p class="contact">…</p>` aus. Es gibt keine CSS-Regel für `p.author`; `a.author` ist nur über `fonts.css` formatiert, `img.author` in `autoren.css` ist ein Bild. Der globale Reset setzt `margin`/`padding` von `p` auf 0. Der Beitragstext steht in `main.page` (`page.css`).

## Goals / Non-Goals

**Goals:** Die Zeile wirkt als Unterschrift: rechts ausgerichtet, mit Luft zum Text.
**Non-Goals:** Wortlaut, Link, Reihenfolge, Schriftart oder Farbe ändern; die Kontaktzeile umgestalten.

## Decisions

1. **Regel in `page.css`:**

   ```css
   main.page p.author {
     text-align: right;
     margin-top: 1.5em;
   }
   ```

   Abstand in `em`, damit er mit der Schriftgröße skaliert; `text-align` statt Flex/Float, weil die Zeile ein einzelner Absatz ist und so auch bei schmalen Bildschirmen und im Seitenumbruch stabil bleibt. Der Abstand zur folgenden Kontaktzeile bleibt wie bisher (kein `margin-bottom`), die Kontaktzeile behält ihre Ausrichtung.
2. **Bewusst nicht** in die allgemeine Absatzregel aus `absatzabstand-beitraege` aufgenommen: Die Unterschrift hat ein eigenes Aussehen; die allgemeine Regel behandelt nur Absätze ohne Klasse.
3. **Test:** Playwright prüft auf einer Beitragsseite mit Autor, dass `p.author` rechtsbündig ist (`text-align: right`, rechter Rand des Textes liegt am rechten Rand des Inhaltsbereichs) und dass zwischen dem letzten Textabsatz und der Zeile ein Abstand von mehr als einer Zeilenhöhe liegt; im hellen und im dunklen Thema. Bei Beiträgen ohne Autor gibt es die Zeile weiterhin nicht (bestehender Test).

## Risks / Trade-offs

- [Überschneidung mit `absatzabstand-beitraege`] → Die Selektoren sind getrennt (`p:not([class])` dort, `p.author` hier); Reihenfolge im Stylesheet ist unerheblich. Beide Changes lassen sich unabhängig voneinander umsetzen.
- [Rechtsbündiger Text bei langem Namen auf schmalem Bildschirm] → Der Absatz bricht normal um und bleibt rechtsbündig.
