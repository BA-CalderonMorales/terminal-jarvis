# Skill: Tool Launch Verification

**Name**: tool-launch-verification  
**Description**: Session-based verification of all Terminal Jarvis command paths for tool operations and runtime flow integrity  
**Trigger**: "verify tool launches", "which tools are actually operational", pre-release launch validation

---

## Goal

Verify real behavior across all supported command paths, not just one launch command shape.

## Source Of Truth

- Runtime entrypoint: `src/main.rs`
- Primary dispatcher: `src/cli.rs`
- Interactive dispatcher: `src/cli_logic/cli_logic_interactive.rs`
- Menu/tool dispatcher: `src/cli_logic/cli_logic_entry_point.rs`
- Deep flow map and author intent: `./references/src-main-flow-map.md`

Before changing flow behavior, re-read these files and update the reference.

## Placeholders

- `[tool]`: tool key (e.g. `claude`, `code`, `kilocode`)
- `[action]`: forwarded tool flag/action (usually `help` or `version`)
- `[binary]`: `terminal-jarvis`, `target/debug/terminal-jarvis`, or `target/release/terminal-jarvis`

## Command Path Matrix

### 1) Source-run path (Cargo)

```bash
cargo run -- [tool] --[action]
cargo run -- run [tool] -- --[action]
```

### 2) Built-binary paths (Debug/Release/Installed)

```bash
[binary] [tool] --[action]
[binary] run [tool] -- --[action]
```

### 3) Tool-management paths

```bash
[binary] list
[binary] info [tool]
[binary] auth help [tool]
[binary] status
```

### 4) Platform-management paths

```bash
[binary] config show
[binary] cache status
[binary] templates list
[binary] benchmark list
[binary] db status
```

### 5) ADK home-screen paths (Go REPL)

```bash
./jarvis.sh
# inside REPL:
/tools
/auth [tool]
/setup
/logout [provider]
launch [tool]
```

## Recommended Session Sweep

```bash
ACTION=help
BINARY=target/debug/terminal-jarvis
TOOLS=(claude gemini qwen opencode codex aider amp copilot goose crush llxprt ollama vibe droid forge cursor-agent jules kilocode letta nanocoder pi code eca)

for tool in "${TOOLS[@]}"; do
  echo "=== $tool ==="
  timeout 20s cargo run -- "$tool" "--$ACTION"
  timeout 20s cargo run -- run "$tool" -- "--$ACTION"
  timeout 20s "$BINARY" "$tool" "--$ACTION"
  timeout 20s "$BINARY" run "$tool" -- "--$ACTION"
  "$BINARY" info "$tool" >/dev/null 2>&1 || true
  "$BINARY" auth help "$tool" >/dev/null 2>&1 || true
  echo
done
```

## Classification

- `VERIFIED`: direct + run paths pass for selected `[action]` on at least one `[binary]`
- `PARTIAL`: command runs but reports environment/install/network/permission issues
- `INTEGRATION-PENDING`: unavailable, cancelled install flow, or missing integration path

## Release Rule

Only advertise `VERIFIED` as operational. Keep `PARTIAL` and `INTEGRATION-PENDING` documented for future release work.

## Maintenance Rule

If a new command path, alias path, or interactive route is introduced, update:

1. `./references/src-main-flow-map.md`
2. This skill's command matrix
3. Any related E2E/verification checks
