/**
 * Benchmark E2E Tests
 * 
 * End-to-end testing of the complete benchmark flow via CLI.
 * Tests actual CLI command execution, JSON export, and result validation.
 */

import { describe, test, expect, beforeAll, afterAll } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath } from '../helpers';
import { loadBenchmarkResult, validateBenchmarkResult } from '../helpers/benchmark-helpers';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { tmpdir } from 'node:os';
import { resolve } from 'node:path';

describe('Benchmark E2E Tests', () => {
  let outputDir: string;
  // Benchmark commands must run from project root where config/benchmarks/scenarios/ exists
  const projectRoot = resolve(__dirname, '../../../..');

  beforeAll(() => {
    // Create temp directory for test outputs
    outputDir = path.join(tmpdir(), 'benchmark-e2e-' + Date.now());
    fs.mkdirSync(outputDir, { recursive: true });
  });

  afterAll(() => {
    // Cleanup temp directory
    if (fs.existsSync(outputDir)) {
      fs.rmSync(outputDir, { recursive: true, force: true });
    }
  });

  test('runs benchmark and exports JSON', async () => {
    const binaryPath = getBinaryPath();
    
    // Run benchmark command from project root
    const instance = await render(binaryPath, [
      'benchmark',
      'run',
      '--scenario',
      'code-completion-basic-001',
      '--tool',
      'test-tool',
      '--export-json',
      outputDir,
    ], { cwd: projectRoot });

    // Wait for execution to complete
    await new Promise(resolve => setTimeout(resolve, 3000));

    // Check exit code
    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }

    // Find exported JSON file
    const files = fs.readdirSync(outputDir);
    const jsonFiles = files.filter(f => f.endsWith('.json'));
    
    expect(jsonFiles.length).toBeGreaterThan(0);

    // Load and validate the result
    const jsonPath = path.join(outputDir, jsonFiles[0]);
    const result = loadBenchmarkResult(jsonPath);

    // Validate schema
    const validation = validateBenchmarkResult(result);
    expect(validation.valid).toBe(true);
    if (!validation.valid) {
      console.error('Validation errors:', validation.errors);
    }

    // Assert expected values
    expect(result.benchmark_id).toBe('code-completion-basic-001');
    expect(result.tool_name).toBe('test-tool');
    expect(result.passed).toBeDefined();
    expect(result.score).toBeGreaterThanOrEqual(0);
    expect(result.score).toBeLessThanOrEqual(10);
    expect(result.validation_details.test_case_results.length).toBeGreaterThan(0);
  }, 15000); // 15 second timeout

  test('lists available benchmarks', async () => {
    const binaryPath = getBinaryPath();
    
    const instance = await render(binaryPath, [
      'benchmark',
      'list',
    ], { cwd: projectRoot });

    // Wait for output
    await new Promise(resolve => setTimeout(resolve, 1000));

    // Get output
    const output = instance.getStdallStr();

    // Should show the code-completion scenario
    expect(output).toContain('code-completion-basic-001');
    expect(output).toContain('Basic Code Completion');
    expect(output).toContain('code-completion');

    // Check exit code
    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }
  }, 5000);

  test('validates scenario files', async () => {
    const binaryPath = getBinaryPath();
    const scenarioPath = 'config/benchmarks/scenarios/code-completion/basic-001.toml';
    
    const instance = await render(binaryPath, [
      'benchmark',
      'validate',
      '--scenario-file',
      scenarioPath,
    ], { cwd: projectRoot });

    // Wait for validation
    await new Promise(resolve => setTimeout(resolve, 1000));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }

    const output = instance.getStdallStr();
    expect(output).toMatch(/success|valid|passed/i);
  }, 5000);

  test('handles invalid scenario gracefully', async () => {
    const binaryPath = getBinaryPath();
    
    const instance = await render(binaryPath, [
      'benchmark',
      'run',
      '--scenario',
      'non-existent-scenario',
      '--tool',
      'test-tool',
    ], { cwd: projectRoot });

    // Wait for error
    await new Promise(resolve => setTimeout(resolve, 2000));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).not.toBe(0); // Should fail
    }

    // Should have error message in output
    const output = instance.getStdallStr();
    expect(output.length).toBeGreaterThan(0);
    expect(output).toMatch(/error|not found|invalid/i);
  }, 5000);

  test('exported JSON matches expected structure', async () => {
    const binaryPath = getBinaryPath();
    
    const instance = await render(binaryPath, [
      'benchmark',
      'run',
      '--scenario',
      'code-completion-basic-001',
      '--tool',
      'schema-test',
      '--export-json',
      outputDir,
    ], { cwd: projectRoot });

    await new Promise(resolve => setTimeout(resolve, 3000));

    // Check exit code
    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }

    // Find the new JSON file
    const files = fs.readdirSync(outputDir);
    const schemaTestFile = files.find(f => f.includes('schema-test'));
    expect(schemaTestFile).toBeDefined();

    if (schemaTestFile) {
      const result = loadBenchmarkResult(path.join(outputDir, schemaTestFile));

      // Check all required fields are present
      expect(result).toHaveProperty('benchmark_id');
      expect(result).toHaveProperty('tool_name');
      expect(result).toHaveProperty('scenario_version');
      expect(result).toHaveProperty('execution_timestamp');
      expect(result).toHaveProperty('execution_time_ms');
      expect(result).toHaveProperty('passed');
      expect(result).toHaveProperty('score');
      expect(result).toHaveProperty('output');
      expect(result).toHaveProperty('validation_details');

      // Check validation_details structure
      expect(result.validation_details).toHaveProperty('passed');
      expect(result.validation_details).toHaveProperty('score');
      expect(result.validation_details).toHaveProperty('test_case_results');
      expect(result.validation_details).toHaveProperty('errors');

      // Verify timestamp is valid ISO 8601
      expect(() => new Date(result.execution_timestamp)).not.toThrow();
      const timestamp = new Date(result.execution_timestamp);
      expect(timestamp.getTime()).toBeGreaterThan(0);

      // Verify execution time is reasonable (not negative, not absurdly large)
      expect(result.execution_time_ms).toBeGreaterThanOrEqual(0);
      expect(result.execution_time_ms).toBeLessThan(300000); // Less than 5 minutes

      // Verify score range
      expect(result.score).toBeGreaterThanOrEqual(0);
      expect(result.score).toBeLessThanOrEqual(10);
    }
  }, 15000);

  test('benchmark run without export-json does not create files', async () => {
    const binaryPath = getBinaryPath();
    
    // Create a unique temp directory for this test
    const testOutputDir = path.join(tmpdir(), 'benchmark-no-export-' + Date.now());
    fs.mkdirSync(testOutputDir, { recursive: true });

    try {
      const instance = await render(binaryPath, [
        'benchmark',
        'run',
        '--scenario',
        'code-completion-basic-001',
        '--tool',
        'no-export-test',
      ], { cwd: projectRoot });

      await new Promise(resolve => setTimeout(resolve, 3000));

      // Check exit code
      const exitInfo = instance.hasExit();
      expect(exitInfo).toBeTruthy();
      if (exitInfo) {
        expect(exitInfo.exitCode).toBe(0);
      }

      // Should have output to console
      const output = instance.getStdallStr();
      expect(output.length).toBeGreaterThan(0);

      // No JSON files should be created in our test directory
      const files = fs.readdirSync(testOutputDir);
      const jsonFiles = files.filter(f => f.endsWith('.json'));
      expect(jsonFiles.length).toBe(0);
    } finally {
      // Cleanup
      if (fs.existsSync(testOutputDir)) {
        fs.rmSync(testOutputDir, { recursive: true, force: true });
      }
    }
  }, 15000);

  test('benchmark run with different tools creates separate files', async () => {
    const binaryPath = getBinaryPath();
    
    // Run benchmark with tool1
    const instance1 = await render(binaryPath, [
      'benchmark',
      'run',
      '--scenario',
      'code-completion-basic-001',
      '--tool',
      'tool-alpha',
      '--export-json',
      outputDir,
    ], { cwd: projectRoot });

    await new Promise(resolve => setTimeout(resolve, 3000));

    const exitInfo1 = instance1.hasExit();
    expect(exitInfo1).toBeTruthy();
    if (exitInfo1) {
      expect(exitInfo1.exitCode).toBe(0);
    }

    // Run benchmark with tool2
    const instance2 = await render(binaryPath, [
      'benchmark',
      'run',
      '--scenario',
      'code-completion-basic-001',
      '--tool',
      'tool-beta',
      '--export-json',
      outputDir,
    ], { cwd: projectRoot });

    await new Promise(resolve => setTimeout(resolve, 3000));

    const exitInfo2 = instance2.hasExit();
    expect(exitInfo2).toBeTruthy();
    if (exitInfo2) {
      expect(exitInfo2.exitCode).toBe(0);
    }

    // Check that we have at least 2 different JSON files
    const files = fs.readdirSync(outputDir);
    const jsonFiles = files.filter(f => f.endsWith('.json'));
    expect(jsonFiles.length).toBeGreaterThanOrEqual(2);

    // Verify that files contain different tool names
    const toolAlphaFile = jsonFiles.find(f => f.includes('tool-alpha'));
    const toolBetaFile = jsonFiles.find(f => f.includes('tool-beta'));

    expect(toolAlphaFile).toBeDefined();
    expect(toolBetaFile).toBeDefined();

    if (toolAlphaFile && toolBetaFile) {
      const result1 = loadBenchmarkResult(path.join(outputDir, toolAlphaFile));
      const result2 = loadBenchmarkResult(path.join(outputDir, toolBetaFile));

      expect(result1.tool_name).toBe('tool-alpha');
      expect(result2.tool_name).toBe('tool-beta');
      expect(result1.benchmark_id).toBe(result2.benchmark_id); // Same scenario
    }
  }, 30000); // 30 second timeout for two runs
});
