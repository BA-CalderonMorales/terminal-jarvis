# Changelog

## [0.1.17] - unreleased

- Full-viewport TUI: on terminals that can hold it (50+ cols, 10+ rows, ANSI),
  the switcher now paints a bordered command-center frame -- title, live
  status row, zoned body, footer prompt with an embedded hint -- and
  repaints in place after every command, so nothing scrolls. Child runs
  (install/update/run/direct) suspend the frame and own the real terminal;
  a terminal that shrinks below the floor falls back to chat mode.
- Viewport hardening: catalog strings are sanitized before they reach the
  frame (color survives; OSC/cursor/query sequences and control bytes do
  not), wide glyphs measure as two cells so borders never drift, signal
  interrupts no longer end the session silently, and idle Ctrl+C leaves the
  frame intact.
- `update --dry-run` (bare) prints the fleet update summary instead of a
  usage error.
- `security` usage errors name the valid forms:
  `usage: terminal-jarvis security [status|audit|<harness>]`.
- Unknown command/harness failures state their next action exactly once.
- Session WRITE failures map to `session_unwritable` (exit 3) advising the
  Terminal Jarvis home directory (`session_invalid` remains the code for
  unreadable/corrupt session files). A failed post-install adopt now warns
  instead of silently claiming success.

## Up next (roadmap, not yet started)
## [0.1.16] - 2026-09-02

- Windows: npm-style `.cmd` shims now resolve when spawning harnesses --
  `terminal-jarvis run <harness>` no longer fails with "not found on PATH"
  for npm-installed agents. The security preflight and the spawn path agree
  on the same binary, gate scans resolve `trivy` the same way, and the
  package check resolves `npm`/`trivy` through PATHEXT.
- The test suite compiles and passes on Windows; pty-driven TUI acceptance
  tests are Unix-only by design.

## [0.1.15] - 2026-08-14

- Headless gate scans are deadline-bounded: a stuck scanner is killed after
  the timeout (env-tunable, clamped to 24h) instead of hanging the command,
  and scan output is never truncated.
- `gate.toml` selection tolerates whitespace, inline comments, and malformed
  lines instead of silently disabling the gate.
- The jules harness reports its real version instead of stale data.
- Mutation CI threads are capped so the baseline no longer dies on the
  runner's thread limit.
- The Trivy walk skips Windows delete-pending files and directories (names
  starting with `~`), so the security gate no longer crashes mid-scan on
  NTFS.

## Up next: 0.1.16 (roadmap, not yet started)

- `status` will surface previously-blocked installs and why they were
  blocked, with a drillable view into the decision (finding details).
- A fast `uninstall|prune [harness]` verb to locate and remove a harness and
  its installed version on disk.

## [0.1.14] - 2026-08-12

- TUI clean frame: right-aligned one-row status (active harness, CWD,
  readiness) on the banner line; a `[>_]::[tj:0.1.14]::[harness:<name>]`
  context indicator that carries the version and agent (plus a `::[debug]`
  slot); home no longer dumps the tool list.
- Live install/update beats rewrite in place: `security scan (trivy)`/`
  package check` lines collapse to `: passed` / `: clean`, installs show a
  progress line, and every lifecycle ends in a green verdict card with
  timing -- including honest `binary not on PATH` warnings.
- A passed gate scan is memoized for the session, so `install` +
  immediately-after `run` performs exactly one scan (invalidated on
  `gate enable/disable`); the verdict also persists the binary-on-PATH
  check across processes.
- Unknown (curl-pipe) installers now install with an interactive intent
  prompt in the TUI instead of failing closed, with an explicit
  "cannot be pre-scanned" advisory; headless mode stays fail-closed.
- Session bracket: `── run <agent> ──` opens around a launch and exits with
  a recap (elapsed time, exit status) or the error, before the prompt
  redraws. Ctrl+C at idle redraws the prompt without a stray `^C`;
  a stuck gate scanner is hard-killed so the terminal always returns.
- `/debug on|off` toggles the narrated raw view, `help` documents the
  switcher in-terminal, and the prompt honors an empty response by staying
  put.
- Acceptance oracle: a pty harness with a tiny ANSI screen model now asserts
  six end-to-end scenarios -- badge layout, idle Ctrl+C, clean
  install+adopt, single-scan memoization, scanner interruption, and debug
  toggle -- while unit tests cover the remainder; 511 tests total.
- Demo pipeline: `scripts/demo/tui.tape` re-records `docs/demo-tui.gif`
  (animated, README primary) and `scripts/demo/capture_frame.sh` +
  `render_frame.go` produce the registry still `docs/demo-tui.png` with the
  stock Windows Terminal (Campbell) palette; both reflect the 0.1.14 frame.

## [0.1.13] - 2026-08-06

- Interactive switcher, instant boot, and live readiness: `terminal-jarvis
  tui` (or bare `tj` on a terminal) opens a chat-style harness switcher where
  names and numbers switch agents, `home`/`clear` reset the frame, `status`
  shows a two-line fleet dashboard, and Ctrl+C aimed at a running agent never
  kills the session. Full command tables and the demo are in `docs/`.

- Brings back the interactive TUI as a small, line-based harness switcher:
  `terminal-jarvis tui` (or bare `tj` on a terminal) opens a clean welcome --
  banner, active harness, readiness, no tool dump -- over a `[>_]` prompt
  whose dimmed `Waiting for input...` hint sits below the line and disappears
  the moment a command commits. Slash commands parse with the exact headless
  grammar (`/list`, `/status`, `/use`, `/plan`, `/install`, ...), bare text
  runs the active harness headless, and a bare number selects a tool after
  `/list` shows it. All commands share the headless intent, dangerous, and
  interactive guards, and the terminal-control sequences come from a new
  std-only `tui::term` layer instead of an external crate.
- Status and diagnostics report live installed harness versions from bounded
  read-only probes, and empty platform claims count as unrestricted in
  readiness the same way they do in the execution guard.
- Elevates the harness catalog out of the fail-closed checkpoint: determined
  documented commands become expected claims, the six locally installed
  harnesses carry verified version probes with disposable-real evidence,
  `--help` fallback probes stay stubs, curl-pipe installers stay unknown, and
  every yolo row is disabled. Freshness evidence is re-stamped to the catalog
  revision date.
- Treats an empty platform claim as unrestricted, so package-manager-driven
  capabilities stop being rejected on every platform while explicit claims
  are still enforced.
- Identifies source-tree builds as the `source` channel (including coverage
  target dirs), reports update.route as cargo, lists shadowing PATH binaries,
  and marks state.cache not-applicable outside the npm launcher.
- Renders tables legibly: proportional column floors, separator-aware
  wrapping for paths and keys, wrapped headers, and compact support summaries
  in `list`.

## [0.1.13] - 2026-08-06 (continued)

- Replaces broad harness claims with a generated 225-row support matrix whose
  support state, evidence, platform, freshness, and side effects come directly
  from the packaged catalog; no first-class harness is promoted without current
  disposable-real evidence.
- Makes `check` the sole diagnostic surface, adds canonical `self-update` intent
  and PTY confirmation behavior, and withholds executable update commands for
  guarded capability rows.
- Hardens the npm native cache with target, architecture, archive, binary,
  catalog, gate, checksum, and source identity plus atomic staged recovery.
- Adds exact-ref development/staged parity, all-descriptor pre-spawn guard,
  redaction, stream/signal, lifecycle, corruption, and recovery evidence.
- Adds a read-only, nonpublishing five-native-target candidate workflow and a
  deterministic offline simulated evaluation kit; publication remains a
  separate explicit operator decision.

## [0.1.12] - 2026-07-09

- Restores non-blocking global npm upgrades when an older Cargo or manual
  `terminal-jarvis` appears first on `PATH`. The install completes and emits
  actionable shadowing guidance instead of leaving the new package uninstalled.
- Adds direct native executable assets for every supported platform. Windows
  also receives a ZIP bundle, and the npm launcher uses it with PowerShell
  extraction and a Windows-native cache location.
- Adds an opt-in Trivy filesystem gate. Enable it with
  `terminal-jarvis gate enable trivy` to scan the current workspace for high
  and critical vulnerabilities, secrets, and misconfigurations before a
  harness command executes; it is disabled by default and never installs Trivy.
- Adds `terminal-jarvis --update --dry-run` for safe cross-platform verification
  of the package-manager update route.
- Adds a feature-gated, noninteractive dashboard preview behind
  `TERMINAL_JARVIS_EXPERIMENTAL_UI=1` while keeping the default CLI headless.
- Adds a release-time core-command matrix to exercise every public command
  surface on each host-native package build without launching coding agents.

## [0.1.11] - 2026-07-09

- Adds real multi-platform release packaging for Linux x64, Linux ARM64,
  macOS Intel, macOS ARM64, and Windows x64 GitHub Release assets.
- Adds native Windows npm launcher support through the `win32-x64` release
  asset and validates cached Windows PE binaries.
- Makes global npm install fail when an older `terminal-jarvis` earlier on
  `PATH` would keep `terminal-jarvis@latest` from being the command users run.
- Lowers the npm wrapper engine requirement to Node `>=18.17`, matching the
  runtime surface used by the wrapper.

## [0.1.10] - 2026-07-08

- Fixes `-v version` to delegate to the `version` subcommand instead of
  erroring, so `terminal-jarvis -v version` behaves consistently with
  `terminal-jarvis version -v`.
- Feature: `--version` now reports the distribution channel as a suffix,
  e.g. `terminal-jarvis 0.1.10 (npm)`, `(source)`, or `(homebrew)`. The
  channel is derived from `TERMINAL_JARVIS_DISTRIBUTION`,
  `TERMINAL_JARVIS_WRAPPER`, or the resolved binary path; when unset the
  plain `terminal-jarvis <version>` line is printed.
- Feature: `--update` performs a self-update based on the distribution.
  npm/wrapped installs re-run `npm install -g terminal-jarvis@latest`,
  Homebrew installs run `brew upgrade terminal-jarvis`, and source/env
  installs run `cargo install terminal-jarvis`. The update runs before the
  harness catalog loads so it works even with a missing/broken catalog.
- Formalizes the headless invocation contract: every harness defines
  `headless/index.toml` with a command and args; `run <harness> <prompt>`
  routes to headless mode when prompt words don't match a reserved
  capability. Three headless patterns are recognized (direct exec,
  `--help` stub, interactive-only). Guidelines documented in
  `docs/harness-capability-contract.md`.
- Fix: failing harness commands now report the harness name, capability, exit
  code, and stderr so operators can diagnose broken `run` invocations directly
  from the error output.

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
