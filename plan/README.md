# Terminal Jarvis Strategic Plan

This branch contains ONLY planning documentation for terminal-jarvis development.

## Current Version: 0.0.71

## Open Issues

| Issue | Title | Priority | Status |
|-------|-------|----------|--------|
| #24 | GLIBC 2.39 hard dependency | High | QA Environment Ready |
| #23 | Download speed needs improvement | Medium | QA Environment Ready |
| #26 | Too many steps to launch tool | Medium | QA Environment Ready |
| #27 | Codex/Gemini auth flow issues | Medium | QA Environment Ready |
| #31 | Database layer integration | Low | Planning |

## QA Testing Environments

Three isolated QA branches for testing:

| Branch | Purpose | Issues Tested |
|--------|---------|---------------|
| `qa/env-debian-bookworm` | GLIBC compatibility | #24 |
| `qa/env-fresh-install` | Download speed, UX | #23, #26 |
| `qa/env-auth-flows` | Authentication workflows | #27 |

## Plan Documents

- [ROADMAP.md](ROADMAP.md) - Strategic development phases
- [issues/](issues/) - Detailed issue analysis and solutions
- [qa/](qa/) - QA testing strategies

## Navigation

```
plan/
  README.md              # This file
  ROADMAP.md             # Development phases
  issues/
    issue-23-download-speed.md
    issue-24-glibc-compatibility.md
    issue-26-ux-streamlining.md
    issue-27-auth-flows.md
    issue-31-database-layer.md
  qa/
    testing-strategy.md
    environments.md
```
