"""Parser tests: conventional subjects, squash suffixes and git log records."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from logic.parse import FIELD_SEP, RECORD_SEP, parse_log, parse_subject  # noqa: E402


class ParseSubjectTests(unittest.TestCase):
    def test_scope_and_pr_suffix(self):
        entry = parse_subject("feat(tui): ship the pages (#240)")
        self.assertEqual(entry.type, "feat")
        self.assertEqual(entry.scope, "tui")
        self.assertEqual(entry.text, "ship the pages")
        self.assertEqual(entry.pr, 240)
        self.assertFalse(entry.breaking)
        self.assertEqual(entry.full, "feat(tui): ship the pages")

    def test_no_scope_no_pr(self):
        entry = parse_subject("fix: close the gap")
        self.assertEqual((entry.type, entry.scope, entry.pr), ("fix", None, None))

    def test_breaking_bang(self):
        entry = parse_subject("refactor(api)!: drop the old flag (#9)")
        self.assertTrue(entry.breaking)
        self.assertEqual(entry.type, "refactor")

    def test_type_is_lowercased(self):
        self.assertEqual(parse_subject("Feat: Upper (#1)").type, "feat")

    def test_non_conventional_subject_has_no_type(self):
        entry = parse_subject("Update the readme (#5)")
        self.assertEqual(entry.type, "")
        self.assertEqual(entry.text, "Update the readme")
        self.assertEqual(entry.pr, 5)

    def test_quoted_revert_records_original(self):
        entry = parse_subject('Revert "feat(x): y (#12)" (#13)')
        self.assertTrue(entry.is_revert)
        self.assertEqual(entry.revert_of, "feat(x): y")
        self.assertEqual(entry.pr, 13)

    def test_conventional_revert_records_original(self):
        entry = parse_subject("revert: feat(x): y (#14)")
        self.assertTrue(entry.is_revert)
        self.assertEqual(entry.revert_of, "feat(x): y")


class ParseLogTests(unittest.TestCase):
    def test_records_and_parent_counts(self):
        output = (
            f"aaa{FIELD_SEP}{FIELD_SEP}feat: one (#1){RECORD_SEP}\n"
            f"bbb{FIELD_SEP}aaa ccc{FIELD_SEP}Merge pull request #2{RECORD_SEP}\n"
        )
        commits = parse_log(output)
        self.assertEqual([c.sha for c in commits], ["aaa", "bbb"])
        self.assertEqual([c.parents for c in commits], [0, 2])
        self.assertEqual(commits[0].subject, "feat: one (#1)")

    def test_blank_and_malformed_records_are_skipped(self):
        self.assertEqual(parse_log(f"\n{RECORD_SEP}\n"), [])


if __name__ == "__main__":
    unittest.main()
