#!/usr/bin/env sh
set -eu

# Pre-commit gate: keeps developer-local ledgers and build junk out of the
# history, and flags foot-guns before they land. Run on every commit via
# .git/hooks/pre-commit (see install-hooks.sh) or by hand:
#   scripts/bash/verify/index.sh pre-commit

branch=$(git symbolic-ref --short HEAD 2>/dev/null || echo detached)
deploy=false
case "$branch" in develop | main) deploy=true ;; esac

staged=$(git diff --cached --name-only 2>/dev/null || true)
[ -n "$staged" ] || exit 0

failures=0
notes=0
for file in $staged; do
  case "$file" in
    default_* | *.profraw | mutants.out* | *GOAL.md | *.tgz | *.zip | *.deb | *.rpm | *.log | .DS_Store | lcov.info | cobertura.xml)
      echo "pre-commit: refuse to commit build or package junk '$file'" >&2
      failures=$((failures + 1))
      ;;
  esac
  for prefix in dist/ coverage/ node_modules/ homebrew/release/ scratch/; do
    case "$file" in
      "$prefix"*)
        echo "pre-commit: refuse to commit build or package junk '$file'" >&2
        failures=$((failures + 1))
        ;;
    esac
  done
  case "$file" in
    plan/ | plan/*)
      if $deploy; then
        echo "pre-commit: refuse to commit developer-local '$file' on $branch" >&2
        failures=$((failures + 1))
      else
        echo "pre-commit: note: '$file' rides along on the next merge to develop/main; remove it first" >&2
        notes=$((notes + 1))
      fi
      ;;
  esac
done

if git diff --cached | grep -nE '^\+<<<<<<< |^\+=======|^\+>>>>>>>' >/dev/null 2>&1; then
  echo "pre-commit: staged diff contains conflict markers" >&2
  failures=$((failures + 1))
fi

if ! git diff --cached --check >/dev/null 2>&1; then
  echo "pre-commit: whitespace errors in the staged diff; run 'git diff --cached --check'" >&2
  failures=$((failures + 1))
fi

if git diff --cached | grep -nE 'AKIA[0-9A-Z]{16}|BEGIN (RSA |OPENSSH |EC )?PRIVATE KEY|ghp_[A-Za-z0-9]{30}' >/dev/null 2>&1; then
  echo "pre-commit: staged diff appears to contain a secret" >&2
  failures=$((failures + 1))
fi

if [ "$failures" -gt 0 ]; then
  echo "pre-commit: ${failures} problem(s); use --no-verify only when you know better" >&2
  exit 1
fi
[ "$notes" -eq 0 ] || echo "pre-commit: ${notes} note(s) above" >&2
exit 0
