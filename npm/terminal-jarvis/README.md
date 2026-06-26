# terminal-jarvis npm wrapper

This is the minimal npm surface for the harness-catalog rewrite.

It does not bundle release binaries yet. In this branch it delegates to:

1. `TERMINAL_JARVIS_BIN`
2. `bin/terminal-jarvis-bin`
3. `target/release/terminal-jarvis`
4. `target/debug/terminal-jarvis`
5. `cargo run --` from the repository root

Run the local smoke check:

```bash
npm --prefix npm/terminal-jarvis run smoke
```
