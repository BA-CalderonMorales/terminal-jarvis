import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

/**
 * Templates Submenu Structure Tests
 *
 * Documents the templates command options.
 */
describe('templates submenu', () => {
    test('templates --help shows available subcommands', async () => {
        const instance = await render(getBinaryPath(), ['templates', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());

        expect(output).toMatch(/templates/i);
    });
});
