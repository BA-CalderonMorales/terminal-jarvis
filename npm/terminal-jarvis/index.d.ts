/**
 * Terminal Jarvis - TypeScript Type Definitions
 *
 * A thin wrapper that provides a unified interface for managing
 * and running AI coding tools including Claude, Gemini, Qwen,
 * OpenCode, Codex, Aider, Goose, Amp, Crush, LLXPRT, and more.
 *
 * @packageDocumentation
 */

/**
 * Options for running Terminal Jarvis
 */
export interface JarvisOptions {
  /** Run in headless mode without interactive prompts */
  headless?: boolean;
  /** UI theme to use */
  theme?: "default" | "classic" | "matrix";
  /** Enable verbose logging */
  verbose?: boolean;
  /** Configuration file path */
  config?: string;
}

/**
 * Execute a terminal-jarvis command with the specified tool
 *
 * @param tool - The AI tool to run (e.g., "claude", "gemini", "codex")
 * @param options - Optional configuration for the execution
 * @returns Promise that resolves when the command completes
 *
 * @example
 * ```typescript
 * import { run } from "terminal-jarvis";
 *
 * await run("claude", { theme: "matrix" });
 * ```
 */
export function run(tool: string, options?: JarvisOptions): Promise<void>;

/**
 * Execute terminal-jarvis with raw command arguments
 *
 * @param args - Array of command-line arguments to pass to terminal-jarvis
 * @returns Promise that resolves with the exit code
 *
 * @example
 * ```typescript
 * import { runRaw } from "terminal-jarvis";
 *
 * const exitCode = await runRaw(["--version"]);
 * ```
 */
export function runRaw(args: string[]): Promise<number>;

/**
 * Get the path to the bundled Rust binary
 *
 * @returns The absolute path to the binary, or null if not found
 */
export function getBinaryPath(): string | null;

/**
 * Check if terminal-jarvis is properly installed and available
 *
 * @returns Promise that resolves to true if available
 */
export function isAvailable(): Promise<boolean>;

/**
 * Supported AI tools that can be managed through Terminal Jarvis
 */
export type SupportedTool =
  | "claude"
  | "gemini"
  | "qwen"
  | "opencode"
  | "codex"
  | "aider"
  | "goose"
  | "amp"
  | "crush"
  | "llxprt"
  | string;

/**
 * Version information for Terminal Jarvis
 */
export interface VersionInfo {
  /** NPM wrapper version */
  npmVersion: string;
  /** Core Rust binary version */
  coreVersion: string;
  /** Platform binary was built for */
  platform: string;
}

/**
 * Get version information for Terminal Jarvis
 *
 * @returns Promise that resolves with version information
 */
export function getVersion(): Promise<VersionInfo>;
