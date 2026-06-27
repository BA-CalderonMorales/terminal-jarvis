# Terminal Jarvis Docs

This repository now centers on the new Rust harness catalog CLI.

The v0.1 minor line intentionally breaks old interfaces so the project can
reduce build time, remove the Go ADK experiment from the active root, make
package hygiene clearer, and keep release confidence tied to compact checks.

The **command and capability contract tables** live in the root
[README.md](../README.md) -- that is the source of truth for the tool's
surface and every harness capability set.

## Documents

- [[architecture|Architecture]]
- [[distribution-hardening|Distribution Hardening]]
- [[migration|Migration]]
- [[release-plan|Release Plan]]
- [[testing|Testing]]
