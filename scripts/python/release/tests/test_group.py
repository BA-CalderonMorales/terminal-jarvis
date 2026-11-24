"""Grouping tests: breaking, reverts, unknown types, PR order and merges."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from logic.group import group  # noqa: E402
from logic.parse import Commit  # noqa: E402


def commit(subject, parents=1, sha=None):
    return Commit(sha or subject, parents, subject)


class GroupTests(unittest.TestCase):
    def test_feat_fix_perf_map_to_sections(self):
        sections = group(
            [
                commit("feat(a): new (#3)"),
                commit("fix: broken (#2)"),
                commit("perf: faster (#1)"),
            ]
        )
        self.assertEqual(list(sections), ["Added", "Changed", "Fixed"])
        self.assertEqual(sections["Added"], ["- **a**: new (#3)"])

    def test_silent_types_are_dropped(self):
        subjects = [
            "docs: a (#1)", "test: b (#2)", "ci: c (#3)", "build: d (#4)",
            "chore(release): e (#5)", "style: f (#6)", "refactor: g (#7)",
        ]
        self.assertEqual(group([commit(s) for s in subjects]), {})

    def test_breaking_goes_to_leading_section_even_for_silent_types(self):
        sections = group(
            [commit("feat!: drop flag (#8)"), commit("refactor!: rename (#9)")]
        )
        self.assertEqual(list(sections), ["Breaking"])
        self.assertEqual(len(sections["Breaking"]), 2)

    def test_pair_of_commit_and_revert_is_dropped(self):
        sections = group(
            [
                commit('Revert "feat(x): thing (#11)" (#12)'),
                commit("feat(x): thing (#11)"),
                commit("fix: other (#10)"),
            ]
        )
        self.assertEqual(sections, {"Fixed": ["- other (#10)"]})

    def test_unpaired_revert_is_listed_under_fixed(self):
        sections = group([commit('Revert "feat(x): gone (#20)" (#21)')])
        self.assertEqual(sections, {"Fixed": ['- Revert "feat(x): gone" (#21)']})

    def test_unknown_and_unparseable_types_go_to_changed(self):
        sections = group([commit("wip(x): odd (#30)"), commit("Tidy the readme (#31)")])
        self.assertEqual(
            sections,
            {"Changed": ["- Tidy the readme (#31)", "- **x**: odd (#30)"]},
        )

    def test_merge_commits_are_dropped_by_parents_and_subject(self):
        sections = group(
            [
                commit("feat: real (#40)"),
                commit("feat: two parents (#41)", parents=2),
                commit("Merge branch 'main'"),
            ]
        )
        self.assertEqual(sections, {"Added": ["- real (#40)"]})

    def test_out_of_order_pr_numbers_are_sorted_and_deduped(self):
        sections = group(
            [
                commit("feat: older (#5)"),
                commit("feat: newer (#9)"),
                commit("feat: middle (#7)"),
                commit("feat: duplicate (#9)"),
                commit("feat: no number"),
            ]
        )
        self.assertEqual(
            sections["Added"],
            [
                "- newer (#9)",
                "- middle (#7)",
                "- older (#5)",
                "- no number",
            ],
        )


if __name__ == "__main__":
    unittest.main()
