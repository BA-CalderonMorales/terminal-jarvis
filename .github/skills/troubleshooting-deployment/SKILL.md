# Troubleshooting Deployment & CI/CD

**A guide to diagnosing and fixing complex deployment failures in the Terminal Jarvis ecosystem.**

## Case Study: Homebrew Tap Synchronization (v0.0.75 Release)

This case study documents a multi-stage failure in the Homebrew deployment pipeline and the systematic fixes applied.

### 1. The "Double-V" Version Error

**Symptom:**
```text
Using specified version: vv0.0.75
ERROR: Version vv0.0.75 does not exist in releases
```

**Root Cause:**
- `local-cd.sh` or the GitHub Workflow was passing `v0.0.75` (tag name) to a downstream script.
- The downstream script (in the Tap repo) or the formula generator automatically prepended `v`.
- Result: `v` + `v0.0.75` = `vv0.0.75`.

**Fix:**
- Updated `.github/workflows/cd-multiplatform.yml` to strip the `v` prefix before passing the version to the commit message.
- Updated `scripts/cicd/local-cd.sh` to use `${VERSION_NUM}` (raw number) instead of `${VERSION}` (tag).

### 2. The "Missing Formula Template" (404 Error)

**Symptom:**
```text
Fetching latest formula from main repository...
ERROR: Failed to download formula from main repository
URL: https://.../main/homebrew/Formula/terminal-jarvis.rb
404 Not Found
```

**Root Cause:**
- The Tap's update script attempts to fetch a "source of truth" Formula from the `main` branch of the *source* repository (`terminal-jarvis`).
- This file (`homebrew/Formula/terminal-jarvis.rb`) did not exist in `main`.

**Fix:**
- Created the missing file in `develop`.
- Cherry-picked/Hotfixed the file directly into `main` to unblock the pipeline immediately.

### 3. The "Premature Deletion" Logic Bug

**Symptom:**
```text
Formula is already up to date (v0.0.75)
Formula versions match but content differs, updating...
...
cp: cannot stat '/tmp/tmp.hZCAfLq6Jh': No such file or directory
```

**Root Cause:**
In `scripts/update-formula.sh` (in the Tap repo):
```bash
if [ "$CURRENT_VERSION" = "$NEW_VERSION" ]; then
    rm "$TEMP_FORMULA"  # <--- File deleted here
    
    if diff -q ...; then
        exit 0
    else
        echo "updating..."
        # Execution continues, but file is gone
    fi
fi
# ...
cp "$TEMP_FORMULA" "$FORMULA_FILE" # <--- Fails here
```

**Fix:**
- Modify the script to strictly manage the temporary file lifecycle.
- Ensure `rm "$TEMP_FORMULA"` is only called when the file is truly no longer needed (e.g., in an `EXIT` trap or strictly after the copy operation).

## General Troubleshooting Strategy

1.  **Isolate the Component**: Identify if the error is in the Source Repo (CI/CD scripts) or the Target Repo (Tap scripts).
2.  **Check Inputs**: Verify exactly what string is being passed between steps (e.g., "0.0.75" vs "v0.0.75").
3.  **Verify External State**: Check if referenced URLs (raw.githubusercontent.com) actually exist.
4.  **Audit Logic Flow**: Trace execution paths, especially around file operations and exit conditions.
