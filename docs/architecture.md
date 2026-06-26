# Architecture

Terminal Jarvis is now data first.

## Boundaries

- `src/contracts/` defines the Rust contracts shared by modules.
- `src/catalog/` loads and validates harness descriptors.
- `src/context/` stores the active harness locally.
- `src/runtime/` plans and runs selected capability commands.
- `src/security/` checks local executable and environment readiness.
- `harnesses/` contains tool-specific policy.

## Core Capability Contract

Every harness must define:

- `download`
- `update`
- `headless`
- `version`
- `stats`
- `models`
- `security`
- `yolo`
- `ui`

The Rust code does not special-case Codex, OpenCode, or any future harness.
Adding support should usually mean adding `index.toml` files, not adding Rust.

## Setup Contract

Each harness root also declares:

- `binary`
- `env_mode`
- `env`

`env_mode` is `none`, `any`, or `all`. This lets Terminal Jarvis tell a user
whether no provider key is needed, one key from a list is enough, or every
listed variable is required.
