#!/bin/sh
# Fake npm for the publish rehearsal. Models an eventually-consistent registry:
# a write is acknowledged immediately but served only after READS_TO_SERVE
# subsequent reads. Not a real client -- it implements only the subcommands the
# release publisher uses.
set -u
reg=${TERMINAL_JARVIS_REHEARSAL_REGISTRY:?registry dir not set}
serves_after=${TERMINAL_JARVIS_REHEARSAL_READS:-3}
mkdir -p "$reg/tags"
reads=0
[ -f "$reg/reads" ] && reads=$(cat "$reg/reads")
bump() { reads=$((reads + 1)); printf '%s' "$reads" >"$reg/reads"; }

cmd=${1:-}
[ "$#" -gt 0 ] && shift
case "$cmd" in
  --version) cat "$reg/npm-version" 2>/dev/null || echo 11.21.0 ;;
  pack) exit 0 ;;
  view)
    bump
    want=${1#terminal-jarvis@}
    if [ -f "$reg/version" ] && [ "$reads" -ge "$(cat "$reg/version_after")" ] &&
      [ "$(cat "$reg/version")" = "$want" ]; then
      cat "$reg/version"
      exit 0
    fi
    exit 1
    ;;
  publish)
    v=$(sed -n 's/.*"version": *"\([^"]*\)".*/\1/p' package.json | head -n 1)
    printf '%s' "$v" >"$reg/version"
    printf '%s' "$((reads + serves_after))" >"$reg/version_after"
    printf '%s' "$v" >"$reg/tags/latest"
    printf '%s' "$((reads + serves_after))" >"$reg/tags/latest.after"
    ;;
  dist-tag)
    case "${1:-}" in
      add)
        printf '%s' "${2#terminal-jarvis@}" >"$reg/tags/$3"
        printf '%s' "$((reads + serves_after))" >"$reg/tags/$3.after"
        ;;
      ls)
        bump
        for f in "$reg"/tags/*; do
          [ -e "$f" ] || continue
          case "$f" in *.after) continue ;; esac
          [ "$reads" -ge "$(cat "$f.after" 2>/dev/null || echo 0)" ] &&
            printf '%s: %s\n' "$(basename "$f")" "$(cat "$f")"
        done
        ;;
    esac
    ;;
esac
exit 0
