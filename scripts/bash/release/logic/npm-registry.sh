# Registry helpers for the npm publisher. Sourced by publish-npm.sh; relies on
# the caller's `fail`, `timeout`, and `poll` variables.

# dist-tags over OIDC need npm 11.21.0+ (11.x) or 12.2.0+ (12.x); a client that
# can publish but not retag would ship a release with stale channels.
dist_tag_capable() { # npm version
  major=${1%%.*}; minor=${1#*.}; minor=${minor%%.*}
  case "$major" in
    11) [ "$minor" -ge 21 ] ;;
    12) [ "$minor" -ge 2 ] ;;
    *) [ "$major" -ge 13 ] ;;
  esac
}

require_npm() {
  npm_version=$(npm --version)
  echo "npm $npm_version"
  dist_tag_capable "$npm_version" ||
    fail "npm 11.21.0+/12.2.0+ required for dist-tags, found $npm_version"
}

# A write is acknowledged before it is served: poll, and fail only on timeout.
wait_for_version() { # version
  elapsed=0
  until npm view "terminal-jarvis@$1" version >/dev/null 2>&1; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for terminal-jarvis@$1"
    sleep "$poll"; elapsed=$((elapsed + poll))
  done
  echo "confirmed: terminal-jarvis@$1 is readable"
}

wait_for_tag() { # tag version
  elapsed=0
  until npm dist-tag ls terminal-jarvis 2>/dev/null | grep -q "^$1: $2\$"; do
    [ "$elapsed" -lt "$timeout" ] || fail "timed out waiting for dist-tag $1 -> $2"
    sleep "$poll"; elapsed=$((elapsed + poll))
  done
  echo "confirmed: $1 -> $2"
}
