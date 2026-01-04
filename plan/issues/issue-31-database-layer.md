# Issue #31: Database Layer Integration

## Problem

Managing commands for update, download, and pipeline operations is noisy and complex.

## Current State

Users must remember various commands:
- Configuration scattered across TOML files
- No central command history
- Pipeline/headless mode not well supported

## Goals

1. Easy access to common operations
2. Non-intrusive
3. Easy to manage
4. Support headless/pipeline usage

## Solutions

### Option 1: SQLite Backend (Current)

Already have libsql integration. Expand usage:

```bash
tj db status           # Show DB state
tj db export           # Export config
tj db import config.json
```

### Option 2: Command Aliases

Store frequently used commands:

```bash
tj alias set update "tj update --all"
tj alias set launch "tj claude --quick"
tj update              # Runs saved alias
```

### Option 3: Pipeline Mode

```bash
tj --headless --tool claude --prompt "Fix bug" < input.txt > output.txt
```

### Option 4: Config Profiles

```bash
tj profile save work
tj profile load home
tj profile list
```

## Implementation Plan

1. Design CLI interface for DB operations
2. Add `--headless` flag
3. Implement alias system
4. Add profile management

## Priority

Low - other issues more impactful to users

## Status

- [ ] Design DB CLI commands
- [ ] Implement --headless
- [ ] Alias system
- [ ] Profile management
