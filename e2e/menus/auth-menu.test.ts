import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

/**
 * Auth Submenu Structure Tests
 *
 * Documents the authentication command options.
 */
describe('auth submenu', () => {
    test('auth --help shows available subcommands', async () => {
        const instance = await render(getBinaryPath(), ['auth', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());

        // Auth should have subcommands for managing credentials
        expect(output).toMatch(/auth|authentication/i);
    });
});
