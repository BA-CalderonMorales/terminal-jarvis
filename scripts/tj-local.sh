#!/data/data/com.termux/files/usr/bin/env bash
#
# tj-local: Run Terminal Jarvis with local Ollama LLM (Gemma 4B for quality)
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

log() { echo -e "${GREEN}[tj-local]${NC} $1"; }
warn() { echo -e "${YELLOW}[tj-local]${NC} $1"; }
error() { echo -e "${RED}[tj-local]${NC} $1"; }
info() { echo -e "${BLUE}[tj-local]${NC} $1"; }

# Help
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "tj-local: Terminal Jarvis with local Ollama (Gemma 4B)"
    echo ""
    echo "Usage: tj-local [options]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help"
    echo "  --model MODEL  Use specific Ollama model (default: gemma4:4b)"
    echo ""
    echo "Environment:"
    echo "  TJ_LOCAL_MODEL    Override default model"
    echo "  OLLAMA_HOST       Ollama server URL (default: http://localhost:11434)"
    echo ""
    echo "Examples:"
    echo "  tj-local                    # Run with gemma4:4b"
    echo "  tj-local --model gemma4:2b  # Run with faster 2B model"
    exit 0
fi

# Check if Ollama is running
OLLAMA_HOST="${OLLAMA_HOST:-http://localhost:11434}"
if ! curl -s "$OLLAMA_HOST/api/tags" > /dev/null 2>&1; then
    error "Ollama is not running at $OLLAMA_HOST"
    error "Start it with: ~/start_ollama.sh"
    exit 1
fi

# Set model
LOCAL_MODEL="${TJ_LOCAL_MODEL:-gemma4:4b}"
if [[ "$1" == "--model" && -n "$2" ]]; then
    LOCAL_MODEL="$2"
    shift 2
fi

# Check if model is available
if ! curl -s "$OLLAMA_HOST/api/tags" | grep -q "\"name\":\"$LOCAL_MODEL\""; then
    warn "Model '$LOCAL_MODEL' not found locally"
    info "Pulling model (this may take a while)..."
    ollama pull "$LOCAL_MODEL"
fi

log "Using local model: $LOCAL_MODEL"

# Set up environment for ADK
export JARVIS_MODEL="ollama/$LOCAL_MODEL"
export OLLAMA_HOST="$OLLAMA_HOST"

# Ensure ADK jarvis is built
if [[ ! -f "$PROJECT_ROOT/adk/jarvis" ]]; then
    log "Building ADK jarvis..."
    cd "$PROJECT_ROOT/adk"
    go build -o jarvis .
    log "Build complete"
fi

log "Starting Terminal Jarvis with local LLM..."
cd "$PROJECT_ROOT"
exec "$PROJECT_ROOT/jarvis.sh"
