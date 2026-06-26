# Testing

The baseline target is 90 percent or better line coverage and mutation score.
This branch starts by making behavior smaller and more testable.

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

For the stronger pre-PR gate, run:

```bash
scripts/local-ci.sh
```

Local CI layers repository hygiene, workflow linting, secret scanning, Socket
supply-chain checks, the standard verification gate, and release package smoke
checks. Missing optional local tools are reported as skips by default; pass
`--strict` when every configured security tool must be installed and runnable.
Pass `--mutation` for the slower mutation gate before opening or updating a PR.

## Optional Gates

```bash
cargo llvm-cov
TJ_MUTATION=1 cargo mutants
scripts/security-check.sh
```

When `cargo-llvm-cov` is installed, `scripts/verify.sh` enforces the 90 percent
line coverage target. Mutation runs remain opt-in with `TJ_MUTATION=1` because
they are intentionally slower than the default CI path.
