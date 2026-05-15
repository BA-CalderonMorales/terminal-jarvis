#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEFAULT_WORK_DIR="$PROJECT_ROOT/tmp/install-latency"

WORK_DIR="$DEFAULT_WORK_DIR"
RUN_NPM=true
RUN_CARGO=true
RUN_BREW=false
ALLOW_BREW_UNINSTALL=false
CLEAR_BREW_CACHE=false
KEEP_WORK_DIR=false
ACTION="run"
RUN_LABEL="$(date -u +%Y%m%dT%H%M%SZ)"

NPM_PACKAGES=(
  "terminal-jarvis"
  "terminal-jarvis@latest"
  "terminal-jarvis@beta"
  "terminal-jarvis@stable"
)

usage() {
  cat <<'EOF'
Usage: scripts/benchmarks/install-latency.sh [options]

Benchmarks cold and warm install latency for Terminal Jarvis release paths.
Results are written under tmp/install-latency by default.

Default matrix:
  - npx --yes terminal-jarvis --version
  - npx --yes terminal-jarvis@latest --version
  - npx --yes terminal-jarvis@beta --version
  - npx --yes terminal-jarvis@stable --version
  - npm install -g terminal-jarvis
  - cargo install terminal-jarvis

Homebrew is opt-in because it uses the real Homebrew prefix:
  --include-brew            Run brew install timing.
  --allow-brew-uninstall    Allow brew uninstall before cold/warm brew runs.
  --clear-brew-cache        Remove Terminal Jarvis Homebrew cache entries before cold brew run.

Other options:
  --npm-only                Run only npm/npx benchmarks.
  --cargo-only              Run only cargo baseline benchmarks.
  --work-dir PATH           Write benchmark logs and temp install roots under PATH.
  --keep-work-dir           Keep previous work dir contents before starting.
  --list-matrix             Print the benchmark matrix without running installs.
  --self-test               Run fast script self-tests without network installs.
  -h, --help                Show this help.

Recommended latency targets:
  - npx warm: under 5s
  - npm install -g warm: under 10s
  - npm/npx cold with binary download: under 30s on broadband
  - Homebrew warm: under 15s after tap metadata is current
  - Homebrew cold: under 45s when downloading a release archive, not building from source
  - cargo install baseline: informational, expected to be slower than binary distribution paths

Notes:
  - npm and cargo run with isolated cache/prefix/root directories.
  - npm runs with foreground scripts and timing logs enabled so package resolution,
    download, postinstall, binary download, extraction, chmod/staging, and verification
    evidence is retained in the per-case logs.
  - On WSL, the summary records the WSL distro and kernel so Windows WSL Ubuntu
    measurements can be compared against native Linux/macOS runs.
EOF
}

list_matrix() {
  if [[ "$RUN_NPM" == true ]]; then
    for mode in cold warm; do
      for package_spec in "${NPM_PACKAGES[@]}"; do
        printf "npm\t%s\tnpx-%s\tnpx --yes %s --version\n" "$mode" "$package_spec" "$package_spec"
      done
      printf "npm\t%s\tnpm-install-g-terminal-jarvis\tnpm install -g terminal-jarvis\n" "$mode"
    done
  fi

  if [[ "$RUN_CARGO" == true ]]; then
    for mode in cold warm; do
      printf "cargo\t%s\tcargo-install-terminal-jarvis\tcargo install terminal-jarvis --root <isolated>\n" "$mode"
    done
  fi

  if [[ "$RUN_BREW" == true ]]; then
    for mode in cold warm; do
      printf "homebrew\t%s\tbrew-install-terminal-jarvis\tbrew install ba-calderonmorales/terminal-jarvis/terminal-jarvis\n" "$mode"
    done
  fi
}

self_test() {
  local matrix
  matrix="$(list_matrix)"

  for expected in \
    $'npm\tcold\tnpx-terminal-jarvis\tnpx --yes terminal-jarvis --version' \
    $'npm\tcold\tnpx-terminal-jarvis@latest\tnpx --yes terminal-jarvis@latest --version' \
    $'npm\tcold\tnpx-terminal-jarvis@beta\tnpx --yes terminal-jarvis@beta --version' \
    $'npm\tcold\tnpx-terminal-jarvis@stable\tnpx --yes terminal-jarvis@stable --version' \
    $'npm\tcold\tnpm-install-g-terminal-jarvis\tnpm install -g terminal-jarvis' \
    $'npm\twarm\tnpm-install-g-terminal-jarvis\tnpm install -g terminal-jarvis' \
    $'cargo\tcold\tcargo-install-terminal-jarvis\tcargo install terminal-jarvis --root <isolated>' \
    $'cargo\twarm\tcargo-install-terminal-jarvis\tcargo install terminal-jarvis --root <isolated>'
  do
    if ! grep -Fxq "$expected" <<< "$matrix"; then
      echo "[ERROR] Missing benchmark matrix entry: $expected" >&2
      return 1
    fi
  done

  if [[ "$(slugify 'npm cold terminal-jarvis@beta')" != "npm-cold-terminal-jarvis-beta" ]]; then
    echo "[ERROR] slugify did not produce the expected stable log name" >&2
    return 1
  fi

  if ! write_context; then
    echo "[ERROR] write_context failed" >&2
    return 1
  fi

  echo "[install-latency] self-test passed"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --include-brew)
      RUN_BREW=true
      shift
      ;;
    --allow-brew-uninstall)
      ALLOW_BREW_UNINSTALL=true
      shift
      ;;
    --clear-brew-cache)
      CLEAR_BREW_CACHE=true
      shift
      ;;
    --npm-only)
      RUN_NPM=true
      RUN_CARGO=false
      RUN_BREW=false
      shift
      ;;
    --cargo-only)
      RUN_NPM=false
      RUN_CARGO=true
      RUN_BREW=false
      shift
      ;;
    --work-dir)
      WORK_DIR="${2:?--work-dir requires a path}"
      shift 2
      ;;
    --keep-work-dir)
      KEEP_WORK_DIR=true
      shift
      ;;
    --list-matrix)
      ACTION="list-matrix"
      shift
      ;;
    --self-test)
      ACTION="self-test"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[ERROR] Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

RUN_DIR="$WORK_DIR/$RUN_LABEL"
SUMMARY="$RUN_DIR/summary.tsv"

log() {
  echo "[install-latency] $*"
}

require_command() {
  local command_name="$1"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "[ERROR] Missing required command: $command_name" >&2
    return 1
  fi
}

duration_ms() {
  local started_ns="$1"
  local ended_ns
  ended_ns="$(date +%s%N)"
  echo $(((ended_ns - started_ns) / 1000000))
}

slugify() {
  echo "$1" | tr '/:@ ' '----' | tr -cd '[:alnum:]_.-'
}

write_context() {
  local context_file="$RUN_DIR/context.txt"

  {
    echo "run_label=$RUN_LABEL"
    echo "project_root=$PROJECT_ROOT"
    echo "work_dir=$WORK_DIR"
    echo "timestamp_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "uname=$(uname -a)"
    echo "os=$(uname -s)"
    echo "arch=$(uname -m)"
    if grep -qi microsoft /proc/version 2>/dev/null; then
      echo "wsl=true"
      echo "wsl_distro=${WSL_DISTRO_NAME:-unknown}"
    else
      echo "wsl=false"
    fi
    if command -v node >/dev/null 2>&1; then
      echo "node=$(node --version)"
    fi
    if command -v npm >/dev/null 2>&1; then
      echo "npm=$(npm --version)"
    fi
    if command -v cargo >/dev/null 2>&1; then
      echo "cargo=$(cargo --version)"
    fi
    if command -v rustc >/dev/null 2>&1; then
      echo "rustc=$(rustc --version)"
    fi
    if command -v brew >/dev/null 2>&1; then
      echo "brew=$(brew --version | head -1)"
    fi
  } > "$context_file"
}

run_case() {
  local manager="$1"
  local mode="$2"
  local name="$3"
  local command_text="$4"
  shift 4

  local slug
  slug="$(slugify "$manager-$mode-$name")"
  local log_file="$RUN_DIR/$slug.log"
  local started_ns
  local exit_code=0
  local ms

  log "Running [$manager][$mode] $name"
  started_ns="$(date +%s%N)"
  set +e
  "$@" > "$log_file" 2>&1
  exit_code=$?
  set -e
  ms="$(duration_ms "$started_ns")"

  printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" \
    "$manager" "$mode" "$name" "$ms" "$exit_code" "$command_text" "$log_file" >> "$SUMMARY"

  if [[ "$exit_code" -ne 0 ]]; then
    log "[WARNING] $name exited with $exit_code; see $log_file"
  fi
}

reset_npm_dirs() {
  rm -rf "$RUN_DIR/npm-cache" "$RUN_DIR/npm-prefix" "$RUN_DIR/npm-home"
  mkdir -p "$RUN_DIR/npm-cache" "$RUN_DIR/npm-prefix" "$RUN_DIR/npm-home"
}

run_npm_matrix() {
  require_command npm
  require_command npx

  reset_npm_dirs

  for mode in cold warm; do
    if [[ "$mode" == "cold" ]]; then
      reset_npm_dirs
    fi

    for package_spec in "${NPM_PACKAGES[@]}"; do
      run_case "npm" "$mode" "npx-$package_spec" "npx --yes $package_spec --version" \
        env \
          HOME="$RUN_DIR/npm-home" \
          npm_config_cache="$RUN_DIR/npm-cache" \
          npm_config_prefix="$RUN_DIR/npm-prefix" \
          npm_config_timing=true \
          npm_config_foreground_scripts=true \
          npx --yes "$package_spec" --version
    done

    run_case "npm" "$mode" "npm-install-g-terminal-jarvis" "npm install -g terminal-jarvis" \
      env \
        HOME="$RUN_DIR/npm-home" \
        npm_config_cache="$RUN_DIR/npm-cache" \
        npm_config_prefix="$RUN_DIR/npm-prefix" \
        npm_config_timing=true \
        npm_config_foreground_scripts=true \
        npm install -g terminal-jarvis
  done
}

reset_cargo_dirs() {
  rm -rf "$RUN_DIR/cargo-home" "$RUN_DIR/cargo-root" "$RUN_DIR/cargo-target"
  mkdir -p "$RUN_DIR/cargo-home" "$RUN_DIR/cargo-root" "$RUN_DIR/cargo-target"
}

run_cargo_matrix() {
  require_command cargo

  reset_cargo_dirs

  for mode in cold warm; do
    if [[ "$mode" == "cold" ]]; then
      reset_cargo_dirs
    fi

    run_case "cargo" "$mode" "cargo-install-terminal-jarvis" "cargo install terminal-jarvis --root <isolated>" \
      env \
        CARGO_HOME="$RUN_DIR/cargo-home" \
        CARGO_TARGET_DIR="$RUN_DIR/cargo-target" \
        cargo install terminal-jarvis --root "$RUN_DIR/cargo-root" --locked
  done
}

run_brew_matrix() {
  require_command brew

  local formula="ba-calderonmorales/terminal-jarvis/terminal-jarvis"
  local cache_dir
  cache_dir="$(brew --cache 2>/dev/null || true)"

  if [[ "$ALLOW_BREW_UNINSTALL" != true ]]; then
    log "[WARNING] Skipping Homebrew install benchmark because --allow-brew-uninstall was not set"
    log "[WARNING] Re-run with --include-brew --allow-brew-uninstall when this machine can modify Homebrew state"
    return 0
  fi

  for mode in cold warm; do
    brew uninstall --force terminal-jarvis >/dev/null 2>&1 || true

    if [[ "$mode" == "cold" && "$CLEAR_BREW_CACHE" == true && -n "$cache_dir" ]]; then
      rm -f "$cache_dir"/downloads/*terminal-jarvis* "$cache_dir"/*terminal-jarvis* 2>/dev/null || true
    fi

    run_case "homebrew" "$mode" "brew-install-terminal-jarvis" "brew install $formula" \
      brew install --verbose "$formula"
  done
}

summarize_signals() {
  local signals_file="$RUN_DIR/signals.txt"

  {
    echo "Install latency diagnostic signals"
    echo
    echo "NPM timing logs:"
    find "$RUN_DIR/npm-cache/_logs" -type f \( -name "*-timing.json" -o -name "*.log" \) -print 2>/dev/null | sort || true
    echo
    echo "Postinstall phase markers:"
    grep -RInE "Detected|Downloading v|Download complete|Extracting|Extraction complete|installed in|Binary installed|Fallback|Error|Warning" "$RUN_DIR"/*.log 2>/dev/null || true
    echo
    echo "Homebrew phase markers:"
    grep -RInE "Downloading|Pouring|Installing|Fetching|Bottle|tar|curl|source|build" "$RUN_DIR"/homebrew-*.log 2>/dev/null || true
  } > "$signals_file"
}

main() {
  cd "$PROJECT_ROOT"

  if [[ "$KEEP_WORK_DIR" != true ]]; then
    rm -rf "$RUN_DIR"
  fi

  mkdir -p "$RUN_DIR"
  printf "manager\tmode\tcase\tduration_ms\texit_code\tcommand\tlog_file\n" > "$SUMMARY"
  write_context

  if [[ "$RUN_NPM" == true ]]; then
    run_npm_matrix
  fi

  if [[ "$RUN_CARGO" == true ]]; then
    run_cargo_matrix
  fi

  if [[ "$RUN_BREW" == true ]]; then
    run_brew_matrix
  fi

  summarize_signals

  log "Summary: $SUMMARY"
  log "Context: $RUN_DIR/context.txt"
  log "Signals: $RUN_DIR/signals.txt"
}

case "$ACTION" in
  list-matrix)
    list_matrix
    ;;
  self-test)
    mkdir -p "$RUN_DIR"
    self_test
    ;;
  run)
    main "$@"
    ;;
  *)
    echo "[ERROR] Unknown action: $ACTION" >&2
    exit 2
    ;;
esac
