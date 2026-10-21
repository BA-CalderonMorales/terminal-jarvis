#!/usr/bin/env bash
# Record the README demo on a real Omarchy desktop.
#
# drive.py drives the session over a pty the demo window owns, not synthetic key
# events: wtype stopped reaching the window partway through an earlier take, and
# a recording that silently stops receiving input is worse than no recording.
# The tui starts only after the window is placed -- one that boots while its
# window is still tiled measures the tiled geometry and composes frames the
# placed window then clips.
#
# usage: record-omarchy.sh <monitor> [workspace-to-restore]     e.g. DP-14 9
# It prints the crop and trim to feed the gif encoder (see docs/demo.md). Every
# window dispatcher acts on the ACTIVE window, so none runs unless the demo
# window holds focus: a click elsewhere must never resize someone else's window.
set -uo pipefail

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repo=$(CDPATH= cd -- "$here/../.." && pwd)
out=${TJ_DEMO_OUT:-$(mktemp -d)}
mon=${1:?monitor name, e.g. DP-14}
restore_ws=${2:-9}
win_w=1000; win_h=620; win_x=460; win_y=160; margin=250; demo_ws=9
font="JetBrainsMono Nerd Font:size=17"
class=tj-demo; go=$out/go; done=$out/done

die() { echo "record-omarchy: $*" >&2; exit 1; }
# never leave a recording running if a step fails mid-demo
trap 'omarchy screenrecord --stop-recording >/dev/null 2>&1 || true' EXIT
d() { [ "$(hyprctl dispatch "$1" 2>&1)" = ok ] || echo "  (dispatch failed: $1)"; }
active() { hyprctl activewindow -j | jq -r '.class // "none"'; }
demo() { hyprctl clients -j | jq -r ".[] | select(.class == \"$class\")"; }
guarded() {
  [ "$(active)" = "$class" ] || die "active window is $(active), not $class; refusing: $1"
  d "$1"
}

rm -f "$go" "$done"
export TJ_DEMO_GO="$go" TJ_DEMO_DONE="$done"

# no toast lands mid-recording
dnd_was=$(omarchy toggle enabled notification-silencing >/dev/null 2>&1 && echo on || echo off)
omarchy toggle notification silencing on >/dev/null 2>&1

# focus the demo monitor on an empty workspace
read -r user_mon mon_x mon_y <<<"$(hyprctl monitors -j |
  jq -r --arg m "$mon" '[.[] | select(.focused) | .name] + ([.[] | select(.name == $m) | .x, .y]) | @tsv')"
d "hl.dsp.focus({ monitor = \"$mon\" })"; d "hl.dsp.focus({ workspace = \"$demo_ws\" })"
sleep 1.5

# launch the window; the driver waits for the go file before starting the tui
setsid -f foot -a "$class" --font "$font" --window-size-pixels="${win_w}x${win_h}" -- \
  python3 "$here/drive.py" "$repo" "$repo/target/release/terminal-jarvis" >/dev/null 2>&1
for _ in $(seq 1 80); do [ -n "$(demo)" ] && break; sleep 0.1; done
[ -n "$(demo)" ] || die "demo window never appeared"
sleep 2

# place it; a stray click can fullscreen it, which refuses resize and move
guarded 'hl.dsp.window.fullscreen({ mode = "fullscreen", action = "unset" })'
guarded 'hl.dsp.window.fullscreen({ mode = "maximized", action = "unset" })'; sleep 0.4
guarded 'hl.dsp.window.float({ action = "on" })'; sleep 0.6
guarded "hl.dsp.window.resize({ x = $win_w, y = $win_h })"; sleep 0.6
# coordinates are GLOBAL: offset by the monitor origin, or the window lands on
# whichever monitor owns those pixels
guarded "hl.dsp.window.move({ x = $((mon_x + win_x)), y = $((mon_y + win_y)) })"
sleep 0.8
size=$(demo | jq -r '"\(.size[0])x\(.size[1])"')
[ "$size" = "${win_w}x${win_h}" ] || die "window is $size, expected ${win_w}x${win_h}"
d "hl.dsp.cursor.move({ x = 1890, y = 1060 })"

# screenrecord captures the focused monitor, so re-focus it and check
d "hl.dsp.focus({ monitor = \"$mon\" })"; d "hl.dsp.focus({ workspace = \"$demo_ws\" })"
sleep 0.8
focused=$(hyprctl monitors -j | jq -r '.[] | select(.focused) | .name')
[ "$focused" = "$mon" ] || die "focused monitor is $focused, not $mon"
omarchy screenrecord --fullscreen
pgrep -f "^gpu-screen-recorder" >/dev/null || die "recording did not start"
t_start=$(date +%s.%N)
sleep 2.5

read -r boot flow <<<"$(python3 "$here/drive.py" --timing | awk '{print $2}' | paste -sd' ')"
touch "$go"
t_go=$(date +%s.%N)
for _ in $(seq 1 240); do [ -f "$done" ] && break; sleep 0.5; done
[ -f "$done" ] || die "the driver never finished"
sleep 1
omarchy screenrecord --stop-recording | tee "$out/recording.txt"

python3 -c "import sys; s,g,b,f=(float(v) for v in sys.argv[1:5]); print(f'SS {g-s+b-0.5:.2f}\nT {f+1.0:.2f}')" \
  "$t_start" "$t_go" "$boot" "$flow" | tee "$out/trim.txt"
echo "crop: $((win_w + 2 * margin))x$((win_y + win_h + 160))+$((win_x - margin))+0"
echo "out: $out"

# close the window and restore the desktop
kill "$(demo | jq -r '.pid' | head -1)" 2>/dev/null
sleep 1
d "hl.dsp.focus({ monitor = \"$mon\" })"; d "hl.dsp.focus({ workspace = \"$restore_ws\" })"
d "hl.dsp.focus({ monitor = \"$user_mon\" })"
[ "$dnd_was" = off ] && omarchy toggle notification silencing off >/dev/null 2>&1
echo "restored: ws $restore_ws on $mon, focus $user_mon"
