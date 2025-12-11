/**
 * Normalize CLI output for consistent assertions
 * Strips ANSI codes and normalizes whitespace
 */
export function normalizeOutput(output: string): string {
    return output
        .replace(/\x1B\[[0-9;]*[JKmsu]/g, '') // Strip ANSI escape codes
        .replace(/\r\n/g, '\n') // Normalize line endings
        .trim();
}

/**
 * Extract semantic version from string
 * Handles formats like "0.0.70", "v0.0.70", "terminal-jarvis 0.0.70"
 */
export function sanitizeVersion(versionString: string): string {
    const match = versionString.match(/v?(\d+\.\d+\.\d+)/);
    return match ? match[1] : versionString.trim();
}
