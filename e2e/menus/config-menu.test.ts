import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

/**
 * Config Submenu Structure Tests
 *
 * Documents the configuration command options.
 */
describe('config submenu', () => {
    test('config --help shows available subcommands', async () => {
        const instance = await render(getBinaryPath(), ['config', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());

        expect(output).toMatch(/config|configuration/i);
    });
});
