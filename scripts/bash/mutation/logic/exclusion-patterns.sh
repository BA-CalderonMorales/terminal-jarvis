#!/usr/bin/env sh
# Decode the TOML exclusion strings exactly as the ledger uses them.
set -eu

awk '
  /^[[:space:]]*exclude_re[[:space:]]*=/ { collecting = 1 }
  collecting {
    line = $0
    sub(/#.*/, "", line)
    while (match(line, /"([^"\\]|\\.)*"/)) {
      raw = substr(line, RSTART, RLENGTH)
      line = substr(line, RSTART + RLENGTH)
      body = substr(raw, 2, length(raw) - 2)
      out = ""
      index_ = 1
      while (index_ <= length(body)) {
        character = substr(body, index_, 1)
        if (character != "\\") {
          out = out character
          index_++
          continue
        }
        escape = substr(body, index_ + 1, 1)
        if (escape == "\\") {
          out = out "\\"
        } else if (escape == "\"") {
          out = out "\""
        } else {
          printf "mutation exclusions: unsupported escape \\%s in %s\n", escape, raw > "/dev/stderr"
          exit 3
        }
        index_ = index_ + 2
      }
      print out
    }
    if (/^[[:space:]]*\]/) { collecting = 0 }
  }
' "$1"
