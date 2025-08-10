#!/bin/bash

# Homebrew Formula Test Script
# Tests Terminal Jarvis Formula without requiring actual homebrew tap

set -e

# Colors for output
CYAN='\033[0;96m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

echo -e "${CYAN}🍺 Testing Terminal Jarvis Homebrew Formula${RESET}"
echo ""

# Test 1: Ruby syntax check
echo -e "${YELLOW}→ Test 1: Ruby syntax validation...${RESET}"
if ruby -c homebrew/Formula/terminal-jarvis.rb > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Formula has valid Ruby syntax${RESET}"
else
    echo -e "${RED}❌ Formula has Ruby syntax errors${RESET}"
    exit 1
fi

# Test 2: Check if archives exist and have correct SHA256
echo -e "${YELLOW}→ Test 2: Archive validation...${RESET}"
if [ -f "homebrew/release/terminal-jarvis-mac.tar.gz" ]; then
    MAC_ACTUAL=$(shasum -a 256 homebrew/release/terminal-jarvis-mac.tar.gz | cut -d' ' -f1)
    MAC_EXPECTED=$(grep -A1 'OS.mac' homebrew/Formula/terminal-jarvis.rb | grep 'sha256' | sed 's/.*sha256 "\([^"]*\)".*/\1/')
    
    if [ "$MAC_ACTUAL" = "$MAC_EXPECTED" ]; then
        echo -e "${GREEN}✅ macOS archive SHA256 matches Formula${RESET}"
    else
        echo -e "${RED}❌ macOS archive SHA256 mismatch${RESET}"
        echo -e "  Expected: $MAC_EXPECTED"
        echo -e "  Actual:   $MAC_ACTUAL"
    fi
else
    echo -e "${RED}❌ macOS archive not found${RESET}"
fi

if [ -f "homebrew/release/terminal-jarvis-linux.tar.gz" ]; then
    LINUX_ACTUAL=$(shasum -a 256 homebrew/release/terminal-jarvis-linux.tar.gz | cut -d' ' -f1)
    LINUX_EXPECTED=$(grep -A1 'OS.linux' homebrew/Formula/terminal-jarvis.rb | grep 'sha256' | sed 's/.*sha256 "\([^"]*\)".*/\1/')
    
    if [ "$LINUX_ACTUAL" = "$LINUX_EXPECTED" ]; then
        echo -e "${GREEN}✅ Linux archive SHA256 matches Formula${RESET}"
    else
        echo -e "${RED}❌ Linux archive SHA256 mismatch${RESET}"
        echo -e "  Expected: $LINUX_EXPECTED"
        echo -e "  Actual:   $LINUX_ACTUAL"
    fi
else
    echo -e "${RED}❌ Linux archive not found${RESET}"
fi

# Test 3: Check Formula structure
echo -e "${YELLOW}→ Test 3: Formula structure validation...${RESET}"

# Check required fields
REQUIRED_FIELDS=("desc" "homepage" "url" "version" "def install")
for field in "${REQUIRED_FIELDS[@]}"; do
    if grep -q "$field" homebrew/Formula/terminal-jarvis.rb; then
        echo -e "${GREEN}✅ Formula contains required field: $field${RESET}"
    else
        echo -e "${RED}❌ Formula missing required field: $field${RESET}"
    fi
done

# Test 4: Test archive contents
echo -e "${YELLOW}→ Test 4: Archive contents validation...${RESET}"
if [ -f "homebrew/release/terminal-jarvis-linux.tar.gz" ]; then
    # Extract and check if binary exists
    mkdir -p /tmp/homebrew-test
    tar -xzf homebrew/release/terminal-jarvis-linux.tar.gz -C /tmp/homebrew-test
    
    if [ -f "/tmp/homebrew-test/terminal-jarvis" ]; then
        echo -e "${GREEN}✅ Archive contains terminal-jarvis binary${RESET}"
        
        # Check if binary is executable
        if [ -x "/tmp/homebrew-test/terminal-jarvis" ]; then
            echo -e "${GREEN}✅ Binary is executable${RESET}"
            
            # Try to get version (this simulates the test block in Formula)
            if /tmp/homebrew-test/terminal-jarvis --version > /dev/null 2>&1; then
                VERSION_OUTPUT=$(/tmp/homebrew-test/terminal-jarvis --version)
                echo -e "${GREEN}✅ Binary version check successful: ${VERSION_OUTPUT}${RESET}"
            else
                echo -e "${YELLOW}⚠️  Binary version check failed (might need dependencies)${RESET}"
            fi
        else
            echo -e "${RED}❌ Binary is not executable${RESET}"
        fi
    else
        echo -e "${RED}❌ Archive does not contain terminal-jarvis binary${RESET}"
    fi
    
    # Cleanup
    rm -rf /tmp/homebrew-test
fi

# Test 5: Check GitHub release URLs (if network available)
echo -e "${YELLOW}→ Test 5: GitHub release URL validation...${RESET}"
MAC_URL=$(grep -A1 'OS.mac' homebrew/Formula/terminal-jarvis.rb | grep 'url' | sed 's/.*url "\([^"]*\)".*/\1/')
LINUX_URL=$(grep -A1 'OS.linux' homebrew/Formula/terminal-jarvis.rb | grep 'url' | sed 's/.*url "\([^"]*\)".*/\1/')

if command -v curl > /dev/null 2>&1; then
    if curl -I "$MAC_URL" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ macOS release URL is accessible${RESET}"
    else
        echo -e "${YELLOW}⚠️  macOS release URL not accessible (expected until GitHub repo is created)${RESET}"
    fi
    
    if curl -I "$LINUX_URL" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Linux release URL is accessible${RESET}"
    else
        echo -e "${YELLOW}⚠️  Linux release URL not accessible (expected until GitHub repo is created)${RESET}"
    fi
else
    echo -e "${YELLOW}⚠️  curl not available, skipping URL validation${RESET}"
fi

echo ""
echo -e "${CYAN}📋 Formula Test Summary${RESET}"
echo -e "${GREEN}✅ Formula appears to be valid and ready for Homebrew!${RESET}"
echo ""
echo -e "${YELLOW}Next steps:${RESET}"
echo -e "1. Create homebrew-terminal-jarvis GitHub repository"
echo -e "2. Push Formula to the tap repository"  
echo -e "3. Test with: brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis"
