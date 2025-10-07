import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, sanitizeVersion } from './helpers';

describe('terminal-jarvis --version', () => {
  test('displays version number', async () => {
    const { findByText } = await render(getBinaryPath(), ['--version']);

    // Match semantic version pattern (e.g., "0.0.69" or "v0.0.69")
    expect(await findByText(/\d+\.\d+\.\d+/)).toBeInTheConsole();
  });

  test('version matches package.json', async () => {
    const packageJson = require('../package.json');
    const expectedVersion = sanitizeVersion(packageJson.version);

    const instance = await render(getBinaryPath(), ['--version']);
    await new Promise(resolve => setTimeout(resolve, 500));

    const output = instance.getStdallStr();
    const versionInOutput = sanitizeVersion(output);

    expect(versionInOutput).toContain(expectedVersion);
  });

  test('exits successfully after displaying version', async () => {
    const instance = await render(getBinaryPath(), ['--version']);

    await new Promise(resolve => setTimeout(resolve, 500));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }
  });

  test('version output is clean and parseable', async () => {
    const instance = await render(getBinaryPath(), ['--version']);

    await new Promise(resolve => setTimeout(resolve, 500));

    const output = instance.getStdallStr();

    // Output should contain version and app name
    expect(output).toMatch(/terminal-jarvis/i);
    expect(output).toMatch(/\d+\.\d+\.\d+/);

    // Should be a simple one-line output (clean parsing)
    const lines = output.trim().split('\n');
    expect(lines.length).toBeLessThanOrEqual(2); // Allow for trailing newline
  });
});
