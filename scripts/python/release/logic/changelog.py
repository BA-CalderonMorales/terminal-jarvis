"""Render a Keep-a-Changelog section and check CHANGELOG.md headings."""

import datetime
import re
import subprocess
from pathlib import Path
from typing import Dict, List

from constants.sections import SECTION_ORDER
from logic.group import group
from logic.parse import FIELD_SEP, RECORD_SEP, Commit, parse_log

VERSION = re.compile(r"^\d+\.\d+\.\d+$")
DATE = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def validate_version(version: str) -> None:
    if not VERSION.match(version):
        raise ValueError(f"version must look like X.Y.Z: {version}")


def validate_date(date: str) -> None:
    if not DATE.match(date):
        raise ValueError(f"date must be YYYY-MM-DD: {date}")
    try:
        datetime.date.fromisoformat(date)
    except ValueError as error:
        raise ValueError(f"date must be YYYY-MM-DD: {date}") from error


def commits_between(since: str, until: str, repo: Path) -> List[Commit]:
    fmt = f"--format=%H{FIELD_SEP}%P{FIELD_SEP}%s{RECORD_SEP}"
    result = subprocess.run(
        ["git", "log", fmt, f"{since}..{until}"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=True,
    )
    return parse_log(result.stdout)


def render(sections: Dict[str, List[str]], version: str, date: str) -> str:
    lines = [f"## [{version}] - {date}", ""]
    if not sections:
        lines += ["No user-facing changes.", ""]
    for name in SECTION_ORDER:
        if name in sections:
            lines += [f"### {name}", ""] + sections[name] + [""]
    return "\n".join(lines).rstrip("\n") + "\n"


def build_section(commits: List[Commit], version: str, date: str) -> str:
    validate_version(version)
    validate_date(date)
    return render(group(commits), version, date)


def has_heading(changelog: str, version: str) -> bool:
    pattern = rf"^## \[{re.escape(version)}\]"
    return re.search(pattern, changelog, re.MULTILINE) is not None
