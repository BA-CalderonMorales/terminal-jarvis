# Phase 5: First-Run Experience

**Status**: PENDING  
**Priority**: HIGH  
**Estimated Sessions**: 2

## Why This Matters

The first 60 seconds determine if someone becomes a user or uninstalls. Current flow:
1. Install via npm/cargo/brew
2. Run `terminal-jarvis`
3. See menu... but no tools installed
4. User has to figure out what to do next

**Target flow:**
1. Install
2. Run `terminal-jarvis`
3. Auto-detect installed AI tools
4. Offer to install missing popular ones
5. Launch directly into the most relevant tool

## Tasks

### 1. Smart Tool Detection on Startup
- [ ] Check for installed tools BEFORE showing menu
- [ ] Cache detection results (don't re-scan every launch)
- [ ] Show "Detected: claude, aider" in welcome screen

**Implementation hint**: `ToolManager::get_installed_tools()` exists, use it earlier in startup.

### 2. First-Run Wizard
- [ ] Detect if this is first run (`~/.terminal-jarvis/initialized` flag)
- [ ] Show condensed welcome: "Terminal Jarvis manages your AI coding tools"
- [ ] Quick-install flow: "Install recommended tools? (claude, aider, codex)"
- [ ] Skip option for power users

**Keep it minimal**: 3 screens max, not a 10-step wizard.

### 3. Smart Default Tool Selection
- [ ] Remember last-used tool
- [ ] Option to set a "default" tool (launches on Enter from main menu)
- [ ] Quick-launch: `terminal-jarvis claude` should skip menus entirely

### 4. Credential Onboarding
- [ ] On first run, offer to configure API keys
- [ ] Show which tools need which keys
- [ ] Link to each provider's API key page

## Agent Instructions

Start by mapping the current startup flow:
```bash
rg "handle_interactive_mode" src/ -l
```

Then trace from there to understand where detection/onboarding can be injected.

The goal is MINIMAL changes that produce MAXIMUM perceived improvement. Don't rebuild the menu system - add a pre-flight check before it.

## Success Criteria

- [ ] First run shows detected tools without prompting
- [ ] User can install + launch a tool in under 60 seconds
- [ ] Returning users skip onboarding automatically
