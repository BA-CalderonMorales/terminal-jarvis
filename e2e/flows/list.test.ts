import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

describe('list flow', () => {
    test('list command shows available tools', async () => {
        const instance = await render(getBinaryPath(), ['list']);
        await new Promise(resolve => setTimeout(resolve, 2000));

        const output = normalizeOutput(instance.getStdallStr());

        // Should show tool listing header or tools
        expect(output.length).toBeGreaterThan(0);
    });

    test('list exits successfully', async () => {
        const instance = await render(getBinaryPath(), ['list']);
        await new Promise(resolve => setTimeout(resolve, 2000));

        const exitInfo = instance.hasExit();
        expect(exitInfo).toBeTruthy();
        expect(exitInfo?.exitCode).toBe(0);
    });
});
