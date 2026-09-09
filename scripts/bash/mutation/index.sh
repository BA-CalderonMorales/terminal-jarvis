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
  echo "Partition a domain with --shard 0/4 (zero-based index/total)."
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
    file_list=$(files)
    [ -n "$file_list" ] || {
      echo "mutation: unknown shard '$domain'" >&2
      usage >&2
      exit 2
    }
    cd "$root"
    base=""
    for path in $file_list; do
      base="$base --file $path"
    done
    # Bound per-runner parallelism; CI partitions the same mutant set with
    # native --shard arguments passed through below. No tests are skipped.
    RUST_TEST_THREADS=1 cargo mutants --config mutants.toml \
      --minimum-test-timeout 30 --jobs 1 --no-shuffle $base "$@"
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
