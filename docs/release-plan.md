# Release Plan

This repository is preparing `v0.1.0`, the first breaking minor revision before
`v1.0.0`.

## User Notice

Terminal Jarvis is moving from a menu-heavy tool manager to a smaller
data-driven harness switcher. Existing interfaces are expected to break so the
project can reduce build time, remove the Go ADK experiment from the new root,
and make harness setup requirements explicit.

## Upgrade Path

For source users:

```bash
git fetch
git checkout develop
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

For the full local CD smoke path without tagging, pushing, or publishing:

```bash
scripts/local-cd.sh --check-auth
```

The package script writes artifacts under `<out>/<version>/<platform>/`:
a binary archive with bundled harness data, a SHA-256 file, an npm package
staging directory with `bin/terminal-jarvis-bin`, and a versioned Homebrew
formula that points at the release archive URL. Platform names are user-facing
labels such as `linux-x64-gnu` or `macos-arm64`; the Rust target triple remains
visible in `scripts/package-release.sh --check`.

`scripts/local-cd.sh` collects the same archive and checksum files into a
`release-assets/v<version>/` directory and verifies every checksum and recorded
archive filename. Release script defaults live in `scripts/release.toml`. In
this workspace, default output is `testing/terminal-jarvis/local-cd/`.

## Distribution Hygiene

Before publishing this minor revision:

1. Run `scripts/verify.sh`.
2. Run `scripts/local-ci.sh`; pass `--strict` when optional security tools must
   be installed instead of skipped.
3. Run `scripts/local-cd.sh --check-auth` and inspect
   `testing/terminal-jarvis/local-cd/release-assets/v0.1.0/`.
4. Run `cargo llvm-cov` with the 90 percent line coverage gate installed when
   coverage is not already covered by `scripts/verify.sh`.
5. Run `TJ_MUTATION=1 scripts/verify.sh` or `scripts/local-ci.sh --mutation`
   with `cargo-mutants` installed before cutting the tag.
6. Test install, update, and version commands through Cargo, npm, and Homebrew.
7. Push a signed or reviewed `v0.1.0` tag only after the release PR is accepted.

The root GitHub workflows intentionally replace the old release flow. CI now
verifies the lean Rust crate, catalog contracts, package metadata, security
gates, npm wrapper, Homebrew syntax, coverage, and mutation. Multi-platform CD
builds host archives for Linux and macOS and publishes draft GitHub release
assets for tagged releases.

Auth boundaries:

- GitHub draft release upload requires `contents: write` through GitHub Actions
  or a local `gh` session with equivalent repository access.
- npm beta and stable workflows build the staged npm package through
  `scripts/package-release.sh`; real publishes or dist-tag updates require a
  renewed npm automation token with package publish rights in `NPM_TOKEN`.
- crates.io publish requires `CARGO_REGISTRY_TOKEN` with publish scope before
  any Cargo publish step is added or run.
