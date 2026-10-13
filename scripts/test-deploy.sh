#!/usr/bin/env bash
# Prüft scripts/deploy.sh ohne Server: Das Ziel ist ein lokales Verzeichnis, `ssh` und `sftp` sind kleine lokale Ersatzprogramme.
# Aufruf: scripts/test-deploy.sh
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1
base="$(mktemp -d)"
trap 'rm -rf "$base"' EXIT
fails=0

ok() { echo "ok   $1"; }
fail() { echo "FEHLER $1"; fails=$((fails + 1)); }
check() { # Beschreibung, Befehl …
  local what="$1"; shift
  if "$@"; then ok "$what"; else fail "$what"; fi
}

# --- Ersatzprogramme für ssh, sftp und mv ----------------------------------------------------------------------------
tools="$base/tools"; mkdir -p "$tools"
real_mv="$(command -v mv)"
cat > "$tools/fake-ssh" <<'SH'
#!/bin/sh
# ignoriert Optionen (-o X) und Host, führt den Befehl lokal aus
while [ "${1#-}" != "$1" ]; do
  [ "$1" = -o ] && shift
  shift
done
shift
exec sh -c "$*"
SH
cat > "$tools/fake-sftp" <<'SH'
#!/bin/sh
# versteht `-b -` mit pwd, -mkdir "p" und put "a" "b"
while IFS= read -r line; do
  case "$line" in
    pwd) echo "Remote working directory: /" ;;
    -mkdir*) p="$(printf '%s' "$line" | sed 's/^-mkdir "\(.*\)"$/\1/')"; mkdir -p "$p" ;;
    put*) a="$(printf '%s' "$line" | sed 's/^put "\(.*\)" "\(.*\)"$/\1/')"; b="$(printf '%s' "$line" | sed 's/^put "\(.*\)" "\(.*\)"$/\2/')"; cp "$a" "$b" || exit 1 ;;
    *) echo "unbekannter Befehl: $line" >&2; exit 1 ;;
  esac
done
SH
cat > "$tools/mv" <<SH
#!/bin/sh
echo "\$@" >> "\$MV_LOG"
exec $real_mv "\$@"
SH
chmod +x "$tools"/*

# --- synthetische dist/ (Seite mit Manifest wie vom Generator) ---------------------------------------------------------------
make_dist() { # Verzeichnis
  local d="$1"
  rm -rf "$d"; mkdir -p "$d/2020/10/kurbis" "$d/2020/10/kurbis/200x" "$d/pagefind" "$d/images"
  printf '<html><head><link rel="canonical" href="https://flauschiversum.de/"></head><body>Start</body></html>' > "$d/index.html"
  printf '<html>Kürbis</html>' > "$d/2020/10/kurbis/index.html"
  printf 'jpg' > "$d/2020/10/kurbis/200x/kürbis thumb.jpg"
  printf 'idx' > "$d/pagefind/pagefind.js"
  printf 'css' > "$d/style.css"
  printf 'png' > "$d/images/Flauschi.png"
  printf '<rss/>' > "$d/rss.xml"
  write_manifest "$d"
}
write_manifest() {
  python3 - "$1" <<'PY'
import hashlib, json, os, sys
d = sys.argv[1]
m = {}
for root, _, files in os.walk(d):
    for f in files:
        if f == ".flauschiversum-manifest":
            continue
        p = os.path.join(root, f)
        data = open(p, "rb").read()
        m["/" + os.path.relpath(p, d)] = {"hash": hashlib.sha256(data).hexdigest(), "len": len(data)}
json.dump(m, open(os.path.join(d, ".flauschiversum-manifest"), "w"), indent=2)
PY
}

run() { # Ziel, Optionen … → $out, $rc
  local remote="$1"; shift
  out="$(DEPLOY_SSH="$tools/fake-ssh" DEPLOY_SFTP="$tools/fake-sftp" DEPLOY_TARGET="fake:$remote/" DEPLOY_DIST="$dist" DEPLOY_LOCK="$base/lock" \
    DEPLOY_METHOD="${METHOD:-auto}" MV_LOG="$base/mv.log" PATH="$tools:$PATH" scripts/deploy.sh --no-build "$@" 2>&1)" && rc=0 || rc=$?
}
tree() { (cd "$1" && find . -type f -printf '%p %s %m\n' | sort); }

dist="$base/dist"
methods="tar sftp"
command -v rsync >/dev/null 2>&1 && methods="$methods rsync"

for m in $methods; do
  echo "== Übertragungsweg: $m"
  METHOD="$m"
  remote="$base/server-$m"; mkdir -p "$remote/.well-known"
  echo "verify" > "$remote/google123.html"; echo "x" > "$remote/.well-known/acme"
  make_dist "$dist"

  : > "$base/mv.log"
  run "$remote"
  check "$m: erstes Deployment gelingt" test "$rc" = 0
  check "$m: Weg wird genannt" grep -q "Übertragungsweg: $m" <<<"$out"
  check "$m: alle Dateien sind da (auch Leerzeichen und Umlaut im Namen)" test -f "$remote/2020/10/kurbis/200x/kürbis thumb.jpg" -a -f "$remote/index.html" -a -f "$remote/pagefind/pagefind.js" -a -f "$remote/rss.xml"
  check "$m: Manifest liegt auf dem Server" test -f "$remote/.flauschiversum-manifest"
  check "$m: fremde Dateien bleiben" test -f "$remote/google123.html" -a -f "$remote/.well-known/acme"
  check "$m: Hinweis auf fremde Einträge" grep -q "nicht von dieser Seite" <<<"$out"
  check "$m: Rechte 644 und 755" test "$(stat -c %a "$remote/index.html")" = 644 -a "$(stat -c %a "$remote/pagefind")" = 755
  check "$m: kein Zwischenverzeichnis bleibt zurück" test -z "$(find "$remote" -maxdepth 1 -name '.deploy-*')"
  # Reihenfolge: erst Nicht-HTML, dann Seiten
  last_asset="$(grep -n 'style.css\|pagefind.js\|Flauschi.png\|kürbis thumb' "$base/mv.log" | tail -1 | cut -d: -f1)"
  first_page="$(grep -n '\.html\|\.xml' "$base/mv.log" | head -1 | cut -d: -f1)"
  check "$m: Medien und Index vor den Seiten eingebaut" test -n "$last_asset" -a -n "$first_page" -a "$last_asset" -lt "$first_page"
  last_move="$(tail -1 "$base/mv.log")"
  check "$m: Manifest wird zuletzt verschoben" grep -q "flauschiversum-manifest \.flauschiversum-manifest" <<<"$last_move"

  run "$remote"
  check "$m: zweiter Lauf ohne Änderung überträgt nichts" grep -q "^0 Dateien übertragen" <<<"$out"

  # Text geändert, Bild entfernt, neue Seite
  printf '<html>Kürbis neu</html>' > "$dist/2020/10/kurbis/index.html"
  rm "$dist/images/Flauschi.png"; rmdir "$dist/images"
  mkdir -p "$dist/neu"; printf '<html>neu</html>' > "$dist/neu/index.html"
  write_manifest "$dist"
  before="$(tree "$remote")"
  run "$remote" --dry-run
  check "$m: Trockenlauf ändert nichts" test "$rc" = 0 -a "$before" = "$(tree "$remote")"
  check "$m: Trockenlauf nennt Übertragung und Löschung" grep -q "Zu übertragen: 2 Dateien" <<<"$out"
  check "$m: Trockenlauf nennt zu löschende Datei" grep -q "images/Flauschi.png" <<<"$out"
  run "$remote"
  check "$m: Änderung gelingt" test "$rc" = 0
  check "$m: nur die geänderten Seiten wurden übertragen" grep -q "Zu übertragen: 2 Dateien" <<<"$out"
  check "$m: geänderter Text ist da" grep -q "Kürbis neu" "$remote/2020/10/kurbis/index.html"
  check "$m: entfernte Datei ist weg, leeres Verzeichnis auch" test ! -e "$remote/images/Flauschi.png" -a ! -d "$remote/images"
  check "$m: fremde Dateien bleiben nach der Löschung" test -f "$remote/google123.html" -a -f "$remote/.well-known/acme"
done
METHOD=tar

echo "== Schutz"
remote="$base/server-schutz"; mkdir -p "$remote"
make_dist "$dist"
sed -i 's#</body>#<div class="staging-bar">Staging: x</div></body>#' "$dist/index.html"
run "$remote"
check "Staging-Build wird abgewiesen" test "$rc" != 0
check "Staging-Build: Meldung nennt den Grund" grep -q "Staging-Build" <<<"$out"
check "Staging-Build: nichts übertragen" test -z "$(ls -A "$remote")"

make_dist "$dist"
sed -i 's#<head>#<head><meta name="robots" content="noindex">#' "$dist/index.html"
run "$remote"
check "noindex auf der Startseite wird abgewiesen" test "$rc" != 0

make_dist "$dist"
sed -i 's#https://flauschiversum.de/#https://staging.flauschiversum.de/#' "$dist/index.html"
run "$remote"
check "falsche kanonische URL wird abgewiesen" test "$rc" != 0

make_dist "$dist"; rm "$dist/.flauschiversum-manifest"
run "$remote"
check "fehlendes Manifest in dist/ wird abgewiesen" test "$rc" != 0

make_dist "$dist"
mkdir -p "$base/außen"; echo "bleibt" > "$base/außen/datei.txt"
printf '{"/../außen/datei.txt": {"hash": "x", "len": 1}, "/index.html": {"hash": "x", "len": 1}}' > "$remote/.flauschiversum-manifest"
run "$remote"
check "Pfad mit .. im Server-Manifest wird abgewiesen, nichts gelöscht" test "$rc" != 0 -a -f "$base/außen/datei.txt"

echo "== Wege und Fehler"
make_dist "$dist"; remote="$base/server-fehler"; mkdir -p "$remote"
METHOD=quatsch run "$remote"
check "unbekannter Weg wird abgewiesen" test "$rc" != 0
METHOD=tar
out="$(DEPLOY_SSH=/nicht/vorhanden DEPLOY_TARGET="fake:$remote/" DEPLOY_DIST="$dist" DEPLOY_LOCK="$base/lock" scripts/deploy.sh --no-build 2>&1)" && rc=0 || rc=$?
check "nicht erreichbarer Server: Abbruch mit Hinweis" test "$rc" != 0 && grep -q "nicht ohne Passwortabfrage erreichbar" <<<"$out"
# weder rsync noch tar noch sftp: `ssh` führt den Befehl in einer leeren PATH-Umgebung aus
cat > "$tools/ssh-ohne-werkzeuge" <<'SH'
#!/bin/sh
while [ "${1#-}" != "$1" ]; do [ "$1" = -o ] && shift; shift; done
shift
exec env PATH=/nonexistent /bin/sh -c "$*"
SH
chmod +x "$tools/ssh-ohne-werkzeuge"
out="$(DEPLOY_SSH="$tools/ssh-ohne-werkzeuge" DEPLOY_SFTP=/nicht/vorhanden DEPLOY_TARGET="fake:$remote/" DEPLOY_DIST="$dist" DEPLOY_LOCK="$base/lock" scripts/deploy.sh --no-build 2>&1)" && rc=0 || rc=$?
check "kein Übertragungsweg: Abbruch vor dem Hochladen" test "$rc" != 0 && grep -q "kein Übertragungsweg nutzbar" <<<"$out"
check "kein Übertragungsweg: nichts im Ziel" test -z "$(ls -A "$remote")"

echo "== Sperre"
( flock 9; sleep 4 ) 9>"$base/lock" &
holder=$!
sleep 1
run "$remote"
check "paralleles Deployment wird abgewiesen" test "$rc" != 0 && grep -q "läuft bereits" <<<"$out"
wait "$holder" 2>/dev/null

echo "== Git-Hook"
hookbase="$base/hook"; mkdir -p "$hookbase"
git init -q --bare "$hookbase/server.git"
out="$(scripts/install-git-hook.sh "$hookbase/server.git" 2>&1)" && rc=0 || rc=$?
check "Hook wird installiert" test "$rc" = 0 -a -x "$hookbase/server.git/hooks/post-receive"
out="$(scripts/install-git-hook.sh "$hookbase/server.git" 2>&1)" && rc=0 || rc=$?
check "zweite Installation desselben Hooks ist in Ordnung" test "$rc" = 0
git init -q --bare "$hookbase/anders.git"; printf '#!/bin/sh\n' > "$hookbase/anders.git/hooks/post-receive"
out="$(scripts/install-git-hook.sh "$hookbase/anders.git" 2>&1)" && rc=0 || rc=$?
check "ein anderer vorhandener Hook wird nicht überschrieben" test "$rc" != 0 -a "$(cat "$hookbase/anders.git/hooks/post-receive")" = "#!/bin/sh"
git init -q "$hookbase/work"
(
  cd "$hookbase/work" || exit 1
  git config user.email t@example.org; git config user.name Test
  mkdir scripts
  cat > scripts/deploy.sh <<'SH'
#!/bin/sh
echo "STUB-DEPLOY pwd=$PWD target=$CARGO_TARGET_DIR cache=$FLAUSCHI_CACHE_DIR"
if [ -f FAIL ]; then echo "Fehler: Build kaputt"; exit 1; fi
exit 0
SH
  chmod +x scripts/deploy.sh
  git add . && git commit -q -m eins && git branch -M master
  git remote add origin "$hookbase/server.git"
) >/dev/null 2>&1
export DEPLOY_STATE_DIR="$hookbase/state"
hook_push() { (cd "$hookbase/work" && git push -q origin "$1" 2>&1); }
out="$(hook_push master)"
check "Push auf master deployt" grep -q "STUB-DEPLOY" <<<"$out"
check "Pusher sieht das Ergebnis" grep -q "Deployment erfolgreich" <<<"$out"
check "Cache und Compiler-Ergebnisse liegen dauerhaft im Zustandsverzeichnis" grep -q "target=$hookbase/state/target cache=$hookbase/state/images" <<<"$out"
check "Arbeitsverzeichnis ist temporär und wird entfernt" bash -c "p=\$(grep -o 'pwd=[^ ]*' <<<\"$out\" | cut -d= -f2); [ -n \"\$p\" ] && [ ! -e \"\$p\" ]"
check "Protokoll wird geschrieben" grep -q "STUB-DEPLOY" "$hookbase/state/deploy.log"
(cd "$hookbase/work" && git checkout -q -b dev && git commit -q --allow-empty -m zwei)
out="$(hook_push dev)"
check "Push auf einen anderen Branch deployt nicht" test -z "$(grep STUB-DEPLOY <<<"$out")"
(cd "$hookbase/work" && git checkout -q master && touch FAIL && git add FAIL && git commit -q -m kaputt)
out="$(hook_push master)"
check "Fehler im Build erreicht den Pusher" grep -q "Build kaputt" <<<"$out" && grep -q "FEHLGESCHLAGEN" <<<"$out"
unset DEPLOY_STATE_DIR

echo
if [ "$fails" -eq 0 ]; then echo "Alle Prüfungen bestanden."; else echo "$fails Prüfung(en) fehlgeschlagen."; fi
exit "$((fails > 0))"
