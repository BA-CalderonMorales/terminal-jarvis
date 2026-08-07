#!/usr/bin/env python3
import argparse
import csv
from pathlib import Path

platforms = {"linux-x64-gnu", "linux-arm64-gnu", "macos-x64", "macos-arm64", "win32-x64"}
channels = {"cargo", "npm-global", "npx", "homebrew", "direct-archive", "direct-executable"}
parser = argparse.ArgumentParser()
parser.add_argument("directory", type=Path)
args = parser.parse_args()
rows = []
for path in sorted(args.directory.rglob("delivery.tsv")):
    with path.open(newline="") as stream:
        rows.extend(csv.DictReader(stream, delimiter="\t"))
keys = {(row["platform"], row["channel"]) for row in rows}
expected = {(platform, channel) for platform in platforms for channel in channels}
if keys != expected or len(rows) != 30:
    raise SystemExit(f"delivery matrix mismatch: missing/extras {sorted(keys ^ expected)}")
unsupported = [row for row in rows if row["result"] == "unsupported"]
passed = [row for row in rows if row["result"] == "pass"]
if len(passed) != 29 or [(row["platform"], row["channel"]) for row in unsupported] != [("win32-x64", "homebrew")]:
    raise SystemExit("delivery matrix must contain 29 passes and Windows/Homebrew unsupported")
refs = {row["ref"] for row in rows}
if len(refs) != 1 or len(refs.pop()) != 40:
    raise SystemExit("delivery matrix is not bound to one full ref")
print("delivery matrix: ok (29 pass, 1 unsupported)")
