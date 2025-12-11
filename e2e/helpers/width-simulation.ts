import type { RenderResult } from "cli-testing-library";
import { render } from "cli-testing-library";

/**
 * Terminal width testing utilities for cli-testing-library
 *
 * These utilities help test CLI applications across different terminal widths
 * to ensure proper responsive layout and formatting.
 */

/**
 * Standard terminal width breakpoints for testing
 *
 * These represent common terminal sizes and edge cases:
 * - VERY_NARROW: Minimum viable terminal (edge case)
 * - NARROW: Small terminal window
 * - MEDIUM: Default terminal size on most systems
 * - WIDE: Larger terminal window
 * - ULTRA_WIDE: Very wide terminal (edge case)
 */
export const BREAKPOINTS = {
  VERY_NARROW: 25,
  NARROW: 40,
  MEDIUM: 60,
  WIDE: 100,
  ULTRA_WIDE: 120,
} as const;

/**
 * Renders a CLI binary with a specific terminal width
 *
 * @param binaryPath - Path to the CLI binary to test
 * @param args - Command line arguments to pass
 * @param width - Terminal width in columns
 * @param height - Terminal height in rows (default: 24)
 * @returns RenderResult from cli-testing-library
 *
 * @example
 * ```typescript
 * const result = await renderWithWidth('/path/to/cli', ['--help'], 80);
 * const output = result.getStdallStr();
 * ```
 */
export async function renderWithWidth(
  binaryPath: string,
  args: string[],
  width: number,
  height = 24,
): Promise<RenderResult> {
  // Set environment variables to simulate terminal size
  const env = {
    ...process.env,
    COLUMNS: width.toString(),
    LINES: height.toString(),
    TERM: "xterm-256color",
  };

  return await render(binaryPath, args, {
    spawnOpts: {
      env,
    },
  });
}

/**
 * Test a CLI binary across multiple terminal widths
 *
 * Executes a validator function for each width, useful for testing
 * responsive layout behavior.
 *
 * @param binaryPath - Path to the CLI binary to test
 * @param args - Command line arguments to pass
 * @param widths - Array of terminal widths to test
 * @param validator - Function to validate output at each width
 * @returns Promise that resolves when all validations complete
 *
 * @example
 * ```typescript
 * await testAcrossWidths(
 *   '/path/to/cli',
 *   ['list'],
 *   [40, 60, 80, 100],
 *   (output, width) => {
 *     // Validate that no line exceeds the terminal width
 *     const lines = output.split('\n');
 *     for (const line of lines) {
 *       const visualWidth = getVisualWidth(line);
 *       expect(visualWidth).toBeLessThanOrEqual(width);
 *     }
 *   }
 * );
 * ```
 */
export async function testAcrossWidths(
  binaryPath: string,
  args: string[],
  widths: number[],
  validator: (output: string, width: number) => void | Promise<void>,
): Promise<void> {
  for (const width of widths) {
    const result = await renderWithWidth(binaryPath, args, width);

    // Wait for output to be captured
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const output = result.getStdallStr();

    try {
      await validator(output, width);
    } catch (error) {
      // Enhance error message with width context
      if (error instanceof Error) {
        error.message = `Validation failed at width ${width}: ${error.message}`;
      }
      throw error;
    }
  }
}

/**
 * Test a CLI binary across all standard breakpoints
 *
 * Convenience function that tests against all predefined breakpoints.
 *
 * @param binaryPath - Path to the CLI binary to test
 * @param args - Command line arguments to pass
 * @param validator - Function to validate output at each width
 * @returns Promise that resolves when all validations complete
 *
 * @example
 * ```typescript
 * await testAcrossBreakpoints(
 *   '/path/to/cli',
 *   ['--help'],
 *   (output, width) => {
 *     const result = validateLineWidths(output, width);
 *     expect(result.valid).toBe(true);
 *   }
 * );
 * ```
 */
export async function testAcrossBreakpoints(
  binaryPath: string,
  args: string[],
  validator: (output: string, width: number) => void | Promise<void>,
): Promise<void> {
  const widths = Object.values(BREAKPOINTS);
  await testAcrossWidths(binaryPath, args, widths, validator);
}

/**
 * Options for width range testing
 */
export interface WidthRangeOptions {
  /** Starting width */
  min: number;
  /** Ending width */
  max: number;
  /** Step size between widths (default: 5) */
  step?: number;
}

/**
 * Test a CLI binary across a range of terminal widths
 *
 * Useful for finding edge cases and specific width thresholds where
 * layout changes occur.
 *
 * @param binaryPath - Path to the CLI binary to test
 * @param args - Command line arguments to pass
 * @param options - Width range configuration
 * @param validator - Function to validate output at each width
 * @returns Promise that resolves when all validations complete
 *
 * @example
 * ```typescript
 * await testAcrossWidthRange(
 *   '/path/to/cli',
 *   ['list'],
 *   { min: 30, max: 100, step: 10 },
 *   (output, width) => {
 *     // Find at which width the layout changes
 *     const lines = output.split('\n');
 *     console.log(`Width ${width}: ${lines.length} lines`);
 *   }
 * );
 * ```
 */
export async function testAcrossWidthRange(
  binaryPath: string,
  args: string[],
  options: WidthRangeOptions,
  validator: (output: string, width: number) => void | Promise<void>,
): Promise<void> {
  const { min, max, step = 5 } = options;
  const widths: number[] = [];

  for (let width = min; width <= max; width += step) {
    widths.push(width);
  }

  await testAcrossWidths(binaryPath, args, widths, validator);
}

/**
 * Captures output at a specific width without validation
 *
 * Useful for manual inspection or custom validation logic.
 *
 * @param binaryPath - Path to the CLI binary to test
 * @param args - Command line arguments to pass
 * @param width - Terminal width in columns
 * @param height - Terminal height in rows (default: 24)
 * @returns Promise resolving to the captured output string
 *
 * @example
 * ```typescript
 * const narrow = await captureAtWidth('/path/to/cli', ['list'], 40);
 * const wide = await captureAtWidth('/path/to/cli', ['list'], 100);
 * // Compare outputs
 * expect(narrow).not.toEqual(wide);
 * ```
 */
export async function captureAtWidth(
  binaryPath: string,
  args: string[],
  width: number,
  height = 24,
): Promise<string> {
  const result = await renderWithWidth(binaryPath, args, width, height);

  // Wait for output to be captured
  await new Promise((resolve) => setTimeout(resolve, 1000));

  return result.getStdallStr();
}
