import { execSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

/**
 * Test helper utilities for terminal-jarvis CLI testing
 */

export interface PackageTestContext {
  testDir: string;
  packagePath: string;
  version: string;
  cleanup: () => void;
}

/**
 * Creates a test environment with terminal-jarvis installed via npm pack
 *
 * This simulates real-world package installation and provides isolated test environment
 * For local testing, we copy the built binary instead of downloading from GitHub
 */
export function createPackageTestEnvironment(): PackageTestContext {
  // Get version from package.json
  const packageJson = require("../package.json");
  const version = packageJson.version;

  // Verify local binary exists
  const localBinary = resolve(__dirname, "../../../target/release/terminal-jarvis");
  if (!existsSync(localBinary)) {
    throw new Error(`Local binary not found at ${localBinary}. Run "cargo build --release" first.`);
  }

  // Create temporary test directory
  const testDir = mkdtempSync(join(tmpdir(), "jarvis-test-"));

  // Build the package tarball
  const projectRoot = resolve(__dirname, "..");

  // Run npm pack in the project root
  execSync("npm pack", {
    cwd: projectRoot,
    stdio: "pipe",
  });

  const packagePath = join(projectRoot, `terminal-jarvis-${version}.tgz`);

  // Verify the tarball was created
  if (!existsSync(packagePath)) {
    throw new Error(`Package tarball not found at ${packagePath}`);
  }

  // Install the package in the test directory
  execSync(`npm init -y`, {
    cwd: testDir,
    stdio: "pipe",
  });

  execSync(`npm install ${packagePath}`, {
    cwd: testDir,
    stdio: "pipe",
  });

  // After installation, copy the local binary to where the postinstall script expects it
  // This bypasses the postinstall GitHub download for local testing
  const installedBinDir = join(testDir, "node_modules/terminal-jarvis/bin");

  // The package structure:
  // - terminal-jarvis (launcher script - committed to repo, installed by npm)
  // - terminal-jarvis-bin (actual binary - downloaded by postinstall or placed by test)
  const binaryDestination = join(installedBinDir, "terminal-jarvis-bin");

  try {
    const fs = require("fs");

    // Ensure bin directory exists
    if (!fs.existsSync(installedBinDir)) {
      fs.mkdirSync(installedBinDir, { recursive: true });
    }

    // Copy the compiled binary as terminal-jarvis-bin
    // The launcher script is already installed by npm from the package
    fs.copyFileSync(localBinary, binaryDestination);
    fs.chmodSync(binaryDestination, 0o755);
  } catch (error) {
    throw new Error(`Failed to setup binary in installed package: ${error}`);
  }

  return {
    testDir,
    packagePath,
    version,
    cleanup: () => {
      // Clean up test directory and tarball
      try {
        rmSync(testDir, { recursive: true, force: true });
        rmSync(packagePath, { force: true });
      } catch (error) {
        console.warn(`Cleanup warning: ${error}`);
      }
    },
  };
}

/**
 * Gets the path to the compiled Rust binary for testing
 */
export function getBinaryPath(): string {
  const binaryPath = resolve(__dirname, "../../../target/release/terminal-jarvis");

  if (!existsSync(binaryPath)) {
    throw new Error(`Binary not found at ${binaryPath}. Run "cargo build --release" first.`);
  }

  return binaryPath;
}

/**
 * Normalizes CLI output for cross-platform testing
 * - Removes ANSI color codes
 * - Normalizes line endings
 * - Trims whitespace
 */
export function normalizeOutput(output: string): string {
  return (
    output
      // Remove ANSI escape codes
      // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape codes require control characters
      .replace(/\x1b\[[0-9;]*m/g, "")
      // Normalize line endings
      .replace(/\r\n/g, "\n")
      // Trim trailing whitespace from each line
      .split("\n")
      .map((line) => line.trimEnd())
      .join("\n")
      // Trim overall
      .trim()
  );
}

/**
 * Cross-platform arrow symbol matcher
 * Windows typically uses ">", Unix systems use "❯"
 */
export function arrowSymbol(): RegExp {
  return /[❯>]/;
}

/**
 * Waits for a specific condition with timeout
 */
export async function waitForCondition(
  condition: () => boolean | Promise<boolean>,
  options: { timeout?: number; interval?: number } = {},
): Promise<void> {
  const { timeout = 5000, interval = 100 } = options;
  const startTime = Date.now();

  while (Date.now() - startTime < timeout) {
    if (await condition()) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, interval));
  }

  throw new Error(`Condition not met within ${timeout}ms`);
}

/**
 * Sanitizes version strings for comparison
 */
export function sanitizeVersion(version: string): string {
  return version.replace(/^v/, "").trim();
}

/**
 * Re-export terminal resize testing utilities from helpers/ directory
 *
 * These advanced helpers provide ANSI parsing, layout validation,
 * and terminal width simulation for comprehensive CLI testing.
 */
export {
  BREAKPOINTS,
  captureAtWidth,
  countAnsiCodes,
  extractAnsiCodes,
  getVisualWidth,
  hasAnsiCodes,
  type LineWidthOptions,
  // Width simulation
  renderWithWidth,
  type SeparatorOptions,
  // ANSI utilities
  stripAnsi,
  testAcrossBreakpoints,
  testAcrossWidthRange,
  testAcrossWidths,
  type ValidationResult,
  validateAnsiBalance,
  validateCentering,
  // Layout validators
  validateLineWidths,
  validateSeparators,
  validateVerticalAlignment,
  type WidthRangeOptions,
} from "./helpers/index";
