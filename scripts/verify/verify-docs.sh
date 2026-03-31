#!/usr/bin/env bash
# Verification: Documentation Drift Prevention
# Purpose: Validate docs are in sync with codebase and release metadata
# Usage: ./scripts/verify/verify-docs.sh [--fix] [--ci]
#
# This script prevents docs drift by validating:
# - Version consistency across all files
# - README sync with npm package
# - CHANGELOG has entry for current version
# - Homebrew formula URLs match version

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$PROJECT_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

# Mode flags
FIX_MODE=false
CI_MODE=false
ERRORS=0
WARNINGS=0

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --fix)
            FIX_MODE=true
            shift
            ;;
        --ci)
            CI_MODE=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Helper functions
log_info() {
    printf "%b[INFO]%b %s\n" "$BLUE" "$RESET" "$1"
}

log_success() {
    printf "%b[PASS]%b %s\n" "$GREEN" "$RESET" "$1"
}

log_error() {
    printf "%b[FAIL]%b %s\n" "$RED" "$RESET" "$1"
    ERRORS=$((ERRORS + 1))
}

log_warn() {
    printf "%b[WARN]%b %s\n" "$YELLOW" "$RESET" "$1"
    WARNINGS=$((WARNINGS + 1))
}

# Get canonical version from Cargo.toml
get_cargo_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Check version in Cargo.toml
check_cargo_version() {
    local version
    version=$(get_cargo_version)
    if [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        log_success "Cargo.toml version: $version"
        echo "$version"
    else
        log_error "Invalid version format in Cargo.toml: $version"
        echo ""
    fi
}

# Check version in npm package.json
check_npm_version() {
    local expected="$1"
    local file="npm/terminal-jarvis/package.json"
    
    if [[ ! -f "$file" ]]; then
        log_error "$file not found"
        return
    fi
    
    local version
    version=$(grep '"version":' "$file" | head -1 | sed 's/.*"version": "\(.*\)".*/\1/')
    
    if [[ "$version" == "$expected" ]]; then
        log_success "NPM package.json version: $version"
    else
        log_error "NPM version mismatch: expected $expected, found $version"
        if [[ "$FIX_MODE" == true ]]; then
            sed -i "s/\"version\": \".*\"/\"version\": \"$expected\"/" "$file"
            log_info "Fixed: Updated $file to version $expected"
        fi
    fi
}

# Check version in TypeScript source
check_ts_version() {
    local expected="$1"
    local file="npm/terminal-jarvis/src/index.ts"
    
    if [[ ! -f "$file" ]]; then
        log_warn "$file not found, skipping"
        return
    fi
    
    local version
    version=$(grep -o 'Terminal Jarvis v[0-9]\+\.[0-9]\+\.[0-9]\+' "$file" | sed 's/Terminal Jarvis v//')
    
    if [[ "$version" == "$expected" ]]; then
        log_success "TypeScript version: $version"
    else
        log_error "TypeScript version mismatch: expected $expected, found $version"
        if [[ "$FIX_MODE" == true ]]; then
            sed -i "s/Terminal Jarvis v[0-9]\+\.[0-9]\+\.[0-9]\+/Terminal Jarvis v$expected/g" "$file"
            log_info "Fixed: Updated $file to version $expected"
        fi
    fi
}

# Check version in Go ADK
check_adk_version() {
    local expected="$1"
    local file="adk/internal/ui/theme.go"
    
    if [[ ! -f "$file" ]]; then
        log_warn "$file not found, skipping"
        return
    fi
    
    local version
    version=$(grep 'Version = ' "$file" | sed 's/.*Version = "v\(.*\)".*/\1/')
    
    if [[ "$version" == "$expected" ]]; then
        log_success "ADK version: $version"
    else
        log_error "ADK version mismatch: expected $expected, found $version"
        if [[ "$FIX_MODE" == true ]]; then
            sed -i "s/Version = \"v[0-9]\+\.[0-9]\+\.[0-9]\+\"/Version = \"v$expected\"/" "$file"
            log_info "Fixed: Updated $file to version $expected"
        fi
    fi
}

# Check Homebrew formula
check_homebrew_formula() {
    local expected="$1"
    local file="homebrew/Formula/terminal-jarvis.rb"
    
    if [[ ! -f "$file" ]]; then
        log_warn "$file not found, skipping"
        return
    fi
    
    local version
    version=$(grep 'version "' "$file" | sed 's/.*version "\(.*\)".*/\1/')
    
    if [[ "$version" == "$expected" ]]; then
        log_success "Homebrew formula version: $version"
    else
        log_error "Homebrew version mismatch: expected $expected, found $version"
        if [[ "$FIX_MODE" == true ]]; then
            sed -i "s/version \"[0-9]\+\.[0-9]\+\.[0-9]\+\"/version \"$expected\"/" "$file"
            log_info "Fixed: Updated $file version to $expected"
        fi
    fi
    
    # Check URL version consistency
    local url_version
    url_version=$(grep -o 'download/v[0-9]\+\.[0-9]\+\.[0-9]\+' "$file" | head -1 | sed 's/download\///')
    
    if [[ "$url_version" == "v$expected" ]]; then
        log_success "Homebrew formula URLs point to: $url_version"
    else
        log_error "Homebrew URL version mismatch: URLs point to $url_version, expected v$expected"
        if [[ "$FIX_MODE" == true ]]; then
            sed -i "s/download\/v[0-9]\+\.[0-9]\+\.[0-9]\+/download\/v$expected/g" "$file"
            log_info "Fixed: Updated Homebrew URLs to v$expected"
        fi
    fi
}

# Check CHANGELOG has entry for version
check_changelog() {
    local expected="$1"
    local file="CHANGELOG.md"
    
    if [[ ! -f "$file" ]]; then
        log_warn "$file not found, skipping"
        return
    fi
    
    if grep -q "\[$expected\]" "$file"; then
        log_success "CHANGELOG.md has entry for v$expected"
    else
        log_error "CHANGELOG.md missing entry for v$expected"
    fi
}

# Check README is synced to npm package
check_readme_sync() {
    local main_readme="README.md"
    local npm_readme="npm/terminal-jarvis/README.md"
    
    if [[ ! -f "$npm_readme" ]]; then
        log_error "$npm_readme not found"
        if [[ "$FIX_MODE" == true ]]; then
            cp "$main_readme" "$npm_readme"
            log_info "Fixed: Copied $main_readme to $npm_readme"
        fi
        return
    fi
    
    # Check if files are identical
    if diff -q "$main_readme" "$npm_readme" >/dev/null 2>&1; then
        log_success "README.md is synced to npm package"
    else
        log_error "README.md is out of sync with npm package"
        if [[ "$CI_MODE" == true ]]; then
            echo ""
            echo "Differences:"
            diff "$main_readme" "$npm_readme" || true
            echo ""
        fi
        if [[ "$FIX_MODE" == true ]]; then
            cp "$main_readme" "$npm_readme"
            log_info "Fixed: Copied $main_readme to $npm_readme"
        fi
    fi
}

# Check Cargo.lock is in sync
check_cargo_lock() {
    if [[ ! -f "Cargo.lock" ]]; then
        log_warn "Cargo.lock not found, skipping"
        return
    fi
    
    # Check if Cargo.lock needs updating
    if cargo check --locked >/dev/null 2>&1; then
        log_success "Cargo.lock is in sync with Cargo.toml"
    else
        log_error "Cargo.lock is out of sync with Cargo.toml"
        if [[ "$FIX_MODE" == true ]]; then
            cargo check
            log_info "Fixed: Updated Cargo.lock"
        fi
    fi
}

# Check for any hardcoded version references in docs
check_doc_version_refs() {
    local expected="$1"
    local issues=0
    
    # Check AGENTS.md for outdated version references
    if [[ -f "AGENTS.md" ]]; then
        local outdated
        outdated=$(grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' AGENTS.md | grep -v "v$expected" || true)
        if [[ -n "$outdated" ]]; then
            log_warn "AGENTS.md contains outdated version references:"
            echo "$outdated" | while read -r ver; do
                echo "  - $ver"
            done
            ((issues++))
        fi
    fi
    
    # Check CLAUDE.md for outdated version references
    if [[ -f "CLAUDE.md" ]]; then
        local outdated
        outdated=$(grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' CLAUDE.md | grep -v "v$expected" || true)
        if [[ -n "$outdated" ]]; then
            log_warn "CLAUDE.md contains outdated version references:"
            echo "$outdated" | while read -r ver; do
                echo "  - $ver"
            done
            ((issues++))
        fi
    fi
    
    if [[ $issues -eq 0 ]]; then
        log_success "Documentation version references are current"
    fi
}

# Main execution
echo "=============================================="
echo "  DOCS DRIFT PREVENTION"
echo "  Version Consistency Validation"
echo "=============================================="
echo ""

if [[ "$FIX_MODE" == true ]]; then
    echo -e "${YELLOW}FIX MODE ENABLED${RESET} - Will attempt to fix issues automatically"
    echo ""
fi

# Get canonical version
CANONICAL_VERSION=$(check_cargo_version)

if [[ -z "$CANONICAL_VERSION" ]]; then
    echo ""
    echo "=============================================="
    echo -e "${RED}  VALIDATION FAILED${RESET}"
    echo "  Cannot determine canonical version"
    echo "=============================================="
    exit 1
fi

echo ""
echo "Checking version consistency across all files..."
echo "----------------------------------------------"

# Run all checks
check_npm_version "$CANONICAL_VERSION"
check_ts_version "$CANONICAL_VERSION"
check_adk_version "$CANONICAL_VERSION"
check_homebrew_formula "$CANONICAL_VERSION"

echo ""
echo "Checking documentation completeness..."
echo "----------------------------------------------"

check_changelog "$CANONICAL_VERSION"
check_readme_sync
check_cargo_lock
check_doc_version_refs "$CANONICAL_VERSION"

# Summary
echo ""
echo "=============================================="
if [[ $ERRORS -eq 0 && $WARNINGS -eq 0 ]]; then
    echo -e "${GREEN}  ALL CHECKS PASSED${RESET}"
    echo "  Version: v$CANONICAL_VERSION"
    echo "  Documentation is in sync"
    echo "=============================================="
    exit 0
elif [[ $ERRORS -eq 0 ]]; then
    echo -e "${YELLOW}  CHECKS PASSED WITH WARNINGS${RESET}"
    echo "  Version: v$CANONICAL_VERSION"
    echo "  Warnings: $WARNINGS"
    echo "=============================================="
    exit 0
else
    echo -e "${RED}  VALIDATION FAILED${RESET}"
    echo "  Version: v$CANONICAL_VERSION"
    echo "  Errors: $ERRORS"
    echo "  Warnings: $WARNINGS"
    echo "=============================================="
    if [[ "$CI_MODE" == true ]]; then
        echo ""
        echo "Run locally with --fix to auto-correct issues:"
        echo "  ./scripts/verify/verify-docs.sh --fix"
    fi
    exit 1
fi
