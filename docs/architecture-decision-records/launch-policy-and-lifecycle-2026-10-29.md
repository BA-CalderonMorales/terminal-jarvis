# Apply one launch policy and own each streamed invocation

Prepared for `release/0.1.23` -> `develop`, 2026-10-29.

**Decision:** Conversation turns use the ordinary support, freshness, platform,
intent, and workspace-gate checks before receiving an authorized spawn. A
conversation needs its own explicit dangerous opt-in and still confirms each
state-changing turn. Remove the unused one-shot bypass. Defaults use installed
or embedded command and scanner catalogs; workspace overrides are explicit.

Freshness compares against the runtime UTC date. Schema validation keeps stale
rows inspectable; execution policy blocks them. Retain the original evidence
dates and downgrade expired verified claims to expected. Test fixtures stamp
their creation date; boundary tests inject a reference date directly, without
a production clock override. Generated claims identify the runtime gate.

A streamed invocation owns its process tree until drop. Unix uses a private
session and group signal. Windows creates the child suspended, assigns it to a
kill-on-close Job Object, then resumes its primary thread. A failure before
assignment/resume kills and reaps the child. Interactive inherited-terminal
launches retain their controlling terminal and receive the default SIGINT.

Use two bounded reader pumps and a bounded channel. Split long output lines
into chunks and cap a conversation reply at 256 KiB. A completed parent cannot
leave readers waiting forever on ordinary descendants. Conversation and
subsequent install cancellation read from the same input owner.

**Context:** The conversation route checked support and spawned directly,
skipping ordinary intent and gate checks. Its loop drained interruption keys
as navigation. Workspace discovery could replace command or scanner policy
without an explicit override. Freshness was frozen at August 5 despite the
30/90-day policy. PTY tests inherited operator settings and leaked processes
when marker waits failed.

**Considered:** Keep separate guards and copy missing checks: rejected because
the two routes could drift again. Renew timestamps at release time: rejected
because a release does not verify upstream behavior. Add a workspace-trust
registry: deferred because explicit overrides solve this boundary with less
state. Kill only the immediate process: rejected because descendants retain
pipes and can keep running. Assign Windows jobs after starting the child:
rejected because it leaves a descendant-creation race.

**Consequence:** The stricter policy can refuse commands formerly allowed by
stale evidence or workspace discovery. There are no newly verified upstream
integrations in this release. Expected claims still expire after 90 days and
need an ongoing verification cadence. Process groups manage ordinary Unix
children; an agent deliberately creating a separate session can escape them.
Neither lifecycle ownership nor prompt wording is a sandbox. The turn limit
bounds calls; word count is advisory and does not bound spend.

Verification uses fake binaries, descendants, blocked gates, date boundaries,
PTY cancellation followed by another command, platform compile/lint checks,
and the existing native Windows CI suite. Real-agent installation and paid
model calls remain disposable-environment work, outside these regressions.

## Mutation follow-up

Calendar tests cover every day in a Gregorian cycle, century leap boundaries,
and calibration against the system clock. Lifecycle tests distinguish killing
from reaping and exercise cleanup without the outer runner. Installed-package
tests verify search priority, rather than only the bundled fallback. Snapshot
captures fix their working directory before layout; normalization afterward
cannot undo a path-dependent layout choice.

The mutation workflow runs on Linux. The Windows-only process module is not
compiled there and has a documented exclusion; native Windows verification
still exercises suspended startup and descendant cleanup. The 99 changed
catalog mutations use eight partitions to fit the existing 30-minute job
budget without dropping cases. Literal chunk sizing and the exact POSIX
`setsid` error return avoid equivalent arithmetic/comparison mutations.

## Implementation references

- [Rust RAII guards](https://rust-unofficial.github.io/patterns/patterns/behavioural/RAII.html)
  motivates keeping cleanup in owned values and `Drop`.
- [Microsoft Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)
  describes descendant membership and kill-on-close semantics.
- [Windows process creation flags](https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
  documents `CREATE_SUSPENDED`.
- [ResumeThread](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)
  documents the thread access right and resume result.

- [Linux setsid contract](https://man7.org/linux/man-pages/man2/setsid.2.html)
  identifies the positive session ID on success and `-1` on failure.
