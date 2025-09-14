# Supported AI Coding Tools - Installation & Sources Guide

Terminal Jarvis supports **10 AI coding tools** with seamless installation, updates, and execution. This comprehensive guide provides official sources, exact installation commands, and detailed tool information.

## Complete Tool Overview

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

| Tool        | NPM Package Name              | NPM Install Command                          | Release Strategy              |
| ----------- | ----------------------------- | -------------------------------------------- | ----------------------------- |
| claude-code | `@anthropic-ai/claude-code`   | `npm install -g @anthropic-ai/claude-code`   | Stable/Latest/Versioned       |
| qwen-code   | `@qwen-code/qwen-code`        | `npm install -g @qwen-code/qwen-code`        | Standard versioning           |
| llxprt      | `@vybestack/llxprt-code-core` | `npm install -g @vybestack/llxprt-code-core` | Frequent updates              |
| codex       | `@openai/codex`               | `npm install -g @openai/codex`               | Stable with experimental flag |
| gemini      | `@google/gemini-cli`          | `npm install -g @google/gemini-cli`          | Stable releases               |
| opencode    | `opencode-ai`                 | `npm install -g opencode-ai`                 | Daily updates                 |
| crush       | `@charmland/crush`            | `npm install -g @charmland/crush`            | Active maintenance            |

**Note:** Four tools use different package names than their common names:

- llxprt → `@vybestack/llxprt-code-core`
- gemini → `@google/gemini-cli`
- opencode → `opencode-ai`
- crush → `@charmland/crush`

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

To ensure all tools install and update correctly, the following configuration mappings are maintained in Terminal Jarvis:

### Configuration File Mappings (`terminal-jarvis.toml.example`):

```toml
[tools]
claude-code = { enabled = true, auto_update = true, install_command = "npm install -g @anthropic-ai/claude-code", update_command = "npm update -g @anthropic-ai/claude-code" }
gemini-cli = { enabled = true, auto_update = false, install_command = "npm install -g @google/gemini-cli", update_command = "npm update -g @google/gemini-cli" }
qwen-code = { enabled = true, auto_update = true, install_command = "npm install -g @qwen-code/qwen-code@latest", update_command = "npm update -g @qwen-code/qwen-code" }
opencode = { enabled = true, auto_update = true, install_command = "npm install -g opencode-ai@latest", update_command = "npm update -g opencode-ai" }
llxprt-code = { enabled = true, auto_update = true, install_command = "npm install -g @vybestack/llxprt-code-core", update_command = "npm update -g @vybestack/llxprt-code-core" }
codex = { enabled = true, auto_update = true, install_command = "npm install -g @openai/codex", update_command = "npm update -g @openai/codex" }
crush = { enabled = true, auto_update = true, install_command = "npm install -g @charmland/crush", update_command = "npm update -g @charmland/crush" }
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

**CRITICAL:** Use `@vybestack/llxprt-code-core` for llxprt (not `@vybestack/llxprt-code` which installs Gemini CLI instead).

### Authentication/Configuration Issues
Terminal Jarvis v0.0.44+ handles authentication gracefully. You'll see "✨ [tool] session ended" instead of error messages for normal authentication flows like `/auth` or `/config` commands.

## Security & Maintenance

**Security:** All packages are installed from official NPM registry with verified signatures. Store API keys securely (environment variables) and never commit them to repositories.

**Maintenance Status:** All tools are actively maintained by their respective organizations (Anthropic, Google, Alibaba, OpenAI, etc.) with regular updates.
