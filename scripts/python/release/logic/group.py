"""Group parsed commits into Keep-a-Changelog sections."""

from typing import Dict, List, Optional

from constants.sections import (
    BREAKING_SECTION,
    REVERT_SECTION,
    SECTION_ORDER,
    SILENT_TYPES,
    TYPE_SECTIONS,
    UNKNOWN_SECTION,
)
from logic.parse import Commit, Entry, parse_subject


def is_merge(commit: Commit) -> bool:
    return commit.parents > 1 or commit.subject.startswith("Merge ")


def cancel_reverts(entries: List[Entry]) -> List[Entry]:
    """Drop each revert together with the commit it undoes.

    Entries are newest first, so a revert only cancels an older entry (a higher
    index), and the nearest such match wins. A revert of a revert cancels the
    revert it undoes, which restores the original. A revert with no match stays.
    """
    cancelled = set()
    for i, revert in enumerate(entries):
        if not revert.is_revert or i in cancelled:
            continue
        for j in range(i + 1, len(entries)):
            if j not in cancelled and entries[j].full == revert.revert_of:
                cancelled.update({i, j})
                break
    return [e for k, e in enumerate(entries) if k not in cancelled]


def dedupe_by_pr(entries: List[Entry]) -> List[Entry]:
    seen = set()
    kept = []
    for entry in entries:
        if entry.pr is not None:
            if entry.pr in seen:
                continue
            seen.add(entry.pr)
        kept.append(entry)
    return kept


def section_for(entry: Entry) -> Optional[str]:
    if entry.breaking:
        return BREAKING_SECTION
    if entry.is_revert:
        return REVERT_SECTION
    if entry.type in SILENT_TYPES:
        return None
    return TYPE_SECTIONS.get(entry.type, UNKNOWN_SECTION)


def bullet(entry: Entry) -> str:
    text = f'Revert "{entry.revert_of}"' if entry.is_revert else entry.text
    if entry.scope:
        text = f"**{entry.scope}**: {text}"
    if entry.pr is not None:
        text = f"{text} (#{entry.pr})"
    return f"- {text}"


def group(commits: List[Commit]) -> Dict[str, List[str]]:
    """Return section name -> bullet lines, newest PR first."""
    entries = [
        parse_subject(c.subject) for c in commits if not is_merge(c)
    ]
    entries = dedupe_by_pr(cancel_reverts(entries))
    entries.sort(key=lambda e: (e.pr is None, -(e.pr or 0)))
    sections: Dict[str, List[str]] = {name: [] for name in SECTION_ORDER}
    for entry in entries:
        name = section_for(entry)
        if name is not None:
            sections[name].append(bullet(entry))
    return {name: lines for name, lines in sections.items() if lines}
