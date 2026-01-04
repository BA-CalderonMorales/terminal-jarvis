#!/usr/bin/env bash
# Download OpenAI Whisper Tiny Model for Terminal Jarvis
# MIT Licensed, 100% open source, no API key required

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MODELS_DIR="$PROJECT_ROOT/models"
MODEL_URL="https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin"
MODEL_FILE="$MODELS_DIR/ggml-tiny.en.bin"

echo "[DOWNLOAD] OpenAI Whisper Tiny Model Setup"
echo ""
echo "Model Information:"
echo "  - Name: OpenAI Whisper Tiny (English-only)"
echo "  - Size: 75 MB"
echo "  - License: MIT (open source)"
echo "  - API Key: Not required"
echo "  - Privacy: 100% offline processing"
echo "  - Source: HuggingFace (ggerganov/whisper.cpp)"
echo ""

# Create models directory if it doesn't exist
if [ ! -d "$MODELS_DIR" ]; then
    echo "[CREATE] Creating models directory: $MODELS_DIR"
    mkdir -p "$MODELS_DIR"
fi

# Check if model already exists
if [ -f "$MODEL_FILE" ]; then
    FILE_SIZE=$(stat -f%z "$MODEL_FILE" 2>/dev/null || stat -c%s "$MODEL_FILE" 2>/dev/null)
    SIZE_MB=$((FILE_SIZE / 1000000))
    echo "[EXISTS] Model already downloaded: $MODEL_FILE ($SIZE_MB MB)"
    echo ""
    read -p "Re-download? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "[SKIPPED] Using existing model"
        exit 0
    fi
    echo "[DELETE] Removing existing model..."
    rm "$MODEL_FILE"
fi

# Download the model
echo "[DOWNLOAD] Downloading model from HuggingFace..."
echo "  URL: $MODEL_URL"
echo "  Destination: $MODEL_FILE"
echo ""

if command -v curl &> /dev/null; then
    curl -L --progress-bar -o "$MODEL_FILE" "$MODEL_URL"
elif command -v wget &> /dev/null; then
    wget --progress=bar:force -O "$MODEL_FILE" "$MODEL_URL"
else
    echo "[ERROR] Neither curl nor wget found. Please install one of them:"
    echo "  - macOS: curl is pre-installed"
    echo "  - Linux: sudo apt-get install curl"
    echo "  - Windows: Install Git for Windows (includes curl)"
    exit 1
fi

# Verify download
if [ -f "$MODEL_FILE" ]; then
    FILE_SIZE=$(stat -f%z "$MODEL_FILE" 2>/dev/null || stat -c%s "$MODEL_FILE" 2>/dev/null)
    SIZE_MB=$((FILE_SIZE / 1000000))

    echo ""
    echo "[SUCCESS] Model downloaded successfully"
    echo "  File: $MODEL_FILE"
    echo "  Size: $SIZE_MB MB"
    echo ""
    echo "[INFO] You can now use voice commands with terminal-jarvis"
    echo ""
    echo "To enable voice mode:"
    echo "  1. Build with: cargo build --features local-voice"
    echo "  2. Run with: terminal-jarvis (select voice option from menu)"
    echo ""
    echo "[PRIVACY] All voice processing happens locally on your device"
    echo "[SECURITY] No internet connection needed after download"
    echo "[COST] Free forever - no API keys or subscriptions"
else
    echo ""
    echo "[ERROR] Download failed"
    exit 1
fi
