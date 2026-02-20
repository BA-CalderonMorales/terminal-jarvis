# Skill: Home Screen (Go ADK)

**Name**: home-screen
**Description**: Setup, start, and stop the Terminal Jarvis ADK home screen implemented in Go (`adk/jarvis`)
**Trigger**: "spin up the home screen", "start jarvis", "set up the ADK", "run jarvis", "stop the home screen"

---

## What It Is

The home screen runs as a Go binary (`adk/jarvis`) and provides two interfaces to the same `terminal-jarvis` Rust binary:

- Slash commands route directly to the binary (no LLM)
- Plain-English requests route through the configured provider chain

All tool logic remains in the Rust binary.

---

## Setup

Requires:
- `terminal-jarvis` available from PATH or repo build outputs
- `adk/.env` with at least one provider configured

Create provider config:

```bash
cp adk/.env.example adk/.env
# Edit adk/.env
```

If the `adk/jarvis` binary is missing, build it:

```bash
cd adk && go build -o jarvis .
```

---

## Start

Preferred:

```bash
./jarvis.sh
```

Direct:

```bash
cd adk && ./jarvis
```

The home screen auto-loads credentials from `adk/.env`.
If no provider is configured, it launches the setup wizard automatically.

---

## Stop

Use one of:

```text
/exit
/quit
Ctrl-C
```

---

## Slash Commands

| Command | Action |
|---------|--------|
| `/help` | List all commands |
| `/tools` | List all AI coding tools |
| `/install <tool>` | Install a tool |
| `/status` | Tool health dashboard |
| `/auth [tool]` | Authentication help |
| `/setup` | Re-run provider setup wizard |
| `/config` | Show current config |
| `/update [tool]` | Update one or all tools |
| `/exit` / `/quit` | Exit |

---

## Key Files

| File | Role |
|------|------|
| `adk/main.go` | Entry point + provider bootstrap |
| `adk/internal/repl/` | REPL loop + slash commands |
| `adk/internal/auth/` | Provider setup/auth flows |
| `adk/internal/providers/` | Provider chain and clients |
| `adk/internal/tools/` | `terminal-jarvis` command wrappers |
| `adk/.env` | Local provider credentials (not committed) |
| `adk/.env.example` | Provider template |

---

## Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `GOOGLE_API_KEY` | Google Gemini provider | — |
| `GEMINI_API_KEY` | Alias for GOOGLE_API_KEY | — |
| `OPENROUTER_API_KEY` | OpenRouter provider | — |
| `JARVIS_MODEL` | Explicit model override | — |
| `OLLAMA_HOST` | Ollama base URL | `http://localhost:11434` |
