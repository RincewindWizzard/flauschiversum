#!/usr/bin/env bash
# Installiert den post-receive-Hook in ein Git-Repository auf dem Server (Bare-Repository oder .git eines Arbeitsverzeichnisses).
# Aufruf: scripts/install-git-hook.sh <Pfad zum Repository>
set -euo pipefail
repo="${1:?Pfad zum (Bare-)Repository fehlt, z.B. ~/git/flauschiversum.git}"
src="$(cd "$(dirname "$0")" && pwd)/git-hooks/post-receive"
if [ -d "$repo/hooks" ]; then hooks="$repo/hooks"
elif [ -d "$repo/.git/hooks" ]; then hooks="$repo/.git/hooks"
else echo "Fehler: $repo ist kein Git-Repository (kein hooks/-Verzeichnis)." >&2; exit 1; fi
dest="$hooks/post-receive"
if [ -e "$dest" ] && ! cmp -s "$src" "$dest"; then
  echo "Fehler: $dest existiert schon und ist ein anderer Hook; bitte von Hand zusammenführen." >&2
  exit 1
fi
install -m 755 "$src" "$dest"
echo "Hook installiert: $dest"
echo "Deployt wird nur bei Pushes auf den Hauptbranch (Branch, auf den HEAD des Repositorys zeigt; anders: DEPLOY_REF). Voraussetzung: der Benutzer"
echo "flauschiversum und die sudo-Regel (README, \"Server für das Deployment einrichten\")."
