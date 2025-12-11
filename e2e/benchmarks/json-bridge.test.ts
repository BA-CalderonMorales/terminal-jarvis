/**
 * JSON Bridge Validation Tests
 * 
 * Tests TypeScript validation layer for Rust-exported benchmark results.
 * Ensures schema compatibility and proper error handling.
 */

import { describe, test, expect, beforeEach, afterEach } from 'vitest';
import { writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import {
  BenchmarkResultSchema,
  ValidationResultSchema,
  TestCaseResultSchema,
  loadBenchmarkResult,
  validateBenchmarkResult,
  createMockBenchmarkResult,
  matchesCriteria,
  type BenchmarkResult,
} from '../helpers/benchmark-helpers';

describe('Benchmark JSON Bridge Validation', () => {
  let testDir: string;

  beforeEach(() => {
    // Create temporary directory for test files
    testDir = mkdtempSync(join(tmpdir(), 'benchmark-test-'));
  });

  afterEach(() => {
    // Cleanup temporary directory
    if (testDir) {
      rmSync(testDir, { recursive: true, force: true });
    }
  });

  describe('Schema Validation', () => {
    test('validates complete benchmark result with all required fields', () => {
      const validResult: BenchmarkResult = {
        benchmark_id: 'code-completion-basic-001',
        tool_name: 'claude',
        scenario_version: '1.0.0',
        execution_timestamp: '2025-10-05T01:30:00Z',
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
              expected: 'valid Rust syntax',
              actual: 'valid Rust syntax',
              error: null,
            },
          ],
          errors: [],
        },
      };

      const result = BenchmarkResultSchema.safeParse(validResult);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.benchmark_id).toBe('code-completion-basic-001');
        expect(result.data.score).toBe(9.5);
        expect(result.data.passed).toBe(true);
      }
    });

    test('validates test case result with optional error field', () => {
      const testCaseWithError = {
        test_name: 'compilation_check',
        passed: false,
        expected: 'compiles without errors',
        actual: 'compilation failed',
        error: 'syntax error on line 5',
      };

      const result = TestCaseResultSchema.safeParse(testCaseWithError);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.error).toBe('syntax error on line 5');
      }
    });

    test('validates test case result without error field', () => {
      const testCaseWithoutError = {
        test_name: 'syntax_validity',
        passed: true,
        expected: 'valid syntax',
        actual: 'valid syntax',
      };

      const result = TestCaseResultSchema.safeParse(testCaseWithoutError);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.error).toBeUndefined();
      }
    });

    test('validates validation result with empty test cases', () => {
      const validationResult = {
        passed: false,
        score: 0.0,
        test_case_results: [],
        errors: ['No output generated'],
      };

      const result = ValidationResultSchema.safeParse(validationResult);
      expect(result.success).toBe(true);
      if (result.success) {
        expect(result.data.test_case_results).toHaveLength(0);
        expect(result.data.errors).toContain('No output generated');
      }
    });
  });

  describe('Invalid Schema Detection', () => {
    test('rejects benchmark result with missing required fields', () => {
      const invalidResult = {
        benchmark_id: 'test-001',
        tool_name: 'claude',
        // Missing: scenario_version, execution_timestamp, etc.
      };

      const result = BenchmarkResultSchema.safeParse(invalidResult);
      expect(result.success).toBe(false);
      if (!result.success) {
        const errorPaths = result.error.issues.map((e) => e.path.join('.'));
        expect(errorPaths).toContain('scenario_version');
        expect(errorPaths).toContain('execution_timestamp');
        expect(errorPaths).toContain('execution_time_ms');
      }
    });

    test('rejects score outside valid range (0.0 - 10.0)', () => {
      const invalidScore = {
        benchmark_id: 'test-001',
        tool_name: 'claude',
        scenario_version: '1.0.0',
        execution_timestamp: '2025-10-05T01:30:00Z',
        execution_time_ms: 1000,
        passed: true,
        score: 11.5, // Invalid: exceeds max
        output: 'test output',
        validation_details: {
          passed: true,
          score: 9.5,
          test_case_results: [],
          errors: [],
        },
      };

      const result = BenchmarkResultSchema.safeParse(invalidScore);
      expect(result.success).toBe(false);
      if (!result.success) {
        const scoreError = result.error.issues.find((e) => e.path.includes('score'));
        expect(scoreError).toBeDefined();
      }
    });

    test('rejects negative execution time', () => {
      const invalidTime = {
        benchmark_id: 'test-001',
        tool_name: 'claude',
        scenario_version: '1.0.0',
        execution_timestamp: '2025-10-05T01:30:00Z',
        execution_time_ms: -100, // Invalid: negative
        passed: true,
        score: 9.5,
        output: 'test output',
        validation_details: {
          passed: true,
          score: 9.5,
          test_case_results: [],
          errors: [],
        },
      };

      const result = BenchmarkResultSchema.safeParse(invalidTime);
      expect(result.success).toBe(false);
    });

    test('rejects invalid field types', () => {
      const invalidTypes = {
        benchmark_id: 123, // Should be string
        tool_name: 'claude',
        scenario_version: '1.0.0',
        execution_timestamp: '2025-10-05T01:30:00Z',
        execution_time_ms: '1000', // Should be number
        passed: 'true', // Should be boolean
        score: '9.5', // Should be number
        output: 'test output',
        validation_details: {
          passed: true,
          score: 9.5,
          test_case_results: [],
          errors: [],
        },
      };

      const result = BenchmarkResultSchema.safeParse(invalidTypes);
      expect(result.success).toBe(false);
      if (!result.success) {
        expect(result.error.issues.length).toBeGreaterThan(0);
      }
    });
  });

  describe('File Loading and Parsing', () => {
    test('loads Rust-exported benchmark result from JSON file', () => {
      const mockResult = createMockBenchmarkResult({
        benchmark_id: 'file-load-test-001',
        tool_name: 'cursor',
        score: 8.5,
      });

      const filePath = join(testDir, 'benchmark-result.json');
      writeFileSync(filePath, JSON.stringify(mockResult, null, 2));

      const loaded = loadBenchmarkResult(filePath);

      expect(loaded.benchmark_id).toBe('file-load-test-001');
      expect(loaded.tool_name).toBe('cursor');
      expect(loaded.score).toBe(8.5);
      expect(loaded.validation_details).toBeDefined();
    });

    test('throws error when file does not exist', () => {
      const nonExistentPath = join(testDir, 'does-not-exist.json');

      expect(() => loadBenchmarkResult(nonExistentPath)).toThrow(
        /Benchmark result file not found/
      );
    });

    test('throws error when JSON is malformed', () => {
      const filePath = join(testDir, 'malformed.json');
      writeFileSync(filePath, '{ invalid json }');

      expect(() => loadBenchmarkResult(filePath)).toThrow(
        /Failed to load benchmark result/
      );
    });

    test('throws error when JSON schema is invalid', () => {
      const invalidResult = {
        benchmark_id: 'test-001',
        // Missing required fields
      };

      const filePath = join(testDir, 'invalid-schema.json');
      writeFileSync(filePath, JSON.stringify(invalidResult));

      expect(() => loadBenchmarkResult(filePath)).toThrow(
        /Invalid benchmark result schema/
      );
    });
  });

  describe('Validation Helper Functions', () => {
    test('validateBenchmarkResult returns success for valid data', () => {
      const validData = createMockBenchmarkResult({
        benchmark_id: 'validation-test-001',
      });

      const validation = validateBenchmarkResult(validData);

      expect(validation.valid).toBe(true);
      if (validation.valid) {
        expect(validation.data.benchmark_id).toBe('validation-test-001');
      }
    });

    test('validateBenchmarkResult returns errors for invalid data', () => {
      const invalidData = {
        benchmark_id: 'test-001',
        // Missing required fields
      };

      const validation = validateBenchmarkResult(invalidData);

      expect(validation.valid).toBe(false);
      if (!validation.valid) {
        expect(validation.errors.length).toBeGreaterThan(0);
        expect(validation.message).toContain('Benchmark validation failed');
      }
    });

    test('formatValidationErrors provides human-readable messages', () => {
      const invalidData = {
        benchmark_id: 123, // Wrong type
        score: 15, // Out of range
      };

      const validation = validateBenchmarkResult(invalidData);

      expect(validation.valid).toBe(false);
      if (!validation.valid) {
        expect(validation.message).toContain('benchmark_id');
        expect(validation.message).toContain('score');
      }
    });
  });

  describe('Criteria Matching', () => {
    test('matchesCriteria validates benchmark_id', () => {
      const result = createMockBenchmarkResult({
        benchmark_id: 'code-completion-001',
      });

      expect(matchesCriteria(result, { benchmark_id: 'code-completion-001' })).toBe(true);
      expect(matchesCriteria(result, { benchmark_id: 'different-id' })).toBe(false);
    });

    test('matchesCriteria validates tool_name', () => {
      const result = createMockBenchmarkResult({ tool_name: 'claude' });

      expect(matchesCriteria(result, { tool_name: 'claude' })).toBe(true);
      expect(matchesCriteria(result, { tool_name: 'cursor' })).toBe(false);
    });

    test('matchesCriteria validates passed status', () => {
      const passedResult = createMockBenchmarkResult({ passed: true });
      const failedResult = createMockBenchmarkResult({ passed: false });

      expect(matchesCriteria(passedResult, { passed: true })).toBe(true);
      expect(matchesCriteria(failedResult, { passed: true })).toBe(false);
    });

    test('matchesCriteria validates minimum score', () => {
      const result = createMockBenchmarkResult({ score: 8.5 });

      expect(matchesCriteria(result, { minScore: 8.0 })).toBe(true);
      expect(matchesCriteria(result, { minScore: 9.0 })).toBe(false);
    });

    test('matchesCriteria validates maximum execution time', () => {
      const result = createMockBenchmarkResult({ execution_time_ms: 1500 });

      expect(matchesCriteria(result, { maxExecutionTimeMs: 2000 })).toBe(true);
      expect(matchesCriteria(result, { maxExecutionTimeMs: 1000 })).toBe(false);
    });

    test('matchesCriteria validates multiple criteria together', () => {
      const result = createMockBenchmarkResult({
        benchmark_id: 'test-001',
        tool_name: 'claude',
        passed: true,
        score: 9.0,
        execution_time_ms: 1200,
      });

      expect(
        matchesCriteria(result, {
          benchmark_id: 'test-001',
          tool_name: 'claude',
          passed: true,
          minScore: 8.5,
          maxExecutionTimeMs: 1500,
        })
      ).toBe(true);

      expect(
        matchesCriteria(result, {
          benchmark_id: 'test-001',
          minScore: 9.5, // Fails: score too low
        })
      ).toBe(false);
    });
  });

  describe('Mock Data Generation', () => {
    test('createMockBenchmarkResult generates valid default data', () => {
      const mock = createMockBenchmarkResult();

      const validation = validateBenchmarkResult(mock);
      expect(validation.valid).toBe(true);
    });

    test('createMockBenchmarkResult applies overrides', () => {
      const mock = createMockBenchmarkResult({
        benchmark_id: 'custom-id',
        tool_name: 'copilot',
        score: 7.5,
        passed: false,
      });

      expect(mock.benchmark_id).toBe('custom-id');
      expect(mock.tool_name).toBe('copilot');
      expect(mock.score).toBe(7.5);
      expect(mock.passed).toBe(false);
    });

    test('createMockBenchmarkResult maintains valid structure with overrides', () => {
      const mock = createMockBenchmarkResult({
        output: 'custom output',
        validation_details: {
          passed: false,
          score: 5.0,
          test_case_results: [],
          errors: ['Test error'],
        },
      });

      const validation = validateBenchmarkResult(mock);
      expect(validation.valid).toBe(true);
      expect(mock.output).toBe('custom output');
      expect(mock.validation_details.errors).toContain('Test error');
    });
  });
});
