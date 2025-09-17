# Supported AI Coding Tools - Installation & Sources Guide

Terminal Jarvis supports a suite of AI coding tools with seamless installation, updates, and execution. This guide is the authoritative reference for tool sources and install commands. The definitive source of truth is the modular configs in `config/tools/*.toml` and the CLI output from `terminal-jarvis list` and `terminal-jarvis info <tool>`.

## Complete Tool Overview

The following tools are currently defined in `config/tools/` and supported by Terminal Jarvis:

- claude
- gemini
- qwen
- opencode
- llxprt
- codex
- crush
- goose
- amp
- aider

| Tool         | Provider    | Status      | GitHub Repository                                | Installation Command                         | Key Features                                                                    |
|--------------|-------------|-------------|--------------------------------------------------|----------------------------------------------|---------------------------------------------------------------------------------|
| **claude**   | Anthropic   | Stable      | [anthropics/claude-code](https://github.com/anthropics/claude-code) | `npm install -g @anthropic-ai/claude-code`   | • Advanced reasoning<br>• Code analysis<br>• Refactoring suggestions           |
| **gemini**   | Google      | Stable      | [google-gemini/gemini-cli](https://github.com/google-gemini/gemini-cli) | `npm install -g @google/gemini-cli`          | • Multi-modal AI<br>• Code generation<br>• Natural language processing         |
| **qwen**     | Alibaba     | Stable      | [QwenLM/qwen-code](https://github.com/QwenLM/qwen-code) | `npm install -g @qwen-code/qwen-code@latest` | • Code completion<br>• Multi-language support<br>• Intelligent suggestions     |
| **opencode** | OpenCode AI | Testing     | [sst/opencode](https://github.com/sst/opencode) | `npm install -g opencode-ai@latest`          | • Terminal-native interface<br>• Code generation<br>• Interactive workflows     |
| **llxprt**   | VybeStack   | Testing     | [acoliver/llxprt-code](https://github.com/acoliver/llxprt-code) | `npm install -g @vybestack/llxprt-code-core` | • Multi-provider support<br>• Flexible AI backends<br>• Extensible architecture |
| **codex**    | OpenAI      | Testing     | [openai/codex](https://github.com/openai/codex) | `npm install -g @openai/codex`               | • Local AI processing<br>• Code completion<br>• OpenAI integration              |
| **crush**   | Charm       | New         | [charmbracelet/crush](https://github.com/charmbracelet/crush) | `npm install -g @charmland/crush`            | • LSP protocol support<br>• Multi-model AI<br>• MCP integration<br>• Beautiful TUI interface |
| **goose**    | Block       | New         | [block/goose](https://github.com/block/goose)   | `curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh \| bash` | • AI-powered coding assistant<br>• Developer toolkit integration<br>• Multiple AI provider support |
| **amp**      | Sourcegraph | New         | [sourcegraph/amp](https://github.com/sourcegraph/amp) | `npm install -g @sourcegraph/amp`             | • Advanced context awareness<br>• Sourcegraph integration<br>• Code intelligence |
| **aider**    | Aider       | New         | [paul-gauthier/aider](https://github.com/paul-gauthier/aider) | `uv tool install --force --python python3.12 --with pip aider-chat@latest` | • AI pair programming<br>• Git repository editing<br>• Local file manipulation |

## Tool Status Indicators

- **Stable** - Production-ready, thoroughly tested
- **Testing** - Feature-complete, seeking community feedback  
- **New** - Recently added, actively being integrated

## Quick Usage with Terminal Jarvis

### Interactive Mode (Recommended)

```bash
terminal-jarvis
```

### Direct Tool Execution

```bash
# Run any supported tool directly
terminal-jarvis run claude --prompt "Explain this code"
terminal-jarvis run gemini --file src/main.rs
terminal-jarvis run qwen --analyze
terminal-jarvis run opencode --generate
terminal-jarvis run llxprt --help
terminal-jarvis run codex --complete
terminal-jarvis run crush --lsp
terminal-jarvis run goose --session
terminal-jarvis run amp --context
terminal-jarvis run aider --git
```

### Tool Management

```bash
# Install specific tools
terminal-jarvis install claude
terminal-jarvis install crush
terminal-jarvis install goose
terminal-jarvis install amp
terminal-jarvis install aider

# Update all installed tools
terminal-jarvis update

# Check tool status
terminal-jarvis list
terminal-jarvis info claude
```

## Summary Table

| Tool        | Package name (if NPM)         | Install command                               | Notes                         |
| ----------- | ----------------------------- | --------------------------------------------- | ----------------------------- |
| claude      | `@anthropic-ai/claude-code`   | `npm install -g @anthropic-ai/claude-code`    | Stable                        |
| gemini      | `@google/gemini-cli`          | `npm install -g @google/gemini-cli`           | Stable                        |
| qwen        | `@qwen-code/qwen-code`        | `npm install -g @qwen-code/qwen-code@latest`  | Stable                        |
| opencode    | `opencode-ai`                 | `npm install -g opencode-ai@latest`           | Testing                       |
| llxprt      | `@vybestack/llxprt-code`      | `npm install -g @vybestack/llxprt-code`       | Testing                       |
| codex       | `@openai/codex`               | `npm install -g @openai/codex`                | Testing/Legacy                |
| crush       | `@charmland/crush`            | `npm install -g @charmland/crush`             | New                           |
| goose       | n/a                           | `curl -fsSL https://github.com/block/goose/releases/download/stable/download_cli.sh | bash` | Stable                        |
| amp         | `@sourcegraph/amp`            | `npm install -g @sourcegraph/amp`             | Stable                        |
| aider       | n/a (uv)                      | `uv tool install --force --python python3.12 --with pip aider-chat@latest` | Stable                        |

Notes:

- Some tools have CLI names that differ from their package names (see table).
- Goose is installed via a publisher-provided script (not NPM).
- Aider is installed via `uv` and Python, not NPM.

## Installation Verification

After installing any tool, verify it works correctly:

```bash
# Test basic functionality
claude --version
gemini --version
qwen --version
opencode --version
llxprt --version
codex --version
crush --version

# Or test help commands if version fails
claude --help
gemini --help
qwen --help
opencode --help
llxprt --help
codex --help
crush --help
```

## Terminal Jarvis Configuration Consistency

Terminal Jarvis now uses a modular configuration system. Each tool has its own TOML file under `config/tools/` (for example, `config/tools/claude.toml`, `config/tools/gemini.toml`, etc.). The app automatically discovers and loads these definitions, so you don't need to maintain a single monolithic mapping file.

Benefits of the modular system:
- Automatic discovery of new tools added to `config/tools/`
- Clear separation of per-tool install/auth/feature metadata
- Reduced drift between docs and implementation

To see the exact configuration for a tool, open its TOML file in `config/tools/` or run:
```
terminal-jarvis info <tool>
```

## Common Installation Issues

### Command Not Found After Installation
**Solution:** Restart terminal or run `source ~/.bashrc` / `source ~/.zshrc`

### Permission Errors During Installation
**Solution:** Use Node Version Manager (nvm) instead of system Node.js:
```bash
# Install nvm first, then:
nvm install node
npm install -g [package-name]
```

### Wrong Package Installed
**Solution:** Always use the exact package names from the tables above.

**CRITICAL:** Use `@vybestack/llxprt-code` for llxprt. Avoid similarly named packages that map to other tools.

### Authentication/Configuration Issues
Terminal Jarvis v0.0.44+ handles authentication gracefully. You'll see "[INFO] [tool] session ended" instead of error messages for normal authentication flows like `/auth` or `/config` commands.

## Security & Maintenance

**Security:** All packages are installed from official NPM registry with verified signatures. Store API keys securely (environment variables) and never commit them to repositories.

**Maintenance Status:** All tools are actively maintained by their respective organizations (Anthropic, Google, Alibaba, OpenAI, etc.) with regular updates.
