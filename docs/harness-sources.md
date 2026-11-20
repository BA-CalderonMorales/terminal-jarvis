# Harness Install Sources

Which channel each harness installs from, verified against the upstream project
on 2026-10-29 (amp, claude, eca, forge, goose, hermes, jules, openclaw and opencode on 2026-11-15). The catalog's `download` rows are the executable truth; this page
records why each channel is the official one.

| Harness | Channel | Evidence |
|---|---|---|
| aider | `uv tool install aider-chat` | PyPI `aider-chat`, project `Aider-AI/aider`; uv form is the documented install |
| amp | `https://ampcode.com/install.sh` | Amp's own installer; Amp moved off npm (ampcode.com/news/npm-package-changes) |
| claude | `https://claude.ai/install.sh` | Anthropic's native installer, documented at docs.claude.com/en/docs/claude-code/setup |
| code | `npm install -g @just-every/code` | npm bin `coder`, repo `just-every/code` |
| codex | `npm install -g @openai/codex` | npm bin `codex`, repo `openai/codex` |
| copilot | `npm install -g @github/copilot` | npm bin `copilot`, repo `github/copilot-cli` |
| crush | `npm install -g @charmland/crush` | npm bin `crush`, repo `charmbracelet/crush` |
| cursor-agent | `curl https://cursor.com/install -fsS \| bash` | Cursor's CLI docs; installer links both `agent` and legacy `cursor-agent` |
| droid | `https://app.factory.ai/cli` | Factory's documented installer |
| eca | `editor-code-assistant/eca` install script, `--dir ~/.local/bin` | Project's own installer (eca.dev/install) writes the `eca` binary; the default `/usr/local/bin` needs root. `eca-cli` is a different binary |
| forge | `https://antinomy.ai/cli` | Forge's own installer; the npm package lags the release line (repo `tailcallhq/forgecode`); caveat: the vendor script removes an existing npm install before downloading, and we cannot patch that |
| gemini | `npm install -g @google/gemini-cli` | npm bin `gemini`, repo `google-gemini/gemini-cli`; **retired 2026-06-18** -- see [graveyard](usage.md) |
| goose | `aaif-goose/goose` stable installer | Documented at goose-docs.ai; needs no sudo |
| hermes | `https://hermes-agent.nousresearch.com/install.sh` | Nous Research's own installer, installs to `~/.local/bin` |
| jules | `npm install -g @google/jules` | npm maintained by Google (`google-wombot`); **retiring 2026-12-31** (jules.google/docs/changelog/2026-10-29), kept for data export |
| kilocode | `npm install -g @kilocode/cli` | npm bins `kilo`/`kilocode`, repo `Kilo-Org/kilocode` |
| letta | `npm install -g @letta-ai/letta-code` | npm bin `letta`, repo `letta-ai/letta-code` |
| llxprt | `npm install -g @vybestack/llxprt-code` | npm bin `llxprt`, repo `vybestack/llxprt-code` |
| nanocoder | `npm install -g @nanocollective/nanocoder` | npm bin `nanocoder`, repo `Nano-Collective/nanocoder` |
| ollama | `https://ollama.com/install.sh` | Ollama's documented installer |
| openclaw | `https://openclaw.ai/install.sh` | OpenClaw's own installer (docs.openclaw.ai/install); npm 11+ needs `--allow-scripts` for the package; removal `npm uninstall -g openclaw` per docs.openclaw.ai/install/uninstall |
| opencode | `npm install -g @opencode/cli@latest` | OpenCode V2 (opencode.ai/docs/migrate-v1); the `opencode-ai` package still installs V1; caveat: `update` is a plain `npm update -g` and does not migrate a V1 install, a maintainer design call left as is |
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
- **opencode** moved to the V2 package `@opencode/cli`. The V1 remover
  declaration was dropped because it is unverified for V2.
- **eca, forge, goose, hermes, openclaw** install through their own scripts,
  run fail-closed (a failed download never reaches the shell) and marked
  `unknown`.

Verification method: npm registry metadata (`name`, `version`, `repository.url`,
`bin`), PyPI JSON, HTTP status of each installer URL, and the upstream install
docs. Script installers are `unknown` support by design; they are not claimed
as deterministically verified.
