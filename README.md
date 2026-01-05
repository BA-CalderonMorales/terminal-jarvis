# Terminal Jarvis v0.0.73 - NVM Fix Test

## Test Objective

Verify that Issue #37 is resolved: **Tool installation fails silently when npm is installed via NVM**.

The problem was that `sudo npm install -g` fails when npm is installed via NVM because NVM sets up npm in the user's home directory, which isn't in sudo's PATH.

## Quick Test

```bash
test-qa.sh
```

## Manual Test Steps

1. **Install terminal-jarvis**:
   ```bash
   npm install -g terminal-jarvis@0.0.73
   ```

2. **Verify installation**:
   ```bash
   terminal-jarvis --version
   terminal-jarvis list
   ```

3. **Test tool installation**:
   ```bash
   terminal-jarvis install claude
   ```

## Expected Results

| Test | Before Fix (v0.0.72) | After Fix (v0.0.73) |
|------|----------------------|---------------------|
| `npm install -g terminal-jarvis` | ✅ Works | ✅ Works |
| `terminal-jarvis install claude` | ❌ Silent failure (`sudo: npm: command not found`) | ✅ Shows progress or auth prompt |
| Error visibility | ❌ Errors hidden | ✅ Errors shown in terminal |

## Fix Details

The fix in v0.0.73:
1. **Removed sudo** for npm global installs - NVM already sets proper permissions
2. **Shows stderr** - Error output now visible instead of being swallowed
3. **Updated configs** - All npm tools have `requires_sudo = false`

## Issue Reference

- GitHub Issue: [#37](https://github.com/BA-CalderonMorales/terminal-jarvis/issues/37)
- Fixed in: v0.0.73

## Environment Info

- Base Image: Ubuntu 22.04
- Node: 20.x (via devcontainer feature)
- This tests the standard npm setup, not NVM

> **Note**: To test with actual NVM, manually install NVM in the Codespace after it starts.
