# Issue #26: Too Many Steps to Launch Tool

## Problem

Launching an AI tool requires too many Enter key presses.

**Current flow (from issue):**
```
$ npx terminal-jarvis
> Select an AI tool to launch: claude     # Step 1: Select + Enter
> Enter arguments for claude: [Enter]     # Step 2: Enter for default
[Authentication Advisory displayed]       # Step 3: Read
Press Enter to continue...                # Step 4: Enter
[Tool launches]
```

**Total: 3 Enter presses, 4 interaction points**

## Target

```
$ npx terminal-jarvis
> Select tool: claude  # Step 1: Select + Enter
[Tool launches]        # Immediate
```

**Target: 1 Enter press, immediate launch**

## Solutions

### 1. Skip Arguments Prompt

If user just wants defaults, don't ask for arguments.

```rust
// Instead of always prompting
let args = prompt_for_args(tool)?;

// Only prompt if tool requires args
if tool.requires_args() {
    let args = prompt_for_args(tool)?;
}
```

### 2. Conditional Auth Advisory

Only show auth warning if:
- API key is not set AND
- Tool requires authentication

```rust
if !has_api_key(tool) && tool.requires_auth() {
    show_auth_advisory(tool);
}
```

### 3. Remove "Press Enter to Continue"

Launch immediately after selection. No confirmation needed.

### 4. Quick Mode Flag

```bash
npx terminal-jarvis --quick  # Minimal prompts
npx terminal-jarvis -q       # Short form
```

### 5. Direct Tool Invocation

```bash
npx terminal-jarvis claude   # Direct launch
tj claude                    # With alias
```

### 6. Remember Last Used

```bash
npx terminal-jarvis          # Shows last used as default
> [claude] Select tool:      # Press Enter for claude
```

## Implementation Plan

1. Remove "Press Enter to continue" after auth advisory
2. Skip arguments prompt for default case
3. Only show auth advisory when API key missing
4. Add `--quick` flag
5. Support positional tool argument

## Code Changes

Files to modify:
- `src/cli_logic/cli_logic_interactive.rs`
- `src/cli_logic/cli_logic_tool_execution.rs`
- `src/auth_manager/auth_warning_system.rs`

## QA Branch

Test environment: `qa/env-fresh-install`

```bash
git checkout qa/env-fresh-install
npm run test:steps
npm run test:first-run
```

## Status

- [x] QA environment created
- [ ] Remove unnecessary Enter prompts
- [ ] Conditional auth advisory
- [ ] Add --quick flag
- [ ] Direct tool invocation
