import { execSync } from 'node:child_process';
import { existsSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

export interface PackageTestContext {
    testDir: string;
    packagePath: string;
    version: string;
    cleanup: () => void;
}

/**
 * Creates a test environment with terminal-jarvis installed via npm pack
 * Simulates real-world package installation for E2E testing
 */
export function createPackageTestEnvironment(): PackageTestContext {
    const packageJson = require('../../npm/terminal-jarvis/package.json');
    const version = packageJson.version;

    const localBinary = resolve(__dirname, '../../target/release/terminal-jarvis');
    if (!existsSync(localBinary)) {
        throw new Error(
            `Local binary not found at ${localBinary}. Run "cargo build --release" first.`,
        );
    }

    const testDir = mkdtempSync(join(tmpdir(), 'jarvis-test-'));
    const projectRoot = resolve(__dirname, '../../npm/terminal-jarvis');

    execSync('npm pack', { cwd: projectRoot, stdio: 'pipe' });
    const packagePath = join(projectRoot, `terminal-jarvis-${version}.tgz`);

    if (!existsSync(packagePath)) {
        throw new Error(`Package tarball not found at ${packagePath}`);
    }

    execSync('npm init -y', { cwd: testDir, stdio: 'pipe' });
    execSync(`npm install ${packagePath}`, { cwd: testDir, stdio: 'pipe' });

    // Copy local binary to installed package
    const installedBinDir = join(testDir, 'node_modules/terminal-jarvis/bin');
    const binaryDestination = join(installedBinDir, 'terminal-jarvis-bin');

    const fs = require('fs');
    if (!fs.existsSync(installedBinDir)) {
        fs.mkdirSync(installedBinDir, { recursive: true });
    }
    fs.copyFileSync(localBinary, binaryDestination);
    fs.chmodSync(binaryDestination, 0o755);

    return {
        testDir,
        packagePath,
        version,
        cleanup: () => {
            try {
                rmSync(testDir, { recursive: true, force: true });
                rmSync(packagePath, { force: true });
            } catch (error) {
                console.warn(`Cleanup warning: ${error}`);
            }
        },
    };
}
