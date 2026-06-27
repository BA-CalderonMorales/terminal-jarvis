# Distribution Hardening

This document records the next release hardening target after `v0.1.2`.

## Invariant

GitHub Releases are the only source for prebuilt Terminal Jarvis binaries.

The git repository, crates.io source crate, npm package payload, and Homebrew
tap must not vendor native binaries. They may contain source, harness metadata,
scripts, formulas, manifests, checksums, and provenance data.

Terminal Jarvis must never bundle OpenCode, Codex, Claude, Gemini, or any other
harness binary. Harness descriptors may name expected commands and reviewed
install plans, but harness executables come from their upstream projects or the
user's existing `PATH`.

## Current Evidence

- The git checkout tracks source, harness metadata, npm wrapper files, and
  release automation. No release archive or native binary is tracked.
- `Cargo.toml` uses an explicit source include list, so the crates.io package is
  source plus metadata, not a built binary.
- `npm/terminal-jarvis` is source-level JS today, but release packaging copies
  `target/release/terminal-jarvis` to
  `bin/terminal-jarvis-bin` before publishing the npm package.
- The npm release, beta, stable, and recovery workflows currently require
  `bin/terminal-jarvis-bin`.
- Generated Homebrew formulas point at GitHub release archives. That is allowed
  because the binary source remains the GitHub Release asset, not the tap.

## Required Change

The next distribution patch must remove `terminal-jarvis-bin` from npm package
payloads and from npm workflow assertions.

The npm package should become a small launcher that resolves the expected
Terminal Jarvis release asset for the current OS and CPU, verifies its checksum
against the matching GitHub Release checksum, installs it into a user or npm
cache, and then executes it. It should print the release URL and cached binary
path when asked for verbose version or provenance output.

The launcher must fail closed when:

- the platform has no supported release asset;
- the release archive or checksum is missing;
- the downloaded checksum does not match;
- the cache path is not executable after extraction.

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

### `0.1.4` UX Fix

- Support `--version`, `-v`, and `version --verbose`.
- Decide whether `--info` is an alias for `version --verbose` or a clear error
  that points to the supported command.
- Replace raw `No such file or directory (os error 2)` failures with messages
  naming the missing catalog, binary, cache, or harness command.
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
