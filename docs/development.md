# Development

Terminal Jarvis is a data-driven Rust command-line tool. `src/contracts/`
defines shared types, `src/catalog/` loads harness descriptors, `src/gates/`
loads optional security policy, `src/context/` stores local selection state,
and `src/runtime/` executes a selected command. Harness and gate policy live in
`harnesses/` and `gates/`, not in Rust branches.

## Verification

```bash
scripts/verify.sh
scripts/local-ci.sh
scripts/package-release.sh build /tmp/terminal-jarvis-package
scripts/local-cd.sh --check-auth
```

`scripts/verify.sh` runs format, lint, tests, catalog checks, security checks,
npm wrapper tests, and package metadata checks. Package builds run both the
artifact integration hardening suite and `scripts/core-command-matrix.sh`.
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
