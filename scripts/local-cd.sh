#!/bin/bash

# Terminal Jarvis Local CD (Continuous Deployment) Script
# Handles deployment: committing, tagging, pushing to GitHub, publishing to crates.io, syncing homebrew tap, and preparing for NPM publishing
# Run local-ci.sh first to validate before using this script

set -e  # Exit on any error

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RED='\033[0;91m'
RESET='\033[0m'

# Get current branch
CURRENT_BRANCH=$(git branch --show-current)
DEFAULT_BRANCH="develop"

# Function to display version status
display_version_status() {
    local cargo_version=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    local npm_version=$(grep '"version":' npm/terminal-jarvis/package.json | sed 's/.*"version": "\(.*\)".*/\1/')
    local ts_version=$(grep "console.log.*Terminal Jarvis v" npm/terminal-jarvis/src/index.ts | sed 's/.*Terminal Jarvis v\([0-9.]*\).*/\1/')
    local homebrew_version=""
    
    if [ -f "homebrew-terminal-jarvis/Formula/terminal-jarvis.rb" ]; then
        homebrew_version=$(grep 'version "' homebrew-terminal-jarvis/Formula/terminal-jarvis.rb | sed 's/.*version "\(.*\)".*/\1/')
    fi
    
    echo -e "${BLUE}📍 Current Version Status:${RESET}"
    echo -e "${BLUE}  • Cargo.toml: ${cargo_version}${RESET}"
    echo -e "${BLUE}  • npm/terminal-jarvis/package.json: ${npm_version}${RESET}"
    echo -e "${BLUE}  • npm/terminal-jarvis/src/index.ts: ${ts_version}${RESET}"
    if [ -n "$homebrew_version" ]; then
        echo -e "${BLUE}  • homebrew-terminal-jarvis/Formula/terminal-jarvis.rb: ${homebrew_version}${RESET}"
    else
        echo -e "${YELLOW}  • homebrew-terminal-jarvis/Formula/terminal-jarvis.rb: NOT FOUND${RESET}"
    fi
    echo -e "${BLUE}  • src/cli_logic.rs: Auto-synced from Cargo.toml${RESET}"
    
    local readme_versions=$(grep -o 'terminal-jarvis@[0-9.]*' README.md 2>/dev/null || echo "none")
    echo -e "${BLUE}  • README.md version refs: ${readme_versions}${RESET}"
    
    # Check if all versions match (including Homebrew)
    local all_match=true
    if [ "$cargo_version" != "$npm_version" ] || [ "$cargo_version" != "$ts_version" ]; then
        all_match=false
    fi
    
    if [ -n "$homebrew_version" ] && [ "$cargo_version" != "$homebrew_version" ]; then
        all_match=false
        echo -e "${RED}🚨 CRITICAL: Homebrew Formula version mismatch!${RESET}"
    fi
    
    if [ "$all_match" = true ]; then
        echo -e "${GREEN}✅ All versions are synchronized${RESET}"
    else
        echo -e "${YELLOW}⚠️  Version mismatch detected${RESET}"
    fi
}

# Function to get canonical version (Cargo.toml)
get_canonical_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Function to update all version references programmatically
update_all_versions() {
    local new_version="$1"
    local current_version="$2"
    
    if [ -z "$new_version" ] || [ -z "$current_version" ]; then
        echo -e "${RED}❌ Error: update_all_versions requires new_version and current_version parameters${RESET}"
        return 1
    fi
    
    echo -e "${BLUE}🔄 Updating all version references from ${current_version} to ${new_version}...${RESET}"
    
    # Update Cargo.toml
    echo -e "${BLUE}  • Updating Cargo.toml${RESET}"
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    
    # Update NPM package.json
    echo -e "${BLUE}  • Updating npm/terminal-jarvis/package.json${RESET}"
    sed -i "s/\"version\": \".*\"/\"version\": \"$new_version\"/" npm/terminal-jarvis/package.json
    
    # Update version display in TypeScript
    echo -e "${BLUE}  • Updating npm/terminal-jarvis/src/index.ts${RESET}"
    sed -i "s/console.log(\"🤖 Terminal Jarvis v[0-9.]*\")/console.log(\"🤖 Terminal Jarvis v$new_version\")/g" npm/terminal-jarvis/src/index.ts
    
    # Update version references in README files (if any exist)
    echo -e "${BLUE}  • Updating version references in documentation${RESET}"
    sed -i "s/terminal-jarvis@[0-9]\+\.[0-9]\+\.[0-9]\+/terminal-jarvis@$new_version/g" README.md 2>/dev/null || echo -e "${BLUE}    (No version references found in README.md)${RESET}"
    sed -i "s/terminal-jarvis@[0-9]\+\.[0-9]\+\.[0-9]\+/terminal-jarvis@$new_version/g" npm/terminal-jarvis/README.md 2>/dev/null || echo -e "${BLUE}    (No version references found in npm README.md)${RESET}"
    
    # Note: src/cli_logic.rs uses env!("CARGO_PKG_VERSION") so it auto-updates from Cargo.toml
    echo -e "${BLUE}  • src/cli_logic.rs: Auto-syncs from Cargo.toml via env!(\"CARGO_PKG_VERSION\")${RESET}"
    
    # Sync homebrew-terminal-jarvis if it exists (but don't fail if it doesn't)
    echo -e "${BLUE}  • Checking homebrew-terminal-jarvis sync...${RESET}"
    if [ -d "homebrew-terminal-jarvis" ]; then
        if sync_homebrew_tap "$new_version"; then
            echo -e "${GREEN}    ✅ Homebrew tap synchronized${RESET}"
        else
            echo -e "${YELLOW}    ⚠️  Homebrew tap sync failed (continuing anyway)${RESET}"
        fi
    else
        echo -e "${BLUE}    (homebrew-terminal-jarvis directory not found - skipping)${RESET}"
    fi
    
    echo -e "${GREEN}✅ All version references updated to ${new_version}${RESET}"
}

# Function to suggest next version based on current version
suggest_next_version() {
    local current_version="$1"
    IFS='.' read -ra VERSION_PARTS <<< "$current_version"
    
    local patch_version="${VERSION_PARTS[0]}.${VERSION_PARTS[1]}.$((VERSION_PARTS[2] + 1))"
    local minor_version="${VERSION_PARTS[0]}.$((VERSION_PARTS[1] + 1)).0"
    local major_version="$((VERSION_PARTS[0] + 1)).0.0"
    
    echo -e "${BLUE}💡 Suggested next versions based on ${current_version}:${RESET}"
    echo -e "${BLUE}  • Patch (${patch_version}): Bug fixes, docs, small features${RESET}"
    echo -e "${BLUE}  • Minor (${minor_version}): New features, no breaking changes${RESET}"
    echo -e "${BLUE}  • Major (${major_version}): Breaking changes${RESET}"
}

# Function to check if CHANGELOG.md needs updating for next version
check_changelog_readiness() {
    local current_version="$1"
    
    echo -e "${BLUE}📋 CHANGELOG.md Status Check:${RESET}"
    if grep -q "\[${current_version}\]" CHANGELOG.md; then
        echo -e "${GREEN}✅ CHANGELOG.md has entry for version ${current_version}${RESET}"
        return 0
    else
        echo -e "${YELLOW}⚠️  CHANGELOG.md missing entry for version ${current_version}${RESET}"
        echo -e "${BLUE}   Add this entry to CHANGELOG.md:${RESET}"
        echo ""
        echo -e "${YELLOW}## [${current_version}] - $(date +%Y-%m-%d)${RESET}"
        echo ""
        echo -e "${YELLOW}### Added${RESET}"
        echo -e "${YELLOW}- New feature descriptions${RESET}"
        echo ""
        echo -e "${YELLOW}### Enhanced${RESET}"
        echo -e "${YELLOW}- Improvements and optimizations${RESET}"
        echo ""
        return 1
    fi
}

# Function to sync homebrew-terminal-jarvis repository
sync_homebrew_tap() {
    local new_version="$1"
    
    echo -e "${BLUE}🍺 Syncing Homebrew Tap Repository...${RESET}"
    
    # Check if homebrew-terminal-jarvis directory exists
    if [ ! -d "homebrew-terminal-jarvis" ]; then
        echo -e "${YELLOW}⚠️  homebrew-terminal-jarvis directory not found${RESET}"
        echo -e "${BLUE}💡 Run: git clone https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis.git${RESET}"
        return 1
    fi
    
    # Ensure Formula directory exists
    if [ ! -d "homebrew-terminal-jarvis/Formula" ]; then
        echo -e "${BLUE}  • Creating Formula directory${RESET}"
        mkdir -p homebrew-terminal-jarvis/Formula
    fi
    
    # Generate updated Formula directly in the tap repository
    echo -e "${BLUE}  • Generating updated Formula for version ${new_version}${RESET}"
    cat > homebrew-terminal-jarvis/Formula/terminal-jarvis.rb << EOL
# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# Based on Federico Terzi's approach: https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/

class TerminalJarvis < Formula
  desc "A unified command center for AI coding tools"
  homepage "https://github.com/BA-CalderonMorales/terminal-jarvis"
  
  if OS.mac?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v${new_version}/terminal-jarvis-mac.tar.gz"
    sha256 "2357ffa2bf837eb97b8183daeabc3ac2d0420f8f5eaaa32fa200511b6fc8f7c7"
  elsif OS.linux?
    url "https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/v${new_version}/terminal-jarvis-linux.tar.gz" 
    sha256 "2357ffa2bf837eb97b8183daeabc3ac2d0420f8f5eaaa32fa200511b6fc8f7c7"
  end
  
  version "${new_version}"

  def install
    bin.install "terminal-jarvis"
  end

  test do
    system "#{bin}/terminal-jarvis", "--version"
  end
end
EOL
    
    # Create/update README.md for the tap
    echo -e "${BLUE}  • Updating Homebrew tap README.md${RESET}"
    cat > homebrew-terminal-jarvis/README.md << 'EOL'
# Homebrew Tap for Terminal Jarvis

🍺 Official Homebrew tap for [Terminal Jarvis](https://github.com/BA-CalderonMorales/terminal-jarvis) - A unified command center for AI coding tools.

## Installation

```bash
# Add the tap
brew tap ba-calderonmorales/terminal-jarvis

# Install Terminal Jarvis
brew install terminal-jarvis

# Verify installation
terminal-jarvis --version
```

## What is Terminal Jarvis?

Terminal Jarvis is a unified command center for AI coding tools. It provides seamless management and execution of:

- **claude** - Anthropic's Claude for code assistance
- **gemini** - Google's Gemini CLI tool  
- **qwen** - Qwen coding assistant
- **opencode** - Terminal-based AI coding agent
- **llxprt** - Multi-provider AI coding assistant
- **codex** - OpenAI Codex CLI for local AI coding
- **crush** - Charm's multi-model AI assistant with LSP support

## Features

- 🤖 Interactive T.JARVIS Interface with ASCII art
- ⚡ One-click tool installation and updates
- 📊 Real-time tool status dashboard
- 🔧 Built-in management options
- 💡 Smart guidance and workflows

## Alternative Installation Methods

- **NPM**: `npm install -g terminal-jarvis`
- **Cargo**: `cargo install terminal-jarvis`
- **NPX**: `npx terminal-jarvis` (try instantly)

## Support

- **GitHub**: [terminal-jarvis](https://github.com/BA-CalderonMorales/terminal-jarvis)
- **Issues**: [Report bugs](https://github.com/BA-CalderonMorales/terminal-jarvis/issues)
- **Discord**: [Join community](https://discord.gg/zNuyC5uG)

## License

MIT License - see the [LICENSE](LICENSE) file for details.
EOL
    
    # Navigate to homebrew-terminal-jarvis and commit changes
    cd homebrew-terminal-jarvis
    
    # Check if there are changes to commit
    if git diff --quiet && git diff --cached --quiet; then
        echo -e "${BLUE}  • No changes to commit in homebrew-terminal-jarvis${RESET}"
        cd ..
        return 0
    fi
    
    echo -e "${BLUE}  • Committing changes to homebrew-terminal-jarvis${RESET}"
    git add Formula/terminal-jarvis.rb README.md
    git commit -m "feat: update Terminal Jarvis to v${new_version}

- Updated Formula to version ${new_version}
- Updated download URLs to point to v${new_version} release
- Refreshed README.md with current feature set"
    
    # Push changes
    echo -e "${BLUE}  • Pushing changes to GitHub${RESET}"
    if git push origin develop; then
        echo -e "${GREEN}✅ Successfully synced homebrew-terminal-jarvis repository${RESET}"
    else
        echo -e "${RED}❌ Failed to push homebrew-terminal-jarvis changes${RESET}"
        echo -e "${BLUE}💡 You may need to push manually or check GitHub token permissions${RESET}"
        cd ..
        return 1
    fi
    
    cd ..
    return 0
}

# Handle standalone version check command
if [ "$1" = "--check-versions" ] || [ "$1" = "-v" ]; then
    echo -e "${CYAN}🔍 Terminal Jarvis Version Status Check${RESET}"
    echo ""
    display_version_status
    echo ""
    echo -e "${BLUE}💡 To update all versions programmatically:${RESET}"
    echo -e "${BLUE}   ./scripts/local-cd.sh --update-version <new_version>${RESET}"
    echo -e "${BLUE}   Example: ./scripts/local-cd.sh --update-version 0.0.46${RESET}"
    exit 0
fi

# Handle standalone version update command
if [ "$1" = "--update-version" ] && [ -n "$2" ]; then
    echo -e "${CYAN}🔄 Terminal Jarvis Programmatic Version Update${RESET}"
    echo ""
    
    # Get current version
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
    NEW_VERSION="$2"
    
    echo -e "${BLUE}Updating from version ${CURRENT_VERSION} to ${NEW_VERSION}...${RESET}"
    echo ""
    
    # Validate version format
    if [[ ! "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo -e "${RED}❌ Invalid version format. Use semantic versioning (e.g., 0.0.46)${RESET}"
        exit 1
    fi
    
    # Update all versions
    update_all_versions "$NEW_VERSION" "$CURRENT_VERSION"
    
    echo ""
    echo -e "${GREEN}✅ Version update completed!${RESET}"
    echo -e "${BLUE}💡 Next steps:${RESET}"
    echo -e "${BLUE}   1. Update CHANGELOG.md with changes for v${NEW_VERSION}${RESET}"
    echo -e "${BLUE}   2. Run ./scripts/local-ci.sh to validate${RESET}"
    echo -e "${BLUE}   3. Run ./scripts/local-cd.sh to deploy${RESET}"
    exit 0
fi

echo -e "${CYAN}🚀 Terminal Jarvis Local CD (Deployment) Pipeline${RESET}"
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${RESET}"
echo -e "${YELLOW}This will commit, tag, push to GitHub, publish to crates.io, sync homebrew tap, and prepare for NPM publishing${RESET}"
echo ""

echo -e "${CYAN}📚 Deployment Documentation:${RESET}"
echo -e "${BLUE}• Full workflow guide: CLAUDE.md (search for 'DEPLOYMENT WORKFLOW')${RESET}"
echo -e "${BLUE}• Copilot instructions: .github/copilot-instructions.md (search for 'DEPLOYMENT COMMANDS')${RESET}"
echo -e "${BLUE}• Version caching feature: docs/VERSION_CACHING.md${RESET}"
echo ""

# Pre-flight checks
echo -e "${CYAN}🔍 Pre-flight Deployment Readiness Check${RESET}"
CANONICAL_VERSION=$(get_canonical_version)

# Check if we need to suggest a version bump
suggest_next_version "$CANONICAL_VERSION"
echo ""

# Check CHANGELOG.md readiness for next version
IFS='.' read -ra VERSION_PARTS <<< "$CANONICAL_VERSION"
SUGGESTED_PATCH="${VERSION_PARTS[0]}.${VERSION_PARTS[1]}.$((VERSION_PARTS[2] + 1))"

if ! check_changelog_readiness "$SUGGESTED_PATCH"; then
    echo -e "${YELLOW}💡 Consider updating CHANGELOG.md for version ${SUGGESTED_PATCH} before proceeding${RESET}"
fi
echo ""

# Show current version status
display_version_status
echo ""

# Prerequisite check
echo -e "${CYAN}📋 Step 0: Prerequisite Verification${RESET}"
echo -e "${YELLOW}⚠️  Have you run ./scripts/local-ci.sh successfully?${RESET}"
echo ""
read -p "Continue with deployment? (y/N): " continue_deploy

if [[ ! $continue_deploy =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}👋 Deployment cancelled. Run ./scripts/local-ci.sh first${RESET}"
    exit 0
fi

echo ""

# CHANGELOG.md Check
echo -e "${CYAN}📝 Step 1: CHANGELOG.md Verification${RESET}"

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}Current version in Cargo.toml: ${CURRENT_VERSION}${RESET}"

# Check if CHANGELOG.md has entry for current version
if ! grep -q "\[${CURRENT_VERSION}\]" CHANGELOG.md; then
    echo -e "${YELLOW}⚠️  CHANGELOG.md does not contain an entry for version ${CURRENT_VERSION}${RESET}"
    echo ""
    echo -e "${BLUE}The CHANGELOG.md should be updated BEFORE deployment.${RESET}"
    echo -e "${BLUE}This ensures proper documentation of changes for the release.${RESET}"
    echo ""
    echo "What would you like to do?"
    echo "1) Edit CHANGELOG.md now (opens in default editor)"
    echo "2) I'll update it manually and re-run this script"
    echo "3) Continue without CHANGELOG.md update (not recommended)"
    echo "4) Exit and handle this later"
    echo ""
    
    read -p "Enter your choice (1-4): " changelog_choice
    
    case $changelog_choice in
        1)
            echo -e "${BLUE}→ Opening CHANGELOG.md in editor...${RESET}"
            echo ""
            echo -e "${YELLOW}📋 Add an entry like this at the top (after the header):${RESET}"
            echo ""
            echo "## [${CURRENT_VERSION}] - $(date +%Y-%m-%d)"
            echo ""
            echo "### Added"
            echo "- New feature descriptions"
            echo ""
            echo "### Fixed"
            echo "- Bug fixes and improvements"
            echo ""
            echo "### Enhanced"
            echo "- Improvements to existing features"
            echo ""
            echo -e "${BLUE}Press Enter to open the editor...${RESET}"
            read -p ""
            
            # Open CHANGELOG.md in default editor
            ${EDITOR:-nano} CHANGELOG.md
            
            # Check again if the entry was added
            if grep -q "\[${CURRENT_VERSION}\]" CHANGELOG.md; then
                echo -e "${GREEN}✅ CHANGELOG.md updated successfully!${RESET}"
            else
                echo -e "${RED}❌ No entry for version ${CURRENT_VERSION} found in CHANGELOG.md${RESET}"
                echo -e "${YELLOW}Please add the entry and re-run this script.${RESET}"
                exit 1
            fi
            ;;
        2)
            echo -e "${BLUE}📝 Please update CHANGELOG.md with changes for version ${CURRENT_VERSION}${RESET}"
            echo -e "${YELLOW}Add an entry at the top following the existing format.${RESET}"
            echo -e "${YELLOW}Then re-run this script: ./scripts/local-cd.sh${RESET}"
            exit 0
            ;;
        3)
            echo -e "${YELLOW}⚠️  Continuing without CHANGELOG.md update${RESET}"
            echo -e "${RED}This is not recommended for proper release documentation.${RESET}"
            ;;
        4)
            echo -e "${BLUE}👋 Exiting. Update CHANGELOG.md and re-run when ready.${RESET}"
            exit 0
            ;;
        *)
            echo -e "${RED}❌ Invalid choice. Exiting.${RESET}"
            exit 1
            ;;
    esac
else
    echo -e "${GREEN}✅ CHANGELOG.md contains entry for version ${CURRENT_VERSION}${RESET}"
fi

echo ""

# Branch Management Decision (only if not on default branch)
if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    echo -e "${YELLOW}🔀 Branch Management Decision${RESET}"
    echo -e "${BLUE}You are currently on branch: ${CURRENT_BRANCH}${RESET}"
    echo ""
    echo "What would you like to do?"
    echo "1) Merge into ${DEFAULT_BRANCH} and deploy (full CD)"
    echo "2) Deploy from current feature branch"
    echo "3) Cancel and exit"
    echo ""
    
    read -p "Enter your choice (1-3): " choice
    
    case $choice in
        1)
            echo -e "${CYAN}🔀 Merging into ${DEFAULT_BRANCH} branch...${RESET}"
            
            # Check if develop branch exists and is up to date
            git fetch origin
            
            # Switch to develop and merge
            git checkout $DEFAULT_BRANCH
            git pull origin $DEFAULT_BRANCH
            git merge $CURRENT_BRANCH --no-ff -m "feat: merge ${CURRENT_BRANCH} - futuristic UX improvements"
            
            echo -e "${GREEN}✅ Successfully merged into ${DEFAULT_BRANCH}!${RESET}"
            ;;
        2)
            echo -e "${BLUE}📌 Deploying from feature branch: ${CURRENT_BRANCH}${RESET}"
            ;;
        3)
            echo -e "${YELLOW}❌ Cancelled by user${RESET}"
            exit 0
            ;;
        *)
            echo -e "${RED}❌ Invalid choice. Exiting.${RESET}"
            exit 1
            ;;
    esac
else
    echo -e "${GREEN}✅ Already on ${DEFAULT_BRANCH} branch${RESET}"
fi

echo ""

# Version Management
echo -e "${CYAN}🚀 Step 2: Version Management${RESET}"

# Get current version
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}Current version: ${CURRENT_VERSION}${RESET}"

# Ask for version bump type
echo ""
echo "What type of version bump?"
echo "1) Patch (0.0.X) - Bug fixes, small improvements"
echo "2) Minor (0.X.0) - New features, no breaking changes"  
echo "3) Major (X.0.0) - Breaking changes"
echo "4) Skip version bump"
echo "5) Publish current version to NPM registry only"
echo "6) Deploy current version (I've already updated all version files manually)"
echo ""

read -p "Enter your choice (1-6): " version_choice

case $version_choice in
    1)
        echo -e "${BLUE}→ Bumping patch version...${RESET}"
        # Calculate new patch version
        IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
        NEW_VERSION="${VERSION_PARTS[0]}.${VERSION_PARTS[1]}.$((VERSION_PARTS[2] + 1))"
        ;;
    2)
        echo -e "${BLUE}→ Bumping minor version...${RESET}"
        IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
        NEW_VERSION="${VERSION_PARTS[0]}.$((VERSION_PARTS[1] + 1)).0"
        ;;
    3)
        echo -e "${BLUE}→ Bumping major version...${RESET}"
        IFS='.' read -ra VERSION_PARTS <<< "$CURRENT_VERSION"
        NEW_VERSION="$((VERSION_PARTS[0] + 1)).0.0"
        ;;
    4)
        echo -e "${BLUE}→ Skipping version bump...${RESET}"
        NEW_VERSION=$CURRENT_VERSION
        ;;
    5)
        echo -e "${BLUE}→ Publishing current version to NPM registry only...${RESET}"
        NEW_VERSION=$CURRENT_VERSION
        SKIP_GIT_OPERATIONS=true
        ;;
    6)
        echo -e "${BLUE}→ Using current version (manually updated)...${RESET}"
        NEW_VERSION=$CURRENT_VERSION
        MANUAL_VERSION_UPDATE=true
        
        # Verify version consistency before proceeding
        echo -e "${YELLOW}🔍 Verifying version consistency across files...${RESET}"
        
        NPM_VERSION=$(grep '"version":' npm/terminal-jarvis/package.json | sed 's/.*"version": "\(.*\)".*/\1/')
        TS_VERSION=$(grep "console.log.*Terminal Jarvis v" npm/terminal-jarvis/src/index.ts | sed 's/.*Terminal Jarvis v\([0-9.]*\).*/\1/')
        
        echo -e "${BLUE}  Cargo.toml: ${CURRENT_VERSION}${RESET}"
        echo -e "${BLUE}  package.json: ${NPM_VERSION}${RESET}"
        echo -e "${BLUE}  index.ts: ${TS_VERSION}${RESET}"
        echo -e "${BLUE}  cli_logic.rs: Uses env!(\"CARGO_PKG_VERSION\") - auto-synced${RESET}"
        
        if [ "$CURRENT_VERSION" = "$NPM_VERSION" ] && [ "$CURRENT_VERSION" = "$TS_VERSION" ]; then
            echo -e "${GREEN}✅ All versions are synchronized${RESET}"
            echo -e "${BLUE}→ Will proceed with commit, tag v${CURRENT_VERSION}, and deployment${RESET}"
        else
            echo -e "${RED}❌ Version mismatch detected!${RESET}"
            echo -e "${YELLOW}Expected all files to have version: ${CURRENT_VERSION}${RESET}"
            echo -e "${YELLOW}Please update all version references manually before using this option.${RESET}"
            echo ""
            echo -e "${BLUE}Files that need updating:${RESET}"
            [ "$CURRENT_VERSION" != "$NPM_VERSION" ] && echo -e "${YELLOW}  • npm/terminal-jarvis/package.json (currently ${NPM_VERSION})${RESET}"
            [ "$CURRENT_VERSION" != "$TS_VERSION" ] && echo -e "${YELLOW}  • npm/terminal-jarvis/src/index.ts (currently ${TS_VERSION})${RESET}"
            echo ""
            echo -e "${BLUE}Note: src/cli_logic.rs auto-updates from Cargo.toml${RESET}"
            exit 1
        fi
        ;;
    *)
        echo -e "${RED}❌ Invalid choice. Using current version.${RESET}"
        NEW_VERSION=$CURRENT_VERSION
        ;;
esac

if [ "$NEW_VERSION" != "$CURRENT_VERSION" ] && [ "${MANUAL_VERSION_UPDATE:-false}" != "true" ]; then
    echo -e "${BLUE}→ Updating version to ${NEW_VERSION}...${RESET}"
    update_all_versions "$NEW_VERSION" "$CURRENT_VERSION"
    echo -e "${GREEN}✅ Version updated to ${NEW_VERSION} in all locations${RESET}"
fi

# Rebuild with new version
echo -e "${BLUE}→ Rebuilding with new version...${RESET}"
cargo build --release
cd npm/terminal-jarvis && npm run build && cd ../..

echo ""

# Git Operations
echo -e "${CYAN}📦 Step 3: Git Operations${RESET}"

if [ "${SKIP_GIT_OPERATIONS:-false}" != "true" ]; then
    echo -e "${BLUE}→ Committing changes...${RESET}"
    git add .
    git commit -m "version: bump to v${NEW_VERSION} with futuristic UX improvements"
    git tag "v${NEW_VERSION}"
    
    # Push to GitHub
    echo -e "${BLUE}→ Pushing to GitHub...${RESET}"
    CURRENT_BRANCH=$(git branch --show-current)  # Refresh current branch after potential merge
    git push origin $CURRENT_BRANCH
    git push origin "v${NEW_VERSION}"
    
    echo -e "${GREEN}✅ Pushed to GitHub with tag v${NEW_VERSION}${RESET}"
else
    echo -e "${YELLOW}→ Skipping git operations (NPM-only publish)...${RESET}"
fi

echo ""

# Crates.io Publishing
echo -e "${CYAN}📦 Step 4: Crates.io Publishing${RESET}"
if [ "${SKIP_GIT_OPERATIONS:-false}" != "true" ]; then
    echo -e "${BLUE}→ Publishing to crates.io...${RESET}"
    echo ""
    echo -e "${YELLOW}📋 Publishing terminal-jarvis v${NEW_VERSION} to crates.io${RESET}"
    
    # Check if logged in to crates.io
    if ! cargo login --registry crates-io --help >/dev/null 2>&1; then
        echo -e "${RED}❌ Error: cargo login not available. Please ensure Rust/Cargo is installed.${RESET}"
        exit 1
    fi
    
    # Publish to crates.io
    if cargo publish; then
        echo -e "${GREEN}✅ Successfully published to crates.io${RESET}"
        echo -e "${BLUE}📦 Crate available at: https://crates.io/crates/terminal-jarvis${RESET}"
        echo -e "${YELLOW}Users can now install with: cargo install terminal-jarvis${RESET}"
    else
        echo -e "${RED}❌ Failed to publish to crates.io${RESET}"
        echo -e "${YELLOW}⚠️  You may need to login first: cargo login${RESET}"
        echo -e "${YELLOW}⚠️  Or check for version conflicts or other publishing issues${RESET}"
        echo -e "${BLUE}💡 You can retry manually with: cargo publish${RESET}"
    fi
else
    echo -e "${YELLOW}→ Skipping crates.io publishing (NPM-only publish)...${RESET}"
fi

echo ""

# Homebrew Tap Sync
echo -e "${CYAN}🍺 Step 5: Homebrew Tap Sync${RESET}"
if [ "${SKIP_GIT_OPERATIONS:-false}" != "true" ]; then
    if sync_homebrew_tap "$NEW_VERSION"; then
        echo -e "${GREEN}✅ Successfully synced homebrew-terminal-jarvis repository${RESET}"
        echo -e "${BLUE}🍺 Users can now install with: brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis${RESET}"
    else
        echo -e "${YELLOW}⚠️  Homebrew tap sync failed or skipped${RESET}"
        echo -e "${BLUE}💡 You may need to manually update the homebrew-terminal-jarvis repository${RESET}"
    fi
else
    echo -e "${YELLOW}→ Skipping Homebrew tap sync (NPM-only publish)...${RESET}"
fi

echo ""

# NPM Publishing
echo -e "${CYAN}📦 Step 6: NPM Publishing${RESET}"
echo -e "${BLUE}Git operations completed successfully!${RESET}"
echo -e "${YELLOW}📋 Manual NPM Publishing Required${RESET}"
echo ""
echo -e "${BLUE}To avoid authentication issues with 2FA, NPM publishing must be done manually.${RESET}"
echo -e "${BLUE}See docs/MAINTAINERS.md for detailed NPM publishing instructions.${RESET}"
echo ""
echo -e "${CYAN}Quick NPM Publishing Commands:${RESET}"
echo -e "${YELLOW}  cd npm/terminal-jarvis${RESET}"
echo -e "${YELLOW}  npm publish --otp=<your-2fa-code>${RESET}"
echo -e "${YELLOW}  npm dist-tag add terminal-jarvis@${NEW_VERSION} beta${RESET}"
echo -e "${YELLOW}  npm dist-tag add terminal-jarvis@${NEW_VERSION} stable${RESET}"
echo ""

echo ""

# Deployment Summary
echo -e "${GREEN}🎉 Git deployment completed successfully!${RESET}"
CURRENT_BRANCH=$(git branch --show-current)  # Refresh current branch
echo -e "${BLUE}Version: ${NEW_VERSION}${RESET}"
echo -e "${BLUE}Branch: ${CURRENT_BRANCH}${RESET}"
echo -e "${BLUE}Git Operations: $([ "${SKIP_GIT_OPERATIONS:-false}" = "true" ] && echo "Skipped" || echo "Completed")${RESET}"
echo -e "${BLUE}Crates.io Publishing: $([ "${SKIP_GIT_OPERATIONS:-false}" = "true" ] && echo "Skipped" || echo "Completed (check output above)")${RESET}"
echo -e "${BLUE}Homebrew Tap Sync: $([ "${SKIP_GIT_OPERATIONS:-false}" = "true" ] && echo "Skipped" || echo "Attempted (check output above)")${RESET}"
echo -e "${BLUE}NPM Publishing: Manual (see below)${RESET}"
echo ""

# Post-Deployment Action Items
echo -e "${CYAN}📋 REQUIRED POST-DEPLOYMENT ACTIONS:${RESET}"
echo ""
echo -e "${YELLOW}1. 📦 Manual NPM Publishing (due to 2FA):${RESET}"
echo -e "${BLUE}   cd npm/terminal-jarvis${RESET}"
echo -e "${BLUE}   npm publish --otp=<your-2fa-code>${RESET}"
echo -e "${BLUE}   npm dist-tag add terminal-jarvis@${NEW_VERSION} stable${RESET}"
echo ""
echo -e "${YELLOW}2. 🔍 Verification Commands:${RESET}"
echo -e "${BLUE}   npm view terminal-jarvis versions --json | tail -10${RESET}"
echo -e "${BLUE}   npm dist-tag ls terminal-jarvis${RESET}"
echo -e "${BLUE}   brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis${RESET}"
echo ""

echo -e "${CYAN}📦 After completing all publishing steps, users can install with:${RESET}"
echo -e "${YELLOW}  Cargo (Rust):    ${RESET}cargo install terminal-jarvis"
echo -e "${YELLOW}  NPM Latest:      ${RESET}npm install -g terminal-jarvis@${NEW_VERSION}"
echo -e "${YELLOW}  NPM Beta:        ${RESET}npm install -g terminal-jarvis@beta"
echo -e "${YELLOW}  Stable release:  ${RESET}npm install -g terminal-jarvis@stable"
echo -e "${YELLOW}  Homebrew:        ${RESET}brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis"

echo ""
echo -e "${CYAN}🏁 Local CD pipeline finished!${RESET}"