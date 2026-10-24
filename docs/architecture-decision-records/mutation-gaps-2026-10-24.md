# Close the mutation gaps the nightly found, and re-anchor the exclusion that drifted

Branch `fix/mutation-gaps-from-develop` -> `develop`, 2026-10-24. Linked from
`README.md`.

**Decision:** The two surviving mutants a runner can kill get tests -- the auth
note's env-mode branch and the candidate list off Windows -- and the four
families a linux runner cannot observe get reasoned exclusions. The stale
`stream.rs` exclusion is re-anchored to the line its variant actually sits on.

**Context:** The scheduled Mutations run on `main` failed three nights running
on the same six shards: ten mutants survived across six files. Pull-request runs
only mutate the diff (30s), so every PR stayed green while the nightly was
red. One exclusion had drifted out of reach -- `stream.rs:12` matched nothing
after the file moved, so the `cfg(not(unix))` exit-code variant's three mutants
surfaced. The rest were real gaps in code that predates this branch: nothing
distinguished the two harness env modes in the auth note, and nothing ran the
candidate list off Windows.

**Considered:** A percentage threshold would stop a handful of survivors from
reddening `main`, but it hides exactly the class of gap that reddened the
0.1.20 release. Rewriting the excluded code to be killable -- a seam for the
clock, an injectable rename error -- changes production design for paths whose
behaviour no runner can observe. Leaving it red is honest but leaves the
nightly unable to say anything new.

**Consequence:** The nightly full-suite run goes green again, and the two new
tests pin behaviour the mutator was previously free to flip. The exclusions
carry their reasoning, and the re-anchored one names its line so the next shift
is visible. The nightly stays the gate that catches a stale exclusion: that is
how this one surfaced, three releases after it was written.
