# Terminal Jarvis Testing Strategy

This document outlines our comprehensive testing approach to ensure core functionality remains stable as the codebase evolves.

## Testing Scripts

### 1. `scripts/smoke-test.sh`

**Quick validation** - Runs in ~5 seconds

- Basic CLI functionality
- Tool listing
- Configuration loading (5 tools present)
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

### 3. `scripts/local-cicd.sh`

**Full CI/CD pipeline** with integrated testing

- Quality checks (fmt, clippy, tests)
- Core functionality validation (calls test-core-functionality.sh)
- Build and packaging
- Version management
- Publishing workflow

```bash
./scripts/local-cicd.sh
```

## Core Functionality Guarantees

Our test suite validates these essential behaviors:

### 1. **Tool Management**

- ✅ All 5 AI tools are available: claude, gemini, qwen, opencode, llxprt
- ✅ All tools use consistent NPM package installation
- ✅ Tool listing shows proper status and requirements
- ✅ Install/update commands work for each tool

### 2. **Configuration System**

- ✅ Default configuration loads properly
- ✅ All tools have NPM install/update commands
- ✅ Example configuration file is maintained
- ✅ Version consistency across Cargo.toml and package.json

### 3. **CLI Interface**

- ✅ Help commands work for all subcommands
- ✅ Error handling for invalid inputs
- ✅ Command structure remains stable

### 4. **Package Management**

- ✅ NPM packages: `@anthropic-ai/claude-code`, `@google/gemini-cli`, `@qwen-code/qwen-code@latest`, `opencode-ai@latest`, `@vybestack/llxprt-code`
- ✅ Concurrent updates work properly
- ✅ Individual tool updates function correctly

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
