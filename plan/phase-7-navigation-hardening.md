# Phase 7: Navigation Hardening

**Status**: PENDING  
**Priority**: MEDIUM  
**Estimated Sessions**: 2-3

## The Vision

Power users should feel like Terminal Jarvis is an extension of their muscle memory. Navigation should be:
- **Fast**: Minimal keystrokes to reach any destination
- **Predictable**: Same action = same result
- **Forgiving**: Easy to undo/go back

## Current Navigation Issues

1. No keyboard shortcuts (must arrow through menus)
2. No command history (can't repeat last action)
3. Full screen clears between menus (lose context)
4. No fuzzy matching (must type exact commands)
5. Post-tool menu requires re-navigation to switch tools

## Tasks

### 1. Number-Key Navigation
- [ ] Press `1-9` to select menu items directly
- [ ] Already partially implemented (menu shows "1. Reopen Claude")
- [ ] Make it work without pressing Enter

**Quick win**: This is partially there, just needs the input handling.

### 2. Quick Tool Switching
- [ ] From any menu, type tool name to jump directly (e.g., type "cla" → Claude)
- [ ] Fuzzy matching: "adier" matches "aider"
- [ ] Show matches as you type

### 3. Command History
- [ ] Track last 10 actions
- [ ] Up arrow recalls previous command
- [ ] Persist history across sessions (`~/.terminal-jarvis/history`)

### 4. Breadcrumb Navigation
- [ ] Show current location: `Main > AI Tools > Claude`
- [ ] Allow clicking/selecting breadcrumb to jump back
- [ ] Escape always goes up one level

### 5. Reduced Screen Clearing
- [ ] Don't full-clear between related menus
- [ ] Keep context visible when diving into submenus
- [ ] Only clear on major context switches

## Agent Instructions

Start with the highest-impact, lowest-effort change: number-key selection.

```bash
rg "create_themed_select" src/ -C 3
```

The Select widget from `inquire` likely supports this. Check the docs or existing usage.

For fuzzy matching, consider the `fuzzy-matcher` crate or simple substring matching first.

## Priority Order

1. Number keys (quick win)
2. Escape to go back (consistency)
3. Fuzzy tool search (power users love this)
4. History (nice to have)
5. Breadcrumbs (polish)

## Success Criteria

- [ ] Can launch any tool in ≤3 keystrokes from main menu
- [ ] Escape always goes back (no confusion)
- [ ] Typo-tolerant search works
