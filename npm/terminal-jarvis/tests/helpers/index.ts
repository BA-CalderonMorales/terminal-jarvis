/**
 * Terminal Jarvis Test Helpers
 *
 * Comprehensive testing utilities for CLI applications using Vitest + cli-testing-library
 *
 * ## Modules
 *
 * ### ANSI Utilities (`ansi-utils.ts`)
 * - Parse and validate ANSI escape codes
 * - Calculate visual width of terminal output
 * - Strip ANSI codes for comparison
 *
 * ### Layout Validators (`layout-validators.ts`)
 * - Validate line widths and wrapping
 * - Check text centering and alignment
 * - Verify separator consistency
 *
 * ### Width Simulation (`width-simulation.ts`)
 * - Test CLI across different terminal widths
 * - Simulate responsive behavior
 * - Standard breakpoint testing
 *
 * @packageDocumentation
 */

// Re-export ANSI utilities
export {
  countAnsiCodes,
  extractAnsiCodes,
  getVisualWidth,
  hasAnsiCodes,
  stripAnsi,
  validateAnsiBalance,
} from "./ansi-utils";

// Re-export layout validators
export {
  type LineWidthOptions,
  type SeparatorOptions,
  type ValidationResult,
  validateCentering,
  validateLineWidths,
  validateSeparators,
  validateVerticalAlignment,
} from "./layout-validators";

// Re-export width simulation utilities
export {
  BREAKPOINTS,
  captureAtWidth,
  renderWithWidth,
  testAcrossBreakpoints,
  testAcrossWidthRange,
  testAcrossWidths,
  type WidthRangeOptions,
} from "./width-simulation";
