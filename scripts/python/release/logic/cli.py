"""Command line for the changelog generator."""

import argparse
import sys
from pathlib import Path
from typing import List, Optional

from constants.paths import CHANGELOG, REPO_ROOT
from logic.changelog import (
    build_section,
    commits_between,
    has_heading,
    validate_version,
)

USAGE = (
    "usage: scripts/python/release/index.py changelog "
    "--since REF --version X.Y.Z --date YYYY-MM-DD [--until REF]\n"
    "       scripts/python/release/index.py changelog --check --version X.Y.Z"
)


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="index.py changelog", usage=USAGE)
    p.add_argument("--since", help="older ref; commits in since..until")
    p.add_argument("--until", default="HEAD", help="newer ref (default HEAD)")
    p.add_argument("--version", required=True, help="X.Y.Z")
    p.add_argument("--date", help="release date YYYY-MM-DD (required to render)")
    p.add_argument("--check", action="store_true", help="verify the heading exists")
    p.add_argument("--file", type=Path, default=CHANGELOG, help=argparse.SUPPRESS)
    return p


def check(version: str, path: Path) -> int:
    validate_version(version)
    if has_heading(path.read_text(encoding="utf-8"), version):
        return 0
    print(f"changelog: {path.name} has no ## [{version}] heading", file=sys.stderr)
    return 1


def run(argv: Optional[List[str]] = None) -> int:
    args = parser().parse_args(argv)
    try:
        if args.check:
            return check(args.version, args.file)
        if not args.since or not args.date:
            parser().error("--since and --date are required to render a section")
        commits = commits_between(args.since, args.until, REPO_ROOT)
        sys.stdout.write(build_section(commits, args.version, args.date))
        return 0
    except ValueError as error:
        print(f"changelog: {error}", file=sys.stderr)
        return 2
