import { getVisualWidth, stripAnsi } from "./ansi-utils";

/**
 * Layout validation utilities for terminal output testing
 *
 * These validators help ensure proper formatting, alignment, and layout
 * consistency in CLI applications across different terminal widths.
 */

/**
 * Detailed validation result with error context
 */
export interface ValidationResult {
  /** Whether validation passed */
  valid: boolean;
  /** Detailed error messages if validation failed */
  errors: string[];
  /** Additional context about the validation */
  context?: Record<string, unknown>;
}

/**
 * Options for line width validation
 */
export interface LineWidthOptions {
  /** Maximum allowed width in columns */
  maxWidth: number;
  /** Tolerance for width violations (default: 0) */
  tolerance?: number;
  /** Whether to ignore empty lines (default: true) */
  ignoreEmptyLines?: boolean;
}

/**
 * Validates that all lines in output respect maximum width constraints
 *
 * @param output - Multi-line terminal output to validate
 * @param maxWidth - Maximum allowed width in columns
 * @param tolerance - Optional tolerance for width violations (default: 0)
 * @returns Validation result with detailed error information
 *
 * @example
 * ```typescript
 * const output = 'Short line\nThis is a very long line that exceeds width';
 * const result = validateLineWidths(output, 20);
 * if (!result.valid) {
 *   console.error(result.errors); // ['Line 2 exceeds max width: 44 > 20']
 * }
 * ```
 */
export function validateLineWidths(
  output: string,
  maxWidth: number,
  tolerance = 0,
): ValidationResult {
  const lines = output.split("\n");
  const errors: string[] = [];
  const violatingLines: { line: number; width: number; content: string }[] = [];

  lines.forEach((line, index) => {
    // Skip empty lines if configured
    const cleanLine = stripAnsi(line);
    if (cleanLine.trim() === "") {
      return;
    }

    const width = getVisualWidth(line);
    const effectiveMax = maxWidth + tolerance;

    if (width > effectiveMax) {
      const lineNum = index + 1;
      violatingLines.push({ line: lineNum, width, content: cleanLine });
      errors.push(
        `Line ${lineNum} exceeds max width: ${width} > ${effectiveMax} (tolerance: ${tolerance})`,
      );
    }
  });

  return {
    valid: errors.length === 0,
    errors,
    context: {
      totalLines: lines.length,
      violatingLines: violatingLines.length,
      maxWidth,
      tolerance,
      violations: violatingLines,
    },
  };
}

/**
 * Validates that a line is properly centered within the expected width
 *
 * @param line - Single line of text to validate
 * @param expectedWidth - Expected terminal width for centering
 * @param tolerance - Allowed deviation from perfect centering (default: 1)
 * @returns True if the line is properly centered
 *
 * @example
 * ```typescript
 * validateCentering('  Hello  ', 10);     // true (centered in 10 chars)
 * validateCentering('Hello', 10);         // false (not centered)
 * validateCentering('    Text    ', 20);  // depends on tolerance
 * ```
 */
export function validateCentering(line: string, expectedWidth: number, tolerance = 1): boolean {
  const cleanLine = stripAnsi(line);
  const contentWidth = getVisualWidth(cleanLine.trim());
  const totalWidth = getVisualWidth(cleanLine);

  // Calculate padding on both sides
  const leftPadding = cleanLine.search(/\S/);
  const rightPadding = cleanLine.length - cleanLine.trimEnd().length;

  // For proper centering, left and right padding should be approximately equal
  const paddingDiff = Math.abs(leftPadding - rightPadding);

  // Also verify the total width is reasonable for the expected width
  const expectedPadding = Math.floor((expectedWidth - contentWidth) / 2);
  const actualPadding = Math.floor((totalWidth - contentWidth) / 2);
  const paddingError = Math.abs(expectedPadding - actualPadding);

  return paddingDiff <= tolerance && paddingError <= tolerance;
}

/**
 * Options for separator validation
 */
export interface SeparatorOptions {
  /** Character(s) used for separators */
  separatorChar?: string;
  /** Minimum expected separator width */
  minWidth?: number;
  /** Maximum expected separator width */
  maxWidth?: number;
  /** Whether all separators must have identical width */
  requireConsistentWidth?: boolean;
}

/**
 * Validates that separator lines have consistent widths
 *
 * Checks for lines that appear to be separators (e.g., "─────", "=====")
 * and validates their consistency.
 *
 * @param output - Multi-line terminal output to validate
 * @param options - Validation options
 * @returns Validation result with detailed error information
 *
 * @example
 * ```typescript
 * const output = '─────────\nContent\n─────────';
 * const result = validateSeparators(output);
 * // result.valid === true (separators have same width)
 *
 * const badOutput = '─────\nContent\n──────────';
 * const badResult = validateSeparators(badOutput);
 * // badResult.valid === false (inconsistent separator widths)
 * ```
 */
export function validateSeparators(
  output: string,
  options: SeparatorOptions = {},
): ValidationResult {
  const { separatorChar, minWidth, maxWidth, requireConsistentWidth = true } = options;

  const lines = output.split("\n");
  const errors: string[] = [];
  const separatorLines: { line: number; width: number; char: string }[] = [];

  // Common separator characters
  const separatorChars = separatorChar ? [separatorChar] : ["─", "═", "-", "=", "_", "━", "—"];

  lines.forEach((line, index) => {
    const cleanLine = stripAnsi(line).trim();

    // Check if line is a separator (mostly repeated characters)
    for (const char of separatorChars) {
      const escapedChar = char.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
      const pattern = new RegExp(`^${escapedChar}+$`);
      if (pattern.test(cleanLine)) {
        const width = getVisualWidth(cleanLine);
        separatorLines.push({ line: index + 1, width, char });

        if (minWidth && width < minWidth) {
          errors.push(`Separator on line ${index + 1} is too narrow: ${width} < ${minWidth}`);
        }
        if (maxWidth && width > maxWidth) {
          errors.push(`Separator on line ${index + 1} is too wide: ${width} > ${maxWidth}`);
        }
        break;
      }
    }
  });

  // Check consistency if required
  if (requireConsistentWidth && separatorLines.length > 1) {
    const widths = separatorLines.map((s) => s.width);
    const firstWidth = widths[0];
    const inconsistent = widths.some((w) => w !== firstWidth);

    if (inconsistent) {
      const widthDetails = separatorLines.map((s) => `line ${s.line}: ${s.width}`).join(", ");
      errors.push(`Inconsistent separator widths: ${widthDetails}`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    context: {
      separatorCount: separatorLines.length,
      separators: separatorLines,
      requireConsistentWidth,
    },
  };
}

/**
 * Validates that output has proper vertical alignment
 *
 * Checks for consistent indentation and alignment of multi-column layouts
 *
 * @param output - Multi-line terminal output to validate
 * @param expectedColumns - Expected number of columns in aligned content
 * @returns Validation result with detailed error information
 */
export function validateVerticalAlignment(
  output: string,
  expectedColumns: number,
): ValidationResult {
  const lines = output.split("\n").filter((l) => stripAnsi(l).trim() !== "");
  const errors: string[] = [];

  // Extract column positions from first line
  const firstLine = lines[0] ? stripAnsi(lines[0]) : "";
  const columnPositions: number[] = [];

  let inWord = false;
  for (let i = 0; i < firstLine.length; i++) {
    const isSpace = /\s/.test(firstLine[i]);
    if (!isSpace && !inWord) {
      columnPositions.push(i);
      inWord = true;
    } else if (isSpace && inWord) {
      inWord = false;
    }
  }

  if (columnPositions.length !== expectedColumns) {
    errors.push(
      `Expected ${expectedColumns} columns but found ${columnPositions.length} in first line`,
    );
  }

  // Validate subsequent lines have content at same positions
  lines.slice(1).forEach((line, idx) => {
    const cleanLine = stripAnsi(line);
    columnPositions.forEach((pos, colIdx) => {
      if (pos < cleanLine.length && /\s/.test(cleanLine[pos])) {
        errors.push(`Line ${idx + 2} column ${colIdx + 1} misaligned at position ${pos}`);
      }
    });
  });

  return {
    valid: errors.length === 0,
    errors,
    context: {
      expectedColumns,
      detectedColumns: columnPositions.length,
      columnPositions,
    },
  };
}
