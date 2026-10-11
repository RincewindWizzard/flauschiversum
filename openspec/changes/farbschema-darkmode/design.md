## Context

`content/style/theme.css` definiert heute ein helles Schema (Cremeweiß, Salbeigrün). Einzelne Werte stehen noch fest in anderen Stylesheets (z. B. Tönungen der Tag-Cloud, Hover-Farben). Das Maskottchen ist ein violett-blauer Planet mit Cyan-Rand (Verlauf `#b859fd` → `#596af4` → `#59fdf4`, Umriss `#5b4ba6`). Die Vorschau `themen-vorschau.html` in diesem Change zeigt alle Vorschläge in Hell und Dunkel, mit Simulation von Farbfehlsichtigkeit, Graustufen und abschaltbarem Sternenhimmel; die Kontraste unten sind mit einem Skript berechnet.

## Goals / Non-Goals

**Goals:** Hell- und Dunkelmodus; ein farbigeres, zum Namen passendes Schema mit Universum-Motiv; gute Lesbarkeit auch bei Sehschwäche und Rot-Grün-Blindheit; Farben an einer Stelle.
**Non-Goals:** Layout, Schriften, Inhalte; mehrere Themen gleichzeitig für Besucher (die Autorin wählt eines, Besucher wählen nur Hell/Dunkel).

## Decisions

### 1. Vier Themenfamilien (Autorin wählt eine)

| Familie | Charakter | Hell | Dunkel |
|---|---|---|---|
| **Flauschi-Nebel** (Empfehlung) | Violett/Indigo wie das Fell des Maskottchens, Cyan und Gold als Sternenlicht; passt am besten zum Icon | Lavendelweiß, Indigo-Links, Violett-Akzent | Tiefes Nachtviolett, helle Lavendel-Links, Gold-Sterne |
| **Mitternachtsblau** | Nachthimmel mit goldenen Sternen; Blau/Orange ist für alle Formen von Farbfehlsichtigkeit am besten trennbar | Warmes Weiß, Blau-Links, Bernstein-Akzent | Dunkelblau, Hellblau-Links, Gold-Akzent |
| **Orchideen-Galaxie** | Magenta und Petrol wie ein Emissionsnebel; verspielt, warm | Rosé-Weiß, Beeren-Links, Petrol-Akzent | Dunkelviolett, Rosa-Links, Türkis-Akzent |
| **Salbei-Kosmos** | Das heutige Salbeigrün, nachts als „Waldnacht“; ruhigste Änderung | Cremeweiß wie heute | Dunkelgrün, helle Salbei-Links, Gold-Sterne |

Alle Werte (Rolle → Hex):

| Thema | Modus | Hintergrund | Karte | Text | Grauer Text | Link | Akzent | Schrift auf Akzent | Tönung | Rand | Stern |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Flauschi-Nebel | hell | `#f6f3fc` | `#ffffff` | `#1d1930` | `#575270` | `#4a38b0` | `#6d4fe0` | `#ffffff` | `#e9e4fa` | `#8f89ab` | `#b8860b` |
| Flauschi-Nebel | dunkel | `#0e0b1f` | `#191537` | `#ebe8f8` | `#aaa5c8` | `#b4a2ff` | `#8f73f5` | `#0e0b1f` | `#2a2459` | `#6f68a0` | `#ffd166` |
| Mitternachtsblau | hell | `#f7f5ee` | `#ffffff` | `#1a2133` | `#505a70` | `#1b4f94` | `#a35500` | `#ffffff` | `#e6ecf6` | `#8a93a8` | `#a35500` |
| Mitternachtsblau | dunkel | `#0a1124` | `#121b36` | `#e9edf8` | `#a3adc8` | `#8dbbff` | `#f2b84b` | `#0a1124` | `#1d2a52` | `#5f6d95` | `#f6c75c` |
| Orchideen-Galaxie | hell | `#fbf4f8` | `#ffffff` | `#2a1c2b` | `#6a4f6a` | `#8a1f6b` | `#0e7a78` | `#ffffff` | `#f4e1ee` | `#a98aa6` | `#9a5b00` |
| Orchideen-Galaxie | dunkel | `#150c19` | `#21142a` | `#f3e9f3` | `#c2a9c2` | `#ff9ad5` | `#3fd0c9` | `#0b1a1a` | `#38214a` | `#7d5f86` | `#ffd27a` |
| Salbei-Kosmos | hell | `#faf7f0` | `#ffffff` | `#2b2b2b` | `#5a554a` | `#2f5d3a` | `#6f9a5b` | `#121a12` | `#e4eedc` | `#8c8369` | `#9a6b00` |
| Salbei-Kosmos | dunkel | `#101612` | `#18211b` | `#e8efe6` | `#a6b5a5` | `#9fd49a` | `#7fb36d` | `#0c120d` | `#24342a` | `#5f7562` | `#f0cf74` |

Abgeleitete Tönungen (Hover, Chips) entstehen im CSS per `color-mix()` aus Akzent und Karte, damit es keine weiteren festen Werte gibt.

### 2. Kontrast und Farbfehlsichtigkeit

Berechnet nach WCAG 2.2 (relative Luminanz). Alle acht Kombinationen erreichen: Text auf Karte und Hintergrund mindestens 14 : 1, grauer Text mindestens 6,9 : 1, Link auf Karte mindestens 7,6 : 1, Schrift auf Akzent mindestens 5,2 : 1 (Mitternachtsblau hell: 5,5), Akzent und Rand als Bedienelemente mindestens 3 : 1 (Rand mindestens 3,1 : 1). Das ist deutlich über AA (4,5 : 1) und für Text nahe AAA (7 : 1).

Gegen Farbfehlsichtigkeit gilt: (a) Kein Merkmal hängt nur am Farbton. Links sind unterstrichen und heller bzw. dunkler als der Text, aktive Elemente (Kategorie-Tag, Knöpfe) unterscheiden sich durch Füllung und Fettdruck, Archiv-Zustände durch Dreieck und Einrückung. (b) Es gibt kein Rot-Grün-Paar als Bedeutungsträger; die rote Staging-Leiste ist weiß auf Rot mit Text. (c) Mit Machado-Matrizen für Protanopie, Deuteranopie und Tritanopie simuliert, bleibt der Link-Kontrast auf der Karte in allen Familien mindestens 7,3 : 1 und Knopftext auf Akzent mindestens 4,7 : 1 (knappster Wert: Orchideen-Galaxie hell). Salbei hat bei Grünton-Akzent und Gold-Sternen die geringste Farbspreizung und verlässt sich am stärksten auf Helligkeit; Mitternachtsblau (Blau/Orange) ist am robustesten.

Ein Test im Repository (`tests/browser/theme.mjs`) liest die berechneten Farben aller Tokens beider Modi und prüft diese Paare, damit spätere Änderungen sie nicht unterschreiten.

### 3. Sternenhimmel und Nebel

Dekoration liegt als `background-image` auf `body` hinter allem (`position: fixed`-Ebene als Pseudo-Element, `z-index: -1`, `pointer-events: none`): zwei Sternenschichten als kleine SVG-Kacheln (400 × 400 px, einmal erzeugt und als Datei ausgeliefert, rund 1 KB) und zwei weiche Nebel als `radial-gradient` in Akzentfarben. Im Dunkelmodus voll sichtbar, im Hellmodus mit 20 % Deckkraft als zarte Punkte. Text liegt immer auf einfarbigen Karten oder dem einfarbigen Hintergrund; Sterne und Nebel kreuzen nie den Lesetext, weil der Beitragstext und alle Listen auf `--bg` bzw. `--card` stehen und die Sterne nur außerhalb der Karten sichtbar sind. Zusätzlich: ein kleiner Sternchen-Trenner (`✦`) in der Fußzeile und ein Sternenband hinter der Topbar. Mit `prefers-contrast: more` und in erzwungenen Farben (`forced-colors: active`) verschwindet die Dekoration; es gibt keine Animation (auch kein Funkeln), damit `prefers-reduced-motion` nichts abschalten muss.

*Alternativen:* animierte Sterne (Ablenkung beim Lesen, Energie), Bilddateien (Ladezeit, Wartung). Verworfen.

### 4. Umschalter und Speicherung

Standard folgt `prefers-color-scheme` ohne Skript (CSS). Ein Dreifach-Umschalter „Automatisch / Hell / Dunkel“ in der Topbar (Knopf mit Sonne/Mond-Symbol als inline SVG, `aria-pressed`/Menü, mindestens 44 px) setzt `data-theme="light|dark"` auf `<html>` und speichert die Wahl in `localStorage` (in `try/catch`). Ein kleines Skript im `<head>` setzt das Attribut vor dem ersten Zeichnen, damit nichts aufblitzt; ohne JavaScript gilt die Systemeinstellung. `<meta name="color-scheme" content="light dark">` und `color-scheme` im CSS sorgen für passende Formularelemente und Scrollleisten. Fotos werden im Dunkelmodus nicht verändert (kein Filter), nur Ränder und Schatten.

### 5. Technik

`theme.css` definiert Tokens (`--bg`, `--card`, `--text`, `--muted`, `--link`, `--accent`, `--ink`, `--soft`, `--border`, `--star`) für `:root` (hell), `@media (prefers-color-scheme: dark) :root:not([data-theme="light"])` und `:root[data-theme="dark"]`. Alle anderen Stylesheets verwenden nur Tokens (feste Hex-Werte entfallen; ein Test sucht sie). Pagefind-Variablen (`--pagefind-ui-*`) und PhotoSwipe-Farben hängen an den Tokens. Maskottchen im Dunkelmodus mit `filter: drop-shadow` als sanfter Schein, weil der dunkle Umriss auf Dunkel verschwindet.

## Risks / Trade-offs

- [Dekoration lenkt ab] → nur außerhalb von Textflächen, ohne Bewegung, abschaltbar durch Systemeinstellungen; Vorschau zeigt „Sternenhimmel aus“.
- [Gold-Sterne im Hellmodus bleiben zu blass] → reine Dekoration, keine Information; Kontrast der Sterne ist nicht relevant.
- [Pagefind-Oberfläche ist schwer zu stylen] → Variablen setzen, Browser-Test auf Kontrast der Treffer.
- [Mehr CSS] → rund 1 KB gzip für Tokens und Sterne.
- [Salbei-Kosmos hat wenig Farbtonunterschied zwischen Link, Akzent und Text] → verlässt sich auf Helligkeit und Unterstreichung; deshalb nicht erste Wahl für Farbfehlsichtigkeit.
