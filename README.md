# Terminal Jarvis

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Downloads](https://img.shields.io/npm/dm/terminal-jarvis.svg)](https://www.npmjs.com/package/terminal-jarvis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A thin Rust wrapper that provides a unified interface for managing and running AI coding tools. In the midst of all the tools out there that you can possibly use to keep track of them, here's a "shovel" that just works to try them all out.

🎉 **Now available on NPM!** Get started instantly with `npx terminal-jarvis`

## Quick Start

```bash
# Try it instantly with npx (no installation required)
npx terminal-jarvis

# Or install globally
npm install -g terminal-jarvis
```

## Features

Terminal Jarvis serves as a command-line orchestrator for various AI coding tools, providing:

- **Unified Interface**: Single CLI to manage multiple AI coding tools
- **Built-in Tool Support**: 
  - `claude-code`
  - `gemini-cli`
  - `qwen-code`
  - `opencode`
- **Extensible Architecture**: Easy addition of new CLI tools
- **Package Management**: 
  - Update all packages at once
  - Update specific packages individually
  - Run individual packages with custom arguments
- **Template Management**: Create and maintain your own GitHub repository for agent templates (requires `gh` CLI and user consent)

## Installation

### NPM (Recommended)

```bash
# Install globally via NPM
npm install -g terminal-jarvis

# Or run directly with npx (no installation required)
npx terminal-jarvis
```

### From Source

```bash
# Clone the repository
git clone https://github.com/BA-CalderonMorales/terminal-jarvis.git
cd terminal-jarvis

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Run a specific tool
terminal-jarvis run claude-code --prompt "Refactor this function"
terminal-jarvis run gemini-cli --file src/main.rs
terminal-jarvis run qwen-code --analyze
terminal-jarvis run opencode --generate

# Update packages
terminal-jarvis update                    # Update all packages
terminal-jarvis update claude-code        # Update specific package

# List available tools
terminal-jarvis list

# Show tool information
terminal-jarvis info claude-code
```

### Template Management

```bash
# Initialize template repository (requires gh CLI)
terminal-jarvis templates init

# Create a new template
terminal-jarvis templates create my-template

# List available templates
terminal-jarvis templates list

# Use a template
terminal-jarvis templates apply my-template
```

## Project Structure

The project follows a modular architecture designed for maintainability and extensibility:

```
src/
├── main.rs           # Entry point - minimal code, delegates to CLI
├── cli.rs            # Clean, expressive CLI interface definitions
├── cli_logic.rs      # Business logic separated from CLI implementation
├── services.rs       # Service layer for external tools (gh CLI, etc.)
├── api.rs            # Modular API endpoint definitions
├── api_base.rs       # Base API route configurations
└── api_client.rs     # HTTP client abstraction layer (reqwest wrapper)
```

### Architecture Philosophy

- **`main.rs`**: Entry point with minimal code - simply bootstraps the CLI
- **`cli.rs`**: Expressive command definitions that clearly show what each command does
- **`cli_logic.rs`**: All business logic separated from CLI parsing for better testability
- **`services.rs`**: Service layer for external integrations (GitHub CLI, package managers)
- **`api.rs`**: Modular API layer for potential future web integrations
- **`api_base.rs`**: Base configurations and route definitions
- **`api_client.rs`**: HTTP client abstraction for easy swapping of underlying libraries

The `cli.rs` file maintains clean separation by calling services and API routes in an understandable, non-overwhelming manner.

## Supported Tools

| Tool | Description | Status |
|------|-------------|--------|
| `claude-code` | Anthropic's Claude for code assistance | ✅ Supported |
| `gemini-cli` | Google's Gemini CLI tool | ✅ Supported |
| `qwen-code` | Qwen coding assistant | ✅ Supported |
| `opencode` | Open-source coding tool | ✅ Supported |

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

## Requirements

- Rust 1.70 or later
- `gh` CLI (for template management features)
- Internet connection (for package updates)

## Configuration

Terminal Jarvis looks for configuration in the following locations:

1. `~/.config/terminal-jarvis/config.toml`
2. `./terminal-jarvis.toml` (project-specific)

Example configuration:

```toml
[tools]
claude-code = { enabled = true, auto_update = true }
gemini-cli = { enabled = true, auto_update = false }
qwen-code = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }

[templates]
repository = "your-username/jarvis-templates"
auto_sync = true
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following the project structure
4. Ensure tests pass (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## NPM Distribution

Terminal Jarvis is also available as an NPM package for easy installation and usage:

```bash
# Install globally via NPM
npm install -g terminal-jarvis

# Or run directly with npx
npx terminal-jarvis --help
```

The NPM packaging approach follows the excellent guidance from [Packaging Rust Applications for the NPM Registry](https://blog.orhun.dev/packaging-rust-for-npm/) by Orhun Parmaksız. This allows us to distribute platform-specific binaries through NPM while maintaining the convenience of `npx` for quick execution.

## Roadmap

- [ ] Enhanced error handling and logging
- [ ] Configuration file validation
- [ ] Plugin system for custom tools
- [ ] Shell completion scripts
- [ ] Docker container support
- [ ] Web dashboard for tool management
