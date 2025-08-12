#!/bin/bash

# Generate Homebrew Release Archives
# This script creates platform-specific archives for Homebrew Formula consumption

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RELEASE_DIR="$PROJECT_ROOT/homebrew/release"

print_info "Terminal Jarvis Homebrew Release Archive Generator"
print_info "Project root: $PROJECT_ROOT"

# Check if we're in the right directory
if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    print_error "Not in Terminal Jarvis project root directory"
    exit 1
fi

# Create release directory
mkdir -p "$RELEASE_DIR"

# Build release binary
print_info "Building release binary..."
cd "$PROJECT_ROOT"
cargo build --release

# Check if binary exists
if [ ! -f "target/release/terminal-jarvis" ]; then
    print_error "Release binary not found at target/release/terminal-jarvis"
    exit 1
fi

# Clean previous archives
print_info "Cleaning previous archives..."
rm -f "$RELEASE_DIR"/*.tar.gz

# Create platform-specific archives
print_info "Creating platform-specific archives..."

# Copy binary to release directory temporarily
cp target/release/terminal-jarvis "$RELEASE_DIR/"

# Create archives
cd "$RELEASE_DIR"
tar -czf terminal-jarvis-mac.tar.gz terminal-jarvis
tar -czf terminal-jarvis-linux.tar.gz terminal-jarvis

# Remove temporary binary (avoid repository bloat)
rm terminal-jarvis

# Calculate checksums
print_info "Calculating checksums..."
MAC_SHA256=$(shasum -a 256 terminal-jarvis-mac.tar.gz | cut -d' ' -f1)
LINUX_SHA256=$(shasum -a 256 terminal-jarvis-linux.tar.gz | cut -d' ' -f1)

# Display results
print_info "Archives created successfully:"
echo "  - terminal-jarvis-mac.tar.gz (SHA256: $MAC_SHA256)"
echo "  - terminal-jarvis-linux.tar.gz (SHA256: $LINUX_SHA256)"

# Get current version
VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')

print_info "Next steps for v$VERSION release:"
echo "1. git add -f homebrew/release/terminal-jarvis-*.tar.gz"
echo "2. git commit -m 'feat: add Homebrew release archives for v$VERSION'"
echo "3. git push origin develop"
echo "4. Upload archives to GitHub release v$VERSION"
echo "5. Update Homebrew Formula with new checksums:"
echo "   - Mac SHA256: $MAC_SHA256"
echo "   - Linux SHA256: $LINUX_SHA256"

# Optional: Auto-stage files if requested
if [[ "${1:-}" == "--stage" ]]; then
    print_info "Auto-staging Homebrew archives..."
    cd "$PROJECT_ROOT"
    git add -f homebrew/release/terminal-jarvis-mac.tar.gz homebrew/release/terminal-jarvis-linux.tar.gz
    print_info "Archives staged for commit. Run: git commit -m 'feat: add Homebrew release archives for v$VERSION'"
fi

print_info "Homebrew release archive generation complete!"
