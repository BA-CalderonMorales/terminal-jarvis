# Troubleshooting

Terminal Jarvis prints the meaningful tail of an error, not the whole log:
start from the last few lines, match a symptom below, and follow the happy
path. Most issues are environmental -- a broken local entry, a missing
scanner, a slow mount -- not bugs in Terminal Jarvis.

## Happy path first

Run `terminal-jarvis` from your project directory, not your home directory.
The optional gate scans the working directory, and a home directory
(AppData, OneDrive, caches) makes the walk slow or fragile. Everything below
assumes you read the last lines of the error before looking further.

## Security scan blocked or failed

Symptom: `security scan (trivy): blocked` or
`security gate 'trivy' blocked harness execution (exit 1)`. Read the tail
and match it:

| Tail says | What it means | Fix |
|---|---|---|
| HIGH/CRITICAL findings listed | Real vulnerabilities or misconfigurations in the working tree | Review the report; `y` continues for that one action only; add `.trivyignore` only for reviewed, documented exceptions |
| `Fatal error ... walk dir error ... no such file or directory` naming a path | The scanner hit a broken filesystem entry it can list but not open | Windows delete-pending entries (names starting with `~`) cannot be opened from WSL. Reboot Windows once -- they self-delete -- or run Terminal Jarvis from a narrower directory |
| `warning: optional gate 'trivy' is enabled but 'trivy' is not on PATH` | The scanner is missing, so nothing is scanned | Install Trivy (see the warning hint) or `terminal-jarvis gate disable` |

Symptom: the scan seems to hang for minutes. Slow trees (OneDrive mounts,
WSL over Windows folders) can take minutes; 0.1.16 and later redraw a
heartbeat (`security scan (trivy) ... 5s scanning workspace`) so it reads as
progress. Ctrl+C no longer cancels the command: it asks
`Skip the scan and continue with <capability>:<harness>? [y/N]`, and `y`
proceeds for that one action only. Piped and `--no-input` runs still abort.

## Install or update blocked

Symptom: `error: installed <tool> blocked`. The pre-install package check
found HIGH/CRITICAL findings in the tool's dependency tree, and the download
is held until you consent. Review the printed report and answer `y`, or in
headless mode pass `--no-input --confirm=package-<capability>:<tool>`.

Symptom: an install warns `cannot be pre-scanned`. Tools installed by custom
scripts (curl|bash, pip, uv) have no lockfile to scan; the warning is
expected and the install continues.

## Deliberate, temporary bypasses

- `terminal-jarvis gate disable` -- persists until you re-enable.
- `TERMINAL_JARVIS_GATE=off` -- bypasses the stored selection for a single
  process.
- Run Terminal Jarvis from the project directory instead of home -- a smaller
  scan root scans faster and hits less junk.

## Still stuck

Open an issue at https://github.com/BA-CalderonMorales/terminal-jarvis/issues
with the exact error tail, your operating system, and the output of
`terminal-jarvis version --verbose`.
