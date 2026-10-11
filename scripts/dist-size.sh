#!/usr/bin/env bash
# Gibt die Größe von dist/ aus und warnt ab 800 MB (Pages-Limit: 1 GB je Seite).
# Aufruf: scripts/dist-size.sh [Verzeichnis]
dir="${1:-dist}"
limit_mb="${DIST_WARN_MB:-800}"
mb=$(( $(du -sk "$dir" | cut -f1) / 1024 ))
echo "Größe von $dir: ${mb} MB (Pages-Limit 1000 MB, Warnung ab ${limit_mb} MB)"
if [ "$mb" -ge "$limit_mb" ]; then
  echo "::warning title=Seite wird zu groß für GitHub Pages::$dir ist ${mb} MB groß; das Limit liegt bei 1 GB."
fi
