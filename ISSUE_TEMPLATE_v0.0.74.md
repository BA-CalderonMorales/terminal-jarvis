# Issue to Submit to GitHub

**Repository:** https://github.com/BA-CalderonMorales/terminal-jarvis/issues/new

**Title:**
```
v0.0.74 [Feature Request] AI Coding Agents Not Yet Supported - Tracking Issue
```

**Body:**
```markdown
## Summary

Tracking issue for AI coding agents that could be added to Terminal Jarvis. This consolidates the landscape of available CLI-based AI coding tools to help prioritize future integrations.

## Currently Supported (10 tools) ✅

| Tool | Package/Install Method |
|------|------------------------|
| `claude` | `@anthropic-ai/claude-code` |
| `gemini` | `@google/gemini-cli` |
| `qwen` | `@qwen-code/qwen-code` |
| `opencode` | `opencode-ai` |
| `codex` | `@openai/codex` |
| `aider` | pip install / pipx |
| `goose` | Install script |
| `amp` | Sourcegraph install |
| `crush` | NPM/install script |
| `llxprt` | `@vybestack/llxprt-code` |

## Not Yet Included (12 agents) 🔲

| Agent | Description | Install Method | Priority |
|-------|-------------|----------------|----------|
| `copilot-cli` | GitHub Copilot CLI - brings Copilot agent to terminal | `@githubnext/github-copilot-cli` | ⬜ TBD |
| `code` | Fork of codex - orchestrates OpenAI, Claude, Gemini, or any provider | TBD | ⬜ TBD |
| `cursor-agent` | Cursor AI code editor CLI tool | TBD | ⬜ TBD |
| `droid` | Factory AI's Droid - AI-powered development agent | TBD | ⬜ TBD |
| `eca` | Editor Code Assistant - AI pair programming, editor-agnostic | TBD | ⬜ TBD |
| `forge` | AI-Enhanced Terminal Development Environment | TBD | ⬜ TBD |
| `jules` | Google's asynchronous coding agent in the terminal | TBD | ⬜ TBD |
| `kilocode-cli` | Open-source AI coding agent, now in terminal | TBD | ⬜ TBD |
| `letta-code` | Memory-first coding agent that learns across sessions | TBD | ⬜ TBD |
| `mistral-vibe` | Minimal CLI coding agent by Mistral AI (Devstral) | TBD | ⬜ TBD |
| `nanocoder` | Local-first coding agent built by the community | TBD | ⬜ TBD |
| `pi` | Terminal-based coding agent | TBD | ⬜ TBD |

## How to Contribute

If you'd like to add support for one of these agents:

1. **Research the tool** - Find the official install method (NPM, pip, cargo, install script, etc.)
2. **Comment on this issue** - Claim the agent you want to work on with install details
3. **Follow the existing patterns** - See `src/tools/` and `config/tools/*.toml` for examples
4. **Submit a PR** - Reference this issue

## Useful Resources

- [AGENTS.md](https://github.com/BA-CalderonMorales/terminal-jarvis/blob/main/AGENTS.md) - AI-assisted development guidelines
- [Architecture Docs](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/architecture/) - Technical deep-dive
- [Contribution Guide](https://ba-calderonmorales.github.io/my-life-as-a-dev/projects/active/terminal-jarvis/details/contributions/)

## Notes

- Agents should have a stable CLI interface
- Preference for tools with NPM/pip/cargo distribution for easier installation
- Each new tool needs: install command, update command, run command, and detection logic

---

*This list was compiled from community suggestions. Feel free to comment with additional agents or corrections.*
```

**Labels to add:**
- `enhancement`
- `good first issue`
- `help wanted`

---

## Instructions to Submit:

1. Go to https://github.com/BA-CalderonMorales/terminal-jarvis/issues/new
2. Copy the **Title** section above and paste it as the issue title
3. Copy the **Body** section above (the markdown between the triple backticks) and paste it as the issue body
4. Add the three labels listed above
5. Click "Submit new issue"
