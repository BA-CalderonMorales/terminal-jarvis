# Supported AI Coding Tools - Installation & Sources Guide

Terminal Jarvis supports **7 AI coding tools** with seamless installation, updates, and execution. This comprehensive guide provides official sources, exact NPM installation commands, and detailed tool information.

## 🤖 Complete Tool Overview

### Stable Tools (Production Ready)

| Tool       | Provider  | Description                                          | Installation                                 | Key Features                                                               |
| ---------- | --------- | ---------------------------------------------------- | -------------------------------------------- | -------------------------------------------------------------------------- |
| **claude** | Anthropic | Claude AI for advanced code assistance and reasoning | `npm install -g @anthropic-ai/claude-code`   | • Advanced reasoning<br>• Code analysis<br>• Refactoring suggestions       |
| **gemini** | Google    | Google's powerful Gemini CLI tool                    | `npm install -g @google/gemini-cli`          | • Multi-modal AI<br>• Code generation<br>• Natural language processing     |
| **qwen**   | Alibaba   | Qwen coding assistant with strong language model     | `npm install -g @qwen-code/qwen-code@latest` | • Code completion<br>• Multi-language support<br>• Intelligent suggestions |

### Testing Tools (Community Feedback Welcome)

| Tool         | Provider    | Description                          | Installation                                 | Key Features                                                                    |
| ------------ | ----------- | ------------------------------------ | -------------------------------------------- | ------------------------------------------------------------------------------- |
| **opencode** | OpenCode AI | Terminal-based AI coding agent       | `npm install -g opencode-ai@latest`          | • Terminal-native interface<br>• Code generation<br>• Interactive workflows     |
| **llxprt**   | VybeStack   | Multi-provider AI coding assistant   | `npm install -g @vybestack/llxprt-code-core` | • Multi-provider support<br>• Flexible AI backends<br>• Extensible architecture |
| **codex**    | OpenAI      | OpenAI Codex CLI for local AI coding | `npm install -g @openai/codex`               | • Local AI processing<br>• Code completion<br>• OpenAI integration              |

### New Tools (Latest Additions)

| Tool         | Provider | Description                               | Installation                      | Key Features                                                                                 |
| ------------ | -------- | ----------------------------------------- | --------------------------------- | -------------------------------------------------------------------------------------------- |
| **crush** ✨ | Charm    | Multi-model AI assistant with LSP support | `npm install -g @charmland/crush` | • LSP protocol support<br>• Multi-model AI<br>• MCP integration<br>• Beautiful TUI interface |

## 🚀 Quick Usage with Terminal Jarvis

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
```

### Tool Management

```bash
# Install specific tools
terminal-jarvis install claude
terminal-jarvis install crush

# Update all installed tools
terminal-jarvis update

# Check tool status
terminal-jarvis list
terminal-jarvis info claude
```

## 🔧 Tool Status Indicators

- ✅ **Stable** - Production-ready, thoroughly tested
- 🧪 **Testing** - Feature-complete, seeking community feedback
- ✨ **New** - Recently added, actively being integrated

---

# Detailed Installation Guide & Official Sources

## 1. claude-code - Anthropic's Claude Coding Assistant CLI Tool

**Exact NPM Install Command:**

```bash
npm install -g @anthropic-ai/claude-code
```

**Package Details:**

- **NPM Package Name:** `@anthropic-ai/claude-code`
- **Status:** Beta release, actively maintained by Anthropic
- **Release Strategy:** Stable (default), Latest, and versioned releases available

**Official Sources:**

- **GitHub Repository:** https://github.com/anthropics/claude-code
- **NPM Registry:** https://www.npmjs.com/package/@anthropic-ai/claude-code
- **Official Documentation:** https://docs.anthropic.com/en/docs/claude-code/overview
- **Product Page:** https://www.anthropic.com/claude-code

**Requirements:**

- Node.js 18+
- Anthropic API key or Claude Pro/Max subscription

## 2. qwen-code - Qwen Coding Assistant CLI

**Exact NPM Install Command:**

```bash
npm install -g @qwen-code/qwen-code
```

**Package Details:**

- **NPM Package Name:** `@qwen-code/qwen-code`
- **Current Version:** 0.0.4 (very recent release)
- **Release Strategy:** Standard NPM versioning, single channel

**Official Sources:**

- **GitHub Repository:** https://github.com/QwenLM/qwen-code
- **NPM Registry:** https://www.npmjs.com/package/@qwen-code/qwen-code
- **Qwen Documentation:** https://qwenlm.github.io/blog/qwen3-coder/
- **Model Repository:** https://github.com/QwenLM/Qwen3-Coder

**Requirements:**

- Node.js 20+
- Alibaba Cloud ModelScope or DashScope API key

## 3. llxprt - Multi-provider AI Coding Assistant

**Exact NPM Install Command:**

```bash
npm install -g @vybestack/llxprt-code-core
```

**Package Details:**

- **NPM Package Name:** `@vybestack/llxprt-code-core` (note: not just "llxprt")
- **Current Version:** 0.1.18 (published 13 hours ago)
- **Release Strategy:** Active development, frequent updates
- **Weekly Downloads:** ~500

**Official Sources:**

- **GitHub Repository:** https://github.com/acoliver/llxprt-code
- **NPM Registry:** https://www.npmjs.com/package/@vybestack/llxprt-code-core

**Requirements:**

- Node.js 16+
- API key for chosen provider (OpenAI, Anthropic, Google, etc.)
- Supports local models via LM Studio or llama.cpp

## 4. codex - OpenAI's AI Coding CLI Tool

**Exact NPM Install Command:**

```bash
npm install -g @openai/codex
```

**Package Details:**

- **NPM Package Name:** `@openai/codex` (official OpenAI package)
- **Current Version:** 0.14.0 (published 1 day ago)
- **Release Strategy:** Stable releases with experimental disclaimer
- **Weekly Downloads:** ~1,200

**Official Sources:**

- **GitHub Repository:** https://github.com/openai/codex
- **NPM Registry:** https://www.npmjs.com/package/@openai/codex
- **OpenAI Help Center:** https://help.openai.com/en/articles/11096431-openai-codex-cli-getting-started

**Requirements:**

- macOS 12+, Ubuntu 20.04+/Debian 10+, or Windows 11 via WSL2
- 4GB RAM minimum (8GB recommended)
- OpenAI API key required

## 5. gemini - Google's Gemini CLI Tool

**Exact NPM Install Command:**

```bash
npm install -g @google/gemini-cli
```

**Package Details:**

- **NPM Package Name:** `@google/gemini-cli`
- **Current Version:** 0.1.5+ (actively maintained)
- **Release Strategy:** Stable releases through NPM registry
- **Weekly Downloads:** 27,386+

**Official Sources:**

- **GitHub Repository:** https://github.com/google-gemini/gemini-cli
- **NPM Registry:** https://www.npmjs.com/package/@google/gemini-cli
- **Google Blog:** https://blog.google/technology/developers/introducing-gemini-cli-open-source-ai-agent/
- **Documentation:** https://codelabs.developers.google.com/gemini-cli-hands-on

**Requirements:**

- Node.js 18+
- Google Cloud Project or personal Google account
- Free tier: 60 requests/minute, 1,000 requests/day

## 6. opencode - Terminal-based AI Coding Agent

**Exact NPM Install Command:**

```bash
npm install -g opencode-ai
```

**Package Details:**

- **NPM Package Name:** `opencode-ai` (note: not just "opencode")
- **Current Version:** 0.3.131 (updated 17 hours ago)
- **Release Strategy:** Very active development with multiple updates per day
- **Weekly Downloads:** 27,386

**Official Sources:**

- **Website:** https://opencode.ai
- **GitHub Repository:** https://github.com/sst/opencode
- **NPM Registry:** https://www.npmjs.com/package/opencode-ai
- **Documentation:** https://opencode.ai/docs/

**Requirements:**

- Node.js 18+
- API key from any supported provider (75+ LLM providers via Models.dev)
- Provider agnostic architecture

## 7. crush - Charm's Multi-Model AI Assistant

**Exact NPM Install Command:**

```bash
npm install -g @charmland/crush
```

**Package Details:**

- **NPM Package Name:** `@charmland/crush`
- **Status:** New addition to Terminal Jarvis ecosystem
- **Release Strategy:** Actively maintained by Charm
- **Special Features:** LSP support, MCP integration, beautiful TUI

**Official Sources:**

- **GitHub Repository:** https://github.com/charmbracelet/crush
- **NPM Registry:** https://www.npmjs.com/package/@charmland/crush
- **Charm Website:** https://charm.sh/
- **Documentation:** https://github.com/charmbracelet/crush#readme

**Requirements:**

- Node.js 16+
- Compatible with multiple AI providers
- LSP protocol support for enhanced IDE integration
- MCP (Model Context Protocol) support

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

### Display Name to CLI Command Mappings (`src/tools.rs`):

- claude → claude
- gemini → gemini
- qwen → qwen
- opencode → opencode
- llxprt → llxprt
- codex → codex
- crush → crush

### Display Name to Configuration Key Mappings (`src/services.rs`):

- claude → claude-code
- gemini → gemini-cli
- qwen → qwen-code
- opencode → opencode
- llxprt → llxprt-code
- codex → codex
- crush → crush

## Common Installation Issues

### Issue 1: Command Not Found After Installation

**Problem:** Tool installs successfully but command not found in PATH.
**Solution:**

1. Restart terminal or run `source ~/.bashrc` / `source ~/.zshrc`
2. Check global npm bin path: `npm config get prefix`
3. Ensure npm global bin directory is in PATH

### Issue 2: Permission Errors During Installation

**Problem:** `EACCES` errors during npm install -g.
**Solution:**

1. Use Node Version Manager (nvm) instead of system Node.js
2. Configure npm prefix: `npm config set prefix ~/.local`
3. Add ~/.local/bin to PATH

### Issue 3: Wrong Package Installed

**Problem:** Installing "llxprt" instead of "@vybestack/llxprt-code-core".
**Solution:** Always use the exact package names from this guide.

**CRITICAL:** The package `@vybestack/llxprt-code` currently installs as Gemini CLI instead of LLXPRT. Use `@vybestack/llxprt-code-core` instead.

### Issue 4: Version Conflicts

**Problem:** Multiple versions or conflicting installations.
**Solution:**

1. Uninstall all versions: `npm uninstall -g [package-name]`
2. Clear npm cache: `npm cache clean --force`
3. Reinstall with exact package name from this guide

### Issue 5: Backslash Commands Causing Exit Errors

**Problem:** Using `/auth` or other backslash commands in LLM tools shows "Error running tool" messages.
**Solution:** This has been fixed in Terminal Jarvis v0.0.44+. The following exit codes are now handled gracefully:

- Exit code 1: Authentication required / configuration needed
- Exit code 2: User cancellation / interrupted operation
- Exit code 3: Configuration not found / setup needed
- Exit code 130: User interrupted with Ctrl+C
- Exit codes 128-255: Signal-based exits (usually user initiated)

Users now see a friendly "✨ [tool] session ended" message instead of error messages for these normal authentication and user interaction flows.

## Verification Commands

After installation, verify each tool works correctly:

```bash
# Test basic functionality
claude --version
gemini --version
qwen --version
opencode --version
llxprt --version
codex --version
crush --version

# Test help commands
claude --help
gemini --help
qwen --help
opencode --help
llxprt --help
codex --help
crush --help
```

## Security Considerations

- All packages are installed from official NPM registry
- Package signatures and authenticity verified through NPM
- API keys should be stored securely (environment variables or config files)
- Never share API keys in code repositories
- Use scoped packages (@org/package) when available for better security

## Package Maintenance Status

| Tool        | Maintenance Status                  | Last Updated | Risk Level |
| ----------- | ----------------------------------- | ------------ | ---------- |
| claude-code | ✅ Actively maintained by Anthropic | Daily        | Low        |
| qwen-code   | ✅ Actively maintained by Alibaba   | Weekly       | Low        |
| llxprt      | ✅ Active development               | Daily        | Medium     |
| codex       | ✅ Maintained by OpenAI             | Weekly       | Low        |
| gemini      | ✅ Maintained by Google             | Weekly       | Low        |
| opencode    | ✅ Very active development          | Daily        | Medium     |
| crush       | ✅ Actively maintained by Charm     | Weekly       | Low        |

**Risk Levels:**

- **Low:** Official packages from major AI companies with established maintenance
- **Medium:** Active development but smaller teams or newer projects
