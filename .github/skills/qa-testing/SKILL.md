# Skill: QA Testing

**Name**: qa-testing
**Description**: Create and manage minimal QA test branches for release verification
**Trigger**: "Test this release", "Create QA branch", "Verify in fresh Codespace", "QA for v0.0.X"

---

## Overview

QA branches are **orphan branches** containing only the minimal files needed to test a release in a fresh environment (Codespace). They do NOT inherit code from develop/main.

## Branch Naming Convention

```
qa/v{VERSION}-{TEST_TYPE}-minimal
```

Examples:
- `qa/v0.0.72-glibc-minimal`
- `qa/v0.0.72-fresh-install-minimal`
- `qa/v0.0.73-auth-flows-minimal`

## Creating a QA Branch

### Step 1: Create Orphan Branch

```bash
# From any branch, create orphan (no history)
git checkout --orphan qa/v{VERSION}-{TEST_TYPE}-minimal
git rm -rf . 2>/dev/null || true
git clean -fd
```

### Step 2: Create Minimal Files

Each QA branch needs only two files:

**`.devcontainer/devcontainer.json`**:
```json
{
  "name": "QA: {TEST_NAME} - v{VERSION}",
  "image": "mcr.microsoft.com/devcontainers/base:{IMAGE}",
  "features": {
    "ghcr.io/devcontainers/features/node:1": { "version": "20" }
  },
  "containerEnv": {
    "QA_TEST": "{test-type}",
    "QA_VERSION": "{VERSION}",
    "QA_ISSUES": "{issue-numbers}"
  },
  "postCreateCommand": "echo '#!/bin/bash\\n{TEST_SCRIPT}' > /usr/local/bin/test-qa.sh && chmod +x /usr/local/bin/test-qa.sh && echo 'Run: test-qa.sh'",
  "customizations": {
    "vscode": {
      "extensions": ["GitHub.copilot", "GitHub.copilot-chat"]
    }
  }
}
```

**`README.md`**:
```markdown
# Terminal Jarvis v{VERSION} - {TEST_NAME}

## Test Objective
{What this test verifies}

## Quick Test
\`\`\`bash
test-qa.sh
\`\`\`

## Expected Results
| Test | Before Fix | After Fix |
|------|------------|-----------|
| ... | ... | ... |
```

### Step 3: Commit and Push

```bash
git add -A
git commit -m "qa({test-type}): minimal {image} test for Issue #{N} - v{VERSION}"
git push -u origin qa/v{VERSION}-{TEST_TYPE}-minimal
```

## Standard QA Test Types

### GLIBC Compatibility (Issue #24)

**Image**: `debian-12` (GLIBC 2.36)

**Test Script**:
```bash
echo "GLIBC: $(ldd --version | head -1)"
npm install -g terminal-jarvis
ldd $(which terminal-jarvis) 2>&1 || echo "(static)"
terminal-jarvis --version
terminal-jarvis list
```

### Fresh Install & UX (Issues #23, #26)

**Image**: `ubuntu-22.04`

**Test Script**:
```bash
time npm install -g terminal-jarvis
terminal-jarvis --version
terminal-jarvis list
terminal-jarvis claude -- --help 2>&1 | head -3 || echo "(not installed)"
```

### Auth Flows (Issue #27)

**Image**: `ubuntu-22.04`

**Test Script**:
```bash
npm install -g terminal-jarvis
terminal-jarvis  # Test interactive menu
# Manual: Select tool, observe auth prompts
```

## Running QA Tests

### Via GitHub Codespace (Recommended)

1. Go to https://github.com/BA-CalderonMorales/terminal-jarvis
2. Switch to QA branch: `qa/v{VERSION}-{TEST_TYPE}-minimal`
3. Click **Code** → **Codespaces** → **Create codespace**
4. Wait for environment to build (~1-2 min)
5. Run `test-qa.sh` in terminal

### Via Docker (Local)

```bash
git checkout qa/v{VERSION}-{TEST_TYPE}-minimal
cd .devcontainer
docker build -t qa-test .
docker run -it qa-test bash
test-qa.sh
```

## Recording Results

After testing, update `plan/ROADMAP.md` on `plan/strategic-roadmap` branch:

```bash
git checkout plan/strategic-roadmap
# Edit plan/ROADMAP.md with results
git commit -m "plan: record QA results for v{VERSION}"
git push
```

## Cleaning Up Old QA Branches

After a release is verified, delete old QA branches:

```bash
# Delete local
git branch -D qa/v0.0.71-glibc-minimal

# Delete remote
git push origin --delete qa/v0.0.71-glibc-minimal
```

Keep only QA branches for the current release being tested.

## Quick Reference

| Test Type | Base Image | Key Verification |
|-----------|------------|------------------|
| glibc | debian-12 | Binary runs on older GLIBC |
| fresh-install | ubuntu-22.04 | Install speed, UX flow |
| auth-flows | ubuntu-22.04 | Auth prompts, login commands |

## Anti-Patterns to Avoid

| ❌ Don't | ✅ Do |
|---------|------|
| Copy full repo to QA branch | Create orphan with minimal files |
| Test in development Codespace | Create fresh Codespace on QA branch |
| Skip recording results | Update plan/ROADMAP.md with findings |
| Keep old QA branches | Delete after verification complete |
| Use complex test scripts | Keep tests simple and manual-runnable |
