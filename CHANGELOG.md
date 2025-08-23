# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.67] - 2025-08-23

### Fixed
- **Workflow Optimization**: Removed unnecessary local Homebrew Formula handling
- **Deployment Reliability**: Eliminated detached HEAD errors from Formula commit attempts
- **Clean Architecture**: Deleted local `homebrew/Formula/` directory - using separate tap repository instead

### Technical
- Removed "Update local Formula file" and "Commit local Formula update" workflow steps
- Simplified Homebrew deployment to only update actual tap repository
- Fixed workflow failures caused by attempting to commit non-existent local Formula

## [0.0.66] - 2025-08-23

### Technical
- **Remote Pipeline Testing**: Deploy with all v0.0.65 fixes to validate GitHub Actions workflow
- **Deployment Verification**: Testing sequential workflow, intelligent Cargo.lock handling, and GH_TOKEN fixes
- **Quality Assurance**: Validating that critical local-cd.sh bug fix resolves crates.io publication issues

## [0.0.65] - 2025-08-23

### Fixed

- **CRITICAL: Deployment Script Bug**: Resolved major flaw in version management workflow
  - **Root Cause**: `local-cd.sh --update-version` was updating `Cargo.toml` but NOT updating `Cargo.lock`
  - **Impact**: Caused persistent workflow failures as `Cargo.lock` contained outdated version numbers
  - **Resolution**: Added `cargo check` to `update_all_versions` function to synchronize `Cargo.lock` with `Cargo.toml` changes
  - **Prevention**: Future version bumps will automatically maintain `Cargo.lock` synchronization

- **Crates.io Publication Workflow**: Enhanced Cargo.lock handling for deployment pipeline
  - **Intelligent Validation**: Pre-publication checks now distinguish between user errors and necessary workflow updates
  - **Conditional --allow-dirty Usage**: Smart logic that only uses --allow-dirty when `Cargo.lock` is updated by crates.io index refresh
  - **Clean State Enforcement**: Ensures repository state is clean while allowing necessary dependency updates during workflow execution

### Enhanced

- **Deployment Pipeline Reliability**: Complete overhaul with lessons learned from debugging session
  - **Sequential Workflow Optimization**: Reordered job dependencies (Crates.io → GitHub Release → Homebrew → NPM)
  - **Fail-Fast Architecture**: If any step fails, subsequent steps are skipped to prevent partial releases
  - **Version Consistency**: Local deployment script now maintains proper synchronization between all version files

### Technical

- **Version Management**: Fixed fundamental flaw where `--update-version` created inconsistent state between `Cargo.toml` and `Cargo.lock`
- **Workflow Debugging**: Extensive troubleshooting session revealed and resolved deployment pipeline issues that had been causing persistent failures
- **Quality Assurance**: Enhanced pre-flight validation prevents version mismatches from reaching CI/CD pipeline

## [0.0.64] - 2025-08-23

### Fixed

- **Deployment Pipeline Reliability**: Resolved critical issues that caused v0.0.63 release failure
  - **Crates.io Publication**: Fixed "uncommitted Cargo.lock" error by implementing intelligent Cargo.lock handling
    - Added pre-publication Cargo.lock validation and update logic
    - Implemented conditional --allow-dirty flag usage when Cargo.lock updates during workflow execution
    - Ensures clean git state while handling necessary Cargo.lock updates from crates.io index refresh
  - **Homebrew Tap Update**: Fixed missing GH_TOKEN environment variable causing exit code 4 failure
  - **Sequential Workflow**: Restructured deployment to be sequential (crates.io → GitHub release → NPM → Homebrew)
  - **Failure Prevention**: If any step fails, subsequent steps are skipped to avoid partial releases

### Enhanced

- **Deployment Workflow Robustness**: Improved error handling and validation
  - Added explicit Cargo.lock update and commit verification in local deployment script
  - Implemented proper sequential dependencies in GitHub Actions workflow
  - Enhanced git state validation with intelligent Cargo.lock management
  - Added comprehensive failure checks to prevent inconsistent multi-platform releases

### Technical

- **CI/CD Pipeline**: Complete workflow restructure to eliminate race conditions and partial deployments
- **Git Operations**: Intelligent Cargo.lock handling that distinguishes between user errors and necessary workflow updates

## [0.0.63] - 2025-08-23

### Fixed

- **CD Pipeline Optimization**: Improved Homebrew Formula synchronization in deployment pipeline
  - Enhanced version management and deployment workflow reliability
  - Better handling of multi-platform distribution requirements

### Enhanced

- **Release Process**: Added automated release process documentation to deployment pipeline
  - Improved deployment script comments for better maintainability
  - Enhanced Homebrew Formula with automated release process documentation

## [0.0.62] - 2025-08-23

### Fixed

- **Clippy Compliance**: Resolved clippy errors in test files preventing CI/CD pipeline execution
  - Fixed `items-after-test-module` error in `tests/codex_functionality_tests.rs` by moving non-test functions before test module
  - Fixed `items-after-test-module` error in `tests/auth_behavior_tests.rs` by moving non-test functions before test module
  - Removed empty line after doc comment to satisfy `empty-line-after-doc-comments` rule
  - All test files now comply with Rust code organization best practices

### Enhanced

- **Documentation Consolidation**: Completed unification of AI assistant guidelines
  - Created comprehensive `AGENTS.md` as single source of truth for all AI coding assistants
  - Streamlined `CLAUDE.md` and `.github/copilot-instructions.md` to reference unified guidelines
  - Eliminated redundant documentation while maintaining consistency across AI assistant workflows

- **Community Links**: Updated Discord server invitation links
  - Changed from expired `discord.gg/zNuyC5uG` to active `discord.gg/WteQm6MTZW`
  - Updated links across 6 files: PR template, README files, CI/CD scripts, CHANGELOG, and documentation

### Technical

- **Code Quality**: All clippy warnings resolved with strict `-D warnings` compliance
- **Test Organization**: Proper Rust file structure ensures reliable compilation and testing
- **CI/CD Pipeline**: Full pipeline now executes successfully with zero warnings

## [0.0.61] - 2025-08-21

### Fixed

- **Homebrew Binary Packaging**: Resolved critical issue where release archives contained debug directories instead of executable binaries
  - Fixed GitHub Actions workflow to exclude debug directories from uploads (contribution by @avivl)
  - Improved archive creation logic to target executable binaries only with `-type f -executable` filter
  - Changed artifact pattern from `terminal-jarvis*` to exact `terminal-jarvis` to prevent debug file inclusion
  - Added local Homebrew Formula for testing and validation
  - Verified fix prevents `terminal-jarvis.d` debug directories from being packaged instead of actual binaries

### Technical

- **Archive Creation Enhancement**: Enhanced multi-platform build system to ensure only executable binaries are packaged in release archives
- **Contributor Recognition**: @avivl's fix resolves deployment pipeline issue affecting Homebrew distribution

## [0.0.60] - 2025-08-19

### Enhanced

- **Homebrew Formula Robustness**: Implemented architecture-specific binary selection for improved multi-platform support
  - Enhanced Formula generation to use modern `on_macos` and `on_linux` blocks with Hardware::CPU detection
  - Added Intel vs ARM architecture-specific archive selection for both macOS and Linux
  - Implemented robust fallback mechanisms for unsupported architectures
  - Preserved simplified archives while adding detailed architecture-specific support
  - Addresses multi-architecture scenarios for optimal binary compatibility across different hardware

### Technical

- **Multi-Architecture Support**: Enhanced CD workflow to generate comprehensive Formula with per-architecture checksums
- **Homebrew Best Practices**: Updated Formula structure to follow modern Homebrew conventions
- **Fallback Strategy**: Robust error handling for edge cases and unsupported platforms

## [0.0.59] - 2025-08-19

### Fixed

- **Multi-Platform CD Pipeline**: Resolved GitHub release creation permissions and archive naming compatibility
  - Added required permissions (contents: write, packages: write, id-token: write) to fix 403 Forbidden errors
  - Enhanced release asset preparation to create both simplified Homebrew-compatible archives and detailed platform-specific archives
  - Preserved Aviv Laufer's detailed platform-specific archive logic while adding universal mac/linux archives
  - Ensured compatibility between automated CD workflow and manual update-formula.sh script

### Technical

- **Archive Strategy**: Dual archive approach for maximum compatibility
  - Simplified archives: terminal-jarvis-mac.tar.gz, terminal-jarvis-linux.tar.gz (for Homebrew)
  - Detailed archives: terminal-jarvis-{platform}-{arch}.tar.gz (for advanced users)
- **Workflow Permissions**: Added comprehensive GitHub Actions permissions for release automation

## [0.0.58] - 2025-08-19

### Enhanced

- **CI/CD Pipeline**: Resolved test race conditions and optimized GitHub Actions workflows
  - Fixed parallel test execution conflicts with ENV_TEST_MUTEX in auth_behavior_tests.rs and integration_auth_tests.rs
  - Corrected README formatting issues in generate-readme-tools.sh AWK logic
  - Streamlined CI workflow by removing redundant build job while maintaining multiplatform-build and npm-test coverage
  - Prepared multi-platform CD workflow for comprehensive deployment testing

### Technical

- **Test Infrastructure**: Implemented mutex-based synchronization for integration tests
- **Documentation Pipeline**: Fixed AWK script logic to prevent double blank lines in generated README sections
- **Workflow Optimization**: Reduced CI resource usage while maintaining comprehensive quality checks

### Contributors

- **Aviv Laufer (@avivl)**: Multi-platform build system enhancements via [PR #5](https://github.com/BA-CalderonMorales/terminal-jarvis/pull/5)
  - Comprehensive multi-platform build infrastructure with GitHub Actions workflows
  - Created advanced build scripts supporting macOS universal binaries and Linux targets
  - Established CD pipeline foundations with build-multiplatform.sh and enhanced Homebrew release generation
  - Added detailed documentation in docs/MULTIPLATFORM_BUILD.md
  - Resolved Windows support complexities by strategically removing Windows targets while maintaining comprehensive cross-platform coverage

### Acknowledgments

- **Claude AI (@anthropic-ai/claude-code)**: Collaborative development partner in multi-platform build system design and implementation

## [0.0.57] - 2025-08-16

### Enhanced

- **Security Policy**: Updated response timelines to reflect realistic hobby project maintainer availability
  - Changed critical vulnerability response from "Immediate (24 hours)" to "Best effort within 1-2 weeks"
  - Adjusted high severity response from "48 hours" to "2-3 weeks"
  - Extended medium severity response to "Within 1 month"
  - Added "maintainer availability" context throughout severity levels

### Technical

- **Major Code Architecture Refactoring**: Complete modularization of large source files
  - Refactored `src/tools/` module from single large file to focused domain modules
  - Refactored `src/services/` module following same modular pattern
  - Refactored `src/config/` module into focused configuration management modules
  - Refactored `src/auth_manager/` module for better separation of concerns
  - Refactored `src/theme/` module to match established patterns
  - Refactored `src/api/` layer to align with domain-based module structure
  - Applied consistent naming convention: `{module}_domain_operations.rs`
  - Eliminated dead code warnings by removing 260+ lines of unused functions
  - Improved code maintainability and developer experience
  - All modules now follow established `mod.rs` re-export pattern

- **Documentation Cleanup**: Removed REFACTOR.md after completing comprehensive refactoring initiative
- **Code Quality**: Applied clippy and formatting improvements across all refactored modules

## [0.0.56] - 2025-08-14

### Added

- **Professional T.JARVIS Advisory System**: Complete tool warning message overhaul
  - Custom T.JARVIS-themed startup guidance for all 7 AI coding tools (claude, gemini, qwen, opencode, llxprt, codex, crush)
  - Professional Unicode box-drawing advisories with perfect 62-character alignment
  - Tool-specific guidance: Authentication advisories for API key tools, startup advisories for TUI tools, setup advisories for configuration tools
  - Eliminated all raw technical warnings (e.g., "WARNING: claude may attempt to open a browser")
  - Theme-integrated message styling using current T.JARVIS theme colors

### Enhanced

- **CLI Warning Messages**: Complete user-friendliness overhaul throughout the application
  - Updated NPM/Node.js warnings: "Node.js runtime environment not detected" instead of "NPM is not installed or not in PATH"
  - OpenCode shell guidance: "OpenCode requires shell environment refresh to update PATH" with quick fix instructions
  - Professional error message styling: "⚠ ADVISORY:", "✗ SYSTEM:", "✓ COMPLETE:" prefixes with theme integration
  - Consistent messaging across all CLI operations and error handling

- **Professional Documentation System**: Complete documentation overhaul for enterprise standards
  - Removed all emojis from docs/ directory following strict NO EMOJIS policy
  - Enhanced ARCHITECTURE.md with comprehensive theme system documentation
  - Unified SOURCES.md table format with GitHub repository links for all 7 tools
  - Streamlined all documentation for professional appearance and accessibility
  - Added theme system architecture details including theme.rs and theme_config.rs modules

### Fixed

- **Perfect Box Alignment**: Corrected all advisory box border misalignments
  - Fixed content lines exceeding 62-character width causing right border displacement
  - Standardized all box components (headers, content, borders) to exact 62-character alignment
  - Eliminated broken bottom-right corners in Unicode box-drawing characters

## [0.0.55] - 2025-08-12

### Added

- **Professional Logging System**: Complete emoji removal and log4net-style timestamp implementation
  - Modular logging architecture following SOLID principles with separated concerns
  - Core logging utilities: `scripts/logger/log_utils.sh`, `scripts/logger/log_config.sh`, `scripts/logger/logger.sh`
  - Professional timestamps format: `[2025-08-12 14:30:15.123]` for all script output
  - Color-coded log levels: RED (errors), GREEN (success), BLUE (info), YELLOW (warnings), PURPLE (debug), CYAN (headers)
  - Environment detection with automatic color disabling for CI/CD pipelines
  - Comprehensive coverage across all 14 shell scripts in scripts/ folder

### Enhanced

- **Terminal Output Quality**: Eliminated all emoji clutter for professional, enterprise-ready terminal experience
- **Script Maintainability**: Centralized logging configuration with consistent interface across all scripts
- **Accessibility**: Professional output compatible with screen readers and enterprise environments
- **CI/CD Integration**: Automatic detection and adaptation for automation environments

### Technical

- **Complete Emoji Removal**: Systematic replacement of visual emojis with professional logging functions across entire scripts folder
- **SOLID Architecture**: Single Responsibility, Open-Closed, and Dependency Inversion principles applied to logging system design
- **Modular Design**: Reusable logging components that can be extended for future enhancements

## [0.0.54] - 2025-08-12

### Added

- **Security Audit Report**: Code and documentation review across the codebase (scope, tools, and limitations documented). If citing "40+ files across 6 directories," ensure the exact scope is stated in and linked from the report.
  - Public security audit notice for v0.0.54 release transparency
  - Systematic directory-by-directory security analysis methodology
  - No critical or high-severity issues were identified during the review; see the Security Audit Report for findings, scope, limitations, and residual risk
  - Assessment aligned to OWASP Top 10 categories; coverage and mapping documented in the report
  - Coordinated vulnerability disclosure process and ongoing security commitment (see SECURITY.md)
  - Enterprise readiness review completed; this is not a formal certification. See the report for scope and non-claims

### Enhanced

- **Security Documentation**: Prominent maintainer recommendations for production users
  - Moved security recommendations to top of report for better visibility  
  - Clear methodology documentation showing AI-assisted comprehensive review process
  - Explicit scope and limitations of the AI-assisted analysis, including out-of-scope areas and potential false-negative risk
  - Ongoing vulnerability management and community contribution guidelines

## [0.0.53] - 2025-08-11

### Enhanced

- **Documentation Consistency**: Updated all documentation files to reflect current v0.0.53 status
  - Consolidated SUPPORTED_TOOLS.md content into comprehensive SOURCES.md
  - Updated tool counts from 6 to 7 tools across all testing documentation
  - Synchronized version references across installation guides
  - Improved README.md structure with centralized tool documentation reference

### Fixed

- **Documentation Accuracy**: Corrected outdated version references and tool counts
  - Fixed TESTING.md references from 5/6 tools to 7 tools
  - Updated INSTALLATION.md current version from v0.0.43 to v0.0.53
  - Ensured all NPM package descriptions include Crush in tool listings

## [0.0.52] - 2025-08-10

### Added

- **Crush Integration**: Added support for Charm's Crush AI coding assistant
  - Multi-model support with OpenAI, Anthropic, Google, Groq, and more
  - LSP-enhanced context for improved code understanding
  - Session-based workflows with context preservation
  - MCP (Model Context Protocol) support for extensibility
  - Flexible configuration with local model support (Ollama, LM Studio)
  - Installation via NPM: `npm install -g @charmland/crush`

### Fixed

- **llxprt Package Configuration**: Fixed inconsistent package name in terminal-jarvis.toml.example
  - Changed from incorrect `@vybestack/llxprt-code-core` to correct `@vybestack/llxprt-code`
  - Resolves concurrent update failures and directory conflicts
  - Ensures proper tool updates without interfering with other packages

### Enhanced

- **Testing Infrastructure**: Updated CI/testing scripts to validate all 7 AI tools
  - Comprehensive test coverage for Crush integration (6 new tests)
  - NPM package validation for all tools including Crush
  - Updated tool count validation from 6 to 7 tools across all test suites

## [Unreleased]

### Added

- **Crush Integration**: Added support for Charm's Crush AI coding assistant
  - Multi-model support with OpenAI, Anthropic, Google, Groq, and more
  - LSP-enhanced context for improved code understanding
  - Session-based workflows with context preservation
  - MCP (Model Context Protocol) support for extensibility
  - Flexible configuration with local model support (Ollama, LM Studio)
  - Installation via NPM: `npm install -g @charmland/crush`

## [0.0.51] - 2025-08-11

### Fixed

- **README.md Title Alignment**: Fixed NPM package README.md to have centered title alignment
  - Root README.md already had proper `<div align="center">` wrapper
  - NPM package README.md was missing title inside the alignment block
  - Ensures consistent branding across all distribution channels

### Enhanced

- **Documentation Quality**: Updated copilot-instructions.md and CLAUDE.md to emphasize Homebrew Formula version synchronization
  - Added explicit warnings about commonly forgotten `homebrew/Formula/terminal-jarvis.rb` version updates
  - Enhanced local-cd.sh script to detect and warn about Homebrew Formula version mismatches
  - Prevents deployment failures due to version inconsistencies

## [0.0.50] - 2025-08-10

### Added

- **Comprehensive Contribution System**: Complete contributor onboarding and governance framework

  - **Discord-First Workflow**: Mandatory community discussion before PR submission via [Discord](https://discord.gg/WteQm6MTZW)
  - **Advanced PR Template**: 8 PR types (docs, feature, bugfix, security, UI, logic, maintenance, testing)
  - **Complete CONTRIBUTIONS.md**: Full contributor guide with coding standards, testing requirements, and realistic expectations
  - **Quality Gate Enforcement**: TDD requirements, code formatting, and comprehensive testing protocols

- **Community Support Integration**: Professional project support and recognition system
  - **Buy Me a Coffee**: Integrated support badges and dedicated project support section
  - **Multi-Platform Badges**: Enhanced badge organization across NPM, Crates.io, and Homebrew distributions
  - **Community-Focused Documentation**: Emphasis on community building and contributor recognition

### Enhanced

- **Developer Experience**: Streamlined contribution process with clear expectations and realistic contributor limitations
- **Project Sustainability**: Professional support system enables continued development and maintenance
- **Documentation Quality**: Comprehensive guides covering everything from first contribution to advanced tool integration
- **Community Governance**: Clear communication channels and discussion framework via Discord

## [0.0.49] - 2025-08-10

### Added

- **Enhanced Exit Options**: Revolutionary context switching system for AI tool workflows

  - **Smart Exit Menu**: Three strategic options after tool completion instead of simple "Press Enter"
  - **Rapid Context Switching**: "🤖 Switch to Another AI Tool" enables seamless LLM transitions
  - **Workflow Preservation**: Maintains user momentum with minimal navigation friction
  - **Complete Control**: Options for main menu return, tool switching, or full exit

- **Interactive UI Improvements**: Complete menu restructure with perfect visual alignment
  - **Emoji-Aware Border Calculation**: Proper visual width handling for 2-column emoji display
  - **Menu Hierarchy**: Organized main menu with "AI CLI Tools" submenu for scalability
  - **Comprehensive Resources**: "Important Links" menu with GitHub, NPM, Cargo, docs access
  - **Consistent Tool Ordering**: BTreeMap implementation ensures predictable tool sequence

### Enhanced

- **User Experience**: 60-70% reduction in navigation overhead for multi-tool workflows
- **Context Switching**: Loop-based AI tools menu enables rapid LLM switching without menu traversal
- **Visual Design**: Perfect border alignment and emoji spacing throughout interface
- **Session Management**: Robust interrupt handling (Ctrl+C) across all menu contexts

### Technical

- **Function Architecture**: Restructured `handle_ai_tools_menu()` with loop support for context switching
- **Visual Width Calculation**: Unicode emoji detection (≥0x1F300) with proper column counting
- **Menu State Management**: Intelligent flow control between main menu, submenus, and tools
- **Error Handling**: Comprehensive user interruption support with graceful fallbacks

## [0.0.48] - 2025-08-10

### Added

- **Version Caching System**: Intelligent caching of NPM distribution tag information to eliminate repeated API calls

  - **Cache Storage**: Local TOML-based cache files with configurable TTL (default: 1 hour)
  - **CLI Management**: `terminal-jarvis cache` commands for status, refresh, and clearing
  - **Automatic Expiration**: Smart cleanup of expired cache entries
  - **Graceful Fallback**: Falls back to direct API calls if caching fails

- **Enhanced Deployment Workflow**: Improved local-cd.sh script with better integration reminders

  - **Pre-flight Checks**: Automatic version suggestions and CHANGELOG.md readiness validation
  - **Homebrew Integration**: Automatic Homebrew Formula updates during version bumps
  - **Documentation Links**: Built-in references to CLAUDE.md and copilot-instructions.md
  - **Post-deployment Actions**: Clear step-by-step instructions for completing Homebrew and NPM publishing

- **Documentation Improvements**: Enhanced deployment guidance across multiple files
  - **CLAUDE.md**: Added comprehensive deployment checklist with step-by-step workflow
  - **copilot-instructions.md**: Enhanced with mandatory pre-deployment reminders and quick reference commands
  - **VERSION_CACHING.md**: Complete documentation of the new caching system with examples and troubleshooting

### Enhanced

- **Performance**: Eliminated API call delays on Terminal Jarvis home page through intelligent caching
- **User Experience**: Faster interactive mode startup with cached version data (loads instantly on subsequent runs)
- **Network Efficiency**: Reduced NPM registry API calls from every startup to once per hour (configurable)
- **Development Workflow**: More seamless deployment process with integrated reminders and automation
- **Error Handling**: Comprehensive error handling for cache operations with non-fatal failures

### Technical

- **Cache Architecture**: TOML-based cache storage in standard config directory (`~/.config/terminal-jarvis/version_cache.toml`)
- **Test Coverage**: Comprehensive unit tests for all caching functionality including expiration, file I/O, and error scenarios
- **Clippy Compliance**: All code passes strict clippy linting with no warnings
- **Backward Compatibility**: Zero breaking changes - all existing functionality preserved

## [0.0.47] - 2025-08-09

### Added

- **Multi-Platform Distribution**: Complete distribution pipeline supporting NPM, Crates.io, and Homebrew

  - **Homebrew Integration**: Added comprehensive Homebrew Formula and release archive system based on Federico Terzi's approach
  - **Homebrew Testing**: Created local testing protocols using local taps and HTTP servers for end-to-end validation
  - **Distribution Channel Separation**: Clear installation paths for different user ecosystems (Node.js, Rust, macOS/Linux package managers)

- **Enhanced Documentation**: Comprehensive documentation updates across multiple files

  - **Homebrew Integration Guide**: Added detailed Homebrew publishing workflow, Formula structure, and testing protocols
  - **Multi-Platform Installation**: Updated README.md with center-aligned badges for all distribution channels
  - **Common Pitfalls Documentation**: Added solutions for archive naming, SHA256 mismatches, binary permissions, and cross-platform issues
  - **Development Guidelines**: Enhanced copilot-instructions.md and CLAUDE.md with Homebrew integration lessons learned

- **Testing Infrastructure**: New testing tools and protocols
  - **test-homebrew-formula.sh**: Comprehensive Homebrew Formula validation script
  - **Local Testing Strategy**: Documentation for testing without requiring actual GitHub repositories
  - **End-to-End Validation**: Proven workflow for complete Homebrew installation testing

### Enhanced

- **Deployment Pipeline**: Enhanced local-cd.sh script with multi-platform publishing support

  - **Step 5: Homebrew Archive Creation**: Automated creation of platform-specific release archives
  - **Crates.io Integration**: Streamlined publishing to crates.io as Step 4 in deployment workflow
  - **Multi-Platform Summary**: Enhanced deployment summary showing status of all three distribution channels

- **Pre-Commit Workflow**: Updated checklists to include Homebrew validation requirements

  - **Homebrew Formula Testing**: Added requirements for local Formula testing before deployment
  - **Version Synchronization**: Extended to include Homebrew Formula version consistency
  - **Archive Validation**: Added SHA256 checksum verification requirements

- **Visual Presentation**: Improved README.md badge organization
  - **Center-Aligned Badges**: All distribution badges (NPM, Crates.io, Homebrew, License) now center-aligned
  - **Grouped Organization**: Maintained clear grouping with HTML comments for different distribution channels
  - **Professional Appearance**: Enhanced visual presentation of multi-platform distribution options

### Technical

- **Homebrew Formula**: Created multi-platform Formula with conditional URL/SHA256 handling

  - **Cross-Platform Support**: Separate configurations for macOS and Linux with proper `on_macos`/`on_linux` blocks
  - **GitHub Release Integration**: Formula URLs point to GitHub release assets for archive downloads
  - **Binary Installation**: Proper `bin.install` configuration with executable permissions preserved

- **Build System**: Enhanced archive creation and validation

  - **Platform-Specific Archives**: Automated creation of `terminal-jarvis-{macos|linux}.tar.gz` with proper naming
  - **SHA256 Generation**: Automatic checksum calculation for Formula verification
  - **Binary Permissions**: Ensured executable permissions are preserved in release archives

- **Documentation Synchronization**: Maintained consistency across all documentation files
  - **NPM Package README**: Automated syncing of README changes to NPM package
  - **Version References**: Consistent version information across all documentation
  - **Installation Instructions**: Updated with all three distribution methods

## [0.0.46] - 2025-08-09

### Enhanced

- **Crates.io Publishing**: Terminal Jarvis now available via `cargo install terminal-jarvis`
  - Integrated crates.io publishing into deployment pipeline as Step 4
  - Enhanced local-cd.sh script with automated Rust crate publishing
  - Added comprehensive crates.io publishing documentation and workflows

### Fixed

- **Version Consistency**: Enhanced version synchronization across all project files
  - Improved version validation and consistency checking in deployment scripts
  - Better error handling for version mismatches during deployment

## [0.0.45] - 2025-08-08

### Fixed

- **Session Continuation Infinite Loop**: Fixed critical issue where exit commands in AI tools caused infinite restart loops
  - **Problem**: Users typing `/exit`, `/quit`, `/bye` in tools like claude would trigger session continuation and endlessly restart the tool
  - **Solution**: Modified session continuation logic to explicitly exclude exit commands from restart triggers
  - **Result**: Exit commands now properly terminate tools and return to Terminal Jarvis interface
  - Only explicit authentication/setup commands (`/auth`, `/login`, `/config`, `/setup`) now trigger session continuation
  - Eliminated false positives from quick tool completions that previously caused unwanted restarts

### Enhanced

- **Version Management**: Improved local-cd.sh script with comprehensive programmatic version updates
  - Added automated version synchronization across all project files (Cargo.toml, package.json, index.ts)
  - Enhanced version validation and consistency checking
  - Improved error handling for version mismatches during deployment

## [0.0.44] - 2025-08-08

### Added

- **Session Continuation System**: Implemented intelligent session continuation for AI coding tools
  - **Backslash Command Fix**: Prevents users from being kicked out of tools when using internal commands like `/auth`, `/help`, `/config`
  - **Seamless Authentication**: Tools that exit after authentication flows are automatically restarted to continue user sessions
  - **Smart Detection**: Identifies internal commands vs intentional exits to provide appropriate behavior
  - **Multi-Tool Support**: Works with llxprt, gemini, claude, and other AI coding assistants
  - Added comprehensive test suite for session continuation validation

### Enhanced

- **Documentation**: Created comprehensive NPM package source documentation
  - Added [`docs/SOURCES.md`](docs/SOURCES.md) with complete installation guide for all 6 AI coding tools
  - Detailed NPM package mappings, installation commands, and troubleshooting information
  - Security considerations and alternative installation methods documented
  - Enhanced user experience with clear package source references

### Fixed

- **CLI User Experience**: Eliminated anti-pattern where users were forced to exit tools during authentication workflows
  - **Previous behavior**: User runs tool → types `/auth` → tool exits → shows completion message → returns to main menu
  - **New behavior**: User runs tool → types `/auth` → authentication completes → tool session continues seamlessly
  - Maintains backward compatibility while improving user workflow continuity

## [0.0.43] - 2025-08-08

### Added

- **OpenAI Codex CLI Integration**: Added complete support for OpenAI Codex CLI as the 6th AI coding tool
  - New tool: `@openai/codex` NPM package for AI coding agent that runs locally
  - Comprehensive codex functionality with authentication via OpenAI API key or ChatGPT account
  - Added 6 dedicated codex functionality tests covering authentication, NPM packages, and terminal compatibility
  - Enhanced smoke tests with 7 codex-specific validation checks
  - Added codex to example configuration file with proper NPM package setup
  - All authentication mechanisms properly tested including CODEX_NO_BROWSER environment handling

### Enhanced

- **CI/CD Pipeline Improvements**: Enhanced local-ci.sh with comprehensive codex validation and improved testing accuracy
  - Updated all tool count references from 5 to 6 tools throughout the codebase
  - Enhanced test suite descriptions to accurately reflect 6-tool validation
  - Improved final validation summary with specific codex functionality mentions
  - All tests now properly validate the complete 6-tool ecosystem (claude, gemini, qwen, opencode, llxprt, codex)
  - Comprehensive test coverage now includes 46 total tests (up from 44)

### Fixed

- **Test Pattern Accuracy**: Fixed codex-specific test patterns in smoke test suite
  - Corrected grep patterns for codex API key detection to work with multi-line auth configuration
  - Fixed codex help message validation to properly detect OpenAI platform URL references
  - All 46 tests now pass consistently, providing reliable CI/CD validation

## [0.0.42] - 2025-08-08

### Enhanced

- **CI/CD Testing Infrastructure**: Improved test coverage and validation systems in preparation for codex integration
  - Enhanced NPM package validation framework to support additional tools
  - Improved authentication testing mechanisms
  - Strengthened configuration consistency validation across all files

## [0.0.41] - 2025-08-08

### Fixed

- **OpenCode Input Focus**: Fixed input box focus issue on fresh installs
  - Added special terminal state preparation for opencode to ensure immediate input focus
  - Implemented minimal terminal clearing sequence to avoid interference with opencode initialization
  - Added 75ms initialization delay to prevent race conditions between Terminal Jarvis and opencode
  - Comprehensive test suite with failing tests → passing tests following TDD approach
  - Eliminates need for manual clicking to focus input box on startup

## [0.0.40] - 2025-08-08

### Fixed

- **Browser Opening Prevention**: Fixed unwanted browser authentication behavior for Gemini CLI and Qwen Code
  - Added AuthManager system to detect headless/CI environments and prevent browser opening
  - Tools now properly prompt for API keys instead of opening browsers in terminal environments
  - Comprehensive integration tests to prevent regression of browser opening behavior
  - Fixed failing integration test for environment variable setup

## [0.0.39] - 2025-08-08

### Fixed

- **NPM Publishing Reliability**: Removed automated NPM publishing from CI/CD scripts to prevent 2FA authentication failures
  - Scripts now handle Git operations (commit/tag/push) reliably
  - Manual NPM publishing prevents terminal-based 2FA authentication issues
- **Maintainer Documentation**: Added comprehensive docs/MAINTAINERS.md with detailed NPM publishing procedures
  - Step-by-step manual publishing instructions with 2FA handling
  - Troubleshooting guide for common NPM publishing issues
  - Distribution tag management procedures

### Enhanced

- **Deployment Workflow**: Improved reliability by separating automated Git operations from manual NPM publishing
- **Error Prevention**: Scripts now provide clear instructions for manual NPM publishing instead of failing on authentication

## [0.0.38] - 2025-08-08

### Added

- **Separated CI/CD Scripts**: Split local-cicd.sh into dedicated local-ci.sh and local-cd.sh scripts
  - `local-ci.sh`: Validation-only script (no commits/pushes) for safe testing
  - `local-cd.sh`: Deployment-only script (commit/tag/push/publish) with version verification
  - Both scripts include option 6 for manually pre-updated versions with safety validation
- **Interactive Workflow Dashboard**: Renamed and enhanced status.sh to workflow-dashboard.sh
  - Updated to reference new CI/CD script separation
  - Provides contextual recommendations based on branch state
- **Personalized CLAUDE.md**: Comprehensive AI assistant guide for project maintenance
  - Enhanced from .github/copilot-instructions.md with project-specific guidance
  - Covers architecture, development standards, version management, and release process

### Enhanced

- **Version Safety Checks**: Both CI and CD scripts now validate version consistency across all files
- **Pre-deployment Validation**: Prevents accidental version mismatches before deployment
- **Development Workflow**: Clear separation between validation (CI) and deployment (CD) operations

## [0.0.37] - 2025-08-06

### Enhanced

- **README Visual Improvements**: Added promo image at 50% width for better visual presentation
- **Documentation Alignment**: Fixed terminal header alignment in text documentation files

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
