# Terminal Jarvis Runtime Flow Map (Source Of Truth)

Last reviewed: `2026-02-21`

This document maps all runtime flows reachable from `src/main.rs` and captures the original design intent inferred from code and comments.

## 1) Boot Flow (`src/main.rs`)

`src/main.rs` is intentionally thin:

1. `Cli::new()` parses arguments (`clap`) in `src/cli.rs`
2. `cli.run().await` dispatches all behavior in `src/cli.rs`

Design intent: keep entrypoint minimal and push all behavior into testable domain modules.

## 2) Primary Dispatch Flow (`src/cli.rs`)

`Cli::run()` routes by top-level command.

### Top-level routes

| Input Shape | Route | Handler |
|---|---|---|
| `terminal-jarvis -q` with no subcommand | quick launch | `cli_logic::handle_quick_launch()` |
| `terminal-jarvis run [tool] [args...]` | explicit run path | `cli_logic::handle_run_tool(&tool, &args)` |
| `terminal-jarvis [tool] [args...]` where `[tool]` is valid | external-subcommand forwarding path | `cli_logic::handle_run_tool(tool_name, tool_args)` |
| `terminal-jarvis [unknown]` | hard failure path | prints invalid tool list, exits `1` |
| `terminal-jarvis` (no subcommand) | interactive shell path | `cli_logic::handle_interactive_mode()` |

### Supported top-level commands

| Command | Handler |
|---|---|
| `install [tool]` | `handle_install_tool` |
| `update [package?]` | `handle_update_packages` |
| `list` | `handle_list_tools` |
| `info [tool]` | `handle_tool_info` |
| `auth manage` | `handle_authentication_menu` |
| `auth help [tool]` | `handle_auth_help` |
| `auth set [--tool ...]` | `handle_auth_set` or `handle_authentication_menu` |
| `templates init/create/list/apply` | template handlers |
| `config reset/show/path` | config handlers |
| `cache clear/status/refresh` | cache handlers |
| `benchmark list/run/validate` | benchmark handlers |
| `db import/status/reset` | db handlers |
| `status` | `handle_status_command` |

## 3) Tool Execution Flow (`src/cli_logic/cli_logic_tool_execution.rs`)

Path used by both explicit `run` and direct invocation.

1. Resolve install command metadata for `[tool]`
2. Validate runtime prerequisites (`npm`, `curl`, `uv`, Node version checks)
3. Check installed status
4. If missing: prompt install and run installer flow
5. Show launch progress
6. Save last-used tool
7. Special terminal prep for `opencode`
8. Execute `ToolManager::run_tool(tool, args)`

Intent: one canonical execution path regardless of invocation style.

## 4) Quick Launch Flow

`handle_quick_launch()`:

1. Load last-used tool (database first, file fallback)
2. If present: launch immediately via `handle_run_tool(last_tool, [])`
3. If missing: print guidance and available tools

Intent: minimal friction startup for repeat usage.

## 5) Interactive Shell Flow (`src/cli_logic/cli_logic_interactive.rs`)

### Startup sequence

1. Initialize DB (best effort) and load persisted theme
2. Export saved credentials to process env
3. Run first-time wizard if initialization marker is absent
4. Render welcome screen and enter input loop

### Slash command routes

| Slash Command | Route |
|---|---|
| `/tools` | AI tools submenu (`handle_ai_tools_menu`) |
| `/evals` | evals menu |
| `/auth` | auth management menu |
| `/links` | links/resources menu |
| `/settings` | settings/tools menu |
| `/db` | db menu |
| `/theme` | theme selection |
| `/dashboard` / `/status` | dashboard |
| `/help` | command list |
| `/exit` / `/quit` | exit |

### Non-slash input behavior

If plain input matches a known tool name, it launches tool directly through `handle_run_tool`.
Otherwise, it reports unknown command guidance.

Intent: keyboard-driven command center with graceful fallbacks.

## 6) AI Tools Menu Flow (`src/cli_logic/cli_logic_entry_point.rs`)

`handle_ai_tools_menu()`:

1. Render all tools with package-manager hints
2. Select tool
3. If missing, optionally install
4. Launch tool via `launch_tool_with_progress`
5. On exit, show post-tool action menu:
   - Reopen tool
   - Run tool-specific auth command (if configured)
   - Back/switch tools
   - Re-enter API key
   - Uninstall tool
   - Exit Terminal Jarvis

Intent: post-launch control loop for fast context switching without restarting app.

## 7) First-Run Flow (`src/cli_logic/cli_logic_first_run.rs`)

Triggered when `~/.terminal-jarvis/initialized` does not exist.

1. Welcome screen
2. Tool detection summary
3. Database init + TOML import
4. Persist initialization marker and first-run metadata

Intent: one-time onboarding and local state bootstrap.

## 8) Author Intent / Invariants To Preserve

1. **Single source of execution truth**: both `run [tool]` and direct `[tool]` must use `handle_run_tool`.
2. **Fast path for power users**: `-q` launches last-used tool directly.
3. **Graceful user interrupts**: menu prompts should return safely, not crash.
4. **Hybrid persistence strategy**: database-backed flows with file fallback for compatibility.
5. **Install flexibility**: missing `npm` should not block non-npm tool install paths.
6. **Post-tool continuity**: interactive tool launches should return to actionable menu, not dead-end.
7. **Explicit invalid command behavior**: unknown direct command should error clearly and exit non-zero.
8. **Minimal `main.rs` orchestration**: business logic belongs in `cli` + `cli_logic` domains.

## 9) Verification Checklist For Flow Changes

```bash
# Primary routes
target/debug/terminal-jarvis --help
target/debug/terminal-jarvis list
target/debug/terminal-jarvis run [tool] -- --help
target/debug/terminal-jarvis [tool] --help

# Command families
target/debug/terminal-jarvis auth --help
target/debug/terminal-jarvis config --help
target/debug/terminal-jarvis cache --help
target/debug/terminal-jarvis templates --help
target/debug/terminal-jarvis benchmark --help
target/debug/terminal-jarvis db --help

# Interactive route
target/debug/terminal-jarvis
```

If any new route is added in `src/cli.rs` or interactive dispatch, update this document immediately.
