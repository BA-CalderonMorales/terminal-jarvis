# terminal-jarvis npm wrapper

This is the npm launcher for Terminal Jarvis.

The package does not include a native binary. Installed npm copies use
`bin/terminal-jarvis`, a Node shebang wrapper, to download the matching Terminal
Jarvis bundle from GitHub Releases, verify the release `.sha256` checksum,
cache the unpacked release, and execute it. Windows uses a ZIP bundle through
PowerShell; Linux and macOS use a tar.gz bundle.

Supported npm release assets are `linux-x64-gnu`, `linux-arm64-gnu`,
`macos-x64`, `macos-arm64`, and `win32-x64`. Native Windows installs through
Command Prompt, PowerShell, and Git Bash use the `win32-x64` ZIP asset and the
local application-data cache directory.

Linux assets require GNU libc. The wrapper checks the runtime before any
download and rejects musl/Alpine, Android/Termux, unidentified libc, and
unlisted architectures with the five supported asset names in its recovery
message. WSL2 with GNU libc selects the matching Linux GNU asset; that asset
selection is compatibility mapping, not a claim that the WSL environment was
verified. WSL1 is not claimed. The launcher requires Node 18.17 or newer.

The wrapper guidance shipped at `bin/README.txt` is included in the npm package
so installed copies can explain the package behavior without relying on the
source checkout.

In source checkouts it delegates to:

1. `TERMINAL_JARVIS_BIN`
2. `target/release/terminal-jarvis` or `target/release/terminal-jarvis.exe`
3. `target/debug/terminal-jarvis` or `target/debug/terminal-jarvis.exe`
4. `cargo run --` from the repository root

Use `TERMINAL_JARVIS_CACHE` to choose the cache directory. Set
`TERMINAL_JARVIS_NO_DOWNLOAD=1` to require an already-cached binary.

Use `TERMINAL_JARVIS_RELEASE_BASE` only for a release mirror that preserves the
exact package version and checksum files.

Run the local smoke check:

```bash
npm --prefix npm/terminal-jarvis run smoke
```
