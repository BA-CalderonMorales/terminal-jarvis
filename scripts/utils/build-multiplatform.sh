#!/bin/bash

# Multi-Platform Build Script for Terminal Jarvis
# This script builds binaries for multiple platforms using cross-compilation

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

# Platform targets
TARGETS=(
    "x86_64-apple-darwin:macos-intel"
    "aarch64-apple-darwin:macos-arm"
    "x86_64-unknown-linux-gnu:linux-x64"
    "aarch64-unknown-linux-gnu:linux-arm64"
)

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

print_info "Terminal Jarvis Multi-Platform Build System"
print_info "Project root: $PROJECT_ROOT"

# Check if we're in the right directory
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    print_error "Not in Terminal Jarvis project root directory"
    exit 1
fi

cd "$PROJECT_ROOT"

# Function to check if target is installed
check_target() {
    local target=$1
    if command -v rustup >/dev/null 2>&1; then
        if rustup target list --installed | grep -q "$target"; then
            return 0
        else
            return 1
        fi
    else
        # If rustup is not available, assume all targets are available
        return 0
    fi
}

# Function to install target if not present
install_target() {
    local target=$1
    if command -v rustup >/dev/null 2>&1; then
        print_step "Installing Rust target: $target"
        rustup target add "$target"
    else
        print_warning "rustup not found, assuming target $target is available"
    fi
}

# Function to build for specific target
build_for_target() {
    local target=$1
    local platform_name=$2
    
    print_step "Building for $platform_name ($target)"
    
    # Check and install target if needed
    if ! check_target "$target"; then
        install_target "$target"
    fi
    
    # Build for the specific target using cross if available for better cross-compilation
    local build_command="cargo"
    if command -v cross >/dev/null 2>&1 && [[ "$target" != "$CURRENT_TARGET" ]]; then
        build_command="cross"
        print_info "Using cross for cross-compilation to $target"
    fi
    
    if $build_command build --release --target "$target"; then
        print_info "Successfully built for $platform_name"
        
        # Check if binary exists and display info
        local binary_path="target/$target/release/terminal-jarvis"
        
        if [ -f "$binary_path" ]; then
            local size=$(du -h "$binary_path" | cut -f1)
            print_info "Binary size: $size"
        fi
        return 0
    else
        print_error "Failed to build for $platform_name"
        return 1
    fi
}

# Parse command line arguments
BUILD_MODE="all"
FORCE_INSTALL=false
CURRENT_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --current-only)
            CURRENT_ONLY=true
            shift
            ;;
        --force-install)
            FORCE_INSTALL=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "OPTIONS:"
            echo "  --current-only    Build only for current platform"
            echo "  --force-install   Force install all targets"
            echo "  --help           Show this help message"
            echo ""
            echo "This script builds Terminal Jarvis for multiple platforms using cross-compilation."
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Detect current platform
CURRENT_OS=$(uname -s)
CURRENT_ARCH=$(uname -m)
print_info "Current platform: $CURRENT_OS ($CURRENT_ARCH)"

# Map current architecture to Rust target
case "$CURRENT_OS-$CURRENT_ARCH" in
    "Darwin-arm64")
        CURRENT_TARGET="aarch64-apple-darwin"
        ;;
    "Darwin-x86_64")
        CURRENT_TARGET="x86_64-apple-darwin"
        ;;
    "Linux-x86_64")
        CURRENT_TARGET="x86_64-unknown-linux-gnu"
        ;;
    "Linux-aarch64")
        CURRENT_TARGET="aarch64-unknown-linux-gnu"
        ;;
    *)
        print_warning "Unknown platform combination: $CURRENT_OS-$CURRENT_ARCH"
        CURRENT_TARGET="unknown"
        ;;
esac

if [ "$CURRENT_ONLY" = true ]; then
    print_info "Building only for current platform"
    if [ "$CURRENT_TARGET" != "unknown" ]; then
        # Find the platform name for current target
        for target_info in "${TARGETS[@]}"; do
            IFS=':' read -ra parts <<< "$target_info"
            if [ "${parts[0]}" = "$CURRENT_TARGET" ]; then
                build_for_target "$CURRENT_TARGET" "${parts[1]}"
                exit $?
            fi
        done
    fi
    
    # Fallback to default build
    print_step "Building with default target"
    cargo build --release
    exit $?
fi

# Build for all targets
print_info "Starting multi-platform build process..."
SUCCESSFUL_BUILDS=()
FAILED_BUILDS=()

for target_info in "${TARGETS[@]}"; do
    IFS=':' read -ra parts <<< "$target_info"
    target="${parts[0]}"
    platform_name="${parts[1]}"
    
    if build_for_target "$target" "$platform_name"; then
        SUCCESSFUL_BUILDS+=("$platform_name")
    else
        FAILED_BUILDS+=("$platform_name")
        print_warning "Continuing with remaining targets..."
    fi
    echo ""
done

# Summary
print_info "Build Summary:"
echo "Successfully built: ${#SUCCESSFUL_BUILDS[@]} platforms"
if [ ${#SUCCESSFUL_BUILDS[@]} -gt 0 ]; then
    for platform in "${SUCCESSFUL_BUILDS[@]}"; do
        echo -e "${GREEN}  ✓ $platform${NC}"
    done
fi

if [ ${#FAILED_BUILDS[@]} -gt 0 ]; then
    echo ""
    echo "Failed builds: ${#FAILED_BUILDS[@]} platforms"
    for platform in "${FAILED_BUILDS[@]}"; do
        echo -e "${RED}  ✗ $platform${NC}"
    done
    echo ""
    print_warning "Some cross-compilation targets failed. This is normal if:"
    print_warning "  • You don't have cross-compilation toolchains installed"
    print_warning "  • You're missing system dependencies for target platforms"
    print_warning "  • The target requires additional setup (e.g., macOS SDK on Linux)"
    echo ""
    print_info "For production releases, consider using:"
    print_info "  • GitHub Actions with multiple runners"
    print_info "  • Docker containers with cross-compilation tools"
    print_info "  • Platform-specific build machines"
fi

# Exit with error if no builds succeeded
if [ ${#SUCCESSFUL_BUILDS[@]} -eq 0 ]; then
    print_error "All builds failed!"
    exit 1
fi

print_info "Multi-platform build process completed!"