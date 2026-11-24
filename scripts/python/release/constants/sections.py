"""Fixed mapping from conventional-commit types to changelog sections."""

BREAKING_SECTION = "Breaking"
REVERT_SECTION = "Fixed"
UNKNOWN_SECTION = "Changed"

# Rendered in this order; empty sections are omitted.
SECTION_ORDER = ("Breaking", "Added", "Changed", "Fixed")

TYPE_SECTIONS = {
    "feat": "Added",
    "fix": "Fixed",
    "perf": "Changed",
}

# Commit types that never reach user-facing output, unless marked breaking.
SILENT_TYPES = frozenset(
    {"docs", "test", "ci", "build", "chore", "style", "refactor"}
)

REVERT_TYPE = "revert"
