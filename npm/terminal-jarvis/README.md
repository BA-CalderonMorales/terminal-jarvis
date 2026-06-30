# terminal-jarvis npm wrapper

This is the npm launcher for Terminal Jarvis.

The package does not include a native binary. Installed npm copies use
`bin/terminal-jarvis`, a Node shebang wrapper, to download the matching Terminal
Jarvis archive from GitHub Releases, verify the release `.sha256` checksum,
cache the unpacked release, and execute it.

Supported npm release assets are `linux-x64-gnu`, `macos-x64`, and
`macos-arm64`. Native Windows installs through Git Bash or cmd are not
supported until CI publishes a `win32-x64` asset; use WSL on Windows.

The wrapper guidance shipped at `bin/README.txt` is included in the npm package
so installed copies can explain the package behavior without relying on the
source checkout.

In source checkouts it delegates to:

1. `TERMINAL_JARVIS_BIN`
2. `target/release/terminal-jarvis`
3. `target/debug/terminal-jarvis`
4. `cargo run --` from the repository root

Use `TERMINAL_JARVIS_CACHE` to choose the cache directory. Set
`TERMINAL_JARVIS_NO_DOWNLOAD=1` to require an already-cached binary.

Use `TERMINAL_JARVIS_RELEASE_BASE` only for a release mirror that preserves the
exact package version and checksum files.

Run the local smoke check:

```bash
npm --prefix npm/terminal-jarvis run smoke
```
