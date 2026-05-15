# Install Latency Benchmarks

Run the repeatable install benchmark from the repository root:

```bash
scripts/benchmarks/install-latency.sh
```

The default run measures cold and warm npm/npx installs plus `cargo install` as
a baseline using isolated cache, prefix, root, and target directories under
`tmp/install-latency/`.

Homebrew is opt-in because it modifies the real Homebrew prefix:

```bash
scripts/benchmarks/install-latency.sh --include-brew --allow-brew-uninstall
```

Add `--clear-brew-cache` when the machine can safely remove cached Terminal
Jarvis Homebrew downloads before the cold run.

Outputs:

- `summary.tsv`: per-case duration, exit code, command, and log path.
- `context.txt`: OS, architecture, WSL distro, Node, npm, Cargo, Rust, and
  Homebrew versions.
- `signals.txt`: npm timing log paths plus postinstall and Homebrew phase
  markers for bottleneck analysis.

Latency targets are printed by `scripts/benchmarks/install-latency.sh --help`.

Fast validation:

```bash
scripts/benchmarks/test-install-latency.sh
```

That test covers the generated matrix, context writer, help text, and a mutation
smoke check that removes an expected benchmark entry and verifies the self-test
catches it.
