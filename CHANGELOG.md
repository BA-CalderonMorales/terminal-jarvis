# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.36] - 2025-08-06

### Added

- **Real-time NPM Distribution Tag Detection**: Dynamic display of all applicable NPM distribution tags
  - Shows all matching tags (e.g., "v0.0.36 (@stable, beta, latest)") for complete transparency
  - Smart progress indicators during NPM tag fetching with "🔍 Checking NPM distribution tags"
  - Development builds show matching tags with "-dev" suffix to distinguish from published versions
- **Enhanced Welcome Interface**: Integrated GitHub and NPM package links directly in T.JARVIS interface
  - Direct GitHub link: https://github.com/BA-CalderonMorales/terminal-jarvis
  - Direct NPM package link: https://www.npmjs.com/package/terminal-jarvis
  - Professional integration within the futuristic ASCII art border design

### Enhanced

- **User Experience**: Comprehensive progress feedback for all network operations
  - NPM tag detection shows clear progress and completion status
  - Users always know when Terminal Jarvis is fetching external information
- **Transparency**: Complete visibility into distribution channel status
  - No more confusion about which NPM tag a user is running
  - Clear distinction between development builds and published versions
- **Debugging Support**: Easy access to source code and package information for troubleshooting

### Fixed

- **NPM Tag Display Logic**: Resolved edge cases where multiple tags point to the same version
  - Previously showed only prioritized tag, now shows all applicable tags
  - Eliminates user confusion when installing via specific tags (e.g., @beta)

## [0.0.35] - 2025-08-06

### Added

- **NPM Distribution Tag Detection**: Dynamic detection and display of NPM distribution tags (@stable, @beta, @latest)
  - Smart tag prioritization: stable > beta > latest when multiple tags point to same version
  - Development builds show all matching tags with "-dev" suffix for transparency
  - Real-time NPM tag fetching with progress indicators
- **Enhanced Welcome Interface**: Added GitHub and NPM package links for easy debugging
  - Direct links to source code: https://github.com/BA-CalderonMorales/terminal-jarvis
  - Direct links to NPM package: https://www.npmjs.com/package/terminal-jarvis
  - Professional integration within the futuristic ASCII art interface

### Enhanced

- **User Experience**: Progress indicators for all network operations (NPM tag detection, tool status loading)
- **Transparency**: Users can now see exactly which distribution channel they're running
- **Documentation**: Updated README.md with limitations, installation guides, and architecture docs
  - Created docs/LIMITATIONS.md for known issues and workarounds
  - Created docs/INSTALLATION.md for platform-specific setup instructions
  - Created docs/ARCHITECTURE.md for technical implementation details
- **Version Display**: Complete version information including distribution channel visibility

### Fixed

- **Authentication Documentation**: Documented known Gemini and Qwen login issues with workarounds
- **macOS Prerequisites**: Clear documentation that Rust toolchain is required on macOS
- **Tool Status Indicators**: Enhanced tool testing status for Opencode and LLxprt

## [0.0.34] - 2025-08-05

### Fixed

- **NPM Registry Sync**: Republished with correct LLxprt integration (v0.0.33 was duplicate of v0.0.32)
- **Package Content**: Ensured NPM package contains actual LLxprt-enabled binary

## [0.0.33] - 2025-08-05

### Added

- **LLxprt Code Integration**: Added support for LLxprt Code multi-provider AI coding assistant
  - New tool: `@vybestack/llxprt-code` for enhanced AI coding features
  - Comprehensive multi-provider AI support with advanced capabilities
  - Integrated into installation and configuration systems

### Fixed

- **Claude/Gemini Installation**: Fixed incorrect NPM package names causing installation failures
  - Claude: `@anthropic-ai/claude-cli` → `@anthropic-ai/claude-code`
  - Gemini: `@google/generative-ai-cli` → `@google/gemini-cli`
- **Configuration Consistency**: Updated all configuration files to use correct package names
- **Services Installation Logic**: Added missing installation cases for Claude and Gemini tools

### Enhanced

- **CI/CD Pipeline**: Added comprehensive NPM package validation to prevent future package name issues
- **Test Suite**: Consolidated testing into single comprehensive script (`smoke-test.sh`)
- **Package Validation**: Validates package existence, installability, and binary name consistency
- **Configuration Validation**: Ensures consistency across installation_arguments.rs, config.rs, and example files
- **Tool Ecosystem**: Expanded supported AI coding tools from 4 to 5 tools

## [0.0.32] - 2025-08-06

### Added

- **NPM Package Tests**: Automatic validation of all NPM packages before release
- **Dry-run Installation Tests**: Validates packages can be installed without actually installing them
- **Binary Name Verification**: Ensures NPM packages provide expected binary names (claude, gemini, etc.)

## [0.0.28] - 2025-01-26

### Fixed

- **NPM Package Configuration**: Fixed "Tool not found in configuration" errors in NPM package installations
- **Configuration Loading**: Added NPM package config path to configuration loading sequence
- **Package Bundling**: Included default configuration files (`config/default.toml`) in NPM package
- **NPX Execution**: Resolved configuration loading issues when using `npx terminal-jarvis` commands

### Enhanced

- **Configuration System**: Improved config path resolution to handle NPM package installations
- **Package Structure**: Added proper config directory bundling for standalone NPM package functionality
- **Debugging**: Enhanced configuration loading to work across different installation methods

## [0.0.23] - 2025-08-03

### Added

- **NPM Distribution Tags**: Added support for stable and beta release channels
- New installation options: `npm install -g terminal-jarvis@stable` and `npm install -g terminal-jarvis@beta`
- Enhanced README with stable/beta badges and installation channel explanations
- Interactive dist-tag prompts in local CI/CD script for optional tagging

### Enhanced

- **Release Process**: Improved local-cicd.sh with interactive npm dist-tag management
- **Documentation**: Updated copilot instructions with comprehensive npm dist-tags guidance
- **User Experience**: Clear visual indicators for different release channels

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
