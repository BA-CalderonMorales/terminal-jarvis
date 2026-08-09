# Security Gates

Terminal Jarvis can run an optional local gate before it executes a coding-agent
harness. Gates are off by default and Terminal Jarvis never installs a scanner
or sends workspace data anywhere on its own. With a gate enabled, `install` and
`update` also get a pre-install vulnerability check of the package behind the
tool.

## Pre-install package check

When a gate is enabled and the tool's plan carries a `package` key (the npm
registry tools), `install <tool>` and `update <tool>` resolve the package's
dependency tree into a lockfile (`npm install --package-lock-only`) in a
temporary directory and scan that lockfile with trivy. A clean verdict proceeds
silently. HIGH/CRITICAL findings print the trivy report and ask
"Continue installing/updating anyway? [y/N]" (default no); noninteractive runs
fail closed unless `--no-input --confirm=package-<capability>:<tool>` is given.
The package is downloaded only after a clean verdict or explicit override.

With no gate enabled, installs warn that they are not vulnerability-checked.
Tools installed by custom scripts (curl|bash, pip, uv) cannot be pre-scanned;
with a gate on, this is stated and the install continues. The package check
needs both `npm` and `trivy` on PATH; if either is missing it warns and
continues.

## Trivy

The bundled `trivy` gate scans the current working directory with Trivy's
filesystem scanner. It enables vulnerability, secret, and misconfiguration
scanners and blocks on HIGH or CRITICAL findings.

```bash
terminal-jarvis gate status
terminal-jarvis gate enable trivy
terminal-jarvis gate status
terminal-jarvis gate disable
```

After it is enabled, `run`, direct harness invocation, `install`, and
`update <harness>` scan before the harness command starts. Read-only commands,
plans, and catalog inspection do not scan. Run `terminal-jarvis gate run trivy`
to see the scanner output without launching a harness.

Install Trivy through the official method for your operating system. The
[Trivy installation guide](https://trivy.dev/docs/latest/getting-started/installation/)
covers Linux, macOS, and Windows. If Trivy is missing while the gate is enabled,
Terminal Jarvis warns on stderr and continues without scanning; it does not
block harness execution (a scanner that cannot run protects nothing) and never
attempts an install. The warning repeats so the setup path stays discoverable
until you install Trivy or run `terminal-jarvis gate disable`. `gate run trivy`
still reports the missing binary explicitly when you ask for a scan.

## Configuration

`terminal-jarvis gate enable trivy` stores the selected gate in the Terminal
Jarvis config home. `TERMINAL_JARVIS_GATE` takes precedence for a single
process: set it to `trivy` to enable or `off` to bypass the stored selection.
`TERMINAL_JARVIS_GATES` can point advanced users at a replacement gate catalog.

The gate is a local quality signal, not a replacement for code review, scoped
credentials, or the release Trivy gate. Use `.trivyignore` only for reviewed,
documented exceptions.
