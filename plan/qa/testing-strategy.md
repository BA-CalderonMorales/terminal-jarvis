# QA Testing Strategy

## Overview

Terminal-jarvis uses isolated QA environments to test specific issues
before code changes are made. This ensures we understand the problem
completely before implementing fixes.

## Environment Branches

Each QA environment is an orphan branch containing only:
- Dockerfile with specific OS/config
- Test scripts (bash + Playwright)
- Documentation

| Branch | OS | Purpose |
|--------|-----|---------|
| `qa/env-debian-bookworm` | Debian 12 | GLIBC compatibility |
| `qa/env-fresh-install` | Ubuntu 22.04 | Download speed, UX |
| `qa/env-auth-flows` | Ubuntu 22.04 | Authentication testing |

## Testing Workflow

### 1. Create Environment

```bash
git checkout qa/env-<name>
# Open in GitHub Codespace or local devcontainer
```

### 2. Run Tests

```bash
npm run test:qa  # All tests
npm run test:<specific>  # Specific test
```

### 3. Document Results

Test results saved to `~/qa-results/` as JSON files.

### 4. Report Findings

Create issue comment or update issue with findings.

## Test Types

### Shell Tests

Fast, simple bash scripts for quick validation.

```bash
#!/bin/bash
npx terminal-jarvis@0.0.71 --version
```

### Playwright Tests

TypeScript tests for complex interactions.

```typescript
test('should install successfully', async () => {
  const { stdout } = await exec('npm install -g terminal-jarvis');
  expect(stdout).toContain('terminal-jarvis');
});
```

### Interaction Tests

Using `expect` for simulating user input.

```bash
spawn npx terminal-jarvis
expect "Select tool"
send "claude\r"
```

## Metrics Collection

All tests output JSON metrics:

```json
{
  "test_suite": "download-speed",
  "version": "0.0.71",
  "timestamp": "2026-01-04T08:00:00Z",
  "tests": [
    {"name": "npm_install", "duration_ms": 15000, "passed": true}
  ]
}
```

## CI Integration

Future: Run QA tests in GitHub Actions matrix.

```yaml
jobs:
  qa-test:
    strategy:
      matrix:
        environment: [debian-bookworm, fresh-install, auth-flows]
```
