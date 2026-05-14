# ADR-001: Config Source Of Truth

Status: Proposed
Date: 2026-05-13
Issue: #31

## Context

Terminal Jarvis currently has two configuration surfaces for tool metadata:

- shipped TOML files under `config/tools/*.toml`
- runtime database state managed through the libSQL/Turso-backed `src/db/` layer

`src/tools/tools_db_bridge.rs` bridges those surfaces by reading database-backed
tool data first and falling back to TOML config when the database is not seeded.
That migration bridge keeps existing users working, but it makes contribution
and debugging paths ambiguous:

- contributors do not have one obvious place to add or review shipped tool metadata
- runtime updates need persistence without editing packaged files
- NPM packaging still needs deterministic copied config files
- README and docs generation need a stable catalog source

## Decision

Adopt a `ConfigStore` boundary and keep the storage responsibilities explicit:

- Shipped catalog metadata is authoritative in versioned TOML files.
- User and runtime state is authoritative in the database.
- Runtime tool overrides are persisted in the database as overlays on top of the shipped catalog.
- Reads go through a `ConfigStore` API that merges shipped catalog metadata with user/runtime overlays.
- Packaging and docs generation continue to read from the shipped TOML catalog until they are moved behind the same `ConfigStore` API.

This chooses the unified API direction rather than a pure database-first or
TOML-only model. TOML remains easy to review in PRs, while the database remains
the right place for local state, imported credentials, installation state,
preferences, and user-specific overrides.

## Scope

This ADR documents the direction only. It does not remove
`src/tools/tools_db_bridge.rs`, change the database schema, or migrate runtime
code in this PR.

## Consequences

Positive:

- Contributors keep a simple version-controlled catalog for shipped tools.
- Runtime state can evolve without mutating installed package files.
- The NPM package can keep deterministic copied configs during the transition.
- Future dynamic README count generation can read the same catalog boundary.

Tradeoffs:

- A merge layer is still required until every caller moves behind `ConfigStore`.
- Database-first features must distinguish between shipped metadata and local overrides.
- Tool update commands need clear semantics: package/catalog updates are not the same as local runtime-state updates.

## Migration Path

1. Define a small `ConfigStore` trait for listing tools, reading one tool, and reading user/runtime overrides.
2. Implement a TOML-backed catalog reader behind that trait.
3. Implement a database-backed overlay reader behind that trait.
4. Replace direct calls to `tools_db_bridge.rs` and raw TOML loaders with the trait.
5. Move README/docs generation to use the same shipped catalog reader.
6. Retire `tools_db_bridge.rs` only after callers no longer depend on bridge-specific fallback behavior.

## Open Questions

- Should user-created tool definitions live as database-only custom tools, or as exported TOML snippets that can be committed?
- Should package-manager update metadata allow runtime overrides, or stay shipped-catalog only?
- Which command should expose catalog refresh or re-seeding without implying package publication?
