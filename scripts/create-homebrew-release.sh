#!/bin/bash

# Terminal Jarvis # Create macOS archive
echo -e "${YELLOW}→ Creating macOS archive...${RESET}"
cd target/release
tar -czf "../../${RELEASE_DIR}/terminal-jarvis-mac.tar.gz" terminal-jarvis
cd ../..

# Calculate SHA256 for macOS
MAC_SHA256=$(shasum -a 256 "${RELEASE_DIR}/terminal-jarvis-mac.tar.gz" | cut -d' ' -f1)
echo -e "${GREEN}✅ macOS archive created: ${MAC_SHA256}${RESET}"elease Creation Script
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
# 
# This script creates release archives for Homebrew installation

set -e

# Colors for output
CYAN='\033[0;96m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

# Get version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

echo -e "${CYAN}🍺 Creating Homebrew Release Archives for Terminal Jarvis v${VERSION}${RESET}"

# Ensure we have a release build
echo -e "${YELLOW}→ Building release binary...${RESET}"
cargo build --release

# Create homebrew directory structure
HOMEBREW_DIR="homebrew"
RELEASE_DIR="${HOMEBREW_DIR}/release"
mkdir -p "${RELEASE_DIR}"
mkdir -p "${HOMEBREW_DIR}/Formula"

# Create macOS archive
echo -e "${YELLOW}→ Creating macOS archive...${RESET}"
cd target/release
tar -czf "../../${RELEASE_DIR}/terminal-jarvis-mac.tar.gz" terminal-jarvis
cd ../..

# Calculate SHA256 hash
MAC_SHA256=$(shasum -a 256 "${RELEASE_DIR}/terminal-jarvis-mac.tar.gz" | cut -d' ' -f1)
echo -e "${GREEN}✅ macOS archive created${RESET}"
echo -e "${YELLOW}   SHA256: ${MAC_SHA256}${RESET}"

# For Linux (if building on compatible system)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "${YELLOW}→ Creating Linux archive...${RESET}"
    cd target/release
    tar -czf "../../${RELEASE_DIR}/terminal-jarvis-linux.tar.gz" terminal-jarvis
    cd ../..
    
    LINUX_SHA256=$(shasum -a 256 "${RELEASE_DIR}/terminal-jarvis-linux.tar.gz" | cut -d' ' -f1)
    echo -e "${GREEN}✅ Linux archive created${RESET}"
    echo -e "${YELLOW}   SHA256: ${LINUX_SHA256}${RESET}"
fi

# Generate Formula template
echo -e "${YELLOW}→ Generating Formula template...${RESET}"

cat > "${HOMEBREW_DIR}/Formula/terminal-jarvis.rb" << EOF
# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v${VERSION}/terminal-jarvis-mac.tar.gz"
    sha256 "${MAC_SHA256}"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v${VERSION}/terminal-jarvis-linux.tar.gz" 
    sha256 "${LINUX_SHA256:-LINUX_SHA256_PLACEHOLDER}"
  end
  
  version "${VERSION}"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
EOF

echo -e "${GREEN}✅ Formula template created at ${HOMEBREW_DIR}/Formula/terminal-jarvis.rb${RESET}"

# Display summary
echo ""
echo -e "${CYAN}📋 Release Summary:${RESET}"
echo -e "${YELLOW}   Version: ${VERSION}${RESET}"
echo -e "${YELLOW}   macOS SHA256: ${MAC_SHA256}${RESET}"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "${YELLOW}   Linux SHA256: ${LINUX_SHA256}${RESET}"
fi
echo ""
echo -e "${CYAN}📦 Next Steps:${RESET}"
echo -e "${YELLOW}1. Upload archives to GitHub Releases:${RESET}"
echo -e "   - ${RELEASE_DIR}/terminal-jarvis-mac.tar.gz"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "   - ${RELEASE_DIR}/terminal-jarvis-linux.tar.gz"
fi
echo -e "${YELLOW}2. Create/update homebrew-terminal-jarvis repository${RESET}"
echo -e "${YELLOW}3. Update Formula/terminal-jarvis.rb with the generated template${RESET}"
echo ""
echo -e "${GREEN}🍺 Homebrew release preparation complete!${RESET}"
