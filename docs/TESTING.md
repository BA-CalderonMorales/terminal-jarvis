# Terminal Jarvis Testing Strategy

This document outlines our comprehensive testing approach to ensure core functionality remains stable as the codebase evolves.

## Testing Scripts

### 1. `scripts/smoke-test.sh`

**Quick validation** - Runs in ~5 seconds

- Basic CLI functionality
- Tool listing
- Configuration loading (7 tools present)
- NPM package consistency

```bash
./scripts/smoke-test.sh
```

### 2. `scripts/test-core-functionality.sh`

**Comprehensive testing** - Runs in ~15-30 seconds

- All smoke test validations
- Command structure validation (update, install, run, info, templates)
- Error handling verification
- Version consistency across files
- Configuration file integrity
- Tool execution validation

```bash
./scripts/test-core-functionality.sh
```

### 3. `scripts/test-homebrew-formula.sh`

**Homebrew Formula validation** - Multi-platform distribution testing

- Formula syntax and structure validation
- SHA256 checksum verification
- Archive integrity testing
- Ruby syntax checking
- Cross-platform compatibility validation

```bash
./scripts/test-homebrew-formula.sh
```

### 4. `scripts/local-cicd.sh`

**Full CI/CD pipeline** with integrated testing

- Quality checks (fmt, clippy, tests)
- Core functionality validation (calls test-core-functionality.sh)
- Build and packaging
- Version management
- Publishing workflow

```bash
./scripts/local-cicd.sh
```

## Test-Driven Bugfix Workflow

**MANDATORY for ALL Bugfixes**: Terminal Jarvis follows a strict test-driven development approach for bug resolution:

### Required TDD Process

1. **Identify the Bug**: Understand exact problem and reproduction steps
2. **Write Failing Test FIRST**:
   - Create test that reproduces the bug behavior
   - Test MUST fail initially (proving bug exists)
   - Place in appropriate location:
     - `tests/` directory for integration tests
     - `src/` with `#[cfg(test)]` for unit tests
3. **Run Test**: Verify it fails for expected reason
4. **Implement Fix**: Make minimal changes to make test pass
5. **Verify Fix**: Test passes, no regressions
6. **Commit**: Include both test and fix with clear message

### Example Test Structure

```rust
#[test]
fn test_bug_opencode_input_focus_on_fresh_install() {
    // Reproduces issue where opencode input box lacks focus on fresh installs
    // Bug: User cannot type directly without manual focus
    // Expected: Input box should be automatically focused

    // Test implementation here
}
```

### Recent TDD Success Stories

- **OpenCode Input Focus** (v0.0.41): Added `opencode_input_focus_tests.rs` with failing → passing tests
- **Browser Prevention** (v0.0.40): Added `integration_auth_tests.rs` for authentication behavior

## Core Functionality Guarantees

Our test suite validates these essential behaviors:

### 1. **Tool Management**

- All 7 AI tools are available: claude, gemini, qwen, opencode, llxprt, codex, crush
- All tools use consistent NPM package installation
- Tool listing shows proper status and requirements
- Install/update commands work for each tool

### 2. **Configuration System**

- Default configuration loads properly
- All tools have NPM install/update commands
- Example configuration file is maintained
- Version consistency across Cargo.toml and package.json

### 3. **CLI Interface**

- Help commands work for all subcommands
- Error handling for invalid inputs
- Command structure remains stable

### 4. **Package Management**

- NPM packages: `@anthropic-ai/claude-code`, `@google/gemini-cli`, `@qwen-code/qwen-code@latest`, `opencode-ai@latest`, `@vybestack/llxprt-code`
- Concurrent updates work properly
- Individual tool updates function correctly

### 5. **Authentication & Environment Management**

- Browser opening prevention in headless/CI environments
- Environment detection (CI, Codespaces, SSH, containers)
- API key validation and guidance for Gemini CLI and Qwen Code
- Authentication behavior integration testing with real tool scenarios
- Regression tests to prevent browser opening in terminal environments

### 6. **Terminal State & Tool Integration**

- OpenCode input focus works immediately on fresh installs
- Terminal state preparation doesn't interfere with tool initialization
- Minimal terminal clearing sequences prevent race conditions
- Tool-specific launch optimizations (initialization delays, state management)

### 5. **Multi-Platform Distribution** (v0.0.47+)

- Homebrew Formula syntax validation
- Cross-platform archive creation (macOS/Linux)
- SHA256 checksum verification
- Binary permissions preservation in archives
- Local tap testing workflow validation

## Homebrew Testing Infrastructure

### Local Formula Testing

**Complete end-to-end validation** without requiring GitHub releases:

```bash
# 1. Validate Formula structure and syntax
./scripts/test-homebrew-formula.sh

# 2. Create local tap for testing
mkdir -p /tmp/homebrew-test-tap/Formula
cp homebrew/Formula/terminal-jarvis.rb /tmp/homebrew-test-tap/Formula/
cd /tmp/homebrew-test-tap && git init && git add . && git commit -m "Test"

# 3. Install via local tap
brew tap local/test /tmp/homebrew-test-tap
brew install local/test/terminal-jarvis

# 4. Test functionality
terminal-jarvis --version
brew test local/test/terminal-jarvis
```

### Archive Testing

**Validates multi-platform release artifacts**:

- Creates platform-specific archives: `terminal-jarvis-{macos|linux}.tar.gz`
- Verifies SHA256 checksums match Formula expectations
- Tests binary extraction and permissions
- Validates Formula URL structure for GitHub releases

## Integration with Development Workflow

### Pre-commit Testing

```bash
# Quick validation before commits
./scripts/smoke-test.sh
```

### Pre-release Testing

```bash
# Comprehensive validation before releases
./scripts/test-core-functionality.sh
```

### Full Release Pipeline

```bash
# Complete CI/CD with testing
./scripts/local-cicd.sh
```

## Regression Prevention Strategy

1. **Automated Testing**: All core functionality is validated automatically
2. **Version Consistency**: Prevents mismatched versions across files
3. **Configuration Validation**: Ensures example configs stay in sync
4. **Package Management**: Validates NPM package consistency
5. **CLI Stability**: Protects against breaking command structure changes

## Adding New Tests

When adding new functionality, extend `scripts/test-core-functionality.sh`:

```bash
# Add new test
run_test "Test N: New feature description" \
    "command_to_test_new_feature"
```

This ensures that new features are protected against future regressions.

## Test Philosophy

- **Fast Feedback**: Smoke tests provide quick validation
- **Comprehensive Coverage**: Core functionality tests catch regressions
- **Developer Friendly**: Clear pass/fail indicators with helpful error messages
- **CI/CD Integration**: Automated testing prevents broken releases
- **Maintainable**: Tests are documented and easy to extend

This testing strategy allows developers to:

- Code freely without fear of breaking existing functionality
- Get immediate feedback on core behavior
- Ensure consistent package management across all tools
- Maintain stable CLI interfaces
- Deliver reliable releases to users
