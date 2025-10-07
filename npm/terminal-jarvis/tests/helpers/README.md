# Terminal Jarvis Test Helpers

Comprehensive testing utilities for CLI applications using Vitest + cli-testing-library.

## Modules

### ANSI Utilities (`ansi-utils.ts`)

Parse and validate ANSI escape codes commonly used in terminal output.

**Key Functions:**
- `stripAnsi(text)` - Remove all ANSI escape codes
- `getVisualWidth(text)` - Calculate Unicode-aware terminal width
- `extractAnsiCodes(text)` - Extract all ANSI sequences
- `validateAnsiBalance(text)` - Verify proper code pairing
- `hasAnsiCodes(text)` - Check if text contains ANSI codes
- `countAnsiCodes(text)` - Count ANSI sequences

**Example:**
```typescript
import { stripAnsi, getVisualWidth } from './helpers';

const colored = '\x1b[32mHello World\x1b[0m';
stripAnsi(colored); // 'Hello World'
getVisualWidth(colored); // 11 (ANSI codes don't count)
getVisualWidth('Hello 👋'); // 7 (emoji counts as 2)
```

### Layout Validators (`layout-validators.ts`)

Validate terminal layout, alignment, and formatting.

**Key Functions:**
- `validateLineWidths(output, maxWidth, tolerance?)` - Check line width constraints
- `validateCentering(line, expectedWidth, tolerance?)` - Verify text centering
- `validateSeparators(output, options?)` - Check separator consistency
- `validateVerticalAlignment(output, expectedColumns)` - Validate column alignment

**Example:**
```typescript
import { validateLineWidths, validateSeparators } from './helpers';

const output = 'Short line\nThis is a very long line that exceeds the maximum width';
const result = validateLineWidths(output, 40);

if (!result.valid) {
  console.error(result.errors);
  // ['Line 2 exceeds max width: 56 > 40 (tolerance: 0)']
}

// Check separator consistency
const withSeparators = '─────────\nContent\n─────────';
const separatorResult = validateSeparators(withSeparators);
expect(separatorResult.valid).toBe(true);
```

### Width Simulation (`width-simulation.ts`)

Test CLI applications across different terminal widths.

**Key Functions:**
- `renderWithWidth(binaryPath, args, width, height?)` - Render at specific width
- `testAcrossWidths(binaryPath, args, widths, validator)` - Test multiple widths
- `testAcrossBreakpoints(binaryPath, args, validator)` - Test standard breakpoints
- `testAcrossWidthRange(binaryPath, args, options, validator)` - Test width range
- `captureAtWidth(binaryPath, args, width, height?)` - Capture output at width

**Standard Breakpoints:**
- `BREAKPOINTS.VERY_NARROW` - 25 columns
- `BREAKPOINTS.NARROW` - 40 columns
- `BREAKPOINTS.MEDIUM` - 60 columns
- `BREAKPOINTS.WIDE` - 100 columns
- `BREAKPOINTS.ULTRA_WIDE` - 120 columns

**Example:**
```typescript
import { testAcrossBreakpoints, validateLineWidths, getBinaryPath } from './helpers';

describe('Responsive Layout Tests', () => {
  test('respects terminal width at all breakpoints', async () => {
    await testAcrossBreakpoints(
      getBinaryPath(),
      ['--help'],
      (output, width) => {
        const result = validateLineWidths(output, width);
        expect(result.valid).toBe(true);
      }
    );
  });
});
```

## Complete Example Test

```typescript
import { describe, test, expect } from 'vitest';
import {
  getBinaryPath,
  renderWithWidth,
  testAcrossWidths,
  validateLineWidths,
  getVisualWidth,
  stripAnsi,
  BREAKPOINTS
} from './helpers';

describe('Terminal Resize Tests', () => {
  test('list command respects terminal width', async () => {
    const result = await renderWithWidth(getBinaryPath(), ['list'], 80);
    
    await new Promise(resolve => setTimeout(resolve, 1000));
    const output = result.getStdallStr();
    
    // Validate all lines fit within 80 columns
    const validation = validateLineWidths(output, 80, 2);
    expect(validation.valid).toBe(true);
  });

  test('adapts to narrow terminals', async () => {
    const narrow = await renderWithWidth(getBinaryPath(), ['list'], 40);
    const wide = await renderWithWidth(getBinaryPath(), ['list'], 100);
    
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    const narrowOutput = narrow.getStdallStr();
    const wideOutput = wide.getStdallStr();
    
    // Outputs should differ based on width
    expect(narrowOutput).not.toEqual(wideOutput);
    
    // Both should respect their respective widths
    expect(validateLineWidths(narrowOutput, 40).valid).toBe(true);
    expect(validateLineWidths(wideOutput, 100).valid).toBe(true);
  });

  test('validates across all breakpoints', async () => {
    await testAcrossWidths(
      getBinaryPath(),
      ['--version'],
      [BREAKPOINTS.NARROW, BREAKPOINTS.MEDIUM, BREAKPOINTS.WIDE],
      (output, width) => {
        // Check width constraints
        const lines = stripAnsi(output).split('\n');
        for (const line of lines) {
          const lineWidth = getVisualWidth(line);
          expect(lineWidth).toBeLessThanOrEqual(width);
        }
      }
    );
  });
});
```

## Integration with Existing Helpers

These new helpers work seamlessly with the existing test utilities:

```typescript
// Existing helpers
import { getBinaryPath, normalizeOutput } from './helpers';

// New helpers
import { validateLineWidths, renderWithWidth } from './helpers';

// Use together
const result = await renderWithWidth(getBinaryPath(), ['list'], 80);
const normalized = normalizeOutput(result.getStdallStr());
const validation = validateLineWidths(normalized, 80);
```

## Best Practices

1. **Always wait for output**: Use `await new Promise(resolve => setTimeout(resolve, 1000))` after render
2. **Use tolerance wisely**: Allow small deviations with tolerance parameter
3. **Test edge cases**: Always test VERY_NARROW and ULTRA_WIDE breakpoints
4. **Normalize before validation**: Use `stripAnsi()` when testing layout without colors
5. **Validate incrementally**: Test one aspect at a time for clearer error messages

## TypeScript Support

All helpers include comprehensive TypeScript types and JSDoc documentation:

```typescript
interface ValidationResult {
  valid: boolean;
  errors: string[];
  context?: Record<string, unknown>;
}

interface SeparatorOptions {
  separatorChar?: string;
  minWidth?: number;
  maxWidth?: number;
  requireConsistentWidth?: boolean;
}
```

## Dependencies

- `string-width` ^5.1.2 - Unicode-aware width calculation
- `cli-testing-library` ^3.0.1 - CLI testing framework
- `vitest` ^3.2.4 - Test runner
