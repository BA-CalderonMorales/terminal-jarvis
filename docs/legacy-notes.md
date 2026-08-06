# Legacy Notes

Compatibility surface and behavior notes for humans and scripts.

## Legacy aliases

Legacy aliases remain available:

- `tools` -> `list`
- `status` -> `check`
- `info <harness>` -> `show <harness>`
- `install <harness>` -> `run <harness> download`
- `update <harness>` -> `run <harness> update`

## Plain output

Human-facing commands use width-aware structured output and color only on an
interactive terminal. For scripts, put `--plain` before the command for stable
line-oriented output; `--no-color` keeps the structured layout without color.

## Removed experiments

The experimental dashboard behind `TERMINAL_JARVIS_EXPERIMENTAL_UI` was
removed with the v0.1 catalog rewrite. The interactive surface today is the
switcher: `terminal-jarvis tui` (bare `terminal-jarvis` on a terminal starts
it too). Setting `TERMINAL_JARVIS_EXPERIMENTAL_UI` has no effect.
