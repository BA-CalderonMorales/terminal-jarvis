# Configuration Guide

Terminal Jarvis works out-of-the-box, but you can customize behavior with configuration files to suit your development workflow.

## Configuration Locations (in priority order)

- `./terminal-jarvis.toml` (project-specific)
- `~/.config/terminal-jarvis/config.toml` (user-wide)

## Tool Configuration Architecture

Terminal Jarvis uses a modular configuration system with individual tool definitions stored in the `config/tools/` directory:

```
config/
├── tools/           # Individual tool configurations
│   ├── claude.toml  # Anthropic Claude configuration
│   ├── gemini.toml  # Google Gemini configuration
│   ├── qwen.toml    # Qwen coding assistant
│   ├── opencode.toml
│   ├── llxprt.toml
│   ├── codex.toml
│   └── crush.toml
└── config.toml      # Global Terminal Jarvis settings
```

## Root config.toml File

The `config.toml` file at the project root controls global Terminal Jarvis behavior:

```toml
# Global Terminal Jarvis Configuration

[app]
version = "0.0.67"
theme = "default"
auto_update_check = true

[logging]
level = "info"
file_logging = false

[authentication]
browser_prevention = true
session_timeout = 3600

[tools]
# Tool-specific overrides (optional)
# Individual tool configs are loaded from config/tools/ directory
auto_update_all = false
install_timeout = 300
```

## Example User Configuration

Create `~/.config/terminal-jarvis/config.toml` for user-wide customization:

```toml
[tools]
claude = { enabled = true, auto_update = true }
gemini = { enabled = true, auto_update = false }
qwen = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }
llxprt = { enabled = true, auto_update = true }
codex = { enabled = true, auto_update = true }
crush = { enabled = true, auto_update = true }

[templates]
repository = "your-username/jarvis-templates"
auto_sync = true
```

## Tool Definition Structure

Each tool configuration in `config/tools/` follows this structure:

```toml
[tool]
display_name = "Tool Name"
config_key = "tool-key"
description = "Tool description"
cli_command = "tool-command"
requires_npm = true
status = "stable"

[tool.install]
command = "npm"
args = ["install", "-g", "package-name"]
verify_command = "tool --version"

[tool.auth]
env_vars = ["TOOL_API_KEY"]
setup_url = "https://tool-setup-url"
auth_instructions = "Setup instructions"
```

## Advanced Configuration

### Custom Tool Installation

You can override default installation commands for specific tools:

```toml
[tools.claude]
install_command = "npm install -g @anthropic-ai/claude-code@beta"
auto_update = false

[tools.custom_tool]
enabled = true
install_command = "cargo install my-custom-ai-tool"
```

### Authentication Settings

Configure authentication behavior:

```toml
[authentication]
browser_prevention = true    # Prevent automatic browser opening
session_timeout = 3600      # Session timeout in seconds
auto_retry = true           # Retry authentication on failure
```

### Logging Configuration

Control logging behavior:

```toml
[logging]
level = "info"              # debug, info, warn, error
file_logging = true         # Enable file-based logging
log_directory = "~/.config/terminal-jarvis/logs"
max_log_files = 10
```

## Environment Variables

Terminal Jarvis respects these environment variables:

- `TERMINAL_JARVIS_CONFIG` - Override config file location
- `TERMINAL_JARVIS_LOG_LEVEL` - Override log level
- `TERMINAL_JARVIS_NO_UPDATE_CHECK` - Disable update checking

## Troubleshooting Configuration

### Common Issues

1. **Config not loading**: Ensure proper TOML syntax
2. **Tool not recognized**: Check `config/tools/` directory
3. **Permission errors**: Verify file permissions

### Debug Configuration

Run with debug logging to troubleshoot configuration issues:

```bash
TERMINAL_JARVIS_LOG_LEVEL=debug terminal-jarvis list
```

### Reset Configuration

To reset to defaults, remove or rename your config files:

```bash
mv ~/.config/terminal-jarvis/config.toml ~/.config/terminal-jarvis/config.toml.backup
```