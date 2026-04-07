#!/data/data/com.termux/files/usr/bin/env bash
#
# tj-fast: Run Terminal Jarvis with local Ollama LLM (Gemma 2B for speed)
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${GREEN}[tj-fast]${NC} $1"; }
warn() { echo -e "${YELLOW}[tj-fast]${NC} $1"; }
error() { echo -e "${RED}[tj-fast]${NC} $1"; }
info() { echo -e "${BLUE}[tj-fast]${NC} $1"; }

# Help
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "tj-fast: Terminal Jarvis with local Ollama (Gemma 2B - fast)"
    echo ""
    echo "Usage: tj-fast [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help"
    echo ""
    echo "Environment:"
    echo "  TJ_FAST_MODEL     Override default model (default: gemma4:2b)"
    echo "  OLLAMA_HOST       Ollama server URL (default: http://localhost:11434)"
    echo ""
    echo "Examples:"
    echo "  tj-fast           # Run with gemma4:2b (fast)"
    exit 0
fi

# Check if Ollama is running
OLLAMA_HOST="${OLLAMA_HOST:-http://localhost:11434}"
if ! curl -s "$OLLAMA_HOST/api/tags" > /dev/null 2>&1; then
    error "Ollama is not running at $OLLAMA_HOST"
    error "Start it with: ~/start_ollama.sh"
    exit 1
fi

# Set model - 2B for speed
FAST_MODEL="${TJ_FAST_MODEL:-gemma4:2b}"

# Check if model is available
if ! curl -s "$OLLAMA_HOST/api/tags" | grep -q "\"name\":\"$FAST_MODEL\""; then
    warn "Model '$FAST_MODEL' not found locally"
    info "Pulling model (this may take a while)..."
    ollama pull "$FAST_MODEL"
fi

log "Using fast local model: $FAST_MODEL"

# Set up environment for ADK
export JARVIS_MODEL="ollama/$FAST_MODEL"
export OLLAMA_HOST="$OLLAMA_HOST"

# Ensure ADK jarvis is built
if [[ ! -f "$PROJECT_ROOT/adk/jarvis" ]]; then
    log "Building ADK jarvis..."
    cd "$PROJECT_ROOT/adk"
    go build -o jarvis .
    log "Build complete"
fi

log "Starting Terminal Jarvis with fast local LLM..."
cd "$PROJECT_ROOT"
exec "$PROJECT_ROOT/jarvis.sh"
