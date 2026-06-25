# Migration

The pre-rewrite implementation is intentionally pruned from this PR.

## Upgrade Path

1. Use Git history as the reference for pre-rewrite behavior.
2. Keep each promoted harness on the shared capability contract.
3. Run `scripts/verify.sh` after each promotion.
4. Use draft PR CI to compare the new root against hosted checks.

The first promotion pass covers 25 initial harnesses. Several capabilities
intentionally point at `--help` or an explicit unavailable plan until the exact
harness-specific command is verified.

## Known Breaks

- The new root no longer exposes the old menu system.
- The Go ADK is not part of the new root.
- NPM and e2e wrapper work needs to be rebuilt against the new CLI surface.
- Release automation needs to be reconnected after the catalog stabilizes.
