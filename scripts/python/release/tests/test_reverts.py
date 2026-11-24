"""Revert cancellation: newest-first order, revert-of-revert and breaking reverts."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from logic.group import group  # noqa: E402
from logic.parse import Commit, parse_subject  # noqa: E402


def commit(subject):
    return Commit(subject, 1, subject)


class RevertCancelTests(unittest.TestCase):
    def test_older_revert_does_not_cancel_newer_same_subject(self):
        # Newest first: A re-added, then A reverted, then the original A.
        sections = group(
            [
                commit("feat(x): A (#2)"),
                commit("revert: feat(x): A"),
                commit("feat(x): A (#1)"),
            ]
        )
        self.assertEqual(sections, {"Added": ["- **x**: A (#2)"]})

    def test_revert_of_revert_restores_original(self):
        # Newest first: R2 reverts R1, R1 reverts A, A was feature A.
        sections = group(
            [
                commit("revert: revert: feat(x): A"),
                commit("revert: feat(x): A"),
                commit("feat(x): A (#1)"),
            ]
        )
        self.assertEqual(sections, {"Added": ["- **x**: A (#1)"]})

    def test_breaking_revert_is_flagged_when_unpaired(self):
        entry = parse_subject("revert!: x")
        self.assertTrue(entry.is_revert)
        self.assertTrue(entry.breaking)
        self.assertEqual(group([commit("revert!: x")]), {"Breaking": ['- Revert "x"']})


if __name__ == "__main__":
    unittest.main()
