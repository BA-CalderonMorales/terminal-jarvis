import { existsSync } from 'node:fs';
import { resolve } from 'node:path';

/**
 * Get the path to the compiled terminal-jarvis binary
 * Used by all E2E tests to execute the CLI
 */
export function getBinaryPath(): string {
    const binaryPath = resolve(__dirname, '../../target/release/terminal-jarvis');

    if (!existsSync(binaryPath)) {
        throw new Error(
            `Binary not found at ${binaryPath}. Run "cargo build --release" first.`
        );
    }

    return binaryPath;
}
