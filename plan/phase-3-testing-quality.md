# Phase 3: Testing & Quality

**Status**: PENDING

## Objective

Ensure comprehensive test coverage and code quality across the entire codebase.

## Prerequisites

- Phase 1 completed
- All clippy warnings resolved

## Tasks

### 1. Fix Remaining Test Failures
- [ ] Voice module tests (3 failing) - see Phase 2
- [ ] Run full test suite: `cargo test --lib`
- [ ] Ensure all 169+ non-voice tests continue passing

### 2. E2E Test Expansion
- [ ] Add E2E tests for new post-session options (uninstall, re-enter API key)
- [ ] Add E2E tests for password-masked input
- [ ] Test tool launch flow without prompts

**Location**: `e2e/`

### 3. Integration Test Coverage
- [ ] Add tests for AuthManager credential flow
- [ ] Add tests for tool uninstall functionality
- [ ] Add tests for startup guidance conditions

**Location**: `tests/`

### 4. Code Quality Sweep
- [ ] Run clippy with all features: `cargo clippy --all-features -- -D warnings`
- [ ] Ensure no dead code warnings
- [ ] Review and document any `#[allow(...)]` directives

### 5. Documentation Quality
- [ ] Update README.md if any user-facing changes
- [ ] Ensure CHANGELOG.md is up-to-date
- [ ] Review inline documentation for accuracy

## Agent Instructions

When starting this phase:

1. Run the full test suite:
   ```bash
   cargo test --lib -- --skip voice
   cargo test voice 2>&1 | tail -20
   ```

2. Check E2E tests:
   ```bash
   cd e2e && npm test
   ```

3. Run quality gates:
   ```bash
   cargo clippy -- -D warnings
   cargo fmt --all --check
   ```

4. Address any failures before adding new tests

5. When adding new tests, follow TDD:
   - Write failing test first
   - Implement fix
   - Verify test passes
