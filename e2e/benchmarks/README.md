# Benchmark JSON Bridge Tests

TypeScript validation layer for Terminal Jarvis benchmark results exported from Rust.

## Overview

This test suite validates the JSON bridge between Rust benchmark execution and TypeScript analysis. It ensures schema compatibility and proper error handling for benchmark results.

## Architecture

```
Rust Benchmarks (src/evals/benchmarks/)
    |
    | JSON Export
    V
TypeScript Validators (tests/helpers/benchmark-helpers.ts)
    |
    | Schema Validation (Zod)
    V
Test Suite (tests/benchmarks/json-bridge.test.ts)
```

## JSON Schema

Benchmark results follow this structure (matching Rust `BenchmarkResult` struct):

```typescript
{
  benchmark_id: string;           // e.g., "code-completion-basic-001"
  tool_name: string;              // e.g., "claude", "cursor", "copilot"
  scenario_version: string;       // e.g., "1.0.0"
  execution_timestamp: string;    // ISO 8601: "2025-10-05T01:30:00Z"
  execution_time_ms: number;      // Unsigned integer
  passed: boolean;                // Overall pass/fail
  score: number;                  // 0.0 - 10.0
  output: string;                 // Tool's generated output
  validation_details: {
    passed: boolean;
    score: number;                // 0.0 - 10.0
    test_case_results: [
      {
        test_name: string;
        passed: boolean;
        expected: string;
        actual: string;
        error?: string;           // Optional
      }
    ];
    errors: string[];
  }
}
```

## Usage

### Loading Benchmark Results

```typescript
import { loadBenchmarkResult } from '../helpers/benchmark-helpers';

// Load and validate from file
const result = loadBenchmarkResult('./benchmark-results/code-completion-001.json');
console.log(result.score); // 9.5
```

### Validating Data

```typescript
import { validateBenchmarkResult } from '../helpers/benchmark-helpers';

const validation = validateBenchmarkResult(jsonData);

if (validation.valid) {
  console.log(`Benchmark ${validation.data.benchmark_id} passed with score ${validation.data.score}`);
} else {
  console.error('Validation errors:', validation.errors);
}
```

### Creating Mock Data

```typescript
import { createMockBenchmarkResult } from '../helpers/benchmark-helpers';

const mockResult = createMockBenchmarkResult({
  benchmark_id: 'test-001',
  tool_name: 'claude',
  passed: true,
  score: 9.0,
});
```

### Criteria Matching

```typescript
import { matchesCriteria } from '../helpers/benchmark-helpers';

const passes = matchesCriteria(result, {
  benchmark_id: 'code-completion-001',
  tool_name: 'claude',
  passed: true,
  minScore: 8.0,
  maxExecutionTimeMs: 2000,
});
```

## Test Coverage

The test suite validates:

1. **Schema Validation**
   - Complete benchmark results with all fields
   - Optional fields (error in TestCaseResult)
   - Nested validation results
   - Empty test case arrays

2. **Invalid Schema Detection**
   - Missing required fields
   - Invalid score ranges (must be 0.0-10.0)
   - Negative execution times
   - Wrong field types

3. **File Loading**
   - Loading Rust-exported JSON
   - Non-existent file handling
   - Malformed JSON detection
   - Schema validation errors

4. **Helper Functions**
   - Success/error validation results
   - Human-readable error formatting
   - Criteria matching (id, tool, score, time)
   - Mock data generation

## Running Tests

```bash
# Run all benchmark tests
npm test -- benchmarks/json-bridge

# Run with coverage
npm run test:coverage -- benchmarks/json-bridge

# Watch mode
npm run test:watch -- benchmarks/json-bridge
```

## Integration with Rust

The TypeScript schemas **exactly match** the Rust structs in `/workspaces/terminal-jarvis/src/evals/benchmarks/results.rs`:

- `BenchmarkResult` → `BenchmarkResultSchema`
- `ValidationResult` → `ValidationResultSchema`
- `TestCaseResult` → `TestCaseResultSchema`

Field names, types, and optionality are synchronized to ensure compatibility.

## Example: Full Integration

```typescript
import { describe, test, expect } from 'vitest';
import { loadBenchmarkResult, matchesCriteria } from '../helpers/benchmark-helpers';

test('validates Claude code completion benchmark', () => {
  // Load result exported from Rust benchmark
  const result = loadBenchmarkResult('./benchmark-results/claude-code-001.json');
  
  // Verify benchmark passed with good score
  expect(matchesCriteria(result, {
    tool_name: 'claude',
    passed: true,
    minScore: 8.0,
  })).toBe(true);
  
  // Check specific validation details
  expect(result.validation_details.test_case_results).toHaveLength(3);
  expect(result.validation_details.errors).toHaveLength(0);
});
```

## Error Handling

All validation errors include detailed context:

```typescript
const validation = validateBenchmarkResult(invalidData);

if (!validation.valid) {
  console.log(validation.message);
  // Output:
  // Benchmark validation failed:
  //   - scenario_version: Required
  //   - score: Number must be less than or equal to 10
  //   - execution_time_ms: Expected number, received string
}
```

## Dependencies

- **zod** (^4.1.11): Schema validation
- **vitest** (^3.2.4): Test framework
- **Node.js fs/path**: File operations

## Test Results

```
[SUCCESS] 24/24 tests passing

Schema Validation:           4 tests
Invalid Schema Detection:    4 tests  
File Loading and Parsing:    4 tests
Validation Helper Functions: 3 tests
Criteria Matching:           6 tests
Mock Data Generation:        3 tests
```
