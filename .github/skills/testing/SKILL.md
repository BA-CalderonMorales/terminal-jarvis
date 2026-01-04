# Skill: Test-Driven Development

**Name**: testing
**Description**: Test-driven bugfixes and testing practices
**Trigger**: "Fix this bug", bug fixes, new features requiring tests

---

## Test-Driven Bugfix Workflow (MANDATORY)

**ALL bug fixes must follow this workflow:**

1. **Write failing test** - Reproduce the bug with a test that fails
2. **Verify test fails** - Run `cargo test` and confirm failure
3. **Implement fix** - Make minimal changes to fix the bug
4. **Verify test passes** - Run `cargo test` and confirm success
5. **Run quality gates** - clippy, fmt, full test suite
6. **Commit both** - Test and fix together in same commit

## Example Workflow

```bash
# 1. Create test that reproduces bug
# tests/cli_integration_tests.rs
#[test]
fn test_tool_launch_single_enter() {
    let output = launch_tool("claude");
    assert!(output.prompts_count <= 1);  // This will fail if 3 prompts
}

# 2. Verify failure
cargo test test_tool_launch_single_enter  # Should fail

# 3. Implement fix in src/cli_logic/cli_logic_tool_execution.rs

# 4. Verify success
cargo test test_tool_launch_single_enter  # Should pass
cargo test                                # All tests pass

# 5. Commit together
git add tests/cli_integration_tests.rs src/cli_logic/cli_logic_tool_execution.rs
git commit -m "fix(cli): streamline tool launch flow"
```

## Test Structure

| Location | Technology | Purpose | Command |
|----------|------------|---------|---------|
| `tests/` | Rust | Unit and integration tests | `cargo test` |
| `e2e/` | TypeScript (Vitest + cli-testing-library) | End-to-end CLI tests | `cd e2e && npm test` |

## E2E Test Setup

```bash
cd e2e
npm install
npm test          # Run all E2E tests
npm test -- --watch  # Watch mode
```

## Key E2E Test Files

- `e2e/helpers.ts` - CLI rendering and spawn utilities
- `e2e/helpers/` - ANSI parsing, layout validation, width simulation
- `e2e/*.test.ts` - Test suites (help, version, installation, themes, etc.)
