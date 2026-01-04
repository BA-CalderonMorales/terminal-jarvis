import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

/**
 * Main Menu Structure Tests
 *
 * Documents and validates the main interactive menu options.
 * Each test case represents a menu option that should exist.
 */
describe('main menu', () => {
    /**
     * Expected main menu options (from help output):
     * - run       Run a specific AI coding tool
     * - install   Install an AI coding tool
     * - update    Update AI coding tools
     * - list      List all available AI coding tools
     * - info      Get information about a specific tool
     * - auth      Manage authentication for AI tools
     * - templates Manage project templates
     * - config    Manage configuration settings
     * - cache     Manage cache settings
     * - help      Print help message
     */

    test('run command is available', async () => {
        const instance = await render(getBinaryPath(), ['run', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/run|usage/i);
    });

    test('install command is available', async () => {
        const instance = await render(getBinaryPath(), ['install', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/install|usage/i);
    });

    test('update command is available', async () => {
        const instance = await render(getBinaryPath(), ['update', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/update|usage/i);
    });

    test('list command is available', async () => {
        const instance = await render(getBinaryPath(), ['list', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/list|usage/i);
    });

    test('info command is available', async () => {
        const instance = await render(getBinaryPath(), ['info', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/info|usage/i);
    });

    test('auth command is available', async () => {
        const instance = await render(getBinaryPath(), ['auth', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/auth|usage/i);
    });

    test('templates command is available', async () => {
        const instance = await render(getBinaryPath(), ['templates', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/templates|usage/i);
    });

    test('config command is available', async () => {
        const instance = await render(getBinaryPath(), ['config', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/config|usage/i);
    });

    test('cache command is available', async () => {
        const instance = await render(getBinaryPath(), ['cache', '--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());
        expect(output).toMatch(/cache|usage/i);
    });
});
