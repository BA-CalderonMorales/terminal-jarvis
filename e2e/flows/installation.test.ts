import { describe, test, expect, beforeAll, afterAll } from 'vitest';
import { render } from 'cli-testing-library';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { createPackageTestEnvironment, type PackageTestContext } from '../utils';

/**
 * NPM Package Installation Flow
 *
 * Tests the complete npm pack -> install -> execute workflow.
 * These tests validate the distribution package works correctly.
 */
describe('npm installation flow', () => {
    let ctx: PackageTestContext;

    beforeAll(async () => {
        ctx = createPackageTestEnvironment();
    }, 60000);

    afterAll(() => {
        ctx?.cleanup();
    });

    test('npm pack creates installable package', () => {
        expect(ctx.testDir).toBeTruthy();
        expect(ctx.packagePath).toBeTruthy();
    });

    test('launcher script exists after install', () => {
        const launcherPath = join(ctx.testDir, 'node_modules/terminal-jarvis/bin/terminal-jarvis');
        expect(existsSync(launcherPath)).toBe(true);
    });

    test('binary exists after install', () => {
        const binaryPath = join(ctx.testDir, 'node_modules/terminal-jarvis/bin/terminal-jarvis-bin');
        expect(existsSync(binaryPath)).toBe(true);
    });

    test('npx terminal-jarvis --version works', async () => {
        const { findByText } = await render('npx', ['terminal-jarvis', '--version'], {
            cwd: ctx.testDir,
        });

        expect(await findByText(/\d+\.\d+\.\d+/)).toBeInTheConsole();
    });

    test('npx terminal-jarvis --help works', async () => {
        const { findByText } = await render('npx', ['terminal-jarvis', '--help'], {
            cwd: ctx.testDir,
        });

        expect(await findByText('Usage:')).toBeInTheConsole();
        expect(await findByText('Commands:')).toBeInTheConsole();
    });

    test('version matches package.json', async () => {
        const { findByText } = await render('npx', ['terminal-jarvis', '--version'], {
            cwd: ctx.testDir,
        });

        expect(await findByText(new RegExp(ctx.version))).toBeInTheConsole();
    });
});
