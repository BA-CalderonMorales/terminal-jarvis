#!/usr/bin/env bash
# Start the Terminal Jarvis home screen.
# Usage: ./jarvis.sh [--debug]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ADK_DIR="$SCRIPT_DIR/adk"
ADK_BIN="$ADK_DIR/jarvis"
GO_BIN=""
GO_CACHE_ROOT="${TMPDIR:-/tmp}/terminal-jarvis-go"
GO_REQUIRED_VERSION=""
GO_VERSION=""

get_go_mod_required_version() {
    awk '/^go [0-9]+\.[0-9]+(\.[0-9]+)?$/ { print $2; exit }' "$ADK_DIR/go.mod"
}

get_go_binary_version() {
    local go_bin="$1"
    local goversion

    goversion="$($go_bin env GOVERSION 2>/dev/null || true)"
    goversion="${goversion#go}"

    if [[ "$goversion" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]]; then
        printf "%s\n" "$goversion"
        return
    fi

    goversion="$($go_bin version 2>/dev/null | awk '{print $3}')"
    goversion="${goversion#go}"
    goversion="${goversion%%[^0-9.]*}"

    printf "%s\n" "$goversion"
}

version_gte() {
    local lhs="$1"
    local rhs="$2"
    [[ "$(printf '%s\n%s\n' "$rhs" "$lhs" | sort -V | tail -n1)" == "$lhs" ]]
}

bootstrap_go_toolchain() {
    local required_version="$1"
    local toolchain_root="$GO_CACHE_ROOT/toolchain"
    local go_os
    local go_arch
    local go_archive
    local go_url

    go_os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    case "$(uname -m)" in
    x86_64)
        go_arch="amd64"
        ;;
    aarch64 | arm64)
        go_arch="arm64"
        ;;
    *)
        return 1
        ;;
    esac

    go_archive="go${required_version}.linux-${go_arch}.tar.gz"
    go_url="https://go.dev/dl/${go_archive}"

    mkdir -p "$toolchain_root"

    if [[ -x "$toolchain_root/go/bin/go" ]]; then
        local existing_version
        existing_version="$(get_go_binary_version "$toolchain_root/go/bin/go")"
        if [[ "$existing_version" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]] && version_gte "$existing_version" "$required_version"; then
            printf "%s\n" "$toolchain_root/go/bin/go"
            return
        fi
    fi

    printf "[jarvis] downloading Go %s toolchain...\n" "$required_version"
    if command -v curl &>/dev/null; then
        if ! curl -fsSL "$go_url" -o "$toolchain_root/$go_archive"; then
            return 1
        fi
    elif command -v wget &>/dev/null; then
        if ! wget -qO "$toolchain_root/$go_archive" "$go_url"; then
            return 1
        fi
    else
        return 1
    fi

    rm -rf "$toolchain_root/go"
    if ! tar -C "$toolchain_root" -xzf "$toolchain_root/$go_archive"; then
        return 1
    fi

    printf "%s\n" "$toolchain_root/go/bin/go"
}

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

GO_REQUIRED_VERSION="$(get_go_mod_required_version)"

if [[ -z "$GO_REQUIRED_VERSION" ]]; then
    printf "[jarvis] failed to read required Go version from %s\n" "$ADK_DIR/go.mod" >&2
    exit 1
fi

if [[ -n "$GO_BIN" ]]; then
    GO_VERSION="$(get_go_binary_version "$GO_BIN")"
    if [[ -z "$GO_VERSION" || ! "$GO_VERSION" =~ ^[0-9]+\.[0-9]+(\.[0-9]+)?$ ]]; then
        GO_BIN=""
    elif ! version_gte "$GO_VERSION" "$GO_REQUIRED_VERSION"; then
        printf "[jarvis] detected Go %s, but ADK requires Go %s+\n" "$GO_VERSION" "$GO_REQUIRED_VERSION"
        GO_BIN=""
    fi
fi

if [[ -z "$GO_BIN" ]]; then
    BOOTSTRAPPED_GO_BIN="$(bootstrap_go_toolchain "$GO_REQUIRED_VERSION" || true)"
    if [[ -n "$BOOTSTRAPPED_GO_BIN" && -x "$BOOTSTRAPPED_GO_BIN" ]]; then
        GO_BIN="$BOOTSTRAPPED_GO_BIN"
        GO_VERSION="$(get_go_binary_version "$GO_BIN")"
    fi
fi

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
        printf "[jarvis] install Go %s+ and run: (cd adk && go build -o jarvis .)\n" "$GO_REQUIRED_VERSION" >&2
        exit 1
    fi
fi

# In headless mode, skip TTY attachment entirely
if [[ "${JARVIS_HEADLESS:-0}" == "1" || "${JARVIS_HEADLESS:-}" == "true" ]]; then
    export JARVIS_HEADLESS=1
    exec "$ADK_BIN" "$@"
fi

if [[ "${JARVIS_NO_TTY_ATTACH:-0}" != "1" && ! -t 0 ]]; then
    if { exec 3<>/dev/tty; } 2>/dev/null; then
        exec "$ADK_BIN" "$@" <&3 >&3 2>&3
    fi
fi

exec "$ADK_BIN" "$@"
