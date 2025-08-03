# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.19] - 2025-08-03

### Fixed
- **OpenCode Integration**: Fixed opencode command structure to work correctly with Terminal Jarvis
- OpenCode now uses proper command structure: `opencode .` for TUI mode and `opencode run [args]` for message mode
- Updated opencode installation configuration from incorrect NPM package to proper curl install script
- OpenCode now provides consistent user experience with other AI CLI tools

## [0.0.18] - 2025-08-03

### Fixed
- **Version Consistency**: Fixed version inconsistencies across all project files
- Updated README.md version reference from 0.0.16 to 0.0.18
- Fixed NPM package.json postinstall script version from 0.0.14 to 0.0.18
- Ensured all version references are synchronized before release

## [0.0.17] - 2025-08-02

### Added
- **Futuristic Terminal UX**: Implemented stunning neon cyan color scheme for T.JARVIS interface
- **Responsive ASCII Art**: T.JARVIS logo adapts to terminal width with fallback for small screens
- **Clean Minimal Design**: Removed cluttered "Ready to Launch" sections for streamlined experience
- **Seamless Navigation**: Moved settings to dedicated submenu for better organization
- **Interactive Mode**: Enhanced user experience with consistent color theming throughout

### Fixed
- **Clippy Issues**: Resolved all format string inlining warnings
- **Code Quality**: Fixed collapsible else-if patterns and improved readability
- Complete CI/CD pipeline validation with format, clippy, and tests

## [0.0.16] - 2025-08-02

### Fixed
- **CI/CD**: Resolved cargo fmt formatting issues in cli_logic.rs
- Removed extra blank lines and reformatted long lines to meet rustfmt standards
- Fixed GitHub Actions continuous integration pipeline failures

## [0.0.14] - 2025-08-02

### Fixed
- **Critical NPX Issue**: Fixed `npx terminal-jarvis` asking to reinstall package every time
- Changed package.json bin configuration from Node.js wrapper to direct binary execution
- Fixed postinstall script syntax error with proper escape sequences
- Refreshed bundled binary with latest v0.0.14 build for improved compatibility
- Ensured proper binary permissions and executable status in NPM package

### Changed
- Package now directly executes Rust binary instead of Node.js wrapper for better NPX compatibility
- **Package size optimized**: Reduced to ~1.2MB compressed / ~2.9MB unpacked (50% reduction)
- Removed redundant platform-specific binary since generic binary works across platforms
- Improved testing methodology with temporary environment validation before publishing

### Technical
- Package includes both generic and platform-specific binaries for maximum compatibility
- This establishes base case for future size optimization efforts
- Prioritizes immediate user experience over package size

## [0.0.13] - 2025-08-02

### Fixed
- Fixed NPX compatibility issue where `npx terminal-jarvis` would re-download package every time
- Changed package.json bin configuration from string to object format for proper NPX recognition
- NPX now correctly caches and reuses the installed package instead of asking to install repeatedly

## [0.0.12] - 2025-08-02

### Added
- Bundled Rust binary directly in NPM package for immediate full functionality
- Users now get complete T.JARVIS interface out-of-the-box with `npm install -g terminal-jarvis`
- No external dependencies or Rust installation required
- Enhanced build process with binary bundling and platform detection

### Changed  
- NPM package now includes pre-compiled binary in `bin/` directory
- TypeScript wrapper prioritizes bundled binary over external installations
- Updated postinstall message to reflect immediate availability of full interface
- Improved error messages for better troubleshooting

### Fixed
- Eliminated fallback mode for users without Rust installation
- Resolved issue where users saw installation instructions instead of T.JARVIS interface

## [0.0.6] - 2025-08-02

### Added
- Complete interactive mode with sleek T.JARVIS terminal interface
- Tool installation management through InstallationManager
- Comprehensive tool detection using ToolManager
- Support for claude, gemini, qwen, and opencode AI coding tools
- Install command for individual tools
- Enhanced CLI with optional commands (defaults to interactive mode)
- Real-time tool installation status checking
- NPM dependency validation and warning system
- Interactive tool selection and argument input
- ASCII art T.JARVIS logo in interactive mode
- Responsive terminal width detection for centered UI
- Tool management menu with install/update/info options
- Multi-select tool installation interface
- Error handling with user-friendly messages
- Background process support for long-running tools

### Changed
- Refactored CLI to support optional subcommands
- Updated tool detection logic to use multiple verification methods
- Improved error messages with emoji indicators
- Enhanced package service with better tool support
- Restructured codebase with separate modules for installation and tools

### Technical
- Added inquire, shell-words, and term_size dependencies
- Implemented proper clippy compliance with #[allow(dead_code)] attributes
- Fixed all format string warnings for better performance
- Added comprehensive tool command mapping system
- Implemented async tool execution with proper stdio inheritance

## [0.0.5] - Previous Release
- Basic CLI structure and commands
- Initial package management functionality
- GitHub service integration
- Template system foundation

## [0.0.4] - Previous Release
- Core CLI framework implementation
- Basic tool detection

## [0.0.3] - Previous Release
- Initial project structure
- NPM packaging setup

## [0.0.2] - Previous Release
- Basic Rust CLI foundation

## [0.0.1] - Initial Release
- Project initialization
- Basic project structure
