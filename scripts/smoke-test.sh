#!/usr/bin/env bash
# Smoke-Test für eine veröffentlichte Seite (Staging oder Produktion).
# Aufruf: scripts/smoke-test.sh <Basis-URL ohne / am Ende> [Beitrags-URL-Pfad]
# Beispiel: scripts/smoke-test.sh https://staging.flauschiversum.de
#           SMOKE_MODE=production scripts/smoke-test.sh https://flauschiversum.de
# Umgebungsvariablen:
#   SMOKE_MODE       staging (Standard: noindex und Staging-Leiste müssen da sein) oder production (dürfen nicht da sein,
#                    zusätzlich robots.txt und sitemap.xml, kanonische URL unter SMOKE_CANONICAL)
#   SMOKE_CANONICAL  erwartete Basis der kanonischen URL im Produktionsmodus (Standard: die Basis-URL)
#   SMOKE_RETRIES    Versuche je URL, 10 s Pause (Standard 1)
set -u
base="${1:?Basis-URL fehlt, z.B. https://staging.flauschiversum.de}"
post="${2:-/2020/10/kurbis/}"
retries="${SMOKE_RETRIES:-1}"   # Pages braucht nach dem Deployment manchmal einen Moment
mode="${SMOKE_MODE:-staging}"
canonical_base="${SMOKE_CANONICAL:-$base}"
failed=0

case "$mode" in staging|production) ;; *) echo "SMOKE_MODE muss staging oder production sein (ist: $mode)"; exit 2 ;; esac

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

expect() { # Beschreibung, Ergebnis (0 = in Ordnung)
  if [ "$2" = 0 ]; then echo "ok   $1"; else echo "FEHLER $1"; failed=1; fi
}

paths=(/ /suche/ /pagefind/pagefind.js /style.css "$post")
[ "$mode" = production ] && paths+=(/robots.txt /sitemap.xml)
for path in "${paths[@]}"; do
  check_status "$path"
done

fetch "$base/"
if [ "$code" = 200 ]; then
  has_noindex=1; grep -q '<meta name="robots" content="noindex">' <<<"$body" && has_noindex=0
  has_bar=1; grep -q 'class="staging-bar"' <<<"$body" && has_bar=0
  if [ "$mode" = staging ]; then
    expect "noindex auf der Startseite" "$has_noindex"
    expect "Staging-Leiste auf der Startseite" "$has_bar"
  else
    expect "kein noindex auf der Produktionsseite" "$((has_noindex == 0 ? 1 : 0))"
    expect "keine Staging-Leiste auf der Produktionsseite" "$((has_bar == 0 ? 1 : 0))"
    canonical="$(grep -o '<link rel="canonical" href="[^"]*"' <<<"$body" | head -1 | sed 's/.*href="//; s/"$//')"
    case "$canonical" in
      "$canonical_base"|"$canonical_base"/*) expect "kanonische URL unter $canonical_base" 0 ;;
      *) expect "kanonische URL unter $canonical_base (ist: ${canonical:-keine})" 1 ;;
    esac
  fi
fi

if [ "$mode" = production ]; then
  fetch "$base/robots.txt"
  if [ "$code" = 200 ]; then
    disallow_all=1; grep -qx 'Disallow: /' <<<"$body" && disallow_all=0
    expect "robots.txt sperrt nicht alles" "$((disallow_all == 0 ? 1 : 0))"
  fi
fi

exit "$failed"
