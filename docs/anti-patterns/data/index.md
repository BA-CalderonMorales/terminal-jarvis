# Data Anti-Patterns

Ledger entries for mistakes in the `harnesses/` data plane (capability
metadata that describes a real CLI). Newest first.

## An executing row claims to run anywhere through a POSIX shell

(first observed in: 0.1.21 claim audit)

**Pattern:** 23 `harnesses/*/version/index.toml` rows set `support` to
`verified`/`expected` and `platforms = []` (which disables the runtime platform
guard) while declaring `command = "sh"`, `args = ["-c", "<tool> --version"]`,
and `executable = "sh"`. The row therefore asserts that the probe works
everywhere and routes it through an executable that a clean Windows `PATH` does
not provide.

**Why it is wrong:** The claim is false on a supported platform and the guard
is off, so nothing refuses it: the version check fails with a shell error the
catalog never predicted. A POSIX shell is itself a platform constraint, so
"runs anywhere" and "requires `sh`" cannot both be true.

**Fix:** Call the binary directly (`command = "<tool>"`, `args = ["--version"]`,
`executable = "<tool>"`), or narrow `platforms` to the POSIX platform the probe
needs. `catalog::logic::command_truth` now rejects an executing row whose
`platforms` are empty and whose executable is `sh` or `bash`, and
`docs/claim-inventory.tsv` marks platform scope `witnessed` only when a real run
recorded the platform.

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
