#!/bin/bash

# Terminal Jarvis Local CD (Continuous Deployment) Script
# Handles deployment: committing, tagging, pushing to GitHub, and publishing to NPM
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

echo -e "${CYAN}🚀 Terminal Jarvis Local CD (Deployment) Pipeline${RESET}"
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${RESET}"
echo -e "${YELLOW}This will commit, tag, push, and publish${RESET}"
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
        
        if [ "$CURRENT_VERSION" = "$NPM_VERSION" ] && [ "$CURRENT_VERSION" = "$TS_VERSION" ]; then
            echo -e "${GREEN}✅ All versions are synchronized${RESET}"
            echo -e "${BLUE}→ Will proceed with commit, tag v${CURRENT_VERSION}, and deployment${RESET}"
        else
            echo -e "${RED}❌ Version mismatch detected!${RESET}"
            echo -e "${YELLOW}Expected all files to have version: ${CURRENT_VERSION}${RESET}"
            echo -e "${YELLOW}Please update all version references manually before using this option.${RESET}"
            echo ""
            echo -e "${BLUE}Files that need updating:${RESET}"
            [ "$CURRENT_VERSION" != "$NPM_VERSION" ] && echo -e "${YELLOW}  • npm/terminal-jarvis/package.json${RESET}"
            [ "$CURRENT_VERSION" != "$TS_VERSION" ] && echo -e "${YELLOW}  • npm/terminal-jarvis/src/index.ts${RESET}"
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
    
    # Update Cargo.toml
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
    
    # Update NPM package.json
    sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" npm/terminal-jarvis/package.json
    
    # Update version display in TypeScript
    sed -i "s/console.log('Terminal Jarvis v.*/console.log('Terminal Jarvis v$NEW_VERSION');/" npm/terminal-jarvis/src/index.ts
    
    # Update version references in README (both root and NPM package)
    echo -e "${BLUE}→ Updating version references in documentation...${RESET}"
    
    # Update root README.md - find and update version references
    sed -i "s/terminal-jarvis@[0-9]\+\.[0-9]\+\.[0-9]\+/terminal-jarvis@$NEW_VERSION/g" README.md
    
    # Update NPM package README.md (will be synced later)
    sed -i "s/terminal-jarvis@[0-9]\+\.[0-9]\+\.[0-9]\+/terminal-jarvis@$NEW_VERSION/g" npm/terminal-jarvis/README.md
    
    echo -e "${GREEN}✅ Version updated to ${NEW_VERSION}${RESET}"
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

# NPM Publishing
echo -e "${CYAN}📦 Step 4: NPM Publishing${RESET}"
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
echo -e "${BLUE}NPM Publishing: Manual (see docs/MAINTAINERS.md)${RESET}"
echo ""
echo -e "${CYAN}📦 After NPM Publishing, users can install with:${RESET}"
echo -e "${YELLOW}  Latest version:  ${RESET}npm install -g terminal-jarvis@${NEW_VERSION}"
echo -e "${YELLOW}  Beta release:    ${RESET}npm install -g terminal-jarvis@beta"
echo -e "${YELLOW}  Stable release:  ${RESET}npm install -g terminal-jarvis@stable"

echo ""
echo -e "${CYAN}🏁 Local CD pipeline finished!${RESET}"