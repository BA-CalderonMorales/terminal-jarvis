terminal-jarvis npm launcher
============================

This directory contains the executable Node wrapper exposed by package.json#bin.
It is not the Rust CLI binary.

What it does:
- Honors TERMINAL_JARVIS_BIN when you want to execute an explicit local binary.
- In source checkouts, delegates to target/release, target/debug, or cargo run.
- In installed npm packages, downloads the matching GitHub Release archive,
  verifies the archive .sha256 file, caches the unpacked release, and executes
  bin/terminal-jarvis from that cache.

Supported downloaded assets:
- linux-x64-gnu
- macos-x64
- macos-arm64

Native Windows npm installs are not supported until a win32-x64 release asset is
published. Use WSL on Windows.

Useful environment variables:
- TERMINAL_JARVIS_BIN: execute a specific local binary instead of downloading.
- TERMINAL_JARVIS_CACHE: choose the release cache directory.
- TERMINAL_JARVIS_NO_DOWNLOAD=1: require an already cached release.
- TERMINAL_JARVIS_RELEASE_BASE: override the GitHub Release asset base URL.

For source installs, use cargo install terminal-jarvis.
For binary installs, use Homebrew or this npm wrapper.
