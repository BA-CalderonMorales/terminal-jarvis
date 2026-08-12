#!/usr/bin/env bash
# Capture a live Terminal Jarvis TUI frame and render it to a PNG.
# Used for the registry showcase (crates.io / npm) where .gif is unsupported;
# the README keeps the animated demo-tui.gif recorded with vhs from tui.tape.
#
# Usage (from the repository root):
#   scripts/demo/capture_frame.sh home3     # boot frame -> docs/demo-home3.png
#   scripts/demo/capture_frame.sh status3   # dashboard -> docs/demo-status3.png
set -euo pipefail

repo=$(cd "$(dirname "$0")/../.." && pwd)
name=${1:?usage: capture_frame.sh <frame-name>}
bin=${RELEASE_BIN:-"$repo/target/release/terminal-jarvis"}

[[ -x "$bin" ]] || { echo "missing binary; build first: cargo build --release" >&2; exit 1; }

font=${FONT:-/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf}
raw=$(mktemp --suffix=.raw)
png="$repo/docs/demo-$name.png"

tmux -f /dev/null new-session -d -x 96 -y 14 -s jvs-shot \
    "export PATH=$(dirname "$bin"):\$PATH; exec terminal-jarvis tui"
tmux set-option -t jvs-shot status off
sleep 3
tmux capture-pane -t jvs-shot -p -e > "$raw"
tmux kill-session -t jvs-shot 2>/dev/null || true

cd "$repo/scripts/demo"
go run . "$raw" "$font" "$png"
rm -f "$raw"
echo "wrote $png"
