# Installation

## Package mechanics

Cargo builds the Rust CLI from the crates.io source package. The npm package is
a Node launcher that downloads the matching Terminal Jarvis GitHub Release
asset, verifies its `.sha256` file, caches it, and then executes it. Homebrew
installs the matching platform release archive from the tap. The npm install
links both `terminal-jarvis` and the shorter `tj` entry point to the same
launcher, so either command name works; `tj` keeps daily use fast.

## Supported platforms

Supported prebuilt assets are `linux-x64-gnu`, `linux-arm64-gnu`,
`macos-x64`, `macos-arm64`, and `win32-x64`. Native Windows npm installs use
the `win32-x64` ZIP bundle and work from Command Prompt, PowerShell, or Git
Bash. Every release also includes a direct native executable for each platform;
downloaded Linux and macOS executables may need `chmod +x` before use.

## Update behavior

An older Cargo or manual install can still win `PATH` resolution after a global
npm upgrade. The npm install now completes and prints the path-order fix rather
than blocking the upgrade; place the npm prefix before the stale location to run
the newly installed command.
