# Release Plan

This branch is preparing the first breaking minor revision before `v1.0.0`.

## User Notice

Terminal Jarvis is moving from a menu-heavy tool manager to a smaller
data-driven harness switcher. Existing interfaces are expected to break so the
project can reduce build time, remove the Go ADK experiment from the new root,
and make harness setup requirements explicit.

## Upgrade Path

For source users:

```bash
git fetch
git checkout minor/harness-catalog-v0.1
cargo run -- list
scripts/verify.sh
```

For npm-from-source testing:

```bash
npm --prefix npm/terminal-jarvis run smoke
```

For Homebrew formula syntax testing:

```bash
ruby -c homebrew/Formula/terminal-jarvis.rb
```

For host release packaging:

```bash
scripts/package-release.sh
```

The package script writes ignored artifacts under `dist/<version>/<platform>/`:
a binary archive with bundled harness data, a SHA-256 file, an npm package
staging directory with `bin/terminal-jarvis-bin`, and a versioned Homebrew
formula that points at the release archive URL. Platform names are user-facing
labels such as `linux-x64-gnu` or `macos-arm64`; the Rust target triple remains
visible in `scripts/package-release.sh --check`.

For packaged users, keep using the current published release until the new npm,
Cargo, and Homebrew surfaces are rebuilt around the harness catalog CLI.

## Distribution Hygiene

Before publishing this minor revision:

1. Run `scripts/verify.sh`.
2. Run `cargo llvm-cov` with the 90 percent line coverage gate installed.
3. Run `TJ_MUTATION=1 scripts/verify.sh` with `cargo-mutants` installed.
4. Run `cargo audit` and `cargo deny check`.
5. Run `npm --prefix npm/terminal-jarvis audit --omit=dev --audit-level=moderate`.
6. Run `scripts/package-release.sh` and publish the generated archive and npm
   staging package for each supported target.
7. Review and publish the generated versioned Homebrew Formula with the real
   SHA-256 checksums.
8. Test install, update, and version commands through Cargo, npm, and Homebrew.

The root GitHub workflows intentionally replace the old release flow. CI now
verifies the lean Rust crate, catalog contracts, package metadata, security
gates, npm wrapper, Homebrew syntax, coverage, and mutation. Multi-platform CD
builds host archives for Linux and macOS and publishes draft GitHub release
assets for tagged releases.

The old implementation is intentionally pruned from this PR to keep review
focused on the v0.1 root; use Git history when legacy release behavior needs
inspection.
