import { describe, expect, test } from "vitest";
import {
  BREAKPOINTS,
  getBinaryPath,
  getVisualWidth,
  renderWithWidth,
  stripAnsi,
  testAcrossWidths,
  validateLineWidths,
} from "./helpers";

/**
 * Example tests demonstrating terminal resize testing utilities
 *
 * This file shows how to use the new helper functions for testing
 * CLI output across different terminal widths.
 */

describe("Terminal Resize - Example Tests", () => {
  test("example: render at specific width", async () => {
    const result = await renderWithWidth(getBinaryPath(), ["--version"], 80);

    // Wait for output to be captured
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const output = result.getStdallStr();

    // Verify output was captured
    expect(output.length).toBeGreaterThan(0);

    // Validate line widths
    const validation = validateLineWidths(output, 80, 5);
    expect(validation.errors).toEqual([]);
  });

  test("example: validate visual width with ANSI codes", async () => {
    const plainText = "Hello World";
    const coloredText = "\x1b[32mHello World\x1b[0m";
    const emojiText = "Hello 👋";

    // Plain text width
    expect(getVisualWidth(plainText)).toBe(11);

    // Colored text width (ANSI codes removed)
    expect(getVisualWidth(coloredText)).toBe(11);

    // Strip ANSI codes
    expect(stripAnsi(coloredText)).toBe(plainText);

    // Emoji text width (emoji counts as 2)
    expect(getVisualWidth(emojiText)).toBe(8);
  });

  test("example: test across multiple widths", async () => {
    const widths = [40, 60, 80];

    await testAcrossWidths(getBinaryPath(), ["--version"], widths, (output, width) => {
      // Validate that output respects the terminal width
      const lines = stripAnsi(output).split("\n");

      for (const line of lines) {
        const lineWidth = getVisualWidth(line);
        // Allow some tolerance for very short version strings
        if (lineWidth > 0) {
          expect(lineWidth).toBeLessThanOrEqual(width + 5);
        }
      }
    });
  });

  test("example: check breakpoints", () => {
    // Verify standard breakpoints are available
    expect(BREAKPOINTS.VERY_NARROW).toBe(25);
    expect(BREAKPOINTS.NARROW).toBe(40);
    expect(BREAKPOINTS.MEDIUM).toBe(60);
    expect(BREAKPOINTS.WIDE).toBe(100);
    expect(BREAKPOINTS.ULTRA_WIDE).toBe(120);
  });
});
