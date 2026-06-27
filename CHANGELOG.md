# Changelog

## [0.1.2] - 2026-06-27

- Restores compatible tool-manager command forms on the v0.1 catalog CLI:
  direct harness invocation, `run <harness>`, free-form headless prompts,
  `install`, `update`, `info`, `auth`, `config`, `cache`, and `security`.
- Expands help and capability errors so users can discover the catalog model.
- Keeps npm `latest`, `stable`, and `beta` channels synchronized during tag CD.

## [0.1.1] - 2026-06-26

- Publishes the npm package with the repository root README.
- Keeps the tag-driven release workflow on patch increments for release and
  packaging repairs.
- Restores the npm release recovery workflow to the current package layout.

## [0.1.0] - 2026-06-26

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
