#!/usr/bin/env bash
# Prüft den post-receive-Hook und den Befehl deploy-commit mit einem lokalen Bare-Repository und Ersatzprogrammen (kein root nötig).
# Aufruf: scripts/test-git-hook.sh
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
src="$PWD"
base="$(mktemp -d)"
trap 'rm -rf "$base"' EXIT
fails=0
check() { if "${@:2}"; then echo "ok   $1"; else echo "FEHLER $1"; fails=$((fails + 1)); fi; }

git init -q --bare "$base/server.git"
git -C "$base/server.git" symbolic-ref HEAD refs/heads/master
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
# --- Hook: sudo und deploy-commit werden durch Ersatzprogramme ersetzt ---
cat > "$base/fake-sudo" <<'SH'
#!/bin/sh
# Ersatz für sudo: prüft die Argumente "-n -u <Benutzer>" und führt den Befehl direkt aus
[ "$1" = -n ] && [ "$2" = -u ] || { echo "fake-sudo: unerwartete Argumente: $*" >&2; exit 1; }
echo "FAKE-SUDO user=$3"
shift 3
exec "$@"
SH
cat > "$base/fake-commit" <<'SH'
#!/bin/sh
echo "STUB-COMMIT arg=$1 n=$#"
if [ "$(git --git-dir="$STUB_REPO" show "$1:FAIL" 2>/dev/null)" ]; then echo "Fehler: Build kaputt"; exit 1; fi
SH
chmod +x "$base/fake-sudo" "$base/fake-commit"
export STUB_REPO="$base/server.git"
export DEPLOY_SUDO="$base/fake-sudo" DEPLOY_COMMIT_COMMAND="$base/fake-commit"
push() { (cd "$base/work" && git push -q origin "$1" 2>&1); }

out="$(push master)"
sha="$(git -C "$base/work" rev-parse master)"
check "Push auf master ruft den Einstiegsbefehl mit genau der Commit-ID auf" grep -q "STUB-COMMIT arg=$sha n=1" <<<"$out"
check "der Befehl läuft als Benutzer flauschiversum" grep -q "FAKE-SUDO user=flauschiversum" <<<"$out"
check "Pusher sieht das Ergebnis" grep -q "Deployment erfolgreich" <<<"$out"
(cd "$base/work" && git checkout -q -b dev && git commit -q --allow-empty -m zwei)
out="$(push dev)"
check "Push auf einen anderen Branch deployt nicht" bash -c "! grep -q STUB-COMMIT <<<'$out'"
(cd "$base/work" && git checkout -q master && echo x > FAIL && git add FAIL && git commit -q -m kaputt)
out="$(push master)"
check "Fehler im Build erreicht den Pusher" bash -c "grep -q 'Build kaputt' <<<'$out' && grep -q FEHLGESCHLAGEN <<<'$out'"
(cd "$base/work" && git rm -q FAIL && git commit -q -m heil)
out="$(DEPLOY_SUDO=false push master)"
check "fehlschlagendes sudo meldet die Einrichtung" bash -c "grep -q 'Einrichtung prüfen' <<<'$out'"

# Hauptbranch richtet sich nach HEAD des Repositorys: bei main deployt main, master nicht mehr
git -C "$base/server.git" symbolic-ref HEAD refs/heads/main
(cd "$base/work" && git checkout -q -b main)
out="$(push main)"
check "Push auf den Hauptbranch main deployt, wenn HEAD darauf zeigt" grep -q "STUB-COMMIT" <<<"$out"
(cd "$base/work" && git checkout -q master && git commit -q --allow-empty -m nochmal)
out="$(push master)"
check "master deployt nicht, wenn HEAD auf main zeigt" bash -c "! grep -q STUB-COMMIT <<<'$out'"
git -C "$base/server.git" symbolic-ref HEAD refs/heads/master

# --- deploy-commit mit Ersatz für cargo ---
dc="$src/scripts/server/deploy-commit"
home="$base/home"; mkdir -p "$home/bin"
cat > "$home/bin/cargo" <<'SH'
#!/bin/sh
echo "STUB-CARGO pwd=$PWD target=$CARGO_TARGET_DIR args=$*"
[ -f FAIL ] && { echo "Fehler: Build kaputt"; exit 1; }
sleep "${STUB_SLEEP:-0}"
exit 0
SH
chmod +x "$home/bin/cargo"
mkdir -p "$home/.cargo"; ln -s "$home/bin" "$home/.cargo/bin"
printf 'REPO=%s\nTARGET=ziel:/pfad/\n' "$base/server.git" > "$base/deploy.conf"
run_dc() { HOME="$home" FLAUSCHI_CONF="$base/deploy.conf" "$dc" "$@" 2>&1; }
good="$(git -C "$base/work" rev-parse master)"

out="$(run_dc "$good")"; rc=$?
check "deploy-commit: gültiger Commit wird gebaut und deployt" bash -c "[ $rc = 0 ] && grep -q 'STUB-CARGO' <<<'$out'"
check "deploy-commit: Ziel und dauerhafter Cache werden übergeben" grep -q "deploy --cache $home/.cache/flauschiversum/images --target ziel:/pfad/" <<<"$out"
check "deploy-commit: Compiler-Ergebnisse liegen im Cache" grep -q "target=$home/.cache/flauschiversum/target" <<<"$out"
dir="$(grep -o 'pwd=[^ ]*' <<<"$out" | head -1 | cut -d= -f2)"
check "deploy-commit: fester Arbeitsbaum im Zustandsverzeichnis" test "$dir" = "$home/.cache/flauschiversum/src"
m1="$(stat -c %Y.%N "$dir/datei" 2>/dev/null; stat -c %y "$dir/datei")"
(cd "$base/work" && git checkout -q master && echo neu > neu.txt && git add neu.txt && git commit -q -m neu && git push -q origin master)
sleep 1.1; run_dc "$(git -C "$base/work" rev-parse master)" >/dev/null
check "deploy-commit: unveränderte Dateien behalten ihre Zeit, neue kommen dazu" bash -c "[ \"\$(stat -c %y '$dir/datei')\" = \"\$(sed -n 2p <<<'$m1')\" ] && [ -f '$dir/neu.txt' ]"
check "deploy-commit: Protokoll wird geschrieben" grep -q STUB-CARGO "$home/.cache/flauschiversum/deploy.log"
for bad in "" abc "${good^^}" "${good}x" "${good:0:39}" "--help"; do
  out="$(run_dc "$bad")"; rc=$?
  check "deploy-commit weist ab: '${bad:0:12}'" bash -c "[ $rc != 0 ] && ! grep -q STUB-CARGO <<<'$out'"
done
out="$(run_dc "$good" zweites)"; rc=$?
check "deploy-commit weist zusätzliche Argumente ab" bash -c "[ $rc != 0 ] && ! grep -q STUB-CARGO <<<'$out'"
out="$(run_dc 0123456789012345678901234567890123456789)"; rc=$?
check "deploy-commit weist unbekannte Commits ab" bash -c "[ $rc != 0 ] && ! grep -q STUB-CARGO <<<'$out'"
(cd "$base/work" && echo x > FAIL && git add FAIL && git commit -q -m kaputt2 && git push -q origin master)
out="$(run_dc "$(git -C "$base/work" rev-parse master)")"; rc=$?
check "deploy-commit gibt den Fehler des Builds weiter" bash -c "[ $rc != 0 ] && grep -q 'Build kaputt' <<<'$out'"
# Sperre: zwei gleichzeitige Läufe laufen nacheinander
(STUB_SLEEP=1 run_dc "$good" > "$base/l1" &) ; sleep 0.3
start=$SECONDS; run_dc "$good" > "$base/l2"; elapsed=$((SECONDS - start))
check "deploy-commit: ein gleichzeitiger zweiter Lauf wartet auf den ersten" test "$elapsed" -ge 1
sleep 1
check "deploy-commit: beide Läufe sind durchgelaufen" bash -c "grep -q STUB-CARGO '$base/l1' && grep -q STUB-CARGO '$base/l2'"

echo
[ "$fails" -eq 0 ] && echo "Alle Prüfungen bestanden." || echo "$fails Prüfung(en) fehlgeschlagen."
exit "$((fails > 0))"
