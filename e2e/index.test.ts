import { describe, test, expect } from 'vitest';
import { existsSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

/**
 * E2E Test Structure Validation
 *
 * Ensures the test suite follows the expected organization.
 * This is the entry point that validates test infrastructure.
 */
describe('e2e test structure', () => {
    const e2eRoot = __dirname;

    test('required directories exist', () => {
        const requiredDirs = ['utils', 'flows', 'menus', 'config'];

        for (const dir of requiredDirs) {
            const dirPath = join(e2eRoot, dir);
            expect(existsSync(dirPath), `Directory ${dir}/ should exist`).toBe(true);
            expect(statSync(dirPath).isDirectory()).toBe(true);
        }
    });

    test('utils/ contains required modules', () => {
        const utilsPath = join(e2eRoot, 'utils');
        const files = readdirSync(utilsPath);

        expect(files).toContain('index.ts');
        expect(files).toContain('binary.ts');
        expect(files).toContain('output.ts');
    });

    test('flows/ contains test files', () => {
        const flowsPath = join(e2eRoot, 'flows');
        const files = readdirSync(flowsPath).filter(f => f.endsWith('.test.ts'));

        expect(files.length).toBeGreaterThan(0);
    });

    test('menus/ contains test files', () => {
        const menusPath = join(e2eRoot, 'menus');
        const files = readdirSync(menusPath).filter(f => f.endsWith('.test.ts'));

        expect(files.length).toBeGreaterThan(0);
    });

    test('config/ exists for test configuration', () => {
        const configPath = join(e2eRoot, 'config');
        expect(existsSync(configPath)).toBe(true);
    });
});

describe('binary availability', () => {
    test('release binary exists', async () => {
        const { getBinaryPath } = await import('./utils');

        // This will throw if binary doesn't exist
        const binaryPath = getBinaryPath();
        expect(existsSync(binaryPath)).toBe(true);
    });
});
