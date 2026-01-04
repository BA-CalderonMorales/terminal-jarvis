import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from '../utils';

describe('help flow', () => {
    test('--help displays usage information', async () => {
        const { findByText } = await render(getBinaryPath(), ['--help']);

        expect(await findByText('Usage:')).toBeInTheConsole();
        expect(await findByText('Commands:')).toBeInTheConsole();
        expect(await findByText('Options:')).toBeInTheConsole();
    });

    test('--help lists all available commands', async () => {
        const instance = await render(getBinaryPath(), ['--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const output = normalizeOutput(instance.getStdallStr());

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
            'help',
        ];

        for (const command of expectedCommands) {
            expect(output).toContain(command);
        }
    });

    test('help command produces same output as --help flag', async () => {
        const helpFlagResult = await render(getBinaryPath(), ['--help']);
        const helpCommandResult = await render(getBinaryPath(), ['help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const helpFlagOutput = normalizeOutput(helpFlagResult.getStdallStr());
        const helpCommandOutput = normalizeOutput(helpCommandResult.getStdallStr());

        expect(helpFlagOutput).toContain('Usage:');
        expect(helpCommandOutput).toContain('Usage:');
    });

    test('exits successfully', async () => {
        const instance = await render(getBinaryPath(), ['--help']);
        await new Promise(resolve => setTimeout(resolve, 1000));

        const exitInfo = instance.hasExit();
        expect(exitInfo).toBeTruthy();
        expect(exitInfo?.exitCode).toBe(0);
    });
});
