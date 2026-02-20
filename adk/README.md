# Terminal Jarvis ADK Home Screen (Go)

The home screen is now implemented in Go and runs from `adk/jarvis`.

## Quick Start

```bash
# Optional (only needed if adk/jarvis is missing)
cd adk && go build -o jarvis .

# Configure provider credentials
cp adk/.env.example adk/.env
# Edit adk/.env and add your key

# Start from project root
./jarvis.sh
```

You can also run directly:

```bash
cd adk && ./jarvis
```

## Providers

Set one of the following in `adk/.env`:

- `GOOGLE_API_KEY` (or `GEMINI_API_KEY`)
- `OPENROUTER_API_KEY`
- Ollama at `http://localhost:11434` (no key required)

Optional overrides:

- `JARVIS_MODEL` for an explicit model string
- `OLLAMA_HOST` to change the default Ollama endpoint

## Notes

- Slash commands run directly against the `terminal-jarvis` binary.
- Plain-English requests are routed through the provider chain.
- Run `/setup` inside the home screen to launch provider setup.
