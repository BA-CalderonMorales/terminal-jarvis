# Security

Terminal Jarvis is a launcher for tools that can execute commands and edit
files. Treat every harness descriptor as executable policy.

## Local Checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
scripts/bash/verify/index.sh security-check
```

`scripts/bash/verify/index.sh security-check` uses `cargo audit` and `cargo deny` when they are
installed. They are optional for local development, but release work should run
both before publishing.

## Yolo Mode

The `yolo` capability must be documented as dangerous in every harness. It is
not selected by default and should only be run after reviewing the command that
`terminal-jarvis plan <harness> yolo` prints.
