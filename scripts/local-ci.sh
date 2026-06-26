#!/usr/bin/env sh
set -eu

strict=0
mutation=0
package=1

usage() {
  cat <<'EOF'
usage: scripts/local-ci.sh [--strict] [--mutation] [--no-package]

Runs the local pre-PR gate without committing, tagging, pushing, or publishing.

Options:
  --strict      Require optional local security tools instead of skipping them.
  --mutation    Run cargo-mutants after the standard verification gate.
  --no-package  Skip the release package smoke build.
  -h, --help    Show this help.

Useful environment:
  TJ_SOCKET=1                 Run socket ci after local socket login.
  TJ_LOCAL_CI_NPX_SOCKET=1    Allow npx --yes socket ci when socket is absent.
  SOCKET_SECURITY_API_KEY     Lets socket ci run non-interactively.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --strict) strict=1 ;;
    --mutation) mutation=1 ;;
    --no-package) package=0 ;;
    -h | --help) usage; exit 0 ;;
    *) echo "local-ci: unknown option $1" >&2; usage >&2; exit 2 ;;
  esac
  shift
done

cd "$(dirname "$0")/.."

step() {
  printf '\n[%s] %s\n' "$1" "$2"
}

fail() {
  echo "local-ci: $*" >&2
  exit 1
}

skip_or_fail() {
  tool=$1
  hint=$2
  if [ "$strict" = "1" ]; then
    fail "$tool is required in --strict mode. $hint"
  fi
  echo "skip: $tool not available. $hint"
}

cargo_version() {
  sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1
}

json_version() {
  sed -n 's/.*"version": "\([^"]*\)".*/\1/p' "$1" | head -n 1
}

check_versions() {
  cargo=$(cargo_version)
  npm=$(json_version npm/terminal-jarvis/package.json)
  lock=$(json_version npm/terminal-jarvis/package-lock.json)
  test -n "$cargo" || fail "Cargo.toml version missing"
  test "$cargo" = "$npm" || fail "npm version $npm != Cargo version $cargo"
  test "$cargo" = "$lock" || fail "package-lock version $lock != Cargo version $cargo"
  grep -q "## \\[$cargo\\]" CHANGELOG.md ||
    fail "CHANGELOG.md missing ## [$cargo]"
  echo "versions ok: $cargo"
}

check_ignores() {
  for path in \
    dist/file target/file mutants.out/missed.txt node_modules/pkg \
    npm/terminal-jarvis/test.tgz homebrew/release/a.tar.gz coverage/out \
    .env .env.local .DS_Store app.log scratch.tmp
  do
    git check-ignore -q "$path" || fail "$path is not ignored"
  done
  if git check-ignore -q .env.example; then
    fail ".env.example must remain trackable"
  fi
  echo "ignore rules ok"
}

run_workflow_lint() {
  if command -v ruby >/dev/null 2>&1; then
    ruby -e 'require "yaml"; ARGV.each { |p| YAML.load_file(p) }' \
      .github/workflows/*.yml
  else
    skip_or_fail ruby "Install Ruby or actionlint to parse workflow YAML."
  fi

  if command -v actionlint >/dev/null 2>&1; then
    actionlint .github/workflows/*.yml
  else
    skip_or_fail actionlint "Install actionlint for GitHub workflow semantics."
  fi
}

run_gitleaks() {
  if command -v gitleaks >/dev/null 2>&1; then
    gitleaks detect --source . --redact --no-banner
  else
    skip_or_fail gitleaks "Install gitleaks to scan for committed secrets."
  fi
}

run_socket() {
  if command -v socket >/dev/null 2>&1; then
    if [ "$strict" = "1" ] || [ "${TJ_SOCKET:-0}" = "1" ] ||
      [ -n "${SOCKET_SECURITY_API_KEY:-}${SOCKET_API_KEY:-}${SOCKET_TOKEN:-}" ]; then
      CI=true socket ci
    else
      echo "skip: socket installed but not enabled; set TJ_SOCKET=1 after socket login"
    fi
  elif [ "${TJ_LOCAL_CI_NPX_SOCKET:-0}" = "1" ] && command -v npx >/dev/null 2>&1; then
    CI=true npx --yes socket ci
  else
    skip_or_fail socket "Install with 'npm install -g socket' or enable npx."
  fi
}

step 1 "repository hygiene"
git rev-parse --is-inside-work-tree >/dev/null
git diff --check
check_versions
check_ignores
status_counts=$(git status --short --untracked-files=no |
  awk '{count[$1]++} END {for (kind in count) print kind "=" count[kind]}')
if [ -n "$status_counts" ]; then
  printf 'tracked changes:\n%s\n' "$status_counts"
else
  echo "tracked changes: clean"
fi

step 2 "workflow lint"
run_workflow_lint

step 3 "standard verification"
TJ_REQUIRE_SECURITY_TOOLS=$strict scripts/verify.sh

step 4 "secret and supply-chain scans"
run_gitleaks
run_socket

step 5 "release package smoke"
if [ "$package" = "1" ]; then
  scripts/package-release.sh
  rm -rf dist
else
  echo "skip: package smoke disabled"
fi

step 6 "mutation"
if [ "$mutation" = "1" ]; then
  if command -v cargo-mutants >/dev/null 2>&1; then
    cargo mutants --minimum-test-timeout 30 --jobs 2
    rm -rf mutants.out mutants.out.old
  else
    skip_or_fail cargo-mutants "Install cargo-mutants or omit --mutation."
  fi
else
  echo "skip: mutation disabled; pass --mutation for the full local gate"
fi

echo
echo "local-ci: ok"
