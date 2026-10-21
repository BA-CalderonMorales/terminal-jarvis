#!/usr/bin/env sh
# Refuse a workflow job that invokes a repository script without checking out
# the repository first.
#
# The v0.1.21 publish-npm job downloaded an artifact, ran
# `scripts/bash/release/index.sh publish-npm --check-toolchain`, and died with
# exit 127 ("No such file or directory") after crates.io, the GitHub release,
# and the Homebrew tap had already shipped. A job that runs `scripts/` needs
# the tree, and no other gate reads the workflows.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../" && pwd)
cd "$root"

for wf in .github/workflows/*.yml .github/workflows/*.yaml; do
  [ -f "$wf" ] || continue
  python3 - "$wf" <<'PY'
import re
import sys

path = sys.argv[1]
lines = open(path, encoding="utf-8").read().splitlines()
in_jobs = False
current = None
block = []
jobs = []


def flush():
    if current:
        jobs.append((current, block))


for line in lines:
    if line.strip() == "jobs:" and not line.startswith(" "):
        in_jobs = True
        continue
    if not in_jobs:
        continue
    if line and not line.startswith(" "):
        break
    if re.match(r"^  [A-Za-z0-9_.-]+:\s*$", line):
        flush()
        current = line.strip()[:-1]
        block = []
    elif current is not None:
        block.append(line)
flush()

bad = []
for name, block in jobs:
    text = "\n".join(block)
    calls = [
        line.strip()
        for line in block
        if "scripts/" in line and not line.strip().startswith("#")
    ]
    if calls and "actions/checkout" not in text:
        bad.append((name, calls))

if bad:
    for name, calls in bad:
        print(
            f"workflow-checkout: {path}:{name} runs a repository script "
            "without actions/checkout",
            file=sys.stderr,
        )
        for call in calls:
            print(f"    {call}", file=sys.stderr)
    sys.exit(1)
PY
done

echo "workflow-checkout: ok"
