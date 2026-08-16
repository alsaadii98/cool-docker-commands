#!/usr/bin/env bash
# Regenerate the SVGs used by README.md and the website.
#
# Everything renders from `--demo`, dok's built-in example stack, so the docs
# show real rendering without leaking whatever runs on the machine that
# generated them. No daemon required.
set -euo pipefail

cd "$(dirname "$0")/.."
mkdir -p docs/img

BIN=${DOK_BIN:-target/release/dok}
[ -x "$BIN" ] || cargo build --release

# Fixed flags keep the SVGs reproducible regardless of the caller's terminal.
shot() {
  local out=$1 title=$2
  shift 2
  "$BIN" "$@" --demo --color=always --icons=unicode \
    | python3 scripts/ansi2svg.py --out "docs/img/$out" --title "$title"
}

shot ps.svg      "dok ps -a"            ps -a
shot images.svg  "dok images"           images
shot df.svg      "dok df -v"            df -v --top 3
shot tree.svg    "dok tree"             tree
shot logs.svg    "dok logs -n 8"        logs -n 8
shot themes.svg  "dok themes"           themes

shot inspect.svg "dok inspect api" inspect api

echo "screenshots written to docs/img/"
