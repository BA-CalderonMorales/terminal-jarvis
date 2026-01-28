# Manual Patching Guide: Homebrew Tap Fix

**Objective**: Fix the `update-formula.sh` script in `BA-CalderonMorales/homebrew-terminal-jarvis` to prevent deployment failures.

## Context
The current script prematurely deletes a temporary file when version numbers match but content differs, causing a "No such file or directory" error during the update process.

## The Patch

### Target File
`scripts/update-formula.sh`

### Logic Change
Move the deletion of the temporary file (`rm "$TEMP_FORMULA"`) inside the specific block where we exit early, ensuring it remains available if an update is needed.

### Code Diff

```diff
  # Check if update is needed
  if [ "$CURRENT_VERSION" = "$NEW_VERSION" ]; then
      echo "Formula is already up to date (v$NEW_VERSION)"
-     rm "$TEMP_FORMULA"
      
      # Still verify the content matches exactly
      if diff -q "$FORMULA_FILE" "$TEMP_FORMULA" > /dev/null 2>&1; then
          echo "Formula content is identical"
+         rm "$TEMP_FORMULA"
          exit 0
      else
          echo "Formula versions match but content differs, updating..."
      fi
  fi
```

## Execution Instructions for Agent

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/BA-CalderonMorales/homebrew-terminal-jarvis.git
    cd homebrew-terminal-jarvis
    git checkout develop
    ```

2.  **Apply the Fix**:
    Open `scripts/update-formula.sh` and locate the version check block (around line 53).
    - Remove `rm "$TEMP_FORMULA"` from before the `if diff` check.
    - Add `rm "$TEMP_FORMULA"` inside the `if diff` success block (before `exit 0`).

3.  **Verify**:
    Ensure the file is NOT deleted if `diff` finds differences (the `else` block).

4.  **Commit and Push**:
    ```bash
    git add scripts/update-formula.sh
    git commit -m "fix(script): prevent premature deletion of temporary formula file during update check"
    git push origin develop
    ```
