# Earn the catalog's platform claims and deny Windows warnings

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** Generate `docs/claim-inventory.tsv` from the manifests so every
assertion carries a verdict; reject `platforms = []` plus a POSIX-shell
executable on an executing row and de-shell the 23 `*/version` rows; scope the
unix-only test support to `unix` and add a `windows-latest` CI job that runs
`cargo clippy --all-targets -- -D warnings` and `cargo test --all-targets`; pin
`docs/support-matrix.md` and `docs/*.tsv` to `text eol=lf`. A row's mechanism is
selected by manifest schema, capability, and support state -- never by field
name alone.

**Context:** The catalog makes hundreds of assertions about software the repo
does not own, and the only thing holding many of them up was a human having
typed them. 23 version probes claimed to run anywhere while requiring a POSIX
shell Windows does not have; the empty platform list disabled the guard that
would have refused them. The only `-D warnings` gate ran on Linux, so a
Windows-only dead-code warning reached a release. And the first pass of the
inventory overstated its own mechanisms: a gate manifest was credited with the
catalog shape gate that never sees it, and `effect`/`network`/`interaction`
were credited to `effect_truth` on capabilities it does not arm.

**Considered:** Narrowing the rows to `linux-x64-gnu` declares a Linux-only
product on Windows. `cargo clippy --fix` deletes helpers Linux needs. A
repo-wide line-ending policy churns 818 of 823 tracked files. Reporting an
`unknown` row's empty platform scope as `inert` reads `guard_policy` as refusing
a row it in fact admits once a terminal session consents.

**Consequence:** Every manifest key has a verdict and the inventory fails
`--check` when it stops covering one; a mechanism is claimed only where an
invariant actually runs (92 rows moved to `asserted`); platform independence
cannot route through `sh`; `win32-x64` is denied warnings in CI rather than
reasoned about; generated artifacts survive a Windows checkout byte-for-byte.
