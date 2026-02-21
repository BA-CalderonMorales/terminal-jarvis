# Skill: Tool Launch Verification

**Name**: tool-launch-verification  
**Description**: Session-based verification of tool launch flows using a reusable command pattern  
**Trigger**: "verify tool launches", "which tools are actually operational", pre-release launch validation

---

## Goal

Verify which tools are truly operational in the current session, then classify each tool for release decisions.

## Core Pattern

Use the same command template for every tool, instead of hardcoding long per-tool lists:

```bash
cargo run -- [tool]
cargo run -- run [tool] -- --[action]
```

- `[tool]`: tool key (for example `claude`, `code`, `kilocode`)
- `[action]`: forwarded CLI action (default `help`; also `version` when supported)

## Recommended Session Check

```bash
# default action
ACTION=help

# sample tool set; expand as needed
TOOLS=(claude gemini qwen opencode codex aider amp copilot goose crush llxprt ollama vibe droid forge cursor-agent jules kilocode letta nanocoder pi code eca)

for tool in "${TOOLS[@]}"; do
  echo "=== $tool ==="
  timeout 20s cargo run -- run "$tool" -- "--$ACTION"
  timeout 20s cargo run -- "$tool" "--$ACTION"
  echo
 done
```

## Classification Rules

- `VERIFIED`: both launch paths succeed for `[action]`
- `PARTIAL`: launches but emits blocking warnings/errors or hangs intermittently
- `INTEGRATION-PENDING`: missing install, broken launch path, or cancelled setup flow

## Release Rule

Only claim `VERIFIED` tools as operational. Keep `PARTIAL` and `INTEGRATION-PENDING` documented for future release work.
