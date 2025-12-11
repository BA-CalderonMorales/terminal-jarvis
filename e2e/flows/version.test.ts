import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, sanitizeVersion, normalizeOutput } from '../utils';

describe('version flow', () => {
    test('--version displays semantic version', async () => {
        const { findByText } = await render(getBinaryPath(), ['--version']);

        expect(await findByText(/\d+\.\d+\.\d+/)).toBeInTheConsole();
    });

    test('version matches package.json', async () => {
        const packageJson = require('../../npm/terminal-jarvis/package.json');
        const expectedVersion = sanitizeVersion(packageJson.version);

        const instance = await render(getBinaryPath(), ['--version']);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();
        const versionInOutput = sanitizeVersion(output);

        expect(versionInOutput).toContain(expectedVersion);
    });

    test('output is clean and parseable', async () => {
        const instance = await render(getBinaryPath(), ['--version']);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = normalizeOutput(instance.getStdallStr());

        expect(output).toMatch(/terminal-jarvis/i);
        expect(output).toMatch(/\d+\.\d+\.\d+/);

        // Should be simple one-line output
        const lines = output.split('\n').filter(l => l.trim());
        expect(lines.length).toBeLessThanOrEqual(2);
    });

    test('exits successfully', async () => {
        const instance = await render(getBinaryPath(), ['--version']);
        await new Promise(resolve => setTimeout(resolve, 500));

        const exitInfo = instance.hasExit();
        expect(exitInfo).toBeTruthy();
        expect(exitInfo?.exitCode).toBe(0);
    });
});
