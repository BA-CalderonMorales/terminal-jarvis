"""Section rendering, date argument and --check heading tests."""

import contextlib
import io
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from logic.changelog import (  # noqa: E402
    build_section,
    commits_between,
    has_heading,
    validate_date,
)
from logic.cli import check  # noqa: E402
from logic.parse import Commit  # noqa: E402


class SectionTests(unittest.TestCase):
    def test_heading_uses_passed_date_not_clock(self):
        commits = [Commit("a", 1, "feat: x (#1)")]
        first = build_section(commits, "1.2.3", "2001-02-03")
        second = build_section(commits, "1.2.3", "2030-12-31")
        self.assertTrue(first.startswith("## [1.2.3] - 2001-02-03\n"))
        self.assertTrue(second.startswith("## [1.2.3] - 2030-12-31\n"))

    def test_empty_range_renders_placeholder(self):
        text = build_section([], "1.2.3", "2026-01-01")
        self.assertEqual(text, "## [1.2.3] - 2026-01-01\n\nNo user-facing changes.\n")

    def test_date_must_be_exact_iso_shape(self):
        validate_date("2026-11-23")
        for bad in ("20261123", "2026-1-3"):
            with self.assertRaises(ValueError):
                validate_date(bad)

    def test_bad_date_and_version_are_rejected(self):
        with self.assertRaises(ValueError):
            build_section([], "1.2.3", "2026-02-30")
        with self.assertRaises(ValueError):
            build_section([], "v1.2.3", "2026-01-01")

    def test_section_layout_matches_keep_a_changelog(self):
        commits = [
            Commit("a", 1, "fix: bug (#2)"),
            Commit("b", 1, "feat!: cut (#3)"),
            Commit("c", 1, "feat: add (#4)"),
        ]
        text = build_section(commits, "1.0.0", "2026-01-01")
        self.assertEqual(
            text,
            "## [1.0.0] - 2026-01-01\n\n### Breaking\n\n- cut (#3)\n\n"
            "### Added\n\n- add (#4)\n\n### Fixed\n\n- bug (#2)\n",
        )


class HeadingCheckTests(unittest.TestCase):
    def test_has_heading_matches_exact_version_only(self):
        text = "# Changelog\n\n## [0.1.25] - 2026-11-22\n\n## [0.1.2]\n"
        self.assertTrue(has_heading(text, "0.1.25"))
        self.assertFalse(has_heading(text, "0.1.2X"))
        self.assertFalse(has_heading(text, "0.1.26"))

    def test_check_exit_codes(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "CHANGELOG.md"
            path.write_text("## [0.1.25] - 2026-11-22\n", encoding="utf-8")
            self.assertEqual(check("0.1.25", path), 0)
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(check("0.1.26", path), 1)


class GitRangeTests(unittest.TestCase):
    def test_commits_between_reads_only_the_range(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp)
            env = {
                "GIT_AUTHOR_NAME": "t", "GIT_AUTHOR_EMAIL": "t@example.invalid",
                "GIT_COMMITTER_NAME": "t", "GIT_COMMITTER_EMAIL": "t@example.invalid",
                "PATH": "/usr/bin:/bin",
            }

            def git(*args):
                subprocess.run(["git", *args], cwd=repo, env=env, check=True,
                               capture_output=True)

            git("init", "-q")
            git("commit", "-q", "--allow-empty", "-m", "fix: before (#1)")
            git("tag", "base")
            git("commit", "-q", "--allow-empty", "-m", "feat: after (#2)")
            commits = commits_between("base", "HEAD", repo)
            self.assertEqual([c.subject for c in commits], ["feat: after (#2)"])


if __name__ == "__main__":
    unittest.main()
