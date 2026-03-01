import { describe, expect, test } from 'vitest';
import { spawnSync } from 'node:child_process';
import { getBinaryPath, normalizeOutput } from '../utils';

type CliRunResult = {
    output: string;
    status: number | null;
    timedOut: boolean;
};

function runCli(args: string[], timeoutMs = 10000): CliRunResult {
    const result = spawnSync(getBinaryPath(), args, {
        encoding: 'utf8',
        env: { ...process.env, CI: 'true', NO_COLOR: '1' },
        timeout: timeoutMs,
        maxBuffer: 2 * 1024 * 1024,
    });

    const err = result.error as NodeJS.ErrnoException | undefined;
    const timedOut = err?.code === 'ETIMEDOUT';
    const output = normalizeOutput(`${result.stdout ?? ''}\n${result.stderr ?? ''}`);

    return {
        output,
        status: result.status,
        timedOut,
    };
}

describe('direct tool invocation and arg forwarding', () => {
    test('direct external command accepts copilot tool name', () => {
        const result = runCli(['copilot', '--help']);

        expect(result.timedOut).toBe(false);
        expect(result.output).not.toContain("is not a valid tool or command");
        expect(result.output.length).toBeGreaterThan(0);
    });

    test('run command without separator consumes --help', () => {
        const result = runCli(['run', 'copilot', '--help']);

        expect(result.timedOut).toBe(false);
        expect(result.status).toBe(0);
        expect(result.output).toContain('Usage: terminal-jarvis run');
    });

    test('run command forwards --help to tool after -- separator', () => {
        const result = runCli(['run', 'copilot', '--', '--help']);

        expect(result.timedOut).toBe(false);
        expect(result.output).not.toContain('Usage: terminal-jarvis run');
        expect(result.output).not.toContain("is not a valid tool or command");
        expect(result.output.length).toBeGreaterThan(0);
    });

    test('direct tool command closes cleanly on repeated launch', () => {
        const first = runCli(['copilot', '--help']);
        const second = runCli(['copilot', '--help']);

        expect(first.timedOut).toBe(false);
        expect(second.timedOut).toBe(false);
        expect(first.output).not.toContain("is not a valid tool or command");
        expect(second.output).not.toContain("is not a valid tool or command");
    });
});
