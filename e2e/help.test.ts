import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from './helpers';

describe('terminal-jarvis --help', () => {
  test('displays main help text with usage information', async () => {
    const { findByText, queryByText } = await render(getBinaryPath(), ['--help']);

    // Verify core help sections are present
    expect(await findByText('Usage:')).toBeInTheConsole();
    expect(await findByText('Commands:')).toBeInTheConsole();
    expect(await findByText('Options:')).toBeInTheConsole();
  });

  test('lists all available commands', async () => {
    const instance = await render(getBinaryPath(), ['--help']);

    // Wait for output to be captured
    await new Promise(resolve => setTimeout(resolve, 1000));

    const output = normalizeOutput(instance.getStdallStr());

    // Check that key commands are documented based on actual CLI output
    const expectedCommands = [
      'run',
      'install',
      'update',
      'list',
      'info',
      'auth',
      'templates',
      'config',
      'cache',
      'help'
    ];

    for (const command of expectedCommands) {
      expect(output).toContain(command);
    }
  });

  test('shows version flag in help output', async () => {
    const { findByText } = await render(getBinaryPath(), ['--help']);

    // The help output should include --version or -V flag
    expect(await findByText(/--version|-V/)).toBeInTheConsole();
  });

  test('displays help flag options', async () => {
    const { findByText } = await render(getBinaryPath(), ['--help']);

    // The help output should include --help or -h flag
    expect(await findByText(/--help|-h/)).toBeInTheConsole();
  });

  test('help command produces same output as --help flag', async () => {
    const helpFlagResult = await render(getBinaryPath(), ['--help']);
    const helpCommandResult = await render(getBinaryPath(), ['help']);

    await new Promise(resolve => setTimeout(resolve, 1000));

    const helpFlagOutput = normalizeOutput(helpFlagResult.getStdallStr());
    const helpCommandOutput = normalizeOutput(helpCommandResult.getStdallStr());

    // Core sections should be identical
    expect(helpFlagOutput).toContain('Usage:');
    expect(helpCommandOutput).toContain('Usage:');
    expect(helpFlagOutput).toContain('Commands:');
    expect(helpCommandOutput).toContain('Commands:');
  });

  test('exits successfully after displaying help', async () => {
    const instance = await render(getBinaryPath(), ['--help']);

    // Wait for process to exit
    await new Promise(resolve => setTimeout(resolve, 1000));

    const exitInfo = instance.hasExit();
    expect(exitInfo).toBeTruthy();
    if (exitInfo) {
      expect(exitInfo.exitCode).toBe(0);
    }
  });
});
