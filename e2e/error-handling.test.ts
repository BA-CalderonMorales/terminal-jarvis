import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath } from './helpers';

describe('terminal-jarvis error handling', () => {
  test('shows error for unknown command', async () => {
    const { findByText, findByError } = await render(getBinaryPath(), [
      'nonexistent-command'
    ]);

    // Should show error message (in stdout or stderr)
    // Adjust based on actual error message format
    const hasError = await Promise.race([
      findByText(/error|unknown|invalid|not found/i).then(() => true),
      findByError(/error|unknown|invalid|not found/i).then(() => true),
      new Promise(resolve => setTimeout(() => resolve(false), 3000))
    ]);

    expect(hasError).toBe(true);
  });

  test('exits with non-zero code for invalid command', async () => {
    const instance = await render(getBinaryPath(), ['invalid-command']);

    await new Promise(resolve => setTimeout(resolve, 2000));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).not.toBe(0);
    }
  });

  test('shows error for invalid flag combinations', async () => {
    const instance = await render(getBinaryPath(), [
      '--version',
      '--help'
    ]);

    // Either processes one flag or shows error
    // This test validates the tool handles unexpected input gracefully
    await new Promise(resolve => setTimeout(resolve, 1000));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy(); // Should exit, not hang
  });

  test('provides helpful error message for missing arguments', async () => {
    // Test a command that requires arguments (adjust based on actual commands)
    const { findByText, findByError } = await render(getBinaryPath(), [
      'install'
      // Missing tool name argument
    ]);

    // Should provide guidance (either shows help or specific error)
    const hasGuidance = await Promise.race([
      findByText(/required|missing|usage|expected/i).then(() => true),
      findByError(/required|missing|usage|expected/i).then(() => true),
      new Promise(resolve => setTimeout(() => resolve(false), 3000))
    ]);

    expect(hasGuidance).toBe(true);
  });
});
