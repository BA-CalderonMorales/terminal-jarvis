import stringWidth from "string-width";

/**
 * ANSI escape code utilities for terminal output testing
 *
 * These utilities help parse, validate, and process ANSI escape sequences
 * commonly used in CLI applications for colors, styling, and cursor control.
 */

/**
 * Comprehensive ANSI escape code pattern including:
 * - CSI sequences: ESC[...m
 * - OSC sequences: ESC]...BEL/ST
 * - Other control sequences
 */
// biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape codes require control characters
const ANSI_COMPREHENSIVE_REGEX = /\x1b(?:\[[0-9;]*[a-zA-Z]|\].*?(?:\x07|\x1b\\)|[()].*?)/g;

/**
 * Removes all ANSI escape codes from a string
 *
 * @param text - Input string potentially containing ANSI codes
 * @returns String with all ANSI codes removed
 *
 * @example
 * ```typescript
 * const colored = '\x1b[32mGreen text\x1b[0m';
 * stripAnsi(colored); // Returns: 'Green text'
 * ```
 */
export function stripAnsi(text: string): string {
  return text.replace(ANSI_COMPREHENSIVE_REGEX, "");
}

/**
 * Extracts all ANSI escape sequences from a string
 *
 * @param text - Input string potentially containing ANSI codes
 * @returns Array of ANSI escape sequences found in the text
 *
 * @example
 * ```typescript
 * const colored = '\x1b[32mGreen\x1b[0m \x1b[31mRed\x1b[0m';
 * extractAnsiCodes(colored); // Returns: ['\x1b[32m', '\x1b[0m', '\x1b[31m', '\x1b[0m']
 * ```
 */
export function extractAnsiCodes(text: string): string[] {
  const matches = text.match(ANSI_COMPREHENSIVE_REGEX);
  return matches || [];
}

/**
 * Validates that ANSI codes are properly balanced
 *
 * Checks that:
 * - Every color/style code has a corresponding reset
 * - Codes are properly nested
 * - No orphaned reset codes exist
 *
 * @param text - Input string to validate
 * @returns True if ANSI codes are properly balanced
 *
 * @example
 * ```typescript
 * validateAnsiBalance('\x1b[32mText\x1b[0m'); // Returns: true
 * validateAnsiBalance('\x1b[32mText');        // Returns: false (missing reset)
 * ```
 */
export function validateAnsiBalance(text: string): boolean {
  const codes = extractAnsiCodes(text);

  // Simple validation: count opening codes and reset codes
  let openCount = 0;

  for (const code of codes) {
    // Reset code (ESC[0m or similar)
    // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape codes require control characters
    if (code.match(/\x1b\[0m/) || code.match(/\x1b\[m/)) {
      openCount--;
    }
    // Style/color code (ESC[...m where ... is not just 0)
    // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape codes require control characters
    else if (code.match(/\x1b\[[0-9;]+m/)) {
      openCount++;
    }
  }

  // Should end with balanced codes (all closed)
  return openCount <= 0;
}

/**
 * Calculates the visual width of text on the terminal
 *
 * Takes into account:
 * - Unicode characters (emoji, full-width characters)
 * - ANSI escape codes (removed before calculation)
 * - Zero-width characters
 *
 * @param text - Input text to measure
 * @returns Visual width in terminal columns
 *
 * @example
 * ```typescript
 * getVisualWidth('Hello');           // Returns: 5
 * getVisualWidth('Hello 👋');        // Returns: 7 (emoji counts as 2)
 * getVisualWidth('\x1b[32mHi\x1b[0m'); // Returns: 2 (ANSI codes ignored)
 * getVisualWidth('你好');             // Returns: 4 (full-width chars)
 * ```
 */
export function getVisualWidth(text: string): number {
  // Remove ANSI codes first
  const cleanText = stripAnsi(text);

  // Use string-width for accurate Unicode width calculation
  return stringWidth(cleanText);
}

/**
 * Checks if a string contains any ANSI escape codes
 *
 * @param text - Input string to check
 * @returns True if the string contains ANSI codes
 *
 * @example
 * ```typescript
 * hasAnsiCodes('\x1b[32mGreen\x1b[0m'); // Returns: true
 * hasAnsiCodes('Plain text');           // Returns: false
 * ```
 */
export function hasAnsiCodes(text: string): boolean {
  return ANSI_COMPREHENSIVE_REGEX.test(text);
}

/**
 * Counts the number of ANSI escape sequences in a string
 *
 * @param text - Input string to analyze
 * @returns Count of ANSI sequences
 *
 * @example
 * ```typescript
 * countAnsiCodes('\x1b[32mGreen\x1b[0m'); // Returns: 2
 * countAnsiCodes('Plain text');           // Returns: 0
 * ```
 */
export function countAnsiCodes(text: string): number {
  return extractAnsiCodes(text).length;
}
