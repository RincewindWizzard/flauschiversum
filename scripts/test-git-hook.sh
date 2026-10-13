#!/usr/bin/env bash
# Prüft den post-receive-Hook mit einem lokalen Bare-Repository und einem Ersatzbefehl für `flauschiversum deploy`.
# Aufruf: scripts/test-git-hook.sh
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
src="$PWD"
base="$(mktemp -d)"
trap 'rm -rf "$base"' EXIT
fails=0
check() { if "${@:2}"; then echo "ok   $1"; else echo "FEHLER $1"; fails=$((fails + 1)); fi; }

git init -q --bare "$base/server.git"
"$src/scripts/install-git-hook.sh" "$base/server.git" >/dev/null
check "Hook wird installiert" test -x "$base/server.git/hooks/post-receive"
check "zweite Installation desselben Hooks ist in Ordnung" "$src/scripts/install-git-hook.sh" "$base/server.git"
git init -q --bare "$base/anders.git"; printf '#!/bin/sh\n' > "$base/anders.git/hooks/post-receive"
check "ein anderer vorhandener Hook wird nicht überschrieben" bash -c "! '$src/scripts/install-git-hook.sh' '$base/anders.git' 2>/dev/null"

git init -q "$base/work"
(
  cd "$base/work" && git config user.email t@example.org && git config user.name Test
  echo eins > datei && git add . && git commit -q -m eins && git branch -M master && git remote add origin "$base/server.git"
) >/dev/null 2>&1
export DEPLOY_STATE_DIR="$base/state"
# shellcheck disable=SC2016
export DEPLOY_COMMAND='echo STUB-DEPLOY pwd=$PWD target=$CARGO_TARGET_DIR; if [ -f FAIL ]; then echo Fehler: Build kaputt; exit 1; fi'
push() { (cd "$base/work" && git push -q origin "$1" 2>&1); }

out="$(push master)"
check "Push auf master deployt" grep -q STUB-DEPLOY <<<"$out"
check "Pusher sieht das Ergebnis" grep -q "Deployment erfolgreich" <<<"$out"
check "Compiler-Ergebnisse liegen dauerhaft im Zustandsverzeichnis" grep -q "target=$base/state/target" <<<"$out"
dir="$(grep -o 'pwd=[^ ]*' <<<"$out" | head -1 | cut -d= -f2)"
check "Arbeitsverzeichnis ist temporär und wird entfernt" bash -c "[ -n '$dir' ] && [ ! -e '$dir' ]"
check "Protokoll wird geschrieben" grep -q STUB-DEPLOY "$base/state/deploy.log"
(cd "$base/work" && git checkout -q -b dev && git commit -q --allow-empty -m zwei)
out="$(push dev)"
check "Push auf einen anderen Branch deployt nicht" bash -c "! grep -q STUB-DEPLOY <<<'$out'"
(cd "$base/work" && git checkout -q master && touch FAIL && git add FAIL && git commit -q -m kaputt)
out="$(push master)"
check "Fehler im Build erreicht den Pusher" bash -c "grep -q 'Build kaputt' <<<'$out' && grep -q FEHLGESCHLAGEN <<<'$out'"

echo
[ "$fails" -eq 0 ] && echo "Alle Prüfungen bestanden." || echo "$fails Prüfung(en) fehlgeschlagen."
exit "$((fails > 0))"
