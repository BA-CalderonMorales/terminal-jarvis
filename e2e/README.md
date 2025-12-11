# Terminal Jarvis CLI Testing

This directory contains end-to-end CLI tests for terminal-jarvis using [cli-testing-library](https://github.com/crutchcorn/cli-testing-library).

## Overview

cli-testing-library enables testing of CLI applications by spawning actual processes and interacting with them the way users do. This provides high confidence that the CLI behaves correctly in real-world usage scenarios.

## Test Structure

```
tests/
├── README.md              # This file
├── setup.ts              # Vitest setup with cli-testing-library matchers
├── helpers.ts            # Shared testing utilities
├── help.test.ts          # Help command tests
├── version.test.ts       # Version flag tests
├── npm-pack.test.ts      # Package installation tests
└── error-handling.test.ts # Error scenario tests
```

## Running Tests

### Prerequisites

Build the Rust binary before running tests:

```bash
npm run build-rust
# or from project root:
cargo build --release
```

**Important: Testing with Cargo**

When testing manually with Cargo, always use the `--` separator to pass flags to your application:

```bash
# CORRECT - Tests your application's help
cargo run -- --help
./target/debug/terminal-jarvis --help
./target/release/terminal-jarvis --help

# INCORRECT - Shows Cargo's help, not your app's help
cargo run --help  # This shows Cargo's help!
```

The `--` tells Cargo: "everything after this goes to the application, not to Cargo."

### Test Commands

```bash
# Run all tests once
npm test

# Watch mode for development
npm run test:watch

# Interactive UI (recommended for debugging)
npm run test:ui

# Generate coverage report
npm run test:coverage

# Build Rust + run tests
npm run test:e2e
```

## Writing Tests

### Basic Pattern

```typescript
import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath } from './helpers';

test('displays help text', async () => {
  const { findByText } = await render(getBinaryPath(), ['--help']);

  expect(await findByText('Usage:')).toBeInTheConsole();
});
```

### Testing with npm pack

The `npm-pack.test.ts` suite tests the actual packaged distribution:

```typescript
import { createPackageTestEnvironment } from './helpers';

let ctx: PackageTestContext;

beforeAll(() => {
  ctx = createPackageTestEnvironment();
});

afterAll(() => {
  ctx.cleanup();
});

test('runs from installed package', async () => {
  const { findByText } = await render('npx', ['terminal-jarvis', '--version'], {
    cwd: ctx.testDir
  });

  expect(await findByText(/\d+\.\d+\.\d+/)).toBeInTheConsole();
});
```

### Available Queries

cli-testing-library provides Testing Library-style queries:

- `findByText(pattern)` - Async query for stdout text
- `findByError(pattern)` - Async query for stderr text
- `queryByText(pattern)` - Sync query for stdout (null if not found)
- `queryByError(pattern)` - Sync query for stderr (null if not found)
- `getByText(pattern)` - Sync query for stdout (throws if not found)

### Helper Utilities

#### `getBinaryPath()`

Returns path to the compiled Rust binary:

```typescript
const binaryPath = getBinaryPath();
const { findByText } = await render(binaryPath, ['--help']);
```

#### `createPackageTestEnvironment()`

Sets up an isolated test environment with the npm package installed:

```typescript
const ctx = createPackageTestEnvironment();
// ctx.testDir - temporary directory with package installed
// ctx.packagePath - path to .tgz file
// ctx.version - package version
// ctx.cleanup() - cleanup function
```

#### `normalizeOutput(output)`

Sanitizes CLI output for cross-platform testing:

```typescript
const output = normalizeOutput(instance.getStdallStr());
expect(output).toContain('expected text');
```

#### `sanitizeVersion(version)`

Normalizes version strings for comparison:

```typescript
const version = sanitizeVersion('v1.2.3'); // Returns '1.2.3'
```

#### `arrowSymbol()`

Cross-platform regex for arrow symbols:

```typescript
// Matches both Unix "❯" and Windows ">"
expect(await findByText(arrowSymbol())).toBeInTheConsole();
```

### Testing Interactive CLIs

For testing interactive prompts and user input:

```typescript
test('interactive navigation', async () => {
  const { findByText, userEvent, clear } = await render(getBinaryPath(), []);

  // Wait for prompt
  await findByText('Select an option:');

  // Clear buffer for cleaner assertions
  clear();

  // Simulate keyboard input
  userEvent.keyboard('[ArrowDown]');
  userEvent.keyboard('[Enter]');

  // Verify selection
  expect(await findByText('You selected:')).toBeInTheConsole();
});
```

### Testing Exit Codes

```typescript
test('exits with error code on failure', async () => {
  const instance = await render(getBinaryPath(), ['invalid-command']);

  await new Promise(resolve => setTimeout(resolve, 1000));

  const exitInfo = instance.hasExit();
  expect(exitInfo).toBeTruthy();
  if (exitInfo) {
    expect(exitInfo.exitCode).not.toBe(0);
  }
});
```

## Testing Best Practices

### 1. Use Async Queries

Prefer `findByText` over `queryByText` for waiting on output:

```typescript
// GOOD - waits for text to appear
expect(await findByText('Success')).toBeInTheConsole();

// AVOID - may fail if output is slow
expect(queryByText('Success')).toBeInTheConsole();
```

### 2. Clear Output Buffer

Use `clear()` when testing sequential interactions:

```typescript
await findByText('First prompt');
clear(); // Clear buffer
userEvent.keyboard('[Enter]');
await findByText('Second prompt'); // Only searches new output
```

### 3. Add Appropriate Timeouts

Long-running commands need extended timeouts:

```typescript
test('handles slow operations', async () => {
  const { findByText } = await render(getBinaryPath(), ['slow-command']);

  await findByText('Complete', {}, { timeout: 10000 }); // 10s timeout
}, 15000); // Test timeout must be longer
```

### 4. Cross-Platform Considerations

- Use regex patterns for platform-specific symbols
- Test on Linux (CI uses Linux by default)
- Use `normalizeOutput()` for consistent assertions
- Set `NO_COLOR=1` environment variable (done in setup.ts)

### 5. Test Real Package Installation

The `npm-pack.test.ts` suite validates the actual distribution:

- Ensures all files are included in package
- Tests postinstall scripts
- Validates binary execution from node_modules
- Simulates real user installation

## Continuous Integration

Tests run automatically in CI via `.github/workflows/`. The CI:

1. Builds the Rust binary with `cargo build --release`
2. Installs Node.js dependencies
3. Runs `npm run test:e2e`

## Troubleshooting

### "Binary not found" Error

Ensure you've built the Rust binary:

```bash
cargo build --release
```

### TypeScript Errors

Run type checking:

```bash
npm run typecheck
```

### Tests Hang

- Check for processes that don't exit
- Ensure interactive prompts have proper input
- Add debug output: `instance.debug()`

### Platform-Specific Failures

- Use `arrowSymbol()` for cross-platform symbol matching
- Use `normalizeOutput()` to handle line endings
- Check ANSI color codes are stripped

## Resources

- [cli-testing-library Documentation](https://github.com/crutchcorn/cli-testing-library)
- [Vitest Documentation](https://vitest.dev/)
- [Testing Library Philosophy](https://testing-library.com/docs/guiding-principles/)

## Contributing

When adding new CLI features, ensure they have corresponding tests:

1. Add test file in `tests/`
2. Use descriptive test names
3. Test both success and error paths
4. Include tests in `npm-pack.test.ts` if behavior affects package distribution
