#!/usr/bin/env bash
# Produktiv-Deployment: baut die Seite und legt sie per SSH auf den Server (Standard: flauschiversum:/httpdocs/).
#
# Aufruf: scripts/deploy.sh [--dry-run] [--no-build] [--help]
#   --dry-run   zeigt Übertragungsweg, zu übertragende und zu löschende Dateien; ändert auf dem Server nichts
#   --no-build  nimmt das vorhandene dist/ (muss ein Produktionsbuild sein)
#
# `flauschiversum` ist ein Host-Eintrag der SSH-Konfiguration (~/.ssh/config) einschließlich Schlüssel; das Skript
# kennt weder Benutzer noch Schlüssel noch Port. Der Server braucht kein rsync: Übertragen wird mit rsync, wenn es dort
# ein rsync gibt, sonst mit tar über SSH, sonst über SFTP.
#
# Umgebungsvariablen (alle optional):
#   DEPLOY_TARGET        Ziel `host:/pfad/` oder ein lokaler Pfad (Standard flauschiversum:/httpdocs/)
#   DEPLOY_METHOD        auto | tar | sftp | rsync (Standard auto)
#   FLAUSCHI_BASE_URL    Basis-URL des Builds (Standard https://flauschiversum.de)
#   FLAUSCHI_CACHE_DIR   Bild-Cache des Builds (Standard .cache)
#   DEPLOY_DIST          Ausgabeverzeichnis des Builds (Standard dist)
#   DEPLOY_LOCK          Sperrdatei gegen parallele Deployments (Standard .deploy.lock)
#   DEPLOY_SSH, DEPLOY_SFTP   ssh- und sftp-Befehl (Standard ssh und sftp; für Tests austauschbar)
set -euo pipefail

target="${DEPLOY_TARGET:-flauschiversum:/httpdocs/}"
method="${DEPLOY_METHOD:-auto}"
base_url="${FLAUSCHI_BASE_URL:-https://flauschiversum.de}"
cache_dir="${FLAUSCHI_CACHE_DIR:-.cache}"
dist="${DEPLOY_DIST:-dist}"
lock_file="${DEPLOY_LOCK:-.deploy.lock}"
ssh_cmd="${DEPLOY_SSH:-ssh}"
sftp_cmd="${DEPLOY_SFTP:-sftp}"
manifest=".flauschiversum-manifest"
dry_run=0
build=1

die() { echo "Fehler: $*" >&2; exit 1; }
info() { echo "$*"; }

usage() { sed -n '2,19p' "$0" | sed 's/^# \{0,1\}//'; }

while [ $# -gt 0 ]; do
  case "$1" in
    --dry-run) dry_run=1 ;;
    --no-build) build=0 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unbekannte Option $1 (siehe --help)" ;;
  esac
  shift
done

cd "$(dirname "$0")/.."
case "$dist" in /*) ;; *) dist="$PWD/$dist" ;; esac
case "$lock_file" in /*) ;; *) lock_file="$PWD/$lock_file" ;; esac

# --- Ziel zerlegen: host:/pfad oder lokaler Pfad -------------------------------------------------------------------
case "$target" in
  /*|./*|../*) mode=local; host=""; root="$target" ;;
  *:*) mode=remote; host="${target%%:*}"; root="${target#*:}" ;;
  *) mode=local; host=""; root="$target" ;;
esac
root="${root%/}"
[ -n "$root" ] || root="/"
[ "$root" != "/" ] || die "das Ziel darf nicht das Wurzelverzeichnis sein"
case "$root" in *[\'\"\\\$\`]*|*" "*) die "unzulässige Zeichen im Zielpfad: $root" ;; esac
case "$method" in auto|tar|sftp|rsync) ;; *) die "DEPLOY_METHOD muss auto, tar, sftp oder rsync sein (ist: $method)" ;; esac

# Befehl auf dem Ziel ausführen (Standardeingabe wird durchgereicht)
rexec() {
  if [ "$mode" = local ]; then sh -c "$1"; else $ssh_cmd -o BatchMode=yes "$host" "$1"; fi
}
# einfaches Anführen für sh
sq() { printf "'%s'" "${1//\'/\'\\\'\'}"; }

# --- Sperre --------------------------------------------------------------------------------------------------------
if command -v flock >/dev/null 2>&1; then
  exec 9>"$lock_file"
  flock -n 9 || die "es läuft bereits ein Deployment (Sperre $lock_file)"
fi

work="$(mktemp -d)"
tmp_name=".deploy-$$-$(date +%s)"
uploaded=0
cleanup() {
  rm -rf "$work"
  if [ "$uploaded" = 1 ]; then rexec "rm -rf $(sq "$root/$tmp_name")" >/dev/null 2>&1 || true; fi
}
trap cleanup EXIT

# --- Bauen ---------------------------------------------------------------------------------------------------------
if [ "$build" = 1 ]; then
  info "Prüfe und baue die Seite für $base_url …"
  cargo run --release --locked -- check --base-url "$base_url" --cache "$cache_dir"
  cargo run --release --locked -- build --base-url "$base_url" --cache "$cache_dir" --out "$dist"
fi

# --- dist/ prüfen: nur ein Produktionsbuild geht live ------------------------------------------------------------------
[ -f "$dist/index.html" ] || die "$dist/index.html fehlt (Seite nicht gebaut?)"
[ -f "$dist/$manifest" ] || die "$dist/$manifest fehlt (Seite nicht mit diesem Generator gebaut?)"
if grep -q 'class="staging-bar"' "$dist/index.html"; then die "dist/ ist ein Staging-Build (Staging-Leiste auf der Startseite)"; fi
if grep -q '<meta name="robots" content="noindex">' "$dist/index.html"; then die "dist/ ist nicht für Produktion gebaut (noindex auf der Startseite)"; fi
canonical="$(grep -o '<link rel="canonical" href="[^"]*"' "$dist/index.html" | head -1 | sed 's/.*href="//; s/"$//')"
case "$canonical" in
  "$base_url"|"$base_url"/*) ;;
  *) die "dist/ gehört nicht zu $base_url (kanonische URL der Startseite: ${canonical:-keine})" ;;
esac
if [ -x scripts/dist-size.sh ]; then scripts/dist-size.sh "$dist" || true; fi

# --- Ziel erreichbar? --------------------------------------------------------------------------------------------------
if [ "$mode" = remote ]; then
  rexec true || die "$host ist per SSH nicht ohne Passwortabfrage erreichbar (SSH-Eintrag '$host' mit Schlüssel in ~/.ssh/config prüfen)"
fi

# --- Übertragungsweg wählen ----------------------------------------------------------------------------------------------
have_remote() { rexec "command -v $1 >/dev/null 2>&1"; }
have_local() { command -v "$1" >/dev/null 2>&1; }
probe_sftp() { [ "$mode" = remote ] && have_local "$sftp_cmd" && echo pwd | $sftp_cmd -q -b - "$host" >/dev/null 2>&1; }
probe() {
  case "$1" in
    rsync) have_local rsync && have_remote rsync ;;
    tar) have_local tar && have_remote tar && have_remote gzip ;;
    sftp) probe_sftp ;;
  esac
}
if [ "$method" = auto ]; then
  for m in rsync tar sftp; do
    if probe "$m"; then method="$m"; break; fi
  done
  [ "$method" != auto ] || die "kein Übertragungsweg nutzbar: weder rsync noch tar (mit gzip) noch SFTP gefunden (geprüft auf $([ "$mode" = remote ] && echo "$host" || echo "dem Zielrechner"))"
else
  probe "$method" || die "der gewählte Übertragungsweg '$method' ist nicht nutzbar"
fi

# --- Was muss übertragen und gelöscht werden? (Abgleich der Manifeste) ---------------------------------------------------
rexec "cat $(sq "$root/$manifest") 2>/dev/null || true" > "$work/remote-manifest.json"
foreign=0
if [ ! -s "$work/remote-manifest.json" ]; then
  foreign="$(rexec "ls -A $(sq "$root") 2>/dev/null | wc -l" | tr -d ' ' || echo 0)"
fi
python3 - "$dist/$manifest" "$work/remote-manifest.json" "$work" <<'PY'
import json, os, sys
local_path, remote_path, work = sys.argv[1:4]
local = json.load(open(local_path))
try:
    remote = json.load(open(remote_path)) if os.path.getsize(remote_path) else {}
except ValueError:
    remote = {}
def rel(key):
    r = key.lstrip("/")
    parts = r.split("/")
    if not r or ".." in parts or r.startswith("/") or "\n" in r or "\0" in r or '"' in r or "'" in r:
        raise SystemExit("Fehler: unzulässiger Pfad im Manifest: %r" % key)
    return r
dist = os.path.dirname(local_path)
upload, size = [], 0
for key, entry in sorted(local.items()):
    r = rel(key)
    if not os.path.isfile(os.path.join(dist, r)):
        raise SystemExit("Fehler: laut Manifest vorhandene Datei fehlt in dist/: %s" % r)
    if remote.get(key, {}).get("hash") != entry.get("hash"):
        upload.append(r)
        size += entry.get("len", 0)
delete = []
for key in sorted(remote):
    if key not in local:
        delete.append(rel(key))
is_page = lambda p: p.endswith((".html", ".xml"))
for name, items in (("assets", [p for p in upload if not is_page(p)]), ("pages", [p for p in upload if is_page(p)]), ("delete", delete)):
    with open(os.path.join(work, name + ".list"), "w") as f:
        f.write("".join(p + "\n" for p in items))
with open(os.path.join(work, "stats"), "w") as f:
    f.write("%d %d %d\n" % (len(upload), size, len(delete)))
PY
read -r n_upload bytes n_delete < "$work/stats"
n_assets="$(wc -l < "$work/assets.list" | tr -d ' ')"
n_pages="$(wc -l < "$work/pages.list" | tr -d ' ')"
mb=$(( (bytes + 1048575) / 1048576 ))
info "Ziel: $target  ·  Übertragungsweg: $method"
info "Zu übertragen: $n_upload Dateien (rund $mb MB; $n_assets Medien/Skripte/Index, $n_pages Seiten), zu löschen: $n_delete"
if [ "$foreign" -gt 0 ]; then info "Hinweis: Im Ziel liegen $foreign Einträge, die nicht von dieser Seite stammen; sie bleiben unberührt (erstes Deployment überlagert nur)."; fi

if [ "$dry_run" = 1 ]; then
  info "Trockenlauf: Es wurde nichts übertragen oder gelöscht."
  if [ "$n_delete" -gt 0 ]; then echo "Zu löschen (erste 20):"; head -20 "$work/delete.list" | sed 's/^/  /'; fi
  if [ "$n_upload" -gt 0 ]; then echo "Zu übertragen (erste 20):"; cat "$work/assets.list" "$work/pages.list" | head -20 | sed 's/^/  /'; fi
  exit 0
fi

if [ "$n_upload" -eq 0 ] && [ "$n_delete" -eq 0 ]; then
  info "0 Dateien übertragen: Der Server ist auf dem Stand von dist/."
  exit 0
fi

# --- Übertragen: alles Neue in ein Zwischenverzeichnis ---------------------------------------------------------------------
cat "$work/assets.list" "$work/pages.list" > "$work/upload.list"
echo "$manifest" >> "$work/upload.list"
uploaded=1
rexec "mkdir -p $(sq "$root/$tmp_name")" || die "Zwischenverzeichnis im Ziel konnte nicht angelegt werden ($root)"
info "Übertrage …"
case "$method" in
  tar)
    tr '\n' '\0' < "$work/upload.list" > "$work/upload.null"
    tar -czf - -C "$dist" --null -T "$work/upload.null" | rexec "tar -xzf - -C $(sq "$root/$tmp_name")"
    ;;
  rsync)
    rsync -rl --from0 --files-from=<(tr '\n' '\0' < "$work/upload.list") -e "$ssh_cmd -o BatchMode=yes" "$dist/" "${host:+$host:}$root/$tmp_name/"
    ;;
  sftp)
    {
      awk -F/ '{ p = ""; for (i = 1; i < NF; i++) { p = (p == "" ? "" : p "/") $i; print p } }' "$work/upload.list" | sort -u | while IFS= read -r d; do
        printf -- '-mkdir "%s"\n' "$root/$tmp_name/$d"
      done
      while IFS= read -r f; do
        printf 'put "%s" "%s"\n' "$dist/$f" "$root/$tmp_name/$f"
      done < "$work/upload.list"
    } | $sftp_cmd -q -b - "$host"
    ;;
esac

# --- Einbauen: Rechte setzen, Seite für Seite an den Platz verschieben, Altes löschen, Manifest zuletzt ----------------------
info "Baue ein …"
{
  printf 'ROOT=%s\nTMP=%s\nMANIFEST=%s\n' "$(sq "$root")" "$(sq "$tmp_name")" "$(sq "$manifest")"
  cat <<'SH'
set -eu
cd "$ROOT"
find "$TMP" -type d -exec chmod 755 {} +
find "$TMP" -type f -exec chmod 644 {} +
move_all() { # $1: find-Ausdruck für Dateinamen (Positiv/Negativ)
  while IFS= read -r f; do
    rel="${f#"$TMP"/}"
    d="$(dirname "$rel")"
    [ "$d" = . ] || mkdir -p "$d"
    mv -f "$f" "$rel"
  done
}
# erst Medien, Stylesheet, Skripte, Suchindex ...
find "$TMP" -type f ! -name '*.html' ! -name '*.xml' ! -name "$MANIFEST" | move_all
# ... dann Seiten, Feed und Sitemap
find "$TMP" -type f \( -name '*.html' -o -name '*.xml' \) | move_all
# veraltete Dateien (nur solche, die ein früheres Deployment selbst abgelegt hat)
while IFS= read -r rel; do
  [ -n "$rel" ] || continue
  rm -f -- "$rel"
  d="$(dirname "$rel")"
  while [ "$d" != . ] && rmdir "$d" 2>/dev/null; do d="$(dirname "$d")"; done
done <<'DELETE_LIST'
SH
  cat "$work/delete.list"
  cat <<'SH'
DELETE_LIST
# zuletzt das neue Manifest
mv -f "$TMP/$MANIFEST" "$MANIFEST"
rm -rf "$TMP"
SH
} | rexec "sh -s"
uploaded=0

info "Fertig: $n_upload Dateien übertragen (rund $mb MB), $n_delete gelöscht, Weg: $method."
