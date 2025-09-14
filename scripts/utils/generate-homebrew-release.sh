#!/bin/bash

# Generate Homebrew Release Archives with True Multi-Platform Support
# This script creates platform-specific archives using cross-compilation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }
print_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Platform targets for cross-compilation
MACOS_TARGET="x86_64-apple-darwin"
MACOS_ARM_TARGET="aarch64-apple-darwin"
LINUX_TARGET="x86_64-unknown-linux-gnu"
LINUX_ARM_TARGET="aarch64-unknown-linux-gnu"

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RELEASE_DIR="$PROJECT_ROOT/homebrew/release"

print_info "Terminal Jarvis Multi-Platform Release Archive Generator"
print_info "Project root: $PROJECT_ROOT"

# Check if we're in the right directory
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    print_error "Not in Terminal Jarvis project root directory"
    exit 1
fi

# Create release directory
mkdir -p "$RELEASE_DIR"

# Function to check if target is installed
check_target() {
    local target=$1
    if rustup target list --installed | grep -q "$target"; then
        return 0
    else
        return 1
    fi
}

# Function to install target if not present
install_target() {
    local target=$1
    print_step "Installing Rust target: $target"
    if command -v rustup >/dev/null 2>&1; then
        rustup target add "$target"
    else
        print_warning "rustup not found, assuming target $target is available"
    fi
}

# Function to build for specific target
build_for_target() {
    local target=$1
    local binary_name=$2
    
    print_step "Building for target: $target"
    
    # Check and install target if needed
    if command -v rustup >/dev/null 2>&1; then
        if ! check_target "$target"; then
            install_target "$target"
        fi
    fi
    
    # Build for the specific target using cross if available for better cross-compilation
    local build_command="cargo"
    if command -v cross >/dev/null 2>&1; then
        # Get current target to avoid using cross for native builds
        local current_os=$(uname -s)
        local current_arch=$(uname -m)
        local current_target=""
        
        case "$current_os-$current_arch" in
            "Darwin-arm64") current_target="aarch64-apple-darwin" ;;
            "Darwin-x86_64") current_target="x86_64-apple-darwin" ;;
            "Linux-x86_64") current_target="x86_64-unknown-linux-gnu" ;;
            "Linux-aarch64") current_target="aarch64-unknown-linux-gnu" ;;
        esac
        
        if [[ "$target" != "$current_target" ]]; then
            build_command="cross"
            print_info "Using cross for cross-compilation to $target"
        fi
    fi
    
    $build_command build --release --target "$target"
    
    # Check if binary exists
    local binary_path="target/$target/release/terminal-jarvis"
    
    if [ -f "$binary_path" ]; then
        cp "$binary_path" "$RELEASE_DIR/$binary_name"
        print_info "Built and copied binary for $target"
    else
        print_error "Binary not found at $binary_path"
        return 1
    fi
}

# Function to create universal macOS binary
create_universal_macos_binary() {
    print_step "Creating universal macOS binary..."
    
    local intel_binary="$RELEASE_DIR/terminal-jarvis-intel"
    local arm_binary="$RELEASE_DIR/terminal-jarvis-arm"
    local universal_binary="$RELEASE_DIR/terminal-jarvis-macos"
    
    # Check if both binaries exist
    if [ -f "$intel_binary" ] && [ -f "$arm_binary" ]; then
        # Create universal binary using lipo
        if command -v lipo >/dev/null 2>&1; then
            lipo -create "$intel_binary" "$arm_binary" -output "$universal_binary"
            print_info "Created universal macOS binary"
            # Remove individual architecture binaries
            rm "$intel_binary" "$arm_binary"
        else
            print_warning "lipo not available, using Intel binary for macOS"
            mv "$intel_binary" "$universal_binary"
            [ -f "$arm_binary" ] && rm "$arm_binary"
        fi
    elif [ -f "$intel_binary" ]; then
        print_warning "Only Intel binary available, using for macOS"
        mv "$intel_binary" "$universal_binary"
    elif [ -f "$arm_binary" ]; then
        print_warning "Only ARM binary available, using for macOS"
        mv "$arm_binary" "$universal_binary"
    else
        print_error "No macOS binaries found"
        return 1
    fi
}

cd "$PROJECT_ROOT"

# Clean previous archives and binaries
print_info "Cleaning previous archives..."
rm -f "$RELEASE_DIR"/*.tar.gz
rm -f "$RELEASE_DIR"/terminal-jarvis-*

# Build for all target platforms
print_info "Building cross-platform binaries..."

# Detect current platform to optimize build strategy
CURRENT_OS=$(uname -s)
CURRENT_ARCH=$(uname -m)

print_info "Current platform: $CURRENT_OS ($CURRENT_ARCH)"

# Build strategy based on current platform
if [[ "$CURRENT_OS" == "Darwin" ]]; then
    # Running on macOS - can build native macOS binaries
    print_step "Building macOS binaries (native compilation)..."
    
    # Enhanced multi-architecture macOS build strategy
    if [[ "$CURRENT_ARCH" == "arm64" ]]; then
        # ARM Mac - build ARM native first (guaranteed to work)
        print_step "Building native ARM64 macOS binary..."
        build_for_target "$MACOS_ARM_TARGET" "terminal-jarvis-arm"
        
        # Try Intel cross-compilation with enhanced setup
        print_step "Attempting Intel cross-compilation..."
        if command -v rustup >/dev/null 2>&1; then
            # Ensure Intel target is properly installed
            if ! check_target "$MACOS_TARGET"; then
                install_target "$MACOS_TARGET"
            fi
            # Try with explicit target installation
            if rustup target list --installed | grep -q "$MACOS_TARGET" && build_for_target "$MACOS_TARGET" "terminal-jarvis-intel" 2>/dev/null; then
                print_info "Intel cross-compilation successful, creating universal binary"
                create_universal_macos_binary
            else
                print_warning "Intel cross-compilation failed (toolchain may be missing), using ARM binary only"
                mv "$RELEASE_DIR/terminal-jarvis-arm" "$RELEASE_DIR/terminal-jarvis-macos"
            fi
        else
            print_warning "rustup not available, using ARM binary only"
            mv "$RELEASE_DIR/terminal-jarvis-arm" "$RELEASE_DIR/terminal-jarvis-macos"
        fi
    else
        # Intel Mac - build Intel native first (guaranteed to work)
        print_step "Building native Intel macOS binary..."
        build_for_target "$MACOS_TARGET" "terminal-jarvis-intel"
        
        # Try ARM cross-compilation with enhanced setup
        print_step "Attempting ARM64 cross-compilation..."
        if command -v rustup >/dev/null 2>&1; then
            # Ensure ARM target is properly installed
            if ! check_target "$MACOS_ARM_TARGET"; then
                install_target "$MACOS_ARM_TARGET"
            fi
            # Try with explicit target installation
            if rustup target list --installed | grep -q "$MACOS_ARM_TARGET" && build_for_target "$MACOS_ARM_TARGET" "terminal-jarvis-arm" 2>/dev/null; then
                print_info "ARM64 cross-compilation successful, creating universal binary"
                create_universal_macos_binary
            else
                print_warning "ARM64 cross-compilation failed (toolchain may be missing), using Intel binary only"
                mv "$RELEASE_DIR/terminal-jarvis-intel" "$RELEASE_DIR/terminal-jarvis-macos"
            fi
        else
            print_warning "rustup not available, using Intel binary only"
            mv "$RELEASE_DIR/terminal-jarvis-intel" "$RELEASE_DIR/terminal-jarvis-macos"
        fi
    fi
    
    # Cross-compile for Linux
    print_step "Cross-compiling for Linux..."
    if ! build_for_target "$LINUX_TARGET" "terminal-jarvis-linux" 2>/dev/null; then
        print_warning "Linux cross-compilation failed (OpenSSL dependency issue), using host binary as fallback"
        print_info "Note: Linux users should install from crates.io for optimal compatibility: cargo install terminal-jarvis"
        cargo build --release
        cp "target/release/terminal-jarvis" "$RELEASE_DIR/terminal-jarvis-linux"
    fi
    
else
    # Running on Linux or other Unix - build for current platform and attempt cross-compilation
    print_step "Building Linux binary (native compilation)..."
    build_for_target "$LINUX_TARGET" "terminal-jarvis-linux" || {
        print_warning "Target compilation failed, using default build"
        cargo build --release
        cp "target/release/terminal-jarvis" "$RELEASE_DIR/terminal-jarvis-linux"
    }
    
    # Cross-compile for macOS
    print_step "Cross-compiling for macOS from Linux..."
    if build_for_target "$MACOS_TARGET" "terminal-jarvis-macos" 2>/dev/null; then
        print_info "macOS cross-compilation successful"
    else
        print_warning "macOS cross-compilation failed (requires macOS SDK), using Linux binary as fallback"
        print_info "Note: For optimal macOS experience, build on macOS or use crates.io: cargo install terminal-jarvis"
        cp "$RELEASE_DIR/terminal-jarvis-linux" "$RELEASE_DIR/terminal-jarvis-macos"
    fi
    
fi

# Verify binaries exist
if [ ! -f "$RELEASE_DIR/terminal-jarvis-macos" ] || [ ! -f "$RELEASE_DIR/terminal-jarvis-linux" ]; then
    print_error "Required binaries not found after build process"
    exit 1
fi

# Create platform-specific archives
print_step "Creating platform-specific archives..."
cd "$RELEASE_DIR"

# Create macOS archive
if [ -f "terminal-jarvis-macos" ]; then
    mv terminal-jarvis-macos terminal-jarvis
    tar -czf terminal-jarvis-mac.tar.gz terminal-jarvis
    rm terminal-jarvis
    print_info "Created macOS archive"
fi

# Create Linux archive
if [ -f "terminal-jarvis-linux" ]; then
    mv terminal-jarvis-linux terminal-jarvis
    tar -czf terminal-jarvis-linux.tar.gz terminal-jarvis
    rm terminal-jarvis
    print_info "Created Linux archive"
fi


# Calculate checksums
print_info "Calculating checksums..."
CHECKSUMS_CALCULATED=false

if [ -f "terminal-jarvis-mac.tar.gz" ]; then
    MAC_SHA256=$(shasum -a 256 terminal-jarvis-mac.tar.gz | cut -d' ' -f1)
    CHECKSUMS_CALCULATED=true
fi

if [ -f "terminal-jarvis-linux.tar.gz" ]; then
    LINUX_SHA256=$(shasum -a 256 terminal-jarvis-linux.tar.gz | cut -d' ' -f1)
    CHECKSUMS_CALCULATED=true
fi


# Display results
print_info "Archives created successfully:"
if [ -n "${MAC_SHA256:-}" ]; then
    echo "  - terminal-jarvis-mac.tar.gz (SHA256: $MAC_SHA256)"
fi
if [ -n "${LINUX_SHA256:-}" ]; then
    echo "  - terminal-jarvis-linux.tar.gz (SHA256: $LINUX_SHA256)"
fi

# Get current version
VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')

print_info "Next steps for v$VERSION release:"
echo "1. git add -f homebrew/release/terminal-jarvis-*"
echo "2. git commit -m 'feat: add multi-platform release archives for v$VERSION'"
echo "3. git push origin develop"
echo "4. Upload archives to GitHub release v$VERSION"
echo "5. Update build system with new checksums:"
if [ -n "${MAC_SHA256:-}" ]; then
    echo "   - Mac SHA256: $MAC_SHA256"
fi
if [ -n "${LINUX_SHA256:-}" ]; then
    echo "   - Linux SHA256: $LINUX_SHA256"
fi

# Optional: Auto-stage files if requested
if [[ "${1:-}" == "--stage" ]]; then
    print_info "Auto-staging multi-platform archives..."
    cd "$PROJECT_ROOT"
    git add -f homebrew/release/terminal-jarvis-*.tar.gz 2>/dev/null || true
    print_info "Archives staged for commit. Run: git commit -m 'feat: add multi-platform release archives for v$VERSION'"
fi

print_info "Homebrew release archive generation complete!"
