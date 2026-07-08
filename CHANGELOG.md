# Changelog

## [0.1.9] - 2026-07-07

- Fixes `hlp()` helper to scan all arguments after the subcommand so `plan yolo --help`,
  `security status --help`, `show opencode --help`, etc. route to help text. The `run`
  subcommand still only checks position 1 so `run codex --help` forwards `--help` to
  the harness as intended.
- Fixes global `-v`/`--version`/`--info` to reject an unexpected subcommand after the
  flag instead of silently discarding it: `terminal-jarvis -v version` and
  `terminal-jarvis --info show opencode` now produce a clear error.
- Fixes `npm postinstall` to exit 0 when a stale cargo binary shadows the npm shim on
  PATH, so `npm install -g terminal-jarvis` no longer fails. The shadow warning is
  still printed to stderr with actionable guidance.
- Updates tests for all three bugfixes.

## [0.1.8] - 2026-07-07

- Fixes `--help`/`-h` parsing on 12 of 14 subcommands so help text is reachable
  on every command that supports it.
- Fixes `version -v --verbose` and similar multi-flag combinations so both flags
  are accepted instead of rejecting valid combinations.
- Fixes `security <unrecognized>` so an unknown harness name produces a usage
  error instead of being silently treated as a harness.
- Fixes session loading so a garbage (non-empty unparseable) `session.toml` emits
  a warning instead of being silently swallowed.

## [0.1.7] - 2026-07-07

- Bumps the release-candidate version to 0.1.7 in `Cargo.toml` and the npm
  package so `version` and every `v{VERSION}` notice report the correct release
  line; previously the `release/0.1.7` candidate still identified as 0.1.6,
  which would publish the RC under the wrong version.
- Fixes `run` so a free-form prompt whose first word is a capability (for example
  `run update my database` or `run yolo clean tmp`) is sent to the harness as a
  headless prompt instead of silently executing the side-effecting or dangerous
  capability. Single-word `run <capability>` and `run <harness> <capability>`
  are unchanged, and `run headless <prompt>` still works as documented.
- Fixes `auth set <harness>` so it no longer implies a mutating action: it now
  states explicitly that terminal-jarvis does not persist credentials and that
  nothing was stored. `auth help <harness>` is unchanged.
- Makes `-v` consistent: top-level `terminal-jarvis -v` and
  `terminal-jarvis version -v` both print the plain version. Verbose provenance
  stays on `--verbose`/`--info` (and `version --verbose`), and these global
  flags are now documented in `help`.
- Improves `cache status` so it explains the cache is wrapper-managed and how to
  enable it when run outside the npm launcher, instead of a bare `unavailable`.
- Fixes the active-harness home to a global config location
  (`$XDG_CONFIG_HOME/terminal-jarvis`, else `~/.config/terminal-jarvis`) instead
  of a CWD-relative `.terminal-jarvis`. `use`/`current`/`plan` (no harness) now
  stay consistent across directories and terminals; `TERMINAL_JARVIS_HOME` still
  overrides for per-project isolation. `config show` now prints the absolute
  home path so state location is never ambiguous.
- Replaces hardcoded `v0.1.2` strings in `auth`/`config`/`update` messages with
  the package version, so compatibility notices never read stale again.
- Differentiates `check` from `security status`: `check` stays a terse per-harness
  binary/env table, while `security` and `security status` now append a `status:
  X/Y harnesses ready` summary. Previously `check`, `security`, and `security status`
  printed identical output, hiding that `security` reports overall readiness.

## [0.1.6] - 2026-06-30

- Hardens npm distribution as a launcher package with a real executable wrapper
  and shipped `bin/README.txt` guidance instead of relying on local behavior.
- Anchors crates.io package contents to the source, harness catalog, tests,
  user-facing docs, changelog, README, lockfile, and license.
- Keeps crates.io README rendering while excluding the large promo image from
  the crate payload.
- Aligns Homebrew tap generation and maintainer guidance with platform-specific
  GitHub Release archives and checksums.

## [0.1.5] - 2026-06-28

- Adds a release preflight gate for tag, Cargo, npm, and main-tip alignment.
- Makes CD release metadata failures explain the mismatch before packaging or
  publishing starts.
- Bumps the release candidate metadata to 0.1.5.
- Keeps the 0.1.5 UX polish release notes intact for the recovered release.

## [0.1.4] - 2026-06-27

- Adds missing CLI tests to kill surviving mutation-test mutants.
- Adds `mutants.toml` to exclude legacy compat wrappers from mutation scan.
- Restores README badges and promo image from v0.0.x header.
- Fixes CI mutation gate to pass --config mutants.toml.
- Fixes file-length and formatting issues found by verify.sh.
- Commits promo image under docs/ for stable relative-path reference.

## [0.1.3] - 2026-06-27

- Removes the embedded `terminal-jarvis-bin` payload from npm release staging.
- Makes the npm wrapper resolve prebuilt Terminal Jarvis binaries from GitHub
  Releases with checksum verification instead of shipping a native binary in
  the npm package.
- Adds distribution payload checks so npm staging fails if it includes the old
  embedded binary or known harness executables.
- Adds `--version`, `-v`, `--info`, and `version --verbose` provenance output.
- Replaces missing catalog `os error 2` output with catalog-path guidance.

## [0.1.2] - 2026-06-27

- Restores compatible tool-manager command forms on the v0.1 catalog CLI:
  direct harness invocation, `run <harness>`, free-form headless prompts,
  `install`, `update`, `info`, `auth`, `config`, `cache`, and `security`.
- Expands help and capability errors so users can discover the catalog model.
- Keeps npm `latest`, `stable`, and `beta` channels synchronized during tag CD.

## [0.1.1] - 2026-06-26

- Publishes the npm package with the repository root README.
- Keeps the tag-driven release workflow on patch increments for release and
  packaging repairs.
- Restores the npm release recovery workflow to the current package layout.

## [0.1.0] - 2026-06-26

- Starts the breaking minor revision around a data-driven harness catalog.
- Prunes the pre-rewrite implementation from the PR to keep review focused on
  the v0.1 root.
- Removes the Go ADK from the new root architecture.
- Adds explicit Rust contracts for harnesses, commands, and capabilities.
- Promotes the initial 25-tool catalog into the new harness descriptor shape.
- Adds harness-level auth environment modes for setup guidance.
- Adds a single verification script for formatting, linting, tests, catalog
  shape, CLI smoke checks, security checks, and optional coverage/mutation gates.
- Adds minimal npm and Homebrew source-build surfaces for the new CLI.
