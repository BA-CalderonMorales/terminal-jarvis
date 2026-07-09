# Distribution Hardening

This document records the distribution hardening target after `v0.1.2`.

## Invariant

GitHub Releases are the only source for prebuilt Terminal Jarvis binaries.

The git repository, crates.io source crate, npm package payload, and Homebrew
tap must not vendor native binaries. They may contain source, harness metadata,
scripts, formulas, manifests, checksums, and provenance data.

Terminal Jarvis must never bundle OpenCode, Codex, Claude, Gemini, or any other
harness binary. Harness descriptors may name expected commands and reviewed
install plans, but harness executables come from their upstream projects or the
user's existing `PATH`.

## Target Model

- Cargo/crates.io is the source distribution. `cargo install terminal-jarvis`
  builds the Rust CLI from the crate, using an anchored package allowlist that
  includes only Cargo metadata, `Cargo.lock`, source, harness descriptors,
  tests, user-facing docs, changelog, README, and license files.
- npm is a launcher/helper distribution. The package ships a real executable
  Node wrapper at `bin/terminal-jarvis` plus `bin/README.txt`; it does not ship
  native binaries or harness executables. Installed copies download the matching
  GitHub Release archive for the npm package version, verify the release
  `.sha256` asset, cache the unpacked release, and execute it.
- Homebrew is the binary installer path. The tap formula points at GitHub
  Release archives with per-platform checksums and installs the
  `terminal-jarvis` command plus the harness catalog.
- The CLI command promise is the same across ecosystems: `terminal-jarvis`
  exposes the same help, version/provenance output, harness list, and
  capability commands. The packaging mechanics differ because Cargo builds from
  source, npm launches a downloaded release asset, and Homebrew installs release
  archives directly.

## Current Evidence

- The git checkout tracks source, harness metadata, npm wrapper files, and
  release automation. No release archive or native binary is tracked.
- `Cargo.toml` uses an explicit source include list, so the crates.io package is
  source plus metadata, not a built binary.
- `npm/terminal-jarvis` is a JS launcher. Release packaging must not copy
  `target/release/terminal-jarvis` into the npm package.
- The npm launcher downloads Terminal Jarvis from GitHub Releases, verifies the
  release `.sha256` checksum, caches the unpacked archive, and then executes the
  cached binary.
- Generated Homebrew formulas point at GitHub release archives. That is allowed
  because the binary source remains the GitHub Release asset, not the tap.

## Current Requirements

Release packaging must keep `terminal-jarvis-bin` out of npm package payloads
and workflow assertions.

The npm package is a small launcher that resolves the expected
Terminal Jarvis release asset for the current OS and CPU, verifies its checksum
against the matching GitHub Release checksum, installs it into a user or npm
cache, and then executes it. It reports the release URL and cached binary
path when asked for verbose version or provenance output.

The launcher must fail closed when:

- the platform has no supported release asset;
- the release archive or checksum is missing;
- the downloaded checksum does not match;
- the cached native binary is missing, invalid, or not executable after
  extraction on platforms that expose executable bits.

Global npm installs must also fail closed when the installed npm shim would be
shadowed by an older `terminal-jarvis` earlier on `PATH`; otherwise
`npm install -g terminal-jarvis@latest` can appear successful while the next
`terminal-jarvis --version` still executes an older Cargo or manual install.

## Release Split

### `0.1.3` Distribution Fix

- Remove the staged npm binary copy from `scripts/package-release.sh`.
- Remove `terminal-jarvis-bin` checks from npm publish and recovery workflows.
- Teach the npm wrapper to download only from
  `https://github.com/BA-CalderonMorales/terminal-jarvis/releases`.
- Add checksum verification using the release `.sha256` asset.
- Add package tests proving `npm pack` contains no native binary payload.
- Add release hardening checks that fail if npm staging includes
  `terminal-jarvis-bin`, `opencode`, or any executable harness binary.
- Update npm docs to state where the binary is downloaded from and how to
  inspect or clear the cache.

### Completed UX Hardening

- Support `--version`, `-v`, `--info`, and `version --verbose`.
- Treat `--info` as an alias for verbose provenance output.
- Replace raw missing catalog `No such file or directory (os error 2)` failures
  with messages naming the catalog path and `TERMINAL_JARVIS_CATALOG`.
- Reject unknown flags such as `--v` before loading the catalog.

- Detect stale global installs that still expose the pre-`0.1.2` help surface.
- Make `npx terminal-jarvis`, global npm installs, Cargo installs, and Homebrew
  installs report their source and binary path consistently.
- Keep `terminal-jarvis help` and `terminal-jarvis --help` aligned with the
  compatibility surface restored in `0.1.2`.

## Verification Gates

Before publishing the distribution fix:

1. `cargo package --list --allow-dirty` shows no built binary or release
   archive.
2. npm source and staged `npm pack --dry-run --json` output contains no
   `terminal-jarvis-bin` and no harness executable.
3. `scripts/package-release.sh` still builds GitHub Release archives and
   checksums, but npm staging contains only the launcher, docs, package
   metadata, and harness catalog data.
4. GitHub release asset checksums are verified before npm launcher execution.
5. `npm dist-tag ls terminal-jarvis` keeps `latest`, `stable`, and `beta` on the
   same patch version after publish.
