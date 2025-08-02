# Terminal Jarvis

[![NPM Version](https://img.shields.io/npm/v/terminal-jarvis.svg)](https://www.npmjs.com/package/terminal-jarvis)
[![NPM Downloads](https://img.shields.io/npm/dm/terminal-jarvis.svg)](https://www.npmjs.com/package/terminal-jarvis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A thin Rust wrapper that provides a unified interface for managing and running AI coding tools like claude-code, gemini-cli, qwen-code, and opencode. Think of it as a package manager and runner for AI coding assistants.

🎉 **Now available on NPM!** Get started instantly with `npx terminal-jarvis`

## Quick Start

```bash
# Try it instantly with npx (no installation required)
npx terminal-jarvis

# Or install globally
npm install -g terminal-jarvis

# For full functionality, install from source:
cargo install --git https://github.com/BA-CalderonMorales/terminal-jarvis
```

> **Note**: The current NPM version (0.0.16) includes full binary functionality with the complete T.JARVIS interface. No additional installation required!

## Features

Terminal Jarvis serves as your AI coding assistant command center, providing:

- **🤖 Interactive T.JARVIS Interface**: Sleek terminal UI with ASCII art logo and responsive design
- **🚀 One-Click Tool Management**: Install, update, and run AI coding tools seamlessly
- **📦 Smart Installation Detection**: Automatically detects installed tools and their status
- **⚡ Quick Launch Mode**: Run tools directly from the interactive interface
- **🔧 Built-in Tool Support**: 
  - `claude` - Anthropic's Claude for code assistance
  - `gemini` - Google's Gemini CLI tool
  - `qwen` - Qwen coding assistant  
  - `opencode` - OpenCode AI coding agent built for the terminal
- **🎯 Intelligent NPM Validation**: Warns about missing dependencies and provides installation guidance
- **📱 Responsive Design**: Adapts to your terminal width for optimal display
- **🔄 Background Process Support**: Handles long-running tools appropriately
- **💬 Interactive Argument Input**: Prompt-based argument collection for tools
- **🛠️ Management Menu**: Organized options for installing, updating, and getting tool information

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
# Launch interactive T.JARVIS interface (recommended)
terminal-jarvis

# Or use direct commands:
terminal-jarvis run claude --prompt "Refactor this function"
terminal-jarvis run gemini --file src/main.rs
terminal-jarvis run qwen --analyze
terminal-jarvis run opencode --generate

# Install specific tools
terminal-jarvis install claude
terminal-jarvis install gemini

# Update packages
terminal-jarvis update                    # Update all packages
terminal-jarvis update claude             # Update specific package

# List available tools with status
terminal-jarvis list

# Show detailed tool information
terminal-jarvis info claude
```

### Interactive Mode Features

When you run `terminal-jarvis` without arguments, you get the full T.JARVIS experience:

- **🎨 Beautiful ASCII Art Interface**: Clean, centered T.JARVIS logo
- **📊 Real-time Tool Status**: See which tools are installed and ready to launch
- **⚡ Quick Launch**: Select tools and provide arguments interactively
- **🔧 Management Options**: Install, update, and get information about tools
- **💡 Smart Guidance**: Helpful tips and warnings about missing dependencies

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
├── main.rs               # Entry point - minimal code, delegates to CLI
├── cli.rs                # Clean, expressive CLI interface definitions  
├── cli_logic.rs          # Business logic with interactive T.JARVIS interface
├── tools.rs              # Tool management and detection logic
├── installation_arguments.rs # Installation commands and NPM validation
├── services.rs           # Service layer for external tools (gh CLI, etc.)
├── config.rs             # TOML configuration management
├── api.rs                # Modular API endpoint definitions (future use)
├── api_base.rs           # Base API route configurations (future use)
└── api_client.rs         # HTTP client abstraction layer (future use)
```

### Architecture Philosophy

- **`main.rs`**: Entry point with minimal code - simply bootstraps the CLI
- **`cli.rs`**: Expressive command definitions with optional subcommands (defaults to interactive mode)
- **`cli_logic.rs`**: Complete business logic including the interactive T.JARVIS interface with ASCII art
- **`tools.rs`**: Comprehensive tool detection using multiple verification methods (`which`, `--version`, `--help`)
- **`installation_arguments.rs`**: Centralized installation commands with NPM dependency validation
- **`services.rs`**: Service layer for external integrations (GitHub CLI, package managers)
- **`config.rs`**: TOML configuration file management
- **API modules**: Framework code for future web integrations (currently unused)

The interactive mode provides a complete T.JARVIS experience with real-time tool status, installation management, and a beautiful terminal interface.

## Supported Tools

| Tool | Description | NPM Package | Status |
|------|-------------|-------------|--------|
| `claude` | Anthropic's Claude for code assistance | `@anthropic-ai/claude-code` | ✅ Supported |
| `gemini` | Google's Gemini CLI tool | `@google/gemini-cli` | ✅ Supported |
| `qwen` | Qwen coding assistant | `@qwen-code/qwen-code` | ✅ Supported |
| `opencode` | OpenCode AI coding agent built for the terminal | Install script | ✅ Supported |

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

- **Node.js and NPM**: Required for most AI coding tools (automatic validation included)
- **Rust 1.70 or later**: For building from source
- **`gh` CLI**: Optional, for template management features
- **Internet connection**: For package updates and installations

Terminal Jarvis automatically detects missing dependencies and provides helpful installation guidance.

## Configuration

Terminal Jarvis looks for configuration in the following locations:

1. `~/.config/terminal-jarvis/config.toml`
2. `./terminal-jarvis.toml` (project-specific)

Example configuration:

```toml
[tools]
claude = { enabled = true, auto_update = true }
gemini = { enabled = true, auto_update = false }
qwen = { enabled = true, auto_update = true }
opencode = { enabled = false, auto_update = false }

[templates]
repository = "your-username/jarvis-templates"
auto_sync = true
```

## Package Information

**NPM Package Size**: ~1.2MB compressed / ~2.9MB unpacked

The NPM package includes pre-compiled Rust binaries for immediate functionality without requiring a Rust toolchain. This ensures you get the complete T.JARVIS experience out-of-the-box with `npx terminal-jarvis`.

**What's Included:**
- Full interactive T.JARVIS interface
- Pre-compiled binary for your platform
- Zero additional dependencies
- Complete tool management capabilities

Future versions will include size optimizations and platform-specific packages.

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

- [x] **Interactive T.JARVIS Interface**: Complete ASCII art terminal UI
- [x] **Smart Tool Detection**: Multi-method tool installation verification
- [x] **One-Click Installation**: Seamless tool installation with NPM validation
- [x] **Responsive Terminal Design**: Adaptive width and centered interface
- [ ] Enhanced error handling and logging
- [ ] Configuration file validation
- [ ] Plugin system for custom tools
- [ ] Shell completion scripts
- [ ] Docker container support
- [ ] Web dashboard for tool management
- [ ] Automated GitHub Actions builds for platform-specific NPM binaries
