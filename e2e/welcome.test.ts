import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput, stripAnsi } from './helpers';

describe('terminal-jarvis welcome screen', () => {
  test('displays T.JARVIS bot ASCII art when launched without arguments', async () => {
    const instance = await render(getBinaryPath(), []);
    await new Promise(resolve => setTimeout(resolve, 2000));
    const output = normalizeOutput(instance.getStdallStr());

    expect(output).toContain('┌─────┐');
    expect(output).toContain('│ T.J │');
    expect(output).toContain('│ ═ ═ │');
    expect(output).toContain('│     │');
    expect(output).toContain('└─────┘');
    expect(output).toContain('Terminal Jarvis');
  });

  test('displays tip integrated with ASCII art', async () => {
    const instance = await render(getBinaryPath(), []);
    await new Promise(resolve => setTimeout(resolve, 2000));
    const output = normalizeOutput(instance.getStdallStr());

    expect(output).toContain('Tip: Check Important Links for docs');
    expect(output).toContain('─────');
  });
});
