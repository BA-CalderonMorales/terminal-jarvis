"""Parse git log records and conventional-commit subjects."""

import re
from dataclasses import dataclass
from typing import List, Optional, Tuple

from constants.sections import REVERT_TYPE

RECORD_SEP = "\x1e"
FIELD_SEP = "\x1f"

CONVENTIONAL = re.compile(
    r"^(?P<type>[A-Za-z]+)(?:\((?P<scope>[^()]+)\))?(?P<bang>!)?: (?P<subject>.+)$"
)
PR_SUFFIX = re.compile(r"\s*\(#(?P<pr>\d+)\)$")
QUOTED_REVERT = re.compile(r'^Revert "(?P<inner>.+)"$')


@dataclass(frozen=True)
class Commit:
    sha: str
    parents: int
    subject: str


@dataclass(frozen=True)
class Entry:
    type: str
    scope: Optional[str]
    breaking: bool
    text: str
    pr: Optional[int]
    revert_of: Optional[str]
    full: str

    @property
    def is_revert(self) -> bool:
        return self.revert_of is not None


def split_pr(subject: str) -> Tuple[str, Optional[int]]:
    """Return (subject without the squash suffix, PR number or None)."""
    match = PR_SUFFIX.search(subject)
    if match is None:
        return subject.strip(), None
    return subject[: match.start()].strip(), int(match.group("pr"))


def parse_subject(raw: str) -> Entry:
    text, pr = split_pr(raw)
    quoted = QUOTED_REVERT.match(text)
    if quoted:
        inner, _ = split_pr(quoted.group("inner"))
        return Entry(REVERT_TYPE, None, False, text, pr, inner, text)
    match = CONVENTIONAL.match(text)
    if match is None:
        return Entry("", None, False, text, pr, None, text)
    kind = match.group("type").lower()
    subject, _ = split_pr(match.group("subject"))
    if kind == REVERT_TYPE:
        return Entry(kind, None, False, subject, pr, subject, text)
    return Entry(
        kind,
        match.group("scope"),
        match.group("bang") == "!",
        subject,
        pr,
        None,
        text,
    )


def parse_log(output: str) -> List[Commit]:
    """Parse `git log --format=%H%x1f%P%x1f%s%x1e` output into commits."""
    commits = []
    for record in output.split(RECORD_SEP):
        fields = record.strip("\n").split(FIELD_SEP)
        if len(fields) != 3 or not fields[0]:
            continue
        sha, parents, subject = fields
        commits.append(Commit(sha, len(parents.split()), subject))
    return commits
