# Development

Terminal Jarvis is a data-driven Rust command-line tool. `src/contracts/`
defines shared types, `src/catalog/` loads harness descriptors, `src/gates/`
loads optional security policy, `src/context/` stores local selection state,
and `src/runtime/` executes a selected command. Harness and gate policy live in
`harnesses/` and `gates/`, not in Rust branches.

## Verification

```bash
scripts/bash/verify/index.sh verify
scripts/bash/verify/index.sh local-ci
scripts/bash/release/index.sh package build /tmp/terminal-jarvis-package
scripts/bash/release/index.sh local-cd --check-auth
```

`verify` step `[7]` also regenerates and checks the generated claim inventory:
`python3 scripts/python/catalog/index.py claim-inventory [--check]`. Two
rerunnable proofs carry the old-fails/new-passes pair for the platform-claim
rule (`scripts/bash/verify/index.sh claim-earned --from REF`) and for the
Windows strict-clippy gate
(`scripts/bash/verify/index.sh windows-clippy --from REF`), where `REF` is the
pre-fix commit; see `docs/claim-earning-study.md`. Each exits nonzero unless its
pre-fix half fails and the working tree passes, so `windows-clippy` belongs on a
host where the pre-fix tree fails (a Windows checkout) and reports
`there is no warning to deny` anywhere else.

## Domain shape

Every Rust domain mirrors the scripts/ bucketing: `src/<domain>/index.rs` is
the domain's public face, with `structs/` (data shapes), `logic/` (behavior),
`constants/` (fixed values), and `tests/` (cfg(test)-only trees pulled by
their subject modules) inside. Integration tests live in
`tests/<domain>/` -- one `[[test]]` binary per domain in `Cargo.toml` -- so a
failure names the domain:

```bash
cargo test --test cli        # cli domain only
cargo test --test diagnostics # diagnostics domain only
```

A domain is imported through its face (`crate::context::platform::...`), never
through `logic/` internals, so blast radius is witnessable per domain before
an agent touches `logic/`. `evaluation/` is the sanctioned exception: it is
the evaluation domain's shipped kit payload (see `evaluation/EVALUATION.md`).

`scripts/bash/verify/index.sh verify` runs format, lint, tests, catalog
checks, security checks, npm wrapper tests, and package metadata checks.
Package builds run both the artifact integration hardening suite and
`scripts/bash/verify/index.sh core-command-matrix`.
The matrix exercises every public command on the host-native binary while
leaving third-party agent installation, update, and interactive launch paths
unexecuted.

## Command Output

Human-facing commands render width-aware tables that fit the `COLUMNS` value
(up to 120 columns) and color headings only when stdout is a terminal. For
automation, use `terminal-jarvis --plain <command>` for stable line-oriented
output or `terminal-jarvis --no-color <command>` to retain the table layout
without terminal color.

## Platform Contract

The core command surface is identical on Linux, macOS, Windows PowerShell,
Command Prompt, and Git Bash. Release builds exercise that surface on each
native GitHub Actions host. Windows executable discovery honors `PATHEXT`, the
npm cache uses the local application-data directory, and npm downloads the
Windows ZIP bundle through PowerShell.

Harness commands remain upstream integrations. Their own availability,
installation syntax, and interactive behavior can differ by operating system;
Terminal Jarvis reports missing binaries and exposes their planned command
instead of claiming unsupported combinations work.

## Release Artifacts

Every supported platform publishes a checksummed `.tar.gz` bundle containing the
binary, harness catalog, and gate catalog. Windows also publishes a checksummed
ZIP bundle for npm. Each platform additionally publishes a checksummed direct
native executable for users who prefer a single-file download; the executable
contains the default harness and gate data as a fallback.

Release preparation stops at a verified `release/X.Y.Z` branch. Tagging,
publishing, protected merges, and the mandatory release Trivy scans use the
separate release workflow and require operator approval.

### npm Trusted Publishing

The npm publish job holds no stored credential. It authenticates with the
workflow's GitHub OIDC identity, so a Trusted Publisher must be registered on
npmjs.com for this repository and the `cd-multiplatform.yml` workflow before a
release tag is pushed.

Two settings on that publisher are easy to miss and both fail *after* the
package is already published:

- **Allow npm dist-tag** is **off by default**. Publishing and moving channels
  are independent permissions, so without it the release publishes and then
  cannot point `latest`, `stable`, or `beta` at the new version.
- The client must support dist-tags over OIDC: **npm 11.21.0+ in the 11.x line,
  or 12.2.0+ in the 12.x line**. The job installs a current npm and then
  asserts the version before it touches the registry, because the toolchain a
  `setup-node` version ships can be older than the publish minimum.

Provenance is generated automatically for a trusted publish from GitHub
Actions; the `--provenance` flag only makes it explicit.

### Registry Confirmation

npm and crates.io acknowledge a write before they serve it. The release jobs
therefore confirm the published version, each dist-tag, and the crates.io entry
by polling to convergence, failing only on timeout. A single read immediately
after a write is not evidence, and treating one as evidence reports a false
failure for a release that actually succeeded.

### Publish Rehearsal

The npm publisher lives in `scripts/bash/release/logic/publish-npm.sh` and the
workflow calls it through the release domain index, so it is not inlined in a
`run:` block. That keeps the shipped bytes rehearsable before a tag exists:

```bash
scripts/bash/verify/index.sh publish-rehearsal --from <pre-fix-ref>
```

The shipped publisher must converge against a fake registry that serves a write
only after N reads, and the pre-fix body extracted from `--from` must publish
and then fail its own verification -- the v0.1.20 shape. The rehearsal never
touches the real registry, git tags, or the network, and it does not remove the
one thing only a real release can decide: the OIDC credential exchange.
