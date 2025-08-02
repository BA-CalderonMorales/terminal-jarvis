# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
