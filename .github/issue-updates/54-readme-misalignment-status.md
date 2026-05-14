# Issue #54: README Misalignment Status

Prepared: 2026-05-13

## Recommendation

Close #54 as satisfied for the original README messaging drift, and keep the
dynamic source-of-truth concern under #31.

## Current Evidence

- Root `README.md` now uses count-consistent wording:
  - `Manage Claude, Gemini, Qwen, and 22 more AI assistants from one terminal interface.`
  - `25 AI Tools Supported`
  - docs link copy says `all 25 supported AI coding assistants`
- `npm/terminal-jarvis/README.md` matches the root README count wording.
- `CHANGELOG.md` current `0.0.81` entry records:
  - `Documentation Counts: Fixed README tool counts to 25 supported tools`
- #80 already tracks and has PR coverage for the remaining PR #77 review feedback around README/changelog count drift.
- #31 remains open for the broader source-of-truth decision across TOML and DB-backed tool metadata.

## Remaining Scope

No README wording fix remains for #54.

The still-useful part of the issue is the statement that tool counts should be
calculated dynamically. That should not stay as a separate README drift issue;
it belongs with #31 because dynamic README generation depends on the same
catalog source-of-truth decision:

- whether TOML remains the source of truth,
- whether DB-backed state becomes primary,
- or whether a unified config API hides the storage split.

## Suggested Issue Update

```markdown
The README messaging drift is now fixed in both the root README and the npm README:

- "Claude, Gemini, Qwen, and 22 more"
- "25 AI Tools Supported"
- docs link copy for all 25 supported tools

The current changelog also records the 25-tool documentation-count fix.

The remaining idea, dynamically calculating tool counts from the catalog, is a source-of-truth problem and should be handled under #31 rather than tracked as a separate README misalignment issue.

Recommended disposition: close #54 as satisfied/duplicated by #80 for the drift fix and #31 for the dynamic source-of-truth work.
```
