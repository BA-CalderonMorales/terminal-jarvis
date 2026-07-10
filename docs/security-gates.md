# Security Gates

Terminal Jarvis can run an optional local gate before it executes a coding-agent
harness. Gates are off by default and Terminal Jarvis never installs a scanner
or sends workspace data anywhere on its own.

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
Terminal Jarvis blocks harness execution with the installation link and a
disable command; it does not attempt an install.

## Configuration

`terminal-jarvis gate enable trivy` stores the selected gate in the Terminal
Jarvis config home. `TERMINAL_JARVIS_GATE` takes precedence for a single
process: set it to `trivy` to enable or `off` to bypass the stored selection.
`TERMINAL_JARVIS_GATES` can point advanced users at a replacement gate catalog.

The gate is a local quality signal, not a replacement for code review, scoped
credentials, or the release Trivy gate. Use `.trivyignore` only for reviewed,
documented exceptions.
