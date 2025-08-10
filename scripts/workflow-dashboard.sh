#!/bin/bash

# Terminal Jarvis Branch Status & CI Options
# Shows current status and available actions

set -e  # Exit on any error

# Colors for output
CYAN='\033[0;96m'
BLUE='\033[0;94m'
GREEN='\033[0;92m'
YELLOW='\033[0;93m'
RESET='\033[0m'

# Get git info
CURRENT_BRANCH=$(git branch --show-current)
DEFAULT_BRANCH="develop"
HAS_CHANGES=$(git status --porcelain | wc -l)
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

clear
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${CYAN}║               🚀 Terminal Jarvis CI/CD Status                ║${RESET}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${RESET}"
echo ""
echo -e "${BLUE}📍 Current Status:${RESET}"
echo -e "   Branch: ${YELLOW}${CURRENT_BRANCH}${RESET}"
echo -e "   Version: ${GREEN}v${CURRENT_VERSION}${RESET}"
echo -e "   Uncommitted changes: $([ $HAS_CHANGES -eq 0 ] && echo -e "${GREEN}None${RESET}" || echo -e "${YELLOW}${HAS_CHANGES} files${RESET}")"
echo ""

# Show last commit
LAST_COMMIT=$(git log -1 --pretty=format:"%h - %s (%cr)" 2>/dev/null || echo "No commits")
echo -e "${BLUE}📝 Last commit:${RESET} ${LAST_COMMIT}"
echo ""

echo -e "${CYAN}🛠️  Available Actions:${RESET}"
echo ""

if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    echo -e "${BLUE}1. 🧪 Local CI (Validation)${RESET}"
    echo -e "   └─ Run quality checks, tests, builds (no commits/pushes)"
    echo -e "   └─ Command: ${YELLOW}./scripts/local-ci.sh${RESET}"
    echo ""
    
    echo -e "${BLUE}2. 🚀 Local CD (Deployment)${RESET}"
    echo -e "   └─ Commit, tag, push, publish (run local-ci.sh first)"
    echo -e "   └─ Will ask: merge to ${DEFAULT_BRANCH} or deploy from branch"
    echo -e "   └─ Command: ${YELLOW}./scripts/local-cd.sh${RESET}"
    echo ""
    
    echo -e "${BLUE}3. 🔀 Manual Git Workflow${RESET}"
    echo -e "   └─ Switch to ${DEFAULT_BRANCH}: ${YELLOW}git checkout ${DEFAULT_BRANCH}${RESET}"
    echo -e "   └─ Merge feature: ${YELLOW}git merge ${CURRENT_BRANCH} --no-ff${RESET}"
    echo ""
    
else
    echo -e "${GREEN}✅ You're on the ${DEFAULT_BRANCH} branch${RESET}"
    echo ""
    echo -e "${BLUE}1. 🧪 Local CI (Validation)${RESET}"
    echo -e "   └─ Run quality checks, tests, builds (no commits/pushes)"
    echo -e "   └─ Command: ${YELLOW}./scripts/local-ci.sh${RESET}"
    echo ""
    
    echo -e "${BLUE}2. 🚀 Local CD (Deployment)${RESET}"
    echo -e "   └─ Version bump + commit + tag + push + publish"
    echo -e "   └─ Command: ${YELLOW}./scripts/local-cd.sh${RESET}"
    echo ""
fi

echo -e "${BLUE}4. 🎯 Test Interactive Mode${RESET}"
echo -e "   └─ See the new futuristic UI in action"
echo -e "   └─ Command: ${YELLOW}./target/release/terminal-jarvis${RESET}"
echo ""

echo -e "${CYAN}💡 Recommended Next Steps:${RESET}"
if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    if [ $HAS_CHANGES -eq 0 ]; then
        echo -e "   ${GREEN}→ Your feature branch looks clean!${RESET}"
        echo -e "   ${BLUE}→ Run: ${YELLOW}./scripts/local-ci.sh${RESET} ${BLUE}then ${YELLOW}./scripts/local-cd.sh${RESET}"
    else
        echo -e "   ${YELLOW}→ You have uncommitted changes${RESET}"
        echo -e "   ${BLUE}→ Commit changes first, then run CI/CD pipeline${RESET}"
    fi
else
    echo -e "   ${GREEN}→ Ready for immediate publish from ${DEFAULT_BRANCH}${RESET}"
    echo -e "   ${BLUE}→ Run: ${YELLOW}./scripts/local-ci.sh${RESET} ${BLUE}then ${YELLOW}./scripts/local-cd.sh${RESET}"
fi

echo ""
echo -e "${CYAN}🏁 Ready to deploy Terminal Jarvis with futuristic UX!${RESET}"
