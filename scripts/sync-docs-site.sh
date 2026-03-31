#!/usr/bin/env bash
# Sync terminal-jarvis version to my-life-as-a-dev docs site
# Usage: ./scripts/sync-docs-site.sh <version>
# Example: ./scripts/sync-docs-site.sh 0.0.80

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DOCS_REPO="${DOCS_REPO:-$HOME/projects/my-life-as-a-dev}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RESET='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${RESET} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${RESET} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${RESET} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${RESET} $1"
}

# Get version from argument or Cargo.toml
if [[ -n "$1" ]]; then
    VERSION="$1"
else
    VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')
fi

log_info "Syncing terminal-jarvis v$VERSION to docs site..."

# Verify docs repo exists
if [[ ! -d "$DOCS_REPO" ]]; then
    log_error "Docs repo not found at $DOCS_REPO"
    log_info "Clone it with: git clone https://github.com/BA-CalderonMorales/my-life-as-a-dev.git ~/projects/my-life-as-a-dev"
    exit 1
fi

# Verify we're on main branch in docs repo
cd "$DOCS_REPO"
git checkout main
git pull origin main

# Get today's date
TODAY=$(date +%Y-%m-%d)

# Update index.md - signal box version
log_info "Updating index.md signal box..."
sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+ \([0-9]\+ ⭐/v$VERSION \1/" docs/projects/active/terminal-jarvis/index.md
sed -i "s/latest release published on [0-9-]\+/latest release published on $TODAY/" docs/projects/active/terminal-jarvis/index.md

# Update highlights in index.md (add new version mention after status line)
if ! grep -q "v$VERSION" docs/projects/active/terminal-jarvis/index.md; then
    log_info "Adding v$VERSION highlight to index.md..."
    # Add new version bullet after "Highlights" section starts
    sed -i "/^## Highlights$/a\\- v$VERSION [brief description - update me]" docs/projects/active/terminal-jarvis/index.md
fi

# Update releases.md
log_info "Updating releases.md..."

# Check if this version already exists in releases.md
if grep -q "## Latest Release: v$VERSION" docs/projects/active/terminal-jarvis/details/releases.md; then
    log_warn "v$VERSION already in releases.md, skipping"
else
    # Create new release section
    RELEASE_SECTION="## Latest Release: v$VERSION

- **Published**: $TODAY
- **GitHub Release**: [v$VERSION](https://github.com/BA-CalderonMorales/terminal-jarvis/releases/tag/v$VERSION)
- **Crates.io Version**: [$VERSION](https://crates.io/crates/terminal-jarvis)

### Highlights

- [Add key changes here]

### Supported Tool Wrappers Updated in v$VERSION

- \`amp\`
- \`claude\`
- \`codex\`
- \`crush\`
- \`gemini\`
- \`goose\`
- \`llxprt\`
- \`opencode\`
- \`qwen\`

### Upgrade Recommendation

If you are on versions older than \`v$VERSION\`, upgrade:

\`\`\`bash
npm update -g terminal-jarvis
\`\`\`

---

## Previous Releases"

    # Replace "## Latest Release:" with previous release header
    sed -i 's/## Latest Release:/## Previous Releases\n\n### Previous Latest/' docs/projects/active/terminal-jarvis/details/releases.md
    
    # Add new release section at the top (after the header)
    # Find the line after "# Release Notes" and insert there
    sed -i "/^# Release Notes$/a\\
\\
$RELEASE_SECTION" docs/projects/active/terminal-jarvis/details/releases.md
    
    log_success "Added v$VERSION section to releases.md"
fi

# Show summary of changes
echo ""
log_info "Changes made to docs repo:"
git diff --stat

# Prompt for commit
echo ""
read -rp "Commit and push these changes? (y/N): " confirm

if [[ "$confirm" =~ ^[Yy]$ ]]; then
    git add docs/projects/active/terminal-jarvis/
    git commit -m "docs(terminal-jarvis): sync for v$VERSION release

- Updated index.md signal box to v$VERSION
- Added v$VERSION release notes
- Synced highlights and tool wrappers list"
    
    git push origin main
    log_success "Docs synced and pushed!"
    
    echo ""
    log_info "Next steps:"
    echo "  1. Wait for GitHub Pages deployment (1-2 minutes)"
    echo "  2. Verify at: https://ba-calderonmorales.github.io/my-life-as-a-dev/latest/"
else
    log_warn "Changes not committed. Review with:"
    echo "  cd $DOCS_REPO"
    echo "  git diff"
fi
