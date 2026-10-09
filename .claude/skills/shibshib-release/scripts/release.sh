#!/usr/bin/env bash
# Build ShibShib rsm for macOS and the web, and publish the web build to GitHub Pages.
#
#   release.sh [--mac] [--web] [--deploy]     (no flags: all three)
#
# Run from anywhere; the repo is ~/Documents/ShibShib/rsm unless SHIBSHIB_ROOT is set.
set -euo pipefail
ROOT="${SHIBSHIB_ROOT:-$HOME/Documents/ShibShib/rsm}"
REPO_URL="https://github.com/7mz410/ShibShib-rsm.git"
# rustup comes from Homebrew and is keg-only; it provides the wasm32 target trunk needs.
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"

mac=0 web=0 deploy=0
[ $# -eq 0 ] && mac=1 web=1 deploy=1
for a in "$@"; do
  case "$a" in
    --mac) mac=1 ;;
    --web) web=1 ;;
    --deploy) deploy=1 ;;
    *) echo "unknown option $a" >&2; exit 2 ;;
  esac
done

cd "$ROOT"

if [ $mac = 1 ]; then
  cargo xtask bundle
  rm -rf "dist/ShibShib rsm.app"
  mv dist/VectorCraft.app "dist/ShibShib rsm.app"
  echo "mac: $ROOT/dist/ShibShib rsm.app"
fi

if [ $web = 1 ]; then
  (cd apps/vectorcraft-web && trunk build --release)
  grep -q "<title>ShibShib rsm" dist/web/index.html || { echo "web: title is not branded; run shibshib/rebrand.py" >&2; exit 1; }
  echo "web: $ROOT/dist/web"
fi

if [ $deploy = 1 ]; then
  [ -f dist/web/index.html ] || { echo "deploy: build the web first (--web)" >&2; exit 1; }
  site="$(mktemp -d)"
  trap 'rm -rf "$site"' EXIT
  cp -R dist/web/. "$site/"
  touch "$site/.nojekyll"
  (
    cd "$site"
    git init -q -b gh-pages
    git add -A
    git -c user.name="Hamza Abu Ayyash" -c user.email=hamza.abu3ayash@gmail.com commit -q -m "Deploy ShibShib rsm web ($(git -C "$ROOT" rev-parse --short HEAD))"
    git push -q -f "$REPO_URL" gh-pages
  )
  echo "deployed: https://7mz410.github.io/ShibShib-rsm/"
fi
