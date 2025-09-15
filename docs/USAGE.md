# How to Use Terminal Jarvis

Terminal Jarvis provides multiple ways to interact with AI coding tools, from a beautiful interactive interface to direct command-line operations.

## Interactive Mode (Recommended)

```bash
# Launch the full T.JARVIS experience
terminal-jarvis
```

Get the complete interface with:

- Beautiful ASCII art welcome screen
- Real-time tool status dashboard
- Quick tool selection and launching
- Built-in management options
- Smart guidance and tips

## Direct Commands

### Tool Management

```bash
# Install and manage tools
terminal-jarvis install claude
terminal-jarvis update               # Update all tools
terminal-jarvis list                # Show tool status
terminal-jarvis info claude         # Tool details
```

### Running Tools

```bash
# Run tools directly
terminal-jarvis run claude --prompt "Refactor this function"
terminal-jarvis run gemini --file src/main.rs
terminal-jarvis run qwen --analyze
terminal-jarvis run opencode --generate
terminal-jarvis run llxprt --help
```

## Template Management

```bash
# Template workflow (requires gh CLI)
terminal-jarvis templates init       # Setup templates repo
terminal-jarvis templates create my-template
terminal-jarvis templates list
terminal-jarvis templates apply my-template
```

## Supported AI Tools

Below is a concise overview. For official sources and exact installation commands, see [SOURCES.md](SOURCES.md).

| Tool       | Description                               | Status     |
| ---------- | ----------------------------------------- | ---------- |
| `claude`   | Anthropic's Claude for code assistance    | Stable     |
| `gemini`   | Google's Gemini CLI tool                  | Stable     |
| `qwen`     | Qwen coding assistant                     | Stable     |
| `opencode` | Terminal-based AI coding agent            | Testing    |
| `llxprt`   | Multi-provider AI coding assistant        | Testing    |
| `codex`    | OpenAI Codex CLI for local AI coding      | Testing    |
| `crush`    | Charm's multi-model AI assistant with LSP | New        |
| `goose`    | Developer toolkit AI assistant            | New        |
| `amp`      | Sourcegraph’s context-aware assistant     | New        |
| `aider`    | AI pair programmer for Git repos          | New        |

For detailed information and install commands, see [SOURCES.md](SOURCES.md) or run `terminal-jarvis info <tool-name>`.

See [LIMITATIONS.md](LIMITATIONS.md) for known issues and workarounds.