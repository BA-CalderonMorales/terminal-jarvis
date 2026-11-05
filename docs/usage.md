# Usage

## Two ways to run

| Mode | Invocation | Best for |
|---|---|---|
| **Headless** | `terminal-jarvis <command>` / `tj <command>` | Scripts, CI, one-shot tasks |
| **Interactive** | `tj tui` (bare `tj` on a terminal) | Switching between coding agents live |

Headless runs one command per invocation, prints stable line-oriented output
with `--plain`. Noninteractive state-changing runs require their exact
`--no-input --confirm=<capability>:<agent>` token.

The interactive tui is a command center. `list` opens the fleet as an overlay:
`↑`/`↓` move the cursor, typing narrows the list live, `Enter` switches, `Esc`
closes, and digits still jump by picker position while the filter is empty.
`list <text>` opens with the filter already applied. The status rule under the
header always names the active agent, fleet readiness, and gate posture, so the
frame never hides what state it is in.

The prompt is vim-flavoured: `Esc` enters normal mode (`j`/`k` and
PageUp/PageDown scroll, `g`/`G` jump to the ends), `i` or `Esc` returns to
typing, `Tab` completes command and agent names, `?` opens help, and `q`
leaves. A terminal that shrinks below the frame says so instead of silently
switching to the line prompt, and a first-use conversation notice is accepted
with a bare `Enter`. A slashed name (`/codex`) launches an agent behind the same
guarded plan-plus-confirm as headless. Ctrl+C aimed at a running agent never
kills the switcher.

`uninstall <harness>` prints the removal command and waits for an explicit
`y`. The tool's own uninstaller runs when its catalog entry documents one;
otherwise the install channel's inverse runs (npm, uv, or cargo). The binary is
re-checked afterward, so a copy installed by another channel is reported as
incomplete instead of being called removed. See
[harness sources](harness-sources.md) for the channel each agent installs from.

`graveyard` is the retirement ledger: every catalog harness whose upstream
vendor stopped maintaining it, dated, with what happened and the successor the
vendor names. It is available headless (`--plain`, `--json`), in the tui
command table, and marked in the fleet picker with a `[sunset]` chip. Retired
tools stay installable and removable -- the catalog tells the truth about the
tool, including how to take it off a machine -- but they are counted apart from
the maintained fleet and never counted as ready.

## Automation

`--json` wraps every command in one envelope
(`{"schema_version":1,"command":…,"ok":…,"exit_code":…,"data":…,"error":…}`).
`list`, `show`, and `plan` publish the catalog as structure beside the rendered
`text`: `harnesses[]` with `name`, `display`, `description`, `binary`, `setup`,
`support` counts, and capability names; `graveyard` publishes `retired[]` with
`sunset.at`, `sunset.note`, `sunset.replaced_by`, and `binary_on_path`, plus the
catalog `total`; `show` adds each capability's
`support`, `evidence`, `effect`, `network`, `interaction`, `platforms`,
`command_line`, `source`, `verified_at`, and `package`; `plan` adds the argv as
`command[]`. `check` publishes diagnostics the same way. A tool can therefore
decide what to run from fields instead of re-parsing prose, while `text` on its
own stays exactly what a human would have seen.

The demo recording, the full agent-handover script, and how to make new demos:
[Demo](demo.md).

## Catalog ownership

Default discovery uses catalogs installed beside the executable or in its
installation prefix, then bundled data. A repository's `harnesses/` and
`gates/` directories never override these defaults automatically.

For an intentional development or custom catalog, set
`TERMINAL_JARVIS_CATALOG=/absolute/path/to/harnesses` or
`TERMINAL_JARVIS_GATES=/absolute/path/to/gates`. Relative explicit paths are
also supported. Invalid explicit overrides fail without silently falling back.
`version --verbose` and `check` identify the selected catalog, including
`embedded:terminal-jarvis` for bundled data.

## Experimental conversations

In the viewport, `converse <turns> <a> <b> <topic...>` starts a conversation
limited to 1–12 turns. Read the first-use notice and repeat the command to
accept it. Every turn passes the ordinary support, freshness, platform,
intent, and workspace-gate checks. State-changing turns ask for consent.
For agents whose headless operation is dangerous, explicitly use
`converse --allow-dangerous <turns> <a> <b> <topic...>` and review each turn's
confirmation. The first-use notice does not authorize future dangerous runs.

Ctrl+C or Esc stops the current turn and ends the conversation. The prompt
remains usable, including cancellation of a subsequent install. Each reply is
limited to 256 KiB; the requested 80-word response length and “No tool use”
remain model instructions, not enforced upstream restrictions. A turn limit
bounds invocation count, not token cost or runtime.

Piped invocations own ordinary descendant processes. Unix uses a separate
session/process group; Windows uses a Job Object assigned before the suspended
child starts. Deliberately detached Unix descendants can escape a process
group. This lifecycle management is not an execution sandbox.
