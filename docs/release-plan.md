# Release Plan

This repository is on the `v0.1.x` line, the first breaking minor revision
before `v1.0.0`.

Patch releases carry packaging, release automation, documentation, and bug
fixes. Do not cut a minor or major version unless that version level is
explicitly requested.

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

For artifact-level integration hardening:

```bash
scripts/integration-hardening.sh
```

For the local CD smoke path without tagging, pushing, or publishing:

```bash
scripts/local-cd.sh --check-auth
```

The package script writes artifacts under `<out>/<version>/<platform>/`:
a binary archive with bundled harness data, a SHA-256 file, an npm package
staging directory with `bin/terminal-jarvis-bin`, and a versioned Homebrew
formula that points at the release archive URL. Platform names are user-facing
labels such as `linux-x64-gnu` or `macos-arm64`; the Rust target triple remains
visible in `scripts/package-release.sh --check`. Packaging also runs
`scripts/integration-hardening.sh` against the staged binary, bundled harnesses,
npm wrapper, and generated Homebrew formula before reporting artifact paths.

`scripts/local-cd.sh` collects the same archive and checksum files into a
`release-assets/v<version>/` directory and verifies every checksum and recorded
archive filename. Release script defaults live in `scripts/release.toml`. In
this workspace, default output is `testing/terminal-jarvis/local-cd/`.

## Distribution Hygiene

Before publishing a `v0.1.x` release:

1. Run `scripts/verify.sh`.
2. Run `scripts/local-ci.sh`; pass `--strict` when optional security tools must
   be installed instead of skipped.
3. Confirm `scripts/integration-hardening.sh` passes for the release candidate
   binary and staged package surfaces.
4. Run `scripts/local-cd.sh --check-auth` and inspect the generated
   `testing/terminal-jarvis/local-cd/release-assets/v<version>/` directory.
5. Run `cargo llvm-cov` with the 90 percent line coverage gate installed when
   coverage is not already covered by `scripts/verify.sh`.
6. Run `TJ_MUTATION=1 scripts/verify.sh` or `scripts/local-ci.sh --mutation`
   with `cargo-mutants` installed before cutting the tag.
7. Test install, update, and version commands through Cargo, npm, and Homebrew.
8. Push a signed or reviewed `v<version>` tag only after the release PR is
   accepted.

The root GitHub workflows keep the tag-push release shape. CI verifies the lean
Rust crate, catalog contracts, package metadata, security gates, npm wrapper,
Homebrew syntax, coverage, and mutation. Multi-platform CD builds host archives
for Linux and macOS, runs integration hardening before publish jobs can proceed,
publishes the crates.io package when needed, publishes the GitHub release
assets, updates the Homebrew tap, and publishes or retags the npm package with
`latest`, `stable`, and `beta` pointing at the same patch version.

Terminal Jarvis release validation should not automatically download arbitrary
harness dependencies. Docker coverage should start as documented baseline images
for reviewed environments that users can extend. First-class Docker dependency
coverage should stay limited to well-known harnesses such as OpenCode, Codex,
Claude Code, Gemini, and Hermes Agent; unsupported OS or harness combinations
should be stated explicitly.

Auth boundaries:

- GitHub release upload requires `contents: write` through GitHub Actions or a
  local `gh` session with equivalent repository access.
- Homebrew tap updates require `HOMEBREW_TAP_TOKEN` with push rights to
  `BA-CalderonMorales/homebrew-terminal-jarvis`.
- npm publish or dist-tag updates for `latest`, `stable`, and `beta` require
  `NPM_TOKEN` with package publish rights. Beta and stable workflows remain
  available for explicit dist-tag recovery.
- crates.io publish requires `CARGO_REGISTRY_TOKEN` with publish scope.
