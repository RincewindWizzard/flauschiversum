# Flauschiversum
Dies ist der Inhalt in die Software, die das Flauschiversum bereitstellt.

## Beschreibung der Ordner
- blogcompile Der Programmcode der die Quellen in deine HTML Seite übersetzt.
    - builder.py Hier ist der Einstieg der alle anderen Knöpfe und Hebel betätigt.
    - sourcewalker.py Der Source Walker durchsucht das src/ Verzeichnis und erzeug die Model Objekte
    - model.py Hier ist das Datenmodell definiert, mit dem alle arbeiten.
    - query.py Hier werden die Modell Objekte gefiltert und gruppiert
    - views.py Erzeugt aus einem Query die zugehörigen Ausgabe Daten
    - urls.py Gibt an, wo was in der Webseite liegt
    - markdown_slideshow.py Nerviges Gefrimel im Markdown 
    - cache.py Zwischenspeichern von skalierten Bildern
- src
    - pages Markdown Dokumente für einzelne Seiten
    - posts Alle Posts liegen hier als Markdown Dokumente vor zusammen mit ihren Bildern.
    - static Diese Daten werden unbearbeitet in die Webseite übernommen
    - style Hier liegen die CSS Dateien im LessCSS Syntax vor, wird alles nach style.css exportiert.
- templates Werden in [Jinja2](https://jinja.palletsprojects.com/en/2.10.x/) geschrieben
    - lib
        - pagination.html Zuständig für das Blättern in Index Seiten
    - layout.html Das grundlegende Layout der Seite. Alles was hier drin steht ist überall zu sehen.
    - index.html Seiten mit Listen von Posts
    - page.html Page-Seiten z.B. Impressum, Autoren, etc.
    - post.html Post-Seiten
- test Hier liegen ein paar Tests (hauptsächlich unwichtiges gefrickel)
- main.py Hiermit wird der Blog compiler gestartet
- settings.py Globale Einstellungen
    