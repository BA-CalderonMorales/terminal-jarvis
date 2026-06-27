# Migration

The pre-rewrite implementation (0.0.x) is intentionally pruned from the
active root. Use Git history for legacy reference.

## Legacy Alias Mapping

The v0.1 rewrite replaced old command names. The following aliases forward
to the new equivalents:

| Legacy (0.0.x) | Current (0.1+) |
|---|---|
| `tools` | `list` |
| `status` | `check` |
| `info <harness>` | `show <harness>` |
| `install <harness>` | `run <harness> download` |
| `update <harness>` | `run <harness> update` |

## Known Breaks (0.0.x vs 0.1+)

- The new root no longer exposes the old menu system.
- The Go ADK is not part of the new root.
- NPM is a minimal source-wrapper surface until release binaries are published.
- Homebrew is source-build by default until versioned release formulas are
  promoted with real archive checksums.

## Upgrade Path

1. Use Git history as the reference for pre-rewrite behavior.
2. Keep each promoted harness on the shared capability contract.
3. Run `scripts/verify.sh` after each promotion.
4. Use pull request CI to compare the new root against hosted checks.
