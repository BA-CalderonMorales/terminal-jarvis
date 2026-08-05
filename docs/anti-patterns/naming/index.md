# Naming Anti-Patterns

File names that encode bookkeeping (plan phases, run dates, author initials)
instead of durable purpose.

## Entries

### Ephemeral phase numbering in committed file names (docs/, scripts/)

**Pattern:** Files like `phase03-adversarial-report.sh`, `phase03-parity.sh`,
`phase03_catalog_walk_tests.rs`, and `phase-01-*.md` were named after the plan
phase that produced them and committed to remote.

**Why it is wrong:** A phase number is a point-in-time bookkeeping label, not a
durable name. It drags the plan's internal cadence into the product surface,
orphans the file the moment the phase closes, and threads a counter through
every reference. It reads as provisional CI scratch, not owned engineering.

**Fix:** Name files by their durable purpose, never by the step that produced
them. `adversarial-report.sh`, `catalog-report.sh`, and `parity-catalog.sh`
name what the script does. Phase-named content stays inside `plan/`, which is
developer-local and never merges to `develop` or `main`.
