#!/bin/bash

# Terminal Jarvis Local CI/CD Script
# Handles feature branch workflow with merge decisions
#
# Release Tagging Strategy:
# - All releases automatically get 'beta' tag (for testing/preview)
# - Production-ready releases also get 'stable' tag (optional)
# - Users can install: @beta (latest features), @stable (production), or @version (specific)

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

echo -e "${CYAN}🚀 Terminal Jarvis Local CI/CD Pipeline${RESET}"
echo -e "${BLUE}Current branch: ${CURRENT_BRANCH}${RESET}"
echo ""

# Step 1: Run Quality Checks
echo -e "${CYAN}📋 Step 1: Running Quality Checks${RESET}"
echo -e "${BLUE}→ Running cargo fmt...${RESET}"
cargo fmt --all

echo -e "${BLUE}→ Running cargo clippy...${RESET}"
cargo clippy --all-targets --all-features -- -D warnings

echo -e "${BLUE}→ Running tests...${RESET}"
cargo test

echo -e "${GREEN}✅ All quality checks passed!${RESET}"
echo ""

# Step 2: Build Release Binary
echo -e "${CYAN}📦 Step 2: Building Release Binary${RESET}"
cargo build --release
echo -e "${GREEN}✅ Release binary built successfully!${RESET}"
echo ""

# Step 3: Update NPM Package
echo -e "${CYAN}📦 Step 3: Building NPM Package${RESET}"
cd npm/terminal-jarvis
npm run build
cd ../..
echo -e "${GREEN}✅ NPM package built successfully!${RESET}"
echo ""

# Step 4: Branch Decision (only if not on default branch)
if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    echo -e "${YELLOW}🔀 Branch Management Decision${RESET}"
    echo -e "${BLUE}You are currently on branch: ${CURRENT_BRANCH}${RESET}"
    echo ""
    echo "What would you like to do?"
    echo "1) Merge into ${DEFAULT_BRANCH} and publish (full CI/CD)"
    echo "2) Keep as feature branch (skip publishing)"
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
            
            # Continue with publishing workflow
            SHOULD_PUBLISH=true
            ;;
        2)
            echo -e "${BLUE}📌 Keeping as feature branch - skipping publish steps${RESET}"
            SHOULD_PUBLISH=false
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
    SHOULD_PUBLISH=true
fi

echo ""

# Step 5: Publishing Workflow (only if merging or on default branch)
if [ "$SHOULD_PUBLISH" = true ]; then
    echo -e "${CYAN}🚀 Step 5: Publishing Workflow${RESET}"
    
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
    echo ""
    
    read -p "Enter your choice (1-4): " version_choice
    
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
        *)
            echo -e "${RED}❌ Invalid choice. Using current version.${RESET}"
            NEW_VERSION=$CURRENT_VERSION
            ;;
    esac
    
    if [ "$NEW_VERSION" != "$CURRENT_VERSION" ]; then
        echo -e "${BLUE}→ Updating version to ${NEW_VERSION}...${RESET}"
        
        # Update Cargo.toml
        sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
        
        # Update NPM package.json
        sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" npm/terminal-jarvis/package.json
        
        # Update version display in TypeScript
        sed -i "s/console.log('Terminal Jarvis v.*/console.log('Terminal Jarvis v$NEW_VERSION');/" npm/terminal-jarvis/src/index.ts
        
        echo -e "${GREEN}✅ Version updated to ${NEW_VERSION}${RESET}"
    fi
    
    # Rebuild with new version
    echo -e "${BLUE}→ Rebuilding with new version...${RESET}"
    cargo build --release
    cd npm/terminal-jarvis && npm run build && cd ../..
    
    # Commit and tag
    echo -e "${BLUE}→ Committing changes...${RESET}"
    git add .
    git commit -m "version: bump to v${NEW_VERSION} with futuristic UX improvements"
    git tag "v${NEW_VERSION}"
    
    # Push to GitHub
    echo -e "${BLUE}→ Pushing to GitHub...${RESET}"
    git push origin $DEFAULT_BRANCH
    git push origin "v${NEW_VERSION}"
    
    # Ask about NPM publish
    echo ""
    read -p "Publish to NPM registry? (y/N): " publish_npm
    if [[ $publish_npm =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}→ Publishing to NPM...${RESET}"
        cd npm/terminal-jarvis
        npm publish --access public
        cd ../..
        echo -e "${GREEN}✅ Published to NPM registry!${RESET}"
        
        # Automatically add beta tag for all releases
        echo ""
        echo -e "${CYAN}📦 NPM Distribution Tags${RESET}"
        echo -e "${BLUE}→ Adding 'beta' tag automatically...${RESET}"
        npm dist-tag add terminal-jarvis@${NEW_VERSION} beta
        echo -e "${GREEN}✅ Added 'beta' tag${RESET}"
        
        # Ask about stable tag
        echo ""
        echo -e "${YELLOW}🏷️  Release Channel Decision${RESET}"
        echo -e "${BLUE}This release has been tagged as 'beta' by default.${RESET}"
        echo ""
        read -p "Is this a stable, production-ready release? Add 'stable' tag? (y/N): " add_stable
        if [[ $add_stable =~ ^[Yy]$ ]]; then
            npm dist-tag add terminal-jarvis@${NEW_VERSION} stable
            echo -e "${GREEN}✅ Added 'stable' tag - this is now a production release${RESET}"
            RELEASE_CHANNEL="beta + stable"
        else
            echo -e "${BLUE}📋 Release will remain as beta-only${RESET}"
            RELEASE_CHANNEL="beta"
        fi
        
        # Show current tags
        echo ""
        echo -e "${BLUE}→ Current distribution tags:${RESET}"
        npm dist-tag ls terminal-jarvis
        
    else
        echo -e "${YELLOW}⏭️  Skipped NPM publish${RESET}"
    fi
    
    echo ""
    echo -e "${GREEN}🎉 Full CI/CD pipeline completed successfully!${RESET}"
    echo -e "${BLUE}Version: ${NEW_VERSION}${RESET}"
    echo -e "${BLUE}Branch: ${DEFAULT_BRANCH}${RESET}"
    echo -e "${BLUE}NPM Published: $([ "$publish_npm" = "y" ] && echo "Yes" || echo "No")${RESET}"
    if [[ $publish_npm =~ ^[Yy]$ ]]; then
        echo -e "${BLUE}Release Channel: ${RELEASE_CHANNEL:-"beta"}${RESET}"
        echo ""
        echo -e "${CYAN}📦 Installation Commands:${RESET}"
        echo -e "${YELLOW}  Beta release:   ${RESET}npm install -g terminal-jarvis@beta"
        if [[ $add_stable =~ ^[Yy]$ ]]; then
            echo -e "${YELLOW}  Stable release: ${RESET}npm install -g terminal-jarvis@stable"
        fi
        echo -e "${YELLOW}  Latest version: ${RESET}npm install -g terminal-jarvis@${NEW_VERSION}"
    fi
    
else
    echo -e "${GREEN}🎉 Quality checks completed for feature branch!${RESET}"
    echo -e "${BLUE}Branch: ${CURRENT_BRANCH}${RESET}"
    echo -e "${YELLOW}💡 Run this script again after merging to ${DEFAULT_BRANCH} to publish${RESET}"
fi

echo ""
echo -e "${CYAN}🏁 Local CI/CD pipeline finished!${RESET}"
