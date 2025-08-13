#!/bin/bash

# Terminal Jarvis Branch Status & CI Options
# Shows current status and available actions

set -e  # Exit on any error

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

# Get git info
CURRENT_BRANCH=$(git branch --show-current)
DEFAULT_BRANCH="develop"
HAS_CHANGES=$(git status --porcelain | wc -l)
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

clear
log_header "Terminal Jarvis CI/CD Status"

log_info_if_enabled "Current Status:"
log_info_if_enabled "   Branch: ${CURRENT_BRANCH}"
log_info_if_enabled "   Version: v${CURRENT_VERSION}"
log_info_if_enabled "   Uncommitted changes: $([ $HAS_CHANGES -eq 0 ] && echo "None" || echo "${HAS_CHANGES} files")"

# Show last commit
LAST_COMMIT=$(git log -1 --pretty=format:"%h - %s (%cr)" 2>/dev/null || echo "No commits")
log_info_if_enabled "Last commit: ${LAST_COMMIT}"

log_separator

log_info_if_enabled "Available Actions:"

if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    echo -e "${BLUE}1. 🧪 Local CI (Validation)${RESET}"
    echo -e "   └─ Run quality checks, tests, builds (no commits/pushes)"
    echo -e "   └─ Command: ${YELLOW}./scripts/cicd/local-ci.sh${RESET}"
    echo ""
    
    echo -e "${BLUE}2. Local CD (Deployment)${RESET}"
    echo -e "   └─ Commit, tag, push, publish (run local-ci.sh first)"
    echo -e "   └─ Will ask: merge to ${DEFAULT_BRANCH} or deploy from branch"
    echo -e "   └─ Command: ${YELLOW}./scripts/cicd/local-cd.sh${RESET}"
    echo ""
    
    echo -e "${BLUE}3. 🔀 Manual Git Workflow${RESET}"
    echo -e "   └─ Switch to ${DEFAULT_BRANCH}: ${YELLOW}git checkout ${DEFAULT_BRANCH}${RESET}"
    echo -e "   └─ Merge feature: ${YELLOW}git merge ${CURRENT_BRANCH} --no-ff${RESET}"
    echo ""
    
else
    log_success "You're on the ${DEFAULT_BRANCH} branch"
    echo ""
    log_info_if_enabled "1. Branch-specific CI (Validation)"
    log_info_if_enabled "   └─ Run quality checks, tests, builds (no commits/pushes)"
    log_info_if_enabled "   └─ Command: ./scripts/cicd/local-ci.sh"
    
    log_info_if_enabled "2. Local CD (Deployment)"
    log_info_if_enabled "   └─ Commit, tag, push, publish (run local-ci.sh first)"
    log_info_if_enabled "   └─ Will ask: merge to ${DEFAULT_BRANCH} or deploy from branch"
    log_info_if_enabled "   └─ Command: ./scripts/cicd/local-cd.sh"
    
    log_info_if_enabled "3. Manual Git Workflow"
    log_info_if_enabled "   └─ Switch to ${DEFAULT_BRANCH}: git checkout ${DEFAULT_BRANCH}"
    log_info_if_enabled "   └─ Merge feature: git merge ${CURRENT_BRANCH} --no-ff"
    
else
    log_success_if_enabled "You're on the ${DEFAULT_BRANCH} branch"
    
    log_info_if_enabled "1. Local CI (Validation)"
    log_info_if_enabled "   └─ Run quality checks, tests, builds (no commits/pushes)"
    log_info_if_enabled "   └─ Command: ./scripts/cicd/local-ci.sh"
    
    log_info_if_enabled "2. Local CD (Deployment)"
    log_info_if_enabled "   └─ Version bump + commit + tag + push + publish"
    log_info_if_enabled "   └─ Command: ./scripts/cicd/local-cd.sh"
fi

log_info_if_enabled "4. Test Interactive Mode"
log_info_if_enabled "   └─ See the new futuristic UI in action"
log_info_if_enabled "   └─ Command: ./target/release/terminal-jarvis"

log_separator

log_info_if_enabled "Recommended Next Steps:"
if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    if [ $HAS_CHANGES -eq 0 ]; then
        log_success_if_enabled "   → Your feature branch looks clean!"
        log_info_if_enabled "   → Run: ./scripts/cicd/local-ci.sh then ./scripts/cicd/local-cd.sh"
    else
        log_warn_if_enabled "   → You have uncommitted changes"
        log_info_if_enabled "   → Commit changes first, then run CI/CD pipeline"
    fi
else
    log_success_if_enabled "   → Ready for immediate publish from ${DEFAULT_BRANCH}"
    log_info_if_enabled "   → Run: ./scripts/cicd/local-ci.sh then ./scripts/cicd/local-cd.sh"
fi

log_success_if_enabled "Ready to deploy Terminal Jarvis with futuristic UX!"
fi

echo -e "${BLUE}4. Test Interactive Mode${RESET}"
echo -e "   └─ See the new futuristic UI in action"
echo -e "   └─ Command: ${YELLOW}./target/release/terminal-jarvis${RESET}"
echo ""

log_info_if_enabled "Recommended Next Steps:"
if [ "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]; then
    if [ $HAS_CHANGES -eq 0 ]; then
        echo -e "   ${GREEN}→ Your feature branch looks clean!${RESET}"
        echo -e "   ${BLUE}→ Run: ${YELLOW}./scripts/cicd/local-ci.sh${RESET} ${BLUE}then ${YELLOW}./scripts/cicd/local-cd.sh${RESET}"
    else
        echo -e "   ${YELLOW}→ You have uncommitted changes${RESET}"
        echo -e "   ${BLUE}→ Commit changes first, then run CI/CD pipeline${RESET}"
    fi
else
    echo -e "   ${GREEN}→ Ready for immediate publish from ${DEFAULT_BRANCH}${RESET}"
    echo -e "   ${BLUE}→ Run: ${YELLOW}./scripts/cicd/local-ci.sh${RESET} ${BLUE}then ${YELLOW}./scripts/cicd/local-cd.sh${RESET}"
fi

echo ""
echo -e "${CYAN}🏁 Ready to deploy Terminal Jarvis with futuristic UX!${RESET}"
