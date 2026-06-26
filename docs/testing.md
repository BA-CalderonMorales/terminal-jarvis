# Testing

The baseline target is 90 percent or better line coverage and mutation score.
The v0.1 root starts by making behavior smaller and more testable.

The current tests enforce the expected 25-harness catalog, the full
nine-capability harness contract, non-interactive update plans, visible yolo
danger warnings, CLI parsing, context persistence, and runtime planning order.

## Current Gates

```bash
scripts/verify.sh
```

The script runs formatting, clippy, tests, the 100-line Rust file invariant,
harness catalog shape checks, CLI smoke checks, security tooling, npm audit,
npm wrapper smoke checks, Homebrew Formula syntax checks, and coverage/mutation
gates. It also checks release-package metadata without writing `dist/`
artifacts.

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

## Optional Gates

```bash
cargo llvm-cov
TJ_MUTATION=1 cargo mutants
scripts/security-check.sh
```

When `cargo-llvm-cov` is installed, `scripts/verify.sh` enforces the 90 percent
line coverage target. Mutation runs remain opt-in with `TJ_MUTATION=1` because
they are intentionally slower than the default CI path.
