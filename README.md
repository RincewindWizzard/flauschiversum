# Flauschiversum
Dies ist der Inhalt in die Software, die das Flauschiversum bereitstellt.


# Abhängigkeiten
- nodejs
- npm
- [LessCss](http://lesscss.org/)


    sudo apt-get -y install nodejs npm
    npm install -g less

Ausführen mit:

    python3 main.py --dst build --cache .cache 

## Beschreibung der Ordner
- docker Hier befinden sich Informationen für das Deployment und wie man das Projekt kompiliert
- blogcompile der Programmcode der die Quellen in deine HTML Seite übersetzt.
    - builder.py hier ist der Einstieg der alle anderen Knöpfe und Hebel betätigt.
    - sourcewalker.py der Source Walker durchsucht das src/ Verzeichnis und erzeug die Model Objekte
    - model.py hier ist das Datenmodell definiert, mit dem alle arbeiten.
    - query.py hier werden die Modell Objekte gefiltert und gruppiert
    - views.py erzeugt aus einem Query die zugehörigen Ausgabe Daten
    - urls.py gibt an, wo was in der Webseite liegt
    - markdown_slideshow.py Nerviges Gefrimel im Markdown 
    - cache.py Zwischenspeichern von skalierten Bildern
- src
    - pages Markdown Dokumente für einzelne Seiten
    - posts Alle Posts liegen hier als Markdown Dokumente vor zusammen mit ihren Bildern.
    - static Diese Daten werden unbearbeitet in die Webseite übernommen
    - style Hier liegen die CSS Dateien im [LessCSS Syntax](http://lesscss.org/) vor, wird alles nach style.css exportiert.
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
    
## Deployment
Man baut den flauschiversum-deploy container:

    cd docker/flauschiversum-deploy
    docker build -t flauschiversum-deploy
    docker run -ti flauschiversum-deploy
    
Beim ersten Start wird der neu erzeugte öffentliche SSH-Deploy-Key angezeigt.
Dieser muss auf dem Repository Host für den user git und dem Deployment Server für den User www-data hinterlegt werden.
Die öffentlichen Fingerprints der Hosts müssen akzeptiert werden.

Danach legt man auf dem Build Host eine Datei _/opt/flauschiversum-deploy_ mit folgendem Inhalt an:

    #!/bin/bash
    docker start flauschiversum-deploy

Diese wird nun über visudo für den Benutzer git freigegeben:

    visudo
    git     ALL = NOPASSWD: /opt/flauschiversum-deploy