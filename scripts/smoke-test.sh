#!/usr/bin/env bash
# Smoke-Test für eine veröffentlichte Staging-Seite.
# Aufruf: scripts/smoke-test.sh <Basis-URL ohne / am Ende> [Beitrags-URL-Pfad]
# Beispiel: scripts/smoke-test.sh https://staging.flauschiversum.de
set -u
base="${1:?Basis-URL fehlt, z.B. https://staging.flauschiversum.de}"
post="${2:-/2020/10/kurbis/}"
retries="${SMOKE_RETRIES:-1}"   # Pages braucht nach dem Deployment manchmal einen Moment
failed=0

fetch() { # URL → Statuscode (und Inhalt in $body)
  body=$(curl -sS -L --max-time 30 -w '\n%{http_code}' "$1" 2>&1) || { code=000; return; }
  code="${body##*$'\n'}"
  body="${body%$'\n'*}"
}

check_status() {
  local url="$base$1" attempt=1
  while :; do
    fetch "$url"
    [ "$code" = 200 ] && { echo "ok   $code $1"; return; }
    [ "$attempt" -ge "$retries" ] && break
    attempt=$((attempt + 1)); sleep 10
  done
  echo "FEHLER $code $url (erwartet 200)"
  failed=1
}

for path in / /suche/ /pagefind/pagefind.js /style.css "$post"; do
  check_status "$path"
done

fetch "$base/"
if [ "$code" = 200 ]; then
  grep -q '<meta name="robots" content="noindex">' <<<"$body" && echo "ok   noindex auf der Startseite" || { echo "FEHLER noindex fehlt auf der Startseite"; failed=1; }
  grep -q 'class="staging-bar"' <<<"$body" && echo "ok   Staging-Leiste auf der Startseite" || { echo "FEHLER Staging-Leiste fehlt auf der Startseite"; failed=1; }
fi

exit "$failed"
