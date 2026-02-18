# Terminal Jarvis ADK Home Screen

A Google ADK-powered home screen that sits in front of the `terminal-jarvis` binary.
Issue commands in plain English **or** slash commands. The Rust binary does all real work.

---

## 60-Second Setup

```bash
# 1. Enter the adk directory
cd adk

# 2. Install dependencies
pip install -r requirements.txt

# 3. Configure your provider
cp .env.example .env
# Edit .env and add your API key (see options below)

# 4. Run
python jarvis.py
```

---

## Provider Options (pick one)

### Option 1: Google Gemini (recommended)

```bash
# In adk/.env:
GOOGLE_API_KEY=your-key-here
```

Get a key at <https://aistudio.google.com/app/apikey>.

### Option 2: OpenRouter (100+ cloud models)

```bash
pip install 'google-adk[litellm]'

# In adk/.env:
OPENROUTER_API_KEY=your-key-here
JARVIS_MODEL=openrouter/anthropic/claude-3-5-sonnet
```

### Option 3: Ollama (local, no API key)

```bash
pip install 'google-adk[litellm]'
ollama pull llama3.2

# adk/.env can stay empty — Ollama is auto-detected at localhost:11434
```

---

## Usage

```
  ╭──────────────────────────────────────────────────────────╮
  │  TERMINAL JARVIS                                v0.0.77  │
  │  AI Command Center                                       │
  │                                                          │
  │  Provider  gemini-2.0-flash                              │
  ╰──────────────────────────────────────────────────────────╯

  Type a command or describe what you want in plain English.

  /tools    browse and launch AI tools
  /install  install a new tool
  /status   tool health dashboard
  /help     all commands
  /exit     exit

  > _
```

Slash commands bypass the LLM for speed. Plain English goes to the ADK agent.

---

## Architecture

```
User input
    |
    +-- starts with "/" --> direct subprocess (no LLM overhead)
    +-- plain English   --> ADK Agent (LLM) --> tool call --> terminal-jarvis binary
                                                                     |
                                                         (Rust binary does all real work)
```

The ADK layer is purely a routing/NL interface. No tool logic lives here.

---

## What It Is NOT

- Not a replacement for the existing TUI — parallel interface, same binary underneath
- Not a general-purpose AI assistant — tightly scoped to Terminal Jarvis commands only
- Not reimplementing any Terminal Jarvis functionality — the Rust binary handles everything
