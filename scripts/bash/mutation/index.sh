#!/usr/bin/env sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../.." && pwd)
cmd=${1:-}
[ $# -gt 0 ] && shift
domain=${1:-}
[ $# -gt 0 ] && shift

files() {
  awk -v want="$domain" '!/^#/ && $1 == want { gsub(/^[^ ]+ /, ""); print }' \
    "$here/constants/shards.txt"
}

usage() {
  echo "usage: scripts/bash/mutation/index.sh <list|run DOMAIN [args...]>"
  echo "DOMAIN: one of the shard keys in constants/shards.txt"
}

case "$cmd" in
  list)
    awk '!/^#/ { print $1 }' "$here/constants/shards.txt" | sort -u
    ;;
  run)
    [ -n "$domain" ] || {
      usage >&2
      exit 2
    }
    shard=$(files)
    [ -n "$shard" ] || {
      echo "mutation: unknown shard '$domain'" >&2
      usage >&2
      exit 2
    }
    cd "$root"
    base=""
    for path in $shard; do
      base="$base --file $path"
    done
    cargo mutants --config mutants.toml --minimum-test-timeout 30 \
      --jobs 4 --no-shuffle $base "$@"
    ;;
  -h | --help)
    usage
    ;;
  "")
    usage >&2
    exit 2
    ;;
  *)
    usage >&2
    exit 2
    ;;
esac
