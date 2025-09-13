# Terminal Jarvis Development Container

This devcontainer is optimized for Terminal Jarvis development with prebuilt Rust environment, Node.js, and all necessary development tools.

## Features

- **Rust Development Environment**: Latest Rust with clippy, rustfmt, and rust-analyzer
- **Node.js LTS**: For NPM package development and testing
- **GitHub CLI**: For GitHub operations and release management
- **Docker-in-Docker**: For container-based builds and testing
- **VS Code Extensions**: Rust Analyzer, Biome, GitHub Copilot, and development essentials

## Quick Start

After the container builds and initializes:

```bash
# Check Rust installation
cargo --version

# Build and check the project
cargo check

# Run Terminal Jarvis
cargo run -- --help
cargo run -- list

# Run tests
cargo test

# Run CI validation
./scripts/cicd/local-ci.sh

# Work on NPM package
cd npm/terminal-jarvis
npm install
npm run build
```

## Development Workflow

### Local Development
```bash
# Edit Rust code in src/
# VS Code will provide real-time analysis and error checking

# Test changes
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test

# Format code
cargo fmt --all
```

### Release Workflow
```bash
# Update CHANGELOG.md first (always!)
# Then run deployment
./scripts/cicd/local-cd.sh --check-versions
./scripts/cicd/local-ci.sh
./scripts/cicd/local-cd.sh
```

### NPM Package Development
```bash
cd npm/terminal-jarvis
npm run sync-readme  # Sync README from root
npm run build        # Build TypeScript
npm run lint         # Check with Biome
npm pack             # Test package creation
```

## Environment Variables

- `RUST_LOG=debug` - Enable debug logging
- `CARGO_TERM_COLOR=always` - Colored Cargo output
- `CARGO_INCREMENTAL=1` - Enable incremental compilation
- `RUST_BACKTRACE=1` - Show backtraces on panic

## Ports

- `3000` - Development server
- `8000` - Local file server (for Homebrew testing)
- `8080` - Alternative server port

## Tips

- The container includes Docker-in-Docker for testing containerized builds
- Rust dependencies are cached in the container for faster rebuilds
- Use the integrated terminal for the best development experience
- GitHub Copilot is preconfigured for AI-assisted development

## Troubleshooting

If you encounter issues:

1. **Rebuild the container**: Command Palette → "Dev Containers: Rebuild Container"
2. **Check Rust installation**: `rustup show`
3. **Update Rust**: `rustup update`
4. **Clear Cargo cache**: `cargo clean`

For project-specific issues, refer to the main project documentation and the AGENTS.md guidelines.
