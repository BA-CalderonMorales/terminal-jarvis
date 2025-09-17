# Multi-Platform Build System

This document describes Terminal Jarvis's multi-platform build system, which enables cross-compilation for multiple target platforms.

## Overview

Terminal Jarvis supports multi-platform builds using Rust's cross-compilation capabilities. The system can build binaries for:

- **macOS**: Intel (x86_64) and Apple Silicon (ARM64) architectures  
- **Linux**: Intel/AMD (x86_64) and ARM64 architectures

**Note**: Windows support was removed due to cross-compilation complexity and toolchain requirements. Windows users can build from source using `cargo install terminal-jarvis`.

## Architecture

### Build Scripts

1. **`scripts/utils/build-multiplatform.sh`** - Core multi-platform build system
2. **`scripts/utils/generate-homebrew-release.sh`** - Enhanced with cross-compilation support
3. **Updated CI/CD pipeline** - Integration with existing deployment workflow

### Target Platforms

| Platform | Target Triple | Status | Notes |
|----------|---------------|--------|--------|
| macOS Intel | `x86_64-apple-darwin` | Yes | Native on Intel Macs, cross-compile on ARM Macs |
| macOS ARM64 | `aarch64-apple-darwin` | Yes | Native on ARM Macs, cross-compile on Intel Macs |
| Linux x64 | `x86_64-unknown-linux-gnu` | Yes | Cross-compile from macOS using `cross` tool |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | 🚧 | Requires additional toolchain setup |

## Usage

### Quick Start

Build for current platform only:
```bash
./scripts/utils/build-multiplatform.sh --current-only
```

Build for all platforms:
```bash
./scripts/utils/build-multiplatform.sh
```

### CI/CD Integration

The multi-platform build system is integrated into the existing CI/CD pipeline:

**Enable multi-platform testing in CI:**
```bash
MULTIPLATFORM_BUILD=true ./scripts/cicd/local-ci.sh
```

**Homebrew release with multi-platform support:**
```bash
./scripts/utils/generate-homebrew-release.sh --stage
```

## Cross-Compilation Setup

### Quick Setup (Basic Cross-Compilation)

Install all Rust targets for basic cross-compilation:

```bash
# Install rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add cross-compilation targets
rustup target add x86_64-apple-darwin      # macOS Intel
rustup target add aarch64-apple-darwin     # macOS ARM64
rustup target add x86_64-unknown-linux-gnu # Linux x64
rustup target add aarch64-unknown-linux-gnu # Linux ARM64
```

### Advanced Setup (Full Cross-Compilation Toolchains)

For production-quality cross-compilation, additional toolchains are needed:

#### On macOS

```bash
# Install Xcode command line tools (if not already installed)
xcode-select --install

# For Linux cross-compilation (using cross)
cargo install cross --git https://github.com/cross-rs/cross
```

#### On Linux (Ubuntu/Debian)

```bash
# Update package lists
sudo apt update

# Install cross-compilation tools
cargo install cross --git https://github.com/cross-rs/cross

# For ARM64 Linux cross-compilation
sudo apt install -y gcc-aarch64-linux-gnu

# For macOS cross-compilation (requires macOS SDK - complex legal setup)
# Recommended: Use macOS runners in CI/CD instead
```


### Cross-Compilation Compatibility Matrix

| Host Platform | Target Platform | Status | Requirements |
|---------------|----------------|--------|--------------|
| macOS | macOS (other arch) | Yes Native | Xcode CLI tools |
| macOS | Linux | 🚧 Limited | `cross` tool (OpenSSL dependency issues) |
| Linux | Linux (other arch) | Yes Native | `gcc-*` packages |
| Linux | macOS | 🚧 Complex | macOS SDK (legal issues) |

**Legend:**
- **Native**: Supported with standard toolchain
- **Cross**: Supported with additional tools  
- 🚧 **Limited/Complex**: Possible but requires extensive setup

### Platform-Specific Requirements

#### macOS → Linux Cross-Compilation

For Linux cross-compilation from macOS, additional tools may be needed:

```bash
# Install cross-compilation toolchain (optional)
brew install FiloSottile/musl-cross/musl-cross

# Or use Docker-based approach
docker run --rm -v "$(pwd)":/usr/src/myapp -w /usr/src/myapp rust:1.89 cargo build --release --target x86_64-unknown-linux-gnu
```

#### Universal macOS Binaries

The build system automatically creates universal macOS binaries when both Intel and ARM builds succeed:

```bash
# This happens automatically in generate-homebrew-release.sh
lipo -create terminal-jarvis-intel terminal-jarvis-arm -output terminal-jarvis-universal
```

## Build Outputs

### Directory Structure

```
target/
├── release/
│   └── terminal-jarvis                    # Host platform binary
├── x86_64-apple-darwin/
│   └── release/
│       └── terminal-jarvis                # macOS Intel binary
├── aarch64-apple-darwin/
│   └── release/
│       └── terminal-jarvis                # macOS ARM binary
├── x86_64-unknown-linux-gnu/
│   └── release/
│       └── terminal-jarvis                # Linux x64 binary
└── aarch64-unknown-linux-gnu/
    └── release/
        └── terminal-jarvis                # Linux ARM64 binary
```

### Release Archives

The system generates platform-specific archives:

```
homebrew/release/
├── terminal-jarvis-mac.tar.gz     # macOS binary (universal if possible)
└── terminal-jarvis-linux.tar.gz   # Linux binary
```

## Advanced Usage

### Build Options

```bash
# Show help
./scripts/utils/build-multiplatform.sh --help

# Build current platform only
./scripts/utils/build-multiplatform.sh --current-only

# Force install all targets (useful for CI)
./scripts/utils/build-multiplatform.sh --force-install
```

### Environment Variables

- `MULTIPLATFORM_BUILD=true` - Enable multi-platform testing in CI
- `CROSS_COMPILE=1` - Force cross-compilation even on matching platforms

### Error Handling

The build system includes comprehensive error handling:

- **Graceful degradation**: Falls back to single-platform builds if cross-compilation fails
- **Clear error messages**: Explains missing toolchains and setup requirements
- **Build summaries**: Shows which platforms succeeded and failed

## Integration with Distribution Channels

### NPM Package

The NPM package continues to use a single binary (from the host platform), ensuring compatibility with existing workflows.

### Homebrew

Homebrew now receives true platform-specific binaries:
- macOS users get optimized macOS binaries
- Linux users get proper Linux binaries

### Crates.io

Crates.io distribution remains unchanged - users build from source for their platform.

## Troubleshooting

### Common Issues

1. **Cross-compilation fails**
   - Install missing targets: `rustup target add <target>`
   - Check system dependencies for target platform
   - Consider using Docker for consistent build environments

2. **Universal macOS binary creation fails**
   - `lipo` not available: Falls back to single architecture
   - Only one architecture built successfully: Uses available binary

3. **Linux cross-compilation from macOS fails**
   - Missing GNU toolchain: Install via Homebrew or use Docker
   - Dependency linking issues: Consider static linking options

### Debug Information

Build with debug information:

```bash
RUST_LOG=debug ./scripts/utils/build-multiplatform.sh
```

## Future Enhancements

### Planned Features

- **musl targets**: Static linking for maximum compatibility  
- **Container builds**: Docker-based consistent cross-compilation
- **ARM64 Linux**: Full ARM64 Linux support with proper toolchains
- **FreeBSD/OpenBSD**: Additional Unix-like platform support

### Performance Optimizations

- **Parallel builds**: Build multiple targets concurrently
- **Build caching**: Cache cross-compilation artifacts
- **Incremental builds**: Only rebuild changed targets

## Security Considerations

- All cross-compilation tools and targets are verified before installation
- Checksums are calculated for all generated binaries
- Build process maintains reproducible builds across platforms

## Compatibility

- **Minimum Rust version**: 1.89.0
- **Supported host platforms**: macOS (Intel/ARM), Linux (x64)
- **Target platforms**: macOS (Intel/ARM), Linux (x64/ARM64)

---

For questions or issues with the multi-platform build system, see:
- [GitHub Issues](https://github.com/BA-CalderonMorales/terminal-jarvis/issues)
- [Known Limitations](./LIMITATIONS.md)
- [Architecture Documentation](./ARCHITECTURE.md)