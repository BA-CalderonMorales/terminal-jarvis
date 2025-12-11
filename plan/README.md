# Terminal Jarvis Development Plan

This folder contains phased development plans for future agent sessions. Each phase builds on the previous one to achieve a seamless, professional CLI experience.

## Quick Navigation

### Tactical Phases (Current Sprint)

| Phase | Focus | Status |
|-------|-------|--------|
| [Phase 1](phase-1-ux-streamlining.md) | UX Streamlining | COMPLETED |
| [Phase 2](phase-2-voice-feature.md) | Voice Feature | DEFERRED |
| [Phase 3](phase-3-testing-quality.md) | Testing & Quality | PENDING |
| [Phase 4](phase-4-distribution.md) | Distribution & Release | PENDING |

### Strategic Phases (Product Vision)

| Phase | Focus | Status |
|-------|-------|--------|
| [Phase 5](phase-5-first-run-magic.md) | First-Run Experience | PENDING |
| [Phase 6](phase-6-release-automation.md) | Release Automation | PENDING |
| [Phase 7](phase-7-navigation-hardening.md) | Navigation Hardening | PENDING |
| [Phase 8](phase-8-credential-intelligence.md) | Credential Intelligence | PENDING |
| [Phase 9](phase-9-ecosystem-growth.md) | Ecosystem Growth | PENDING |

See [STRATEGIC-ROADMAP.md](STRATEGIC-ROADMAP.md) for the product vision and guiding principles.

## Current State (2025-12-11)

### Completed This Session
- Merged 3 dependency PRs (bump cli-testing-library, vitest, zod)
- Consolidated CLAUDE.md into AGENTS.md (single source of truth)
- Reorganized E2E tests from npm/terminal-jarvis/tests/ to e2e/ at root
- All 38 E2E tests passing
- UX improvements:
  - Removed /voice from menu (deferred feature)
  - Simplified startup guidance (only show tips when API keys missing)
  - Removed "Press Enter to continue" pause before tool launch
  - Masked API key input with asterisks (security)
  - Added "Re-enter API Key" option to post-session menu
  - Added "Uninstall" option to post-session menu

### Known Issues
- 3 voice module tests failing (pre-existing, unrelated to changes)
- Voice feature deferred until audio hardware available

## How to Use This Plan

1. Start each session by reading the relevant phase document
2. Check the "Prerequisites" section before beginning work
3. Follow the "Agent Instructions" for clear task breakdown
4. Update the phase status when complete
