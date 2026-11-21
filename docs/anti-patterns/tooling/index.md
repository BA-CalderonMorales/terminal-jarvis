# Tooling Anti-Patterns

Reaching for a slower or hand-rolled tool when a standard, installed one
exists.

## Entries

### A shared-state lock that only the writer holds (0.1.25 flake sweep)

**Pattern:** `ENV_LOCK` protects process-global environment variables, and the
tests that *set* `TERMINAL_JARVIS_GATES` take it. A test that only *reads* that
state -- streaming a child, which resolves the gate policy -- took nothing, and
inherited a fixture gate named `blocked` from a parallel test. It failed as
"security gate blocked harness execution" with a fixture it never wrote, on
maybe one run in ten, only under CPU load.

**Why it is wrong:** A lock has one meaning: every access to the protected state
goes through it. Writer-only locking leaves the race that the lock exists to
close, and it burns the most time -- the failure is rare, load-dependent, and
points at a subsystem the failing test has no relationship with, so it reads as
a real bug in the gate.

**Fix:** Take the lock on both sides, including read-only consumers of the same
process-global state, and say in the test why. When a flake survives a stress
run, reproduce it under synthetic load before touching the assertion; a
"flaky test" is often a test observing state another test owns.

### A fresh executable plus a marker wait that guesses the child (0.1.25 flake sweep)

**Pattern:** The `gate_skip` PTY tests wrote a `fixture-gate-kill` script and
executed it moments later, inside a test process whose other threads fork
children. A concurrent fork inherits the still-open write fd until its own
exec, so the kernel answers the fixture's exec with ETXTBSY. The scan then read
as "missing" rather than interrupted: the skip prompt never appeared (the wait
timed out) or `--no-input` exited with the wrong code. About 2% of execs in a
standalone probe under CPU load; it never reproduced in isolation.

**Why it is wrong:** Time-guessing (a sleep, a short deadline) was the first
suspect and the wrong one: waiting for an observable marker with a generous
deadline is necessary, but it cannot rescue a child that never starts. The
failure also read as a product bug in the gate.

**Fix:** Do not exec a file the test just wrote when a system binary can do the
job (`sh -c 'kill -TERM $$'`); wait on markers, never on sleeps; and reproduce
under synthetic load (`yes > /dev/null` per core) before changing a deadline.

### Terminal tests inherit the operator's UI mode (0.1.23)

**Pattern:** PTY tests inherited `NO_COLOR`, used the GET ioctl while trying
to set a window size, and waited for markers from a UI mode they never opened.
Failed tests could leave a child running.

**Why it is wrong:** A green run can exercise the fallback instead of the
screen under test. An environment change produces failures unrelated to the
intended behavior, and orphaned fixtures interfere with later work.

**Fix:** Set terminal variables and geometry explicitly, choose viewport or
chat per scenario, bound waits, and own cleanup through panic and timeout.


### Release logic embedded in workflow YAML cannot be rehearsed

**Pattern:** The npm publish body -- version guard, publish/retag, and the
poll-to-convergence confirmation -- lived inside the `run:` block of
`.github/workflows/cd-multiplatform.yml`. Its only execution was a real release
tag, and the commit that fixed it asserted an old-fails/new-passes proof in its
message that was never committed.

**Why it is wrong:** Logic that runs only on a tag cannot be reviewed or
rehearsed, so the v0.1.20 release published successfully and then failed its own
single-read verification. A fix whose evidence is prose instead of an artifact
is a claim held up by nothing -- the same failure the catalog claim inventory
exists to prevent, one layer up.

**Fix:** Keep release logic in `scripts/bash/release/logic/` behind the domain
index and have the workflow call it. Prove it with a re-runnable artifact
(`scripts/bash/verify/index.sh publish-rehearsal`) that runs the shipped bytes
against a fake registry and shows the pre-fix body failing. Extract workflow
steps by name so the proof cannot drift from the workflow.

### Substring filters recursively launch a test's parent

**Pattern:** A subprocess invokes the current Rust test binary with a short
filter such as `heartbeat_probe`, also matching the parent's module path.

**Why it is wrong:** The child runs the parent again, recursively spawning
test binaries until memory, process limits, or the CI deadline is exhausted.
Reducing thread counts or increasing job timeouts masks the cause.

**Fix:** Use `--exact` with the fully qualified probe name, assert child
success and one executed test, and check selection with `--list` first.

### Plain grep when ripgrep is available (docs/)

**Pattern:** Content searches were issued with plain `grep -r` even where
`rg` was installed.

**Why it is wrong:** ripgrep honors `.gitignore`, skips binaries, and is much
faster on large trees; `grep -r` drags in vendored and ignored directories and
returns stale noise. The habit degrades search quality on every repo.

**Fix:** Use `rg` (ripgrep) for every content search. Fall back to plain
`grep` only when `rg` is not installed.

### Forgetting fzf for interactive selection (docs/)

**Pattern:** Interactive or terminal flows reach for hand-rolled pickers and
ad-hoc filters when a fuzzy finder is available on PATH.

**Why it is wrong:** A custom filter duplicates an installed, tuned tool and
produces a worse interaction for the reader.

**Fix:** Leverage `fzf` (fuzzy finder) for interactive selection of files,
lines, git objects, and history when it is present -- commonly piped from `rg`
(e.g. `rg <pattern> | fzf`). Write a custom picker only when `fzf` is absent.

### let-command-status leak kills set -eu pipelines (scripts/)

**Pattern:** `var=$(find ... | sort | while read -r x; do ...; done)`
where what looks like a failure-path-only exit (a trailing
`[ cond ] && echo` whose condition is false, or `read` hitting EOF)
becomes the loop's exit status, which becomes the whole command
substitution's status, which `set -eu` treats as a hard abort.

**Why it is wrong:** The script dies silently between two stages with no
error text; the failure is content-dependent (it passed until the last
sorted file in the tree happened to sit under the guard line). This one
ate half the verify pipeline in 0.1.14 pre-PR.

**Fix:** End the pipeline with `done || true` when the while body's last
command is conditionally non-zero, or make the body's final statement
unconditionally succeed. Reproduce deliberately when diagnosing invisible
`set -e` exits: the loop body's last command is the loop's status.
