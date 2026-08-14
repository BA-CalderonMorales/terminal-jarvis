# Data Anti-Patterns

Ledger entries for mistakes in the `harnesses/` data plane (capability
metadata that describes a real CLI). Newest first.

## Capability data declared from assumption, not from the real CLI

(first observed in: 0.1.15 readiness audit)

**Pattern:** Harness capability metadata was written from assumption about
the agent's surface instead of running it: `codex`/`opencode` were declared
to require API-key env vars though they authenticate via login files
(`codex login`, `opencode auth login`), gating their readiness as
`not-ready` for logged-in users; `jules` declared `jules --version` though
the CLI's actual command is `jules version` (the probe failed every time).
Both entries share the source tag `internal:phase-01@fab5848`.

**Why it is wrong:** Runtime behavior lies to users in the dashboard: the
active, working harness reads `not-ready`, and a capability appears
verified when its probe never succeeds. Data-plane claims are the product's
trust surface.

**Fix:** Verify every capability against the real CLI before committing it:
run the exact command and flag; check the auth surface (`--help` / `login`
subcommand) before declaring env requirements; when a command cannot be
verified, mark the capability unsupported instead of guessing.
