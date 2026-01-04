# Skill: Tool Configuration

**Name**: tool-config
**Description**: Adding and configuring AI tools in Terminal Jarvis
**Trigger**: "Add new AI tool", tool configuration, new tool integration

---

## Objective

All AI tools follow consistent modular configuration pattern.

## Modular Tool Configuration System

**Location**: `/config/tools/<tool-name>.toml`

## Configuration Template

```toml
[tool]
name = "Tool Name"
command = "tool-command"
description = "Brief description"
category = "ai-coding-assistant"

[installation]
npm = "npm install -g package-name"
cargo = "cargo install package-name"
other = "curl -fsSL install-script.sh | sh"

[authentication]
required = true
method = "api_key"
env_var = "TOOL_API_KEY"
instructions = "Get your API key from https://provider.com/keys"

[features]
supports_chat = true
supports_code_generation = true
supports_refactoring = true
supports_testing = false
```

## Adding New Tool Workflow

1. **Create config file**: `/config/tools/newtool.toml`
2. **Define tool metadata**: name, command, description
3. **Specify installation**: npm, cargo, or custom script
4. **Document authentication**: API keys, env vars, instructions
5. **List features**: capabilities for UI display
6. **Test detection**: `cargo run -- list` should show new tool
7. **Test execution**: `cargo run -- run newtool` should work
8. **Update CHANGELOG**: Add under `### Added`

## Existing Tools

Located in `/config/tools/`:
- `claude.toml` - Claude Code
- `gemini.toml` - Gemini CLI
- `qwen.toml` - Qwen Code
- `opencode.toml` - OpenCode
- `codex.toml` - OpenAI Codex
- `aider.toml` - Aider
- `goose.toml` - Goose
- `amp.toml` - Amp
- `crush.toml` - Crush
- `llxprt.toml` - LLXPRT
