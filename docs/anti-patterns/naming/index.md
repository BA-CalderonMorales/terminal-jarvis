# Naming Anti-Patterns

File names that encode bookkeeping (plan phases, run dates, author initials)
instead of durable purpose.

## Entries

### Ephemeral phase numbering in committed file names (docs/, scripts/, tests/)

**Pattern:** Files like `phase03-adversarial-report.sh`, `phase03-parity.sh`,
`phase03_catalog_walk_tests.rs`, `phase-01-*.md`, and the `phase02_*` /
`phase03_*` test files, fixtures, and support modules in `tests/` were named
after the plan phase that produced them and committed to remote.

**Why it is wrong:** A phase number is a point-in-time bookkeeping label, not a
durable name. It drags the plan's internal cadence into the product surface,
orphans the file the moment the phase closes, threads a counter through every
reference, and leaks the phase vocabulary into CI job names and artifact names.
It reads as provisional CI scratch, not owned engineering.

**Fix:** Name files by their durable purpose, never by the step that produced
them. `adversarial-report.sh`, `catalog-report.sh`, and `parity-catalog.sh`
name what the script does; test files name the behavior they witness
(`exit_matrix.rs`, `catalog_walk.rs`). Phase-named content stays inside
`plan/`, which is developer-local and never merges to `develop` or `main`.
The phase vocabulary also disappears from workflow and artifact names.
