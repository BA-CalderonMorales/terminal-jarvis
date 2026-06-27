# terminal-jarvis npm wrapper

This is the npm launcher for Terminal Jarvis.

The package does not include a native binary. Installed npm copies download the
matching Terminal Jarvis archive from GitHub Releases, verify the release
`.sha256` checksum, cache the unpacked binary, and execute it.

In source checkouts it delegates to:

1. `TERMINAL_JARVIS_BIN`
2. `target/release/terminal-jarvis`
3. `target/debug/terminal-jarvis`
4. `cargo run --` from the repository root

Use `TERMINAL_JARVIS_CACHE` to choose the cache directory. Set
`TERMINAL_JARVIS_NO_DOWNLOAD=1` to require an already-cached binary.

Run the local smoke check:

```bash
npm --prefix npm/terminal-jarvis run smoke
```
