/**
 * E2E Test Utilities
 *
 * Shared utilities for terminal-jarvis CLI end-to-end tests.
 */

export { getBinaryPath } from './binary';
export { normalizeOutput, sanitizeVersion } from './output';
export { createPackageTestEnvironment, type PackageTestContext } from './package-test';
