# Changelog

## [0.1.0] - Unreleased

- Starts the breaking minor revision around a data-driven harness catalog.
- Prunes the pre-rewrite implementation from the PR to keep review focused on
  the v0.1 root.
- Removes the Go ADK from the new root architecture.
- Adds explicit Rust contracts for harnesses, commands, and capabilities.
- Promotes the initial 25-tool catalog into the new harness descriptor shape.
- Adds harness-level auth environment modes for setup guidance.
- Adds a single verification script for formatting, linting, tests, catalog
  shape, CLI smoke checks, security checks, and optional coverage/mutation gates.
- Adds minimal npm and Homebrew source-build surfaces for the new CLI.
