# Architecture and Technical Details

This document provides technical information about Terminal Jarvis's architecture, design decisions, and internal structure.

## Project Structure

The project follows a domain-based modular architecture designed for maintainability, extensibility, and clear separation of concerns:

```
src/
├── main.rs                    # Entry point - minimal code, delegates to CLI
├── lib.rs                     # Library entry point for module organization
├── cli.rs                     # Clean, expressive CLI interface definitions
├── installation_arguments.rs # Installation commands and NPM validation
├── progress_utils.rs          # Theme-integrated messaging and progress indicators
│
├── cli_logic/                 # Business logic domain (9 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── cli_logic_entry_point.rs        # Main CLI logic entry point
│   ├── cli_logic_interactive.rs        # Interactive T.JARVIS interface
│   ├── cli_logic_tool_execution.rs     # Tool execution workflows
│   ├── cli_logic_update_operations.rs  # Tool update management
│   ├── cli_logic_list_operations.rs    # Tool listing operations
│   ├── cli_logic_info_operations.rs    # Tool information display
│   ├── cli_logic_config_management.rs  # Configuration management
│   ├── cli_logic_template_operations.rs # Template system operations
│   └── cli_logic_utilities.rs          # Shared utility functions
│
├── auth_manager/              # Authentication domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── auth_entry_point.rs            # Authentication system entry point
│   ├── auth_environment_detection.rs  # Environment detection logic
│   ├── auth_environment_setup.rs      # Environment configuration
│   ├── auth_api_key_management.rs     # API key handling
│   └── auth_warning_system.rs         # Browser prevention warnings
│
├── config/                    # Configuration domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── config_entry_point.rs          # Configuration system entry point
│   ├── config_structures.rs           # TOML configuration structures
│   ├── config_file_operations.rs      # File I/O operations
│   ├── config_manager.rs              # Configuration management logic
│   └── config_version_cache.rs        # Version caching system
│
├── services/                  # External integrations domain (5 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── services_entry_point.rs        # Service layer entry point
│   ├── services_package_operations.rs # Package management operations
│   ├── services_npm_operations.rs     # NPM-specific operations
│   ├── services_github_integration.rs # GitHub CLI integration
│   └── services_tool_configuration.rs # Tool configuration mapping
│
├── theme/                     # UI theming domain (7 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── theme_entry_point.rs           # Theme system entry point
│   ├── theme_definitions.rs           # Theme color definitions
│   ├── theme_config.rs                # Theme configuration management
│   ├── theme_global_config.rs         # Global theme state
│   ├── theme_background_layout.rs     # Background and layout styling
│   ├── theme_text_formatting.rs       # Text formatting utilities
│   └── theme_utilities.rs             # Theme utility functions
│
├── tools/                     # Tool management domain (6 focused modules)
│   ├── mod.rs                 # Module coordination and re-exports
│   ├── tools_entry_point.rs           # Tool system entry point
│   ├── tools_detection.rs             # Tool detection and verification
│   ├── tools_command_mapping.rs       # Command-to-tool mapping
│   ├── tools_execution_engine.rs      # Tool execution engine
│   ├── tools_process_management.rs    # Process lifecycle management
│   └── tools_startup_guidance.rs      # Tool startup guidance system
│
└── api/                       # API framework domain (4 focused modules)
    ├── mod.rs                 # Module coordination and re-exports
    ├── api_base.rs            # Base API route configurations
    ├── api_client.rs          # HTTP client abstraction layer
    └── api_tool_operations.rs # API tool operation endpoints
```

## Architecture Philosophy

Terminal Jarvis follows a **domain-based modular architecture** where large functional areas are broken into focused modules within dedicated folders. This approach provides:

### Core Design Principles

- **Domain Separation**: Each major functional area (CLI logic, authentication, configuration, etc.) is organized into its own folder with focused modules
- **Module Coordination**: Each domain folder contains a `mod.rs` file that handles re-exports and minimal coordination logic
- **Focused Modules**: Individual modules average 150-200 lines, handling single responsibilities within their domain
- **Clear Entry Points**: Each domain has a dedicated entry point module for external interaction

### Domain Architecture

- **`main.rs` & `lib.rs`**: Minimal entry points that delegate to domain modules
- **`cli.rs`**: Expressive command definitions with optional subcommands (defaults to interactive mode)
- **`cli_logic/`**: Complete business logic domain including the interactive T.JARVIS interface, tool execution workflows, and operation management
- **`auth_manager/`**: Authentication domain with environment detection, browser prevention, and API key management
- **`config/`**: Configuration domain handling TOML file operations, structure management, and version caching
- **`services/`**: External integrations domain for package management, NPM operations, and GitHub CLI integration
- **`theme/`**: UI theming domain with color definitions, global state management, and formatting utilities
- **`tools/`**: Tool management domain covering detection, command mapping, execution, and process lifecycle
- **`api/`**: API framework domain for future web integrations (currently unused)

### Modular Benefits

- **Maintainability**: Clear separation of concerns makes code easier to understand and modify
- **Testability**: Focused modules can be tested independently with clear interfaces
- **Extensibility**: New functionality can be added within appropriate domains without affecting other areas
- **Refactoring Safety**: Compilation-driven refactoring ensures changes don't break existing functionality
- **Code Quality**: Smaller modules reduce complexity and eliminate dead code more effectively

The interactive mode provides a complete T.JARVIS experience with real-time tool status, installation management, and a beautiful terminal interface, all coordinated through this modular architecture.

## Dependency Policy and Security (cargo-deny)

Terminal Jarvis enforces dependency hygiene and supply-chain safety using cargo-deny, configured via the repository-level `deny.toml`.

What cargo-deny does for us:
- Security advisories: Fails on known-vulnerable crates and surfaces audit risks.
- License allowlist: Ensures all dependencies comply with permissive licenses approved for this project.
- Duplicate/banned crates: Highlights multiple versions and optionally denies problematic crates.
- Source policy: Restricts dependencies to the official crates.io index and flags unknown git sources.

How to run it locally:
- Full suite: `cargo deny check`
- Specific checks: `cargo deny check advisories`, `cargo deny check licenses`, `cargo deny check bans`, `cargo deny check sources`

Key policy highlights (see `deny.toml`):
- Licenses allowlist includes MIT, Apache-2.0, BSD-2/3-Clause, ISC, Zlib, 0BSD, MPL-2.0, CC0-1.0, and a few tooling-specific exceptions (e.g., LLVM exception) with a detection confidence threshold of 0.8.
- Advisories: no global ignores by default; any temporary ignore must include justification.
- Bans: multiple versions are warned (to encourage deduplication) and wildcards are not used in this workspace. Explicit allow/deny lists are supported for escalations.
- Sources: Only the crates.io index is allowed; unknown registries and git sources are warned by default to prevent accidental drift.

Team guidance:
- When adding or updating dependencies, verify cargo-deny passes without new errors.
- If a license or advisory exception is truly needed, add it narrowly in `deny.toml` with a reason comment and open a follow-up to remove it.
- Keep dependency updates small and frequent; prefer upstream fixes for advisories.

Quality gates integration:
- We treat cargo-deny as part of our pre-commit checks alongside `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- CI should run `cargo deny check` to prevent merging policy regressions.

## SBOM Scanning and Vulnerability Gate

We generate a Software Bill of Materials (SBOM) and scan it for known vulnerabilities using Anchore's scan action in CI. This provides a cross-ecosystem view that covers both Rust (Cargo) and Node (NPM) dependencies.

CI policy (tuned to reduce noise):
- Severity threshold: critical (we fail only on Critical vulnerabilities).
- Fail behavior: Build fails on main when a Critical is found.
- PR experience: The job is marked as allowed to continue on pull_request events so findings surface without blocking developer workflows.
- Visibility: Results are uploaded as SARIF to GitHub Code Scanning for centralized visibility under the Security tab.

Rationale:
- Defense in depth across ecosystems (Rust + Node) with a single gate.
- Reduced flakiness by gating only on Critical severity while still surfacing High issues in SARIF.
- Non-blocking signal during PR iteration, with enforcement at merge time.

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

Terminal Jarvis is designed to make adding new CLI tools straightforward using the modular architecture:

1. **Define CLI interface** in `cli.rs` with new command structure
2. **Add tool configuration** in `services/services_tool_configuration.rs` for display name mapping
3. **Update tool detection** in `tools/tools_detection.rs` and `tools/tools_command_mapping.rs`
4. **Implement tool execution** in `cli_logic/cli_logic_tool_execution.rs`
5. **Add service operations** in appropriate `services/` modules if external integrations are needed

Example structure for adding a new tool:

```rust
// In cli.rs
#[derive(Parser)]
pub struct NewToolArgs {
    // Tool-specific command arguments
}

// In tools/tools_command_mapping.rs
pub fn get_command_mapping() -> HashMap<&'static str, &'static str> {
    let mut commands = HashMap::new();
    // Add new tool mapping
    commands.insert("newtool", "new-tool-cli");
    commands
}

// In services/services_tool_configuration.rs
pub fn get_display_name_to_config_mapping() -> HashMap<String, String> {
    let mut mapping = HashMap::new();
    // Add new tool configuration mapping
    mapping.insert("New Tool".to_string(), "newtool".to_string());
    mapping
}

// In cli_logic/cli_logic_tool_execution.rs
pub fn handle_new_tool_execution(args: &NewToolArgs) -> Result<()> {
    // Tool-specific execution logic here
}
```

This modular approach ensures that new tools integrate cleanly with all existing systems (detection, execution, configuration, and service management) while maintaining the separation of concerns.

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
