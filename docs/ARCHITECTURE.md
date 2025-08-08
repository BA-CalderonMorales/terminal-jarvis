# Architecture and Technical Details

This document provides technical information about Terminal Jarvis's architecture, design decisions, and internal structure.

## Project Structure

The project follows a modular architecture designed for maintainability and extensibility:

```
src/
├── main.rs               # Entry point - minimal code, delegates to CLI
├── cli.rs                # Clean, expressive CLI interface definitions
├── cli_logic.rs          # Business logic with interactive T.JARVIS interface
├── tools.rs              # Tool management and detection logic
├── auth_manager.rs       # Authentication management and browser prevention
├── installation_arguments.rs # Installation commands and NPM validation
├── services.rs           # Service layer for external tools (gh CLI, etc.)
├── config.rs             # TOML configuration management
├── api.rs                # Modular API endpoint definitions (future use)
├── api_base.rs           # Base API route configurations (future use)
└── api_client.rs         # HTTP client abstraction layer (future use)
```

## Architecture Philosophy

- **`main.rs`**: Entry point with minimal code - simply bootstraps the CLI
- **`cli.rs`**: Expressive command definitions with optional subcommands (defaults to interactive mode)
- **`cli_logic.rs`**: Complete business logic including the interactive T.JARVIS interface with ASCII art
- **`tools.rs`**: Comprehensive tool detection using multiple verification methods (`which`, `--version`, `--help`)
- **`auth_manager.rs`**: Authentication management, environment detection, and browser opening prevention for headless/CI environments
- **`installation_arguments.rs`**: Centralized installation commands with NPM dependency validation
- **`services.rs`**: Service layer for external integrations (GitHub CLI, package managers)
- **`config.rs`**: TOML configuration file management
- **API modules**: Framework code for future web integrations (currently unused)

The interactive mode provides a complete T.JARVIS experience with real-time tool status, installation management, and a beautiful terminal interface.

## Adding New Tools

Terminal Jarvis is designed to make adding new CLI tools straightforward:

1. Define the tool configuration in `cli_logic.rs`
2. Add the command interface in `cli.rs`
3. Implement any required services in `services.rs`
4. Update the tool registry

Example structure for adding a new tool:

```rust
// In cli_logic.rs
pub fn handle_new_tool(args: &NewToolArgs) -> Result<()> {
    // Tool-specific logic here
}

// In cli.rs
#[derive(Parser)]
pub struct NewToolArgs {
    // Tool arguments
}
```

## NPM Distribution Technical Details

Terminal Jarvis is packaged for NPM following the approach outlined in [Packaging Rust Applications for the NPM Registry](https://blog.orhun.dev/packaging-rust-for-npm/) by Orhun Parmaksız. This allows us to distribute platform-specific binaries through NPM while maintaining the convenience of `npx` for quick execution.

### Package Contents

- Pre-compiled Rust binaries for immediate functionality
- TypeScript wrapper for NPM integration
- Zero additional runtime dependencies
- Complete tool management capabilities

### What's Included:

- Full interactive T.JARVIS interface
- Pre-compiled binary for your platform
- Zero additional dependencies
- Complete tool management capabilities

### Size Considerations

**Current Package Size**: ~1.2MB compressed / ~2.9MB unpacked

The package includes bundled Rust binaries to ensure immediate functionality without requiring a Rust toolchain installation. Future optimizations planned:

- Platform-specific packages to reduce download size
- Binary compression techniques
- Splitting debug symbols
- On-demand binary downloading

## Tool Detection Architecture

Terminal Jarvis uses a multi-method approach for tool detection:

1. **PATH Detection**: Uses `which` command to locate binaries
2. **Version Verification**: Attempts `--version` flag to confirm functionality
3. **Help Validation**: Falls back to `--help` for tools without version flags
4. **Caching**: Results are cached to improve subsequent performance

This approach ensures reliable detection across different installation methods and environments.

## Configuration System

Terminal Jarvis uses TOML configuration files with the following precedence:

1. `./terminal-jarvis.toml` (project-specific, highest priority)
2. `~/.config/terminal-jarvis/config.toml` (user-specific)
3. Built-in defaults (fallback)

### Configuration Schema

```toml
[tools]
claude = { enabled = true, auto_update = true }
gemini = { enabled = true, auto_update = false }
qwen = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }
llxprt = { enabled = true, auto_update = true }

[templates]
repository = "your-username/jarvis-templates"
auto_sync = true

[ui]
show_ascii_art = true
center_output = true
```

## Future Architecture Plans

- **Plugin System**: Modular plugin architecture for custom tools
- **Web Dashboard**: Optional web interface for tool management
- **Enhanced API**: REST API for integration with other tools
- **Docker Support**: Containerized deployment options
- **Shell Integration**: Native shell completion and integration
