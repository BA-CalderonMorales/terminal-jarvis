# Testing

The baseline target is 90 percent or better line coverage and mutation score.
The v0.1 root starts by making behavior smaller and more testable.

The current tests enforce the expected 25-harness catalog, the full
nine-capability harness contract, non-interactive update plans, visible yolo
danger warnings, CLI parsing, context persistence, runtime planning order, and
artifact-level integration smoke checks.

## Current Gates

```bash
scripts/verify.sh
```

The script runs formatting, clippy, tests, the 100-line Rust file invariant,
harness catalog shape checks, CLI smoke checks, the integration hardening
matrix, security tooling, npm audit, npm wrapper smoke checks, Homebrew Formula
syntax checks, and coverage/mutation gates. It also checks release-package
metadata without writing `dist/` artifacts.

## Integration Hardening Matrix

```bash
scripts/integration-hardening.sh
```

This gate builds or receives a Terminal Jarvis binary, then exercises `help`,
`list`, `check`, `use`, `current`, `show`, and `plan` across every harness and
all nine capabilities. When an npm wrapper or Homebrew formula is present, it
also verifies the wrapper list path and formula syntax/installation contract.

The gate deliberately plans lifecycle commands instead of running arbitrary
harness `download`, `update`, `headless`, or `yolo` commands. Terminal Jarvis
must not automatically download or execute arbitrary harness dependencies
during verification. Deeper start/stop tests should run only in disposable or
remote environments with reviewed commands and scoped credentials.

For the stronger local gate, run:

```bash
scripts/local-ci.sh
```

Local CI layers repository hygiene, workflow linting, secret scanning, Socket
supply-chain checks, the standard verification gate, and release package smoke
checks. Missing optional local tools are reported as skips by default; pass
`--strict` when every configured security tool must be installed and runnable.
Pass `--mutation` for the slower mutation gate before opening or updating a PR.

For release asset smoke testing, run:

```bash
scripts/local-cd.sh --check-auth
```

That script delegates package creation to `scripts/package-release.sh`, collects
the same archive/checksum files that CD uploads, verifies checksums, and reports
GitHub, npm, and Cargo auth boundaries without printing secrets.
It also checks that each `.sha256` file names the archive basename so flattened
release assets work with `sha256sum -c`.

## Environment Safety

Use a remote or disposable Linux workspace when testing harness installation,
updates, headless execution, or yolo-style plans. These commands are designed to
orchestrate coding agents and local binaries, so the safest default is a
short-lived workspace with only the target repository mounted and only the
provider tokens needed for that test.

Docker images should be treated as extendable baseline environments, not magic
installers for every harness dependency. First-class Docker dependency coverage
should be limited to well-known harnesses such as OpenCode, Codex, Claude Code,
Gemini, and Hermes Agent. Other harnesses should get explicit limitations or
unsupported-OS messages instead of silent best-effort installation.

## Optional Gates

```bash
cargo llvm-cov
TJ_MUTATION=1 cargo mutants
scripts/security-check.sh
```

When `cargo-llvm-cov` is installed, `scripts/verify.sh` enforces the 90 percent
line coverage target. Mutation runs remain opt-in with `TJ_MUTATION=1` because
they are intentionally slower than the default CI path.
