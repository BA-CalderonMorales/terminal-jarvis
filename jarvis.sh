#!/usr/bin/env bash
# Start the Terminal Jarvis home screen.
# Usage: ./jarvis.sh [--debug]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ADK_DIR="$SCRIPT_DIR/adk"
ADK_BIN="$ADK_DIR/jarvis"
GO_BIN=""
GO_CACHE_ROOT="${TMPDIR:-/tmp}/terminal-jarvis-go"

if [[ -x "/usr/local/go/bin/go" ]]; then
    GO_BIN="/usr/local/go/bin/go"
elif command -v go &>/dev/null; then
    GO_BIN="$(command -v go)"
fi

if [[ "${1:-}" == "--debug" ]]; then
    shift
else
    export RUST_LOG=warn
fi

if [[ -z "${GOCACHE:-}" ]]; then
    GOCACHE="$GO_CACHE_ROOT/go-build"
fi

if [[ -z "${GOMODCACHE:-}" ]]; then
    GOMODCACHE="$GO_CACHE_ROOT/go-mod"
fi

if [[ -z "${GOPATH:-}" ]]; then
    GOPATH="$GO_CACHE_ROOT/go-path"
fi

mkdir -p "$GOCACHE" "$GOMODCACHE" "$GOPATH"
export GOCACHE GOMODCACHE GOPATH
export GOTOOLCHAIN="${GOTOOLCHAIN:-auto}"

needs_rebuild=0
if [[ ! -x "$ADK_BIN" ]]; then
    needs_rebuild=1
elif find "$ADK_DIR" -type f -name '*.go' -newer "$ADK_BIN" -print -quit | grep -q .; then
    needs_rebuild=1
elif [[ "${JARVIS_REBUILD_ADK:-0}" == "1" ]]; then
    needs_rebuild=1
fi

if [[ "$needs_rebuild" == "1" ]]; then
    if [[ -n "$GO_BIN" ]]; then
        printf "[jarvis] building adk home screen...\n"
        if ! (
            cd "$ADK_DIR" &&
                "$GO_BIN" build -o jarvis .
        ); then
            printf "[jarvis] failed to build adk home screen.\n" >&2
            if [[ ! -x "$ADK_BIN" ]]; then
                exit 1
            fi
            printf "[jarvis] using existing binary at %s\n" "$ADK_BIN" >&2
        fi
    else
        printf "[jarvis] adk binary not found: %s\n" "$ADK_BIN" >&2
        printf "[jarvis] install Go and run: (cd adk && go build -o jarvis .)\n" >&2
        exit 1
    fi
fi

if [[ "${JARVIS_NO_TTY_ATTACH:-0}" != "1" && ! -t 0 ]]; then
    if { exec 3<>/dev/tty; } 2>/dev/null; then
        exec "$ADK_BIN" "$@" <&3 >&3 2>&3
    fi
fi

exec "$ADK_BIN" "$@"
