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
├── theme.rs              # Professional theme definitions and ANSI color management
├── theme_config.rs       # Global theme state management and configuration
├── progress_utils.rs     # Theme-integrated messaging and progress indicators
├── api.rs                # Modular API endpoint definitions (future use)
├── api_base.rs           # Base API route configurations (future use)
└── api_client.rs         # HTTP client abstraction layer (future use)
```

## Architecture Philosophy

- **`main.rs`**: Entry point with minimal code - simply bootstraps the CLI
- **`cli.rs`**: Expressive command definitions with optional subcommands (defaults to interactive mode)
- **`cli_logic.rs`**: Complete business logic including the interactive T.JARVIS interface with ASCII art
- **`tools.rs`**: Comprehensive tool detection using multiple verification methods (`which`, `--version`, `--help`)
- **Authentication & Browser Prevention**: Built-in system for detecting headless/CI environments and preventing unwanted browser authentication flows
- **`installation_arguments.rs`**: Centralized installation commands with NPM dependency validation
- **`services.rs`**: Service layer for external integrations (GitHub CLI, package managers)
- **`config.rs`**: TOML configuration file management
- **Theme System**: Professional theming architecture with global consistency
  - **`theme.rs`**: Three professionally designed themes (T.JARVIS, Classic, Matrix) with complete ANSI color definitions
  - **`theme_config.rs`**: Global theme state management and runtime theme switching
  - **`progress_utils.rs`**: Theme-integrated messaging system for consistent visual experience
- **API modules**: Framework code for future web integrations (currently unused)

The interactive mode provides a complete T.JARVIS experience with real-time tool status, installation management, and a beautiful terminal interface.

## Authentication & Environment Management

Terminal Jarvis includes sophisticated authentication management to prevent browser opening in headless environments:

- **Environment Detection**: Automatically detects CI environments, SSH sessions, Docker containers, and Codespaces
- **Browser Prevention**: Prevents tools like Gemini CLI and Qwen Code from opening browsers in terminal environments
- **API Key Guidance**: Provides clear instructions for API key setup when browser authentication is blocked
- **Seamless Integration**: Works transparently with all supported AI coding tools

## Terminal State Management

For optimal tool integration, Terminal Jarvis implements careful terminal state management:

- **Minimal Interference**: Uses minimal terminal clearing sequences to avoid conflicts with tool initialization
- **State Preparation**: Prepares terminal state for tools that require specific input focus (like OpenCode)
- **Race Condition Prevention**: Includes initialization delays to prevent conflicts between Terminal Jarvis and launched tools

## Theme System Architecture (v0.0.56+)

Terminal Jarvis implements a comprehensive theming system for professional, consistent visual experience:

### Core Theme Components

- **`theme.rs`**: Theme definitions with complete ANSI color palettes
  - **T.JARVIS Theme**: Professional blue-based corporate theme with cyan accents
  - **Classic Theme**: Traditional grayscale terminal aesthetic 
  - **Matrix Theme**: Green-on-black retro computing style
  - **Color Management**: Comprehensive ANSI escape code handling for backgrounds, borders, and text

- **`theme_config.rs`**: Global theme state and configuration management
  - **Runtime Theme Switching**: Dynamic theme changes without restart
  - **Theme Persistence**: User theme preferences maintained across sessions
  - **Global State**: Centralized theme state accessible throughout application

- **`progress_utils.rs`**: Theme-integrated messaging system
  - **Consistent Colors**: All messages use current theme's color palette
  - **Professional Presentation**: Success, warning, and error messages with themed colors
  - **T.JARVIS Branding**: Integrated T.JARVIS advisory system for tool guidance

### Visual Design Features

- **Perfect Box Borders**: Unicode box-drawing characters (╔══╗║║╚══╝) with precise alignment
- **Background Fills**: Complete background color coverage for professional appearance
- **Menu Integration**: inquire library integration with custom RenderConfig for themed menus
- **ASCII Art Integration**: T.JARVIS ASCII art with theme-appropriate coloring
- **Advisory Boxes**: Professional tool startup guidance with perfect 62-character alignment

### Theme Integration Points

- **Interactive Interface**: Main CLI interface uses current theme consistently
- **Tool Execution**: Pre-launch advisory messages themed appropriately
- **Error Handling**: Theme-aware error and warning message presentation
- **Menu Systems**: All interactive menus respect current theme selection
- **Progress Indicators**: Loading and status messages integrate with theme colors

This architecture ensures visual consistency throughout the entire Terminal Jarvis experience while maintaining professional appearance standards.

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
theme = "T.JARVIS"  # Options: "T.JARVIS", "Classic", "Matrix"

[theme]
# Theme-specific customizations (future use)
custom_colors = false
border_style = "rounded"  # Options: "rounded", "sharp", "minimal"
```

## Future Architecture Plans

- **Plugin System**: Modular plugin architecture for custom tools
- **Web Dashboard**: Optional web interface for tool management
- **Enhanced API**: REST API for integration with other tools
- **Docker Support**: Containerized deployment options
- **Shell Integration**: Native shell completion and integration
- **Theme Customization**: User-defined custom themes and color schemes
- **Dynamic Theming**: Context-aware themes based on tool or environment
- **Accessibility Themes**: High-contrast and colorblind-friendly theme variants
