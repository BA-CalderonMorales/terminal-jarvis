/**
 * Benchmark Testing Helpers
 * 
 * TypeScript validation and utilities for Rust-exported benchmark results.
 * Ensures JSON bridge compatibility between Rust benchmarks and TypeScript tests.
 */

import { z } from 'zod';
import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';

/**
 * Zod schema for TestCaseResult
 * 
 * Matches Rust struct:
 * ```rust
 * pub struct TestCaseResult {
 *     pub test_name: String,
 *     pub passed: bool,
 *     pub expected: String,
 *     pub actual: String,
 *     pub error: Option<String>,
 * }
 * ```
 */
export const TestCaseResultSchema = z.object({
  test_name: z.string(),
  passed: z.boolean(),
  expected: z.string(),
  actual: z.string(),
  error: z.string().optional().nullable(),
});

/**
 * Zod schema for ValidationResult
 * 
 * Matches Rust struct:
 * ```rust
 * pub struct ValidationResult {
 *     pub passed: bool,
 *     pub score: f64,
 *     pub test_case_results: Vec<TestCaseResult>,
 *     pub errors: Vec<String>,
 * }
 * ```
 */
export const ValidationResultSchema = z.object({
  passed: z.boolean(),
  score: z.number().min(0).max(10),
  test_case_results: z.array(TestCaseResultSchema),
  errors: z.array(z.string()),
});

/**
 * Zod schema for BenchmarkResult
 * 
 * Matches Rust struct:
 * ```rust
 * pub struct BenchmarkResult {
 *     pub benchmark_id: String,
 *     pub tool_name: String,
 *     pub scenario_version: String,
 *     pub execution_timestamp: String,
 *     pub execution_time_ms: u64,
 *     pub passed: bool,
 *     pub score: f64,
 *     pub output: String,
 *     pub validation_details: ValidationResult,
 * }
 * ```
 */
export const BenchmarkResultSchema = z.object({
  benchmark_id: z.string(),
  tool_name: z.string(),
  scenario_version: z.string(),
  execution_timestamp: z.string(),
  execution_time_ms: z.number().int().nonnegative(),
  passed: z.boolean(),
  score: z.number().min(0).max(10),
  output: z.string(),
  validation_details: ValidationResultSchema,
});

// TypeScript types derived from schemas
export type TestCaseResult = z.infer<typeof TestCaseResultSchema>;
export type ValidationResult = z.infer<typeof ValidationResultSchema>;
export type BenchmarkResult = z.infer<typeof BenchmarkResultSchema>;

/**
 * Validation error with detailed context
 */
export interface BenchmarkValidationError {
  valid: false;
  errors: z.ZodIssue[];
  message: string;
}

/**
 * Successful validation result
 */
export interface BenchmarkValidationSuccess {
  valid: true;
  data: BenchmarkResult;
}

export type BenchmarkValidation = BenchmarkValidationSuccess | BenchmarkValidationError;

/**
 * Load and parse a benchmark result from a JSON file
 * 
 * @param filePath - Absolute or relative path to the benchmark result JSON file
 * @returns Parsed BenchmarkResult
 * @throws Error if file doesn't exist or JSON is invalid
 * 
 * @example
 * ```typescript
 * const result = loadBenchmarkResult('./benchmark-results/code-completion-001.json');
 * console.log(result.benchmark_id); // "code-completion-basic-001"
 * ```
 */
export function loadBenchmarkResult(filePath: string): BenchmarkResult {
  const absolutePath = resolve(filePath);
  
  if (!existsSync(absolutePath)) {
    throw new Error(`Benchmark result file not found: ${absolutePath}`);
  }

  try {
    const fileContent = readFileSync(absolutePath, 'utf-8');
    const jsonData = JSON.parse(fileContent);
    
    // Validate and parse with Zod
    const result = BenchmarkResultSchema.parse(jsonData);
    return result;
  } catch (error) {
    if (error instanceof z.ZodError) {
      throw new Error(`Invalid benchmark result schema: ${error.message}`);
    }
    throw new Error(`Failed to load benchmark result: ${error}`);
  }
}

/**
 * Validate a benchmark result object against the schema
 * 
 * @param data - Unknown data to validate
 * @returns Validation result with typed data or detailed errors
 * 
 * @example
 * ```typescript
 * const validation = validateBenchmarkResult(jsonData);
 * if (validation.valid) {
 *   console.log(`Score: ${validation.data.score}`);
 * } else {
 *   console.error(validation.message);
 *   validation.errors.forEach(err => console.error(err));
 * }
 * ```
 */
export function validateBenchmarkResult(data: unknown): BenchmarkValidation {
  const result = BenchmarkResultSchema.safeParse(data);
  
  if (result.success) {
    return {
      valid: true,
      data: result.data,
    };
  }

  return {
    valid: false,
    errors: result.error.issues,
    message: formatValidationErrors(result.error.issues),
  };
}

/**
 * Format Zod validation errors into a human-readable message
 */
function formatValidationErrors(errors: z.ZodIssue[]): string {
  const errorMessages = errors.map(err => {
    const path = err.path.join('.');
    return `  - ${path}: ${err.message}`;
  });
  
  return `Benchmark validation failed:\n${errorMessages.join('\n')}`;
}

/**
 * Create a mock benchmark result for testing
 * 
 * @param overrides - Partial fields to override defaults
 * @returns Valid BenchmarkResult for testing
 * 
 * @example
 * ```typescript
 * const mockResult = createMockBenchmarkResult({
 *   benchmark_id: 'test-001',
 *   passed: false,
 *   score: 5.0,
 * });
 * ```
 */
export function createMockBenchmarkResult(
  overrides: Partial<BenchmarkResult> = {}
): BenchmarkResult {
  const defaultResult: BenchmarkResult = {
    benchmark_id: 'mock-benchmark-001',
    tool_name: 'claude',
    scenario_version: '1.0.0',
    execution_timestamp: new Date().toISOString(),
    execution_time_ms: 1234,
    passed: true,
    score: 9.5,
    output: 'fn add(a: i32, b: i32) -> i32 { a + b }',
    validation_details: {
      passed: true,
      score: 9.5,
      test_case_results: [
        {
          test_name: 'syntax_validity',
          passed: true,
          expected: 'valid Rust function',
          actual: 'valid Rust function',
          error: null,
        },
      ],
      errors: [],
    },
  };

  return { ...defaultResult, ...overrides };
}

/**
 * Validate that a benchmark result matches expected criteria
 * 
 * @param result - Benchmark result to check
 * @param criteria - Expected values to validate against
 * @returns True if all criteria match
 * 
 * @example
 * ```typescript
 * const matches = matchesCriteria(result, {
 *   benchmark_id: 'code-completion-001',
 *   passed: true,
 *   minScore: 8.0,
 * });
 * ```
 */
export function matchesCriteria(
  result: BenchmarkResult,
  criteria: {
    benchmark_id?: string;
    tool_name?: string;
    passed?: boolean;
    minScore?: number;
    maxExecutionTimeMs?: number;
  }
): boolean {
  if (criteria.benchmark_id !== undefined && result.benchmark_id !== criteria.benchmark_id) {
    return false;
  }
  
  if (criteria.tool_name !== undefined && result.tool_name !== criteria.tool_name) {
    return false;
  }
  
  if (criteria.passed !== undefined && result.passed !== criteria.passed) {
    return false;
  }
  
  if (criteria.minScore !== undefined && result.score < criteria.minScore) {
    return false;
  }
  
  if (criteria.maxExecutionTimeMs !== undefined && result.execution_time_ms > criteria.maxExecutionTimeMs) {
    return false;
  }
  
  return true;
}
