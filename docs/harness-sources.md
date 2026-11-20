# Harness Install Sources

Which channel each harness installs from, verified against the upstream project
on 2026-10-29. The catalog's `download` rows are the executable truth; this page
records why each channel is the official one.

| Harness | Channel | Evidence |
|---|---|---|
| aider | `uv tool install aider-chat` | PyPI `aider-chat`, project `Aider-AI/aider`; uv form is the documented install |
| amp | `npm install -g @sourcegraph/amp` | npm package exists; Sourcegraph publishes it |
| claude | `https://claude.ai/install.sh` | Anthropic's installer, linked from Claude Code docs |
| code | `npm install -g @just-every/code` | npm bin `coder`, repo `just-every/code` |
| codex | `npm install -g @openai/codex` | npm bin `codex`, repo `openai/codex` |
| copilot | `npm install -g @github/copilot` | npm bin `copilot`, repo `github/copilot-cli` |
| crush | `npm install -g @charmland/crush` | npm bin `crush`, repo `charmbracelet/crush` |
| cursor-agent | `curl https://cursor.com/install -fsS \| bash` | Cursor's CLI docs; installer links both `agent` and legacy `cursor-agent` |
| droid | `https://app.factory.ai/cli` | Factory's documented installer |
| eca | `editor-code-assistant/eca` install script | Project's own GitHub installer |
| forge | `npm install -g forgecode` | npm bin `forge`, repo `antinomyhq/forge` |
| gemini | `npm install -g @google/gemini-cli` | npm bin `gemini`, repo `google-gemini/gemini-cli`; **retired 2026-06-18** -- see [graveyard](usage.md) |
| goose | `block/goose` stable installer | Block's documented installer |
| hermes | `NousResearch/hermes-agent` installer | Project's own GitHub installer |
| jules | `npm install -g @google/jules` | npm maintained by Google (`google-wombot`), homepage `jules.google` |
| kilocode | `npm install -g @kilocode/cli` | npm bins `kilo`/`kilocode`, repo `Kilo-Org/kilocode` |
| letta | `npm install -g @letta-ai/letta-code` | npm bin `letta`, repo `letta-ai/letta-code` |
| llxprt | `npm install -g @vybestack/llxprt-code` | npm bin `llxprt`, repo `vybestack/llxprt-code` |
| nanocoder | `npm install -g @nanocollective/nanocoder` | npm bin `nanocoder`, repo `Nano-Collective/nanocoder` |
| ollama | `https://ollama.com/install.sh` | Ollama's documented installer |
| openclaw | `npm install -g openclaw` | npm bin `openclaw`, repo `openclaw/openclaw` |
| opencode | `npm install -g opencode-ai` | npm bin `opencode`; OpenCode docs list npm alongside its installer |
| pi | `npm install -g --ignore-scripts @earendil-works/pi-coding-agent` | Pi docs require `--ignore-scripts` for npm; installer is the pinned alternative |
| qwen | `npm install -g @qwen-code/qwen-code` | npm bin `qwen`, repo `QwenLM/qwen-code` |
| vibe | `https://mistral.ai/vibe/install.sh` | Mistral's documented installer |

## Corrections made in this audit

- **cursor-agent** installed `cursor-agent-tools` from PyPI, a third-party
  package that is not Cursor's CLI. It now uses Cursor's official installer.
  Its support claim dropped from `expected` to `unknown` because an installer
  script is not deterministically verifiable.
- **pi** now passes `--ignore-scripts`, as the upstream docs require for a
  normal npm installation.
- **opencode** declares its own noninteractive remover
  (`opencode uninstall --force --keep-config --keep-data`), so a script install
  is removed by the channel that owns it.

Verification method: npm registry metadata (`name`, `version`, `repository.url`,
`bin`), PyPI JSON, HTTP status of each installer URL, and the upstream install
docs. Script installers are `unknown` support by design; they are not claimed
as deterministically verified.
