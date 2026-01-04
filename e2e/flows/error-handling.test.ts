import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath } from '../utils';

describe('error handling flow', () => {
    test('unknown command shows error', async () => {
        const { findByText, findByError } = await render(getBinaryPath(), [
            'nonexistent-command',
        ]);

        const hasError = await Promise.race([
            findByText(/error|unknown|invalid|not found/i).then(() => true),
            findByError(/error|unknown|invalid|not found/i).then(() => true),
            new Promise(resolve => setTimeout(() => resolve(false), 3000)),
        ]);

        expect(hasError).toBe(true);
    });

    test('invalid command exits with non-zero code', async () => {
        const instance = await render(getBinaryPath(), ['invalid-command']);
        await new Promise(resolve => setTimeout(resolve, 2000));

        const exitInfo = instance.hasExit();
        expect(exitInfo).toBeTruthy();
        expect(exitInfo?.exitCode).not.toBe(0);
    });

    test('conflicting flags are handled gracefully', async () => {
        const instance = await render(getBinaryPath(), ['--version', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        // Should exit, not hang
        const exitInfo = instance.hasExit();
        expect(exitInfo).toBeTruthy();
    });

    test('missing arguments provides guidance', async () => {
        const { findByText, findByError } = await render(getBinaryPath(), [
            'install',
        ]);

        const hasGuidance = await Promise.race([
            findByText(/required|missing|usage|expected/i).then(() => true),
            findByError(/required|missing|usage|expected/i).then(() => true),
            new Promise(resolve => setTimeout(() => resolve(false), 3000)),
        ]);

        expect(hasGuidance).toBe(true);
    });
});
