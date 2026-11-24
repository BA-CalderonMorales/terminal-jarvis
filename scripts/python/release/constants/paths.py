"""Repository paths the changelog generator reads and checks."""

from pathlib import Path

# constants/ -> release/ -> python/ -> scripts/ -> repository root
REPO_ROOT = Path(__file__).resolve().parents[4]
CHANGELOG = REPO_ROOT / "CHANGELOG.md"
