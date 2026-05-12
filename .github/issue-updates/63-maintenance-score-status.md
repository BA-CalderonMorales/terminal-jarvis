# Issue #63 Maintenance Score Status

## Recommendation

Keep #63 open, but narrow it to current measurable maintenance-score gaps that still apply after the latest repository and package evidence.

## Current Evidence

- `.github/workflows/cd-multiplatform.yml` already creates GitHub releases from `v*` tags and uses generated release notes through `softprops/action-gh-release@v2`.
- `CHANGELOG.md` is present, follows versioned release entries, and includes the current `0.0.81` entry.
- `scripts/verify/verify-docs.sh`, `scripts/cicd/local-ci.sh`, and `scripts/cicd/local-cd.sh` include CHANGELOG/version readiness checks.
- `.github/workflows/weekly-maintenance.yml` runs a scheduled issue-health workflow and README sync workflow.
- `.github/FUNDING.yml` exists and includes Buy Me a Coffee sponsorship metadata.
- `SECURITY.md` exists and documents supported versions, private reporting, supply-chain checks, and response expectations for security reports.
- `Cargo.toml` includes repository, homepage, readme, license, keywords, and category metadata.
- `npm/terminal-jarvis/package.json` includes repository, homepage, license, keywords, Node engine, bin, and TypeScript metadata.
- Current npm registry evidence confirms `terminal-jarvis@0.0.81` is published and `latest`, `beta`, and `stable` all point to `0.0.81`.
- Socket.dev live score verification is still blocked from this environment: `https://socket.dev/npm/package/terminal-jarvis` returns HTTP 403 with a Cloudflare challenge.

## Changes Added For #63

- Added `.github/ISSUE_TEMPLATE/bug_report.yml`.
- Added `.github/ISSUE_TEMPLATE/feature_request.yml`.
- Added `.github/ISSUE_TEMPLATE/security_vulnerability.yml`.
- Added `.github/ISSUE_TEMPLATE/config.yml` with private security disclosure contact.
- Added root `CONTRIBUTING.md` with branch flow, validation expectations, and maintainer response targets.
- Renamed `.github/pull_request_template.md` to `.github/PULL_REQUEST_TEMPLATE.md` and fixed its stale contribution-guide link.

## Proposed #63 Remaining Scope

After this branch, the original #63 checklist should be considered satisfied except for external score verification:

- Verify the current Socket.dev Maintenance score in an authenticated or browser-capable environment.
- If Socket.dev still reports a maintenance-score finding, update #63 with the exact current finding text and the repository artifact it expects.
- Close #63 if Socket.dev no longer reports a concrete current maintenance-score gap.

## Validation

Run before commit:

```bash
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc RUSTFLAGS= ./scripts/verify/verify-change.sh --quick
```
