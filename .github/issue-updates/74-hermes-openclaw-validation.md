# Issue #74: Hermes/OpenClaw Validation

Status: ready to close as implemented, with one optional follow-up for live install testing.

Validation date: 2026-05-13
Branch: `triage/hermes-openclaw-validation-isolated`
Base: `origin/develop` at `640e82f`

## Catalog Evidence

- `config/tools/hermes.toml` exists and defines:
  - `config_key = "hermes"`
  - `cli_command = "hermes"`
  - installer command using the canonical Nous Research install script with `--skip-setup`
  - update command `hermes update`
  - provider/auth metadata for OpenRouter, AI Gateway, Hugging Face, Anthropic, OpenAI-compatible endpoints, and Google Gemini
- `config/tools/openclaw.toml` exists and defines:
  - `config_key = "openclaw"`
  - `cli_command = "openclaw"`
  - npm install command `npm install -g openclaw`
  - update command `openclaw update --yes --no-restart`
  - OpenRouter onboarding/auth metadata
- `npm/terminal-jarvis/config/tools/hermes.toml` and `npm/terminal-jarvis/config/tools/openclaw.toml` match the root catalog entries.
- `CHANGELOG.md` already records Hermes Agent and OpenClaw catalog support in the current release entry.

## Test Evidence

Command:

```bash
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc RUSTFLAGS= cargo test --locked --test tool_installation_detection_tests
```

Result:

- Passed.
- 14 tests passed, 0 failed.
- Includes `test_hermes_catalog_metadata`.
- Includes `test_openclaw_catalog_metadata`.

## CLI Evidence

Command:

```bash
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc RUSTFLAGS= cargo run --locked -- info hermes
```

Result:

- Exited 0.
- Displayed `Tool Information: hermes`.
- Displayed the Hermes description from the catalog.
- Displayed command `hermes`.
- Displayed installation command `bash -lc 'curl -fsSL https://raw.githubusercontent.com/NousResearch/hermes-agent/main/scripts/install.sh | bash -s -- --skip-setup'`.
- Displayed all configured auth environment variables and setup URL.

Command:

```bash
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc RUSTFLAGS= cargo run --locked -- info openclaw
```

Result:

- Exited 0.
- Displayed `Tool Information: openclaw`.
- Displayed the OpenClaw description from the catalog.
- Displayed command `openclaw`.
- Displayed installation command `npm install -g openclaw`.
- Displayed `NPM Required: Available`.
- Displayed configured `OPENROUTER_API_KEY` auth metadata and setup URL.

## Remaining Risk

This validation confirms catalog presence, metadata shape, npm-copied config availability, and CLI `info` rendering. It does not perform live installation of Hermes or OpenClaw, because #74 can be closed as catalog support and live installation would execute third-party installers on the local machine.

Recommended issue disposition: close #74 as implemented. If maintainers want live installer smoke tests, track that separately as an explicit external-install validation task.
