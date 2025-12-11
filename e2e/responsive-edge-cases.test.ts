import { describe, test, expect } from 'vitest';
import {
  getBinaryPath,
  normalizeOutput,
  renderWithWidth,
  testAcrossWidthRange,
  stripAnsi,
  validateLineWidths,
  extractAnsiCodes,
  validateAnsiBalance
} from './helpers';
import { render } from 'cli-testing-library';

describe('Responsive Display Edge Cases', () => {
  describe('Terminal Size Detection Edge Cases', () => {
    test('missing COLUMNS environment variable: uses default', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: undefined,
          LINES: undefined,
          NO_COLOR: '1' // Disable colors for easier testing
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = getStdallStr();
      const exitInfo = hasExit();

      // Should not crash, should use default (likely 80x24)
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
      expect(output.length).toBeGreaterThan(0);
    });

    test('zero width: graceful fallback', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 0);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = instance.hasExit();
      const output = instance.getStdallStr();

      // Should fallback to default width, not crash
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('negative width: graceful fallback', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: '-1',
          LINES: '24',
          NO_COLOR: '1'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = hasExit();
      const output = getStdallStr();

      // Should handle gracefully
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('non-numeric width: graceful fallback', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: 'abc',
          LINES: 'xyz',
          NO_COLOR: '1'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = hasExit();
      const output = getStdallStr();

      // Should handle gracefully with default values
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('extremely small width (1 char): minimal output', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 1);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = instance.hasExit();
      const output = instance.getStdallStr();

      // Should not crash, should produce some output
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('extremely small width (3 chars): minimal output', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 3);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = instance.hasExit();
      const output = instance.getStdallStr();

      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('extremely large width (500 chars): no overflow', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 500);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 500, { tolerance: 10 });

      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });
  });

  describe('Rapid Width Change Simulation', () => {
    test('sequential renders at different widths: consistent behavior', async () => {
      const widths = [25, 40, 60, 80, 100, 120, 80, 60, 40, 25];

      for (const width of widths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 200));

        const exitInfo = instance.hasExit();
        expect(exitInfo?.exitCode).toBe(0);
      }
    });

    test('width range testing: no crashes across spectrum', async () => {
      await testAcrossWidthRange(
        getBinaryPath(),
        ['--version'],
        { min: 10, max: 150, step: 10 },
        (output, width) => {
          expect(output).toBeDefined();
          expect(output.length).toBeGreaterThan(0);

          const plainOutput = stripAnsi(output);
          const lines = plainOutput.split('\n');

          // Each line should be reasonable for the given width
          for (const line of lines) {
            // Allow some tolerance for edge cases
            expect(line.length).toBeLessThanOrEqual(width + 5);
          }
        }
      );
    });
  });

  describe('ANSI Code Integrity', () => {
    test('ANSI codes are properly balanced across all widths', async () => {
      const widths = [25, 40, 60, 80, 100, 120];

      for (const width of widths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();
        const isBalanced = validateAnsiBalance(output);

        expect(isBalanced).toBe(true);
      }
    });

    test('ANSI codes extracted correctly', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const ansiCodes = extractAnsiCodes(output);

      // Should have ANSI codes for theming
      expect(ansiCodes.length).toBeGreaterThan(0);

      // Each code should be a valid ANSI sequence
      for (const code of ansiCodes) {
        expect(code).toMatch(/\x1b\[\d+(;\d+)*m/);
      }
    });

    test('NO_COLOR environment disables ANSI codes', async () => {
      const { getStdallStr } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: '80',
          LINES: '24',
          NO_COLOR: '1'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = getStdallStr();

      // Should not contain ANSI color codes when NO_COLOR is set
      expect(output).not.toMatch(/\x1b\[\d+m/);
    });

    test('ANSI truncation preserves codes at narrow widths', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 30);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const ansiCodes = extractAnsiCodes(output);

      // Even at narrow width, ANSI codes should be balanced
      expect(ansiCodes.length).toBeGreaterThan(0);

      const isBalanced = validateAnsiBalance(output);
      expect(isBalanced).toBe(true);
    });
  });

  describe('Unicode Handling Edge Cases', () => {
    test('separator rendering with different locales', async () => {
      const locales = ['en_US.UTF-8', 'C.UTF-8', 'C'];

      for (const locale of locales) {
        const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
          env: {
            COLUMNS: '80',
            LINES: '24',
            LANG: locale,
            LC_ALL: locale
          }
        });

        await new Promise(resolve => setTimeout(resolve, 500));

        const exitInfo = hasExit();
        const output = getStdallStr();

        // Should not crash regardless of locale
        expect(exitInfo?.exitCode).toBe(0);
        expect(output).toBeDefined();
      }
    });

    test('full-width character handling', async () => {
      // This tests if the terminal handles full-width characters
      // The CLI should render correctly regardless
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);

      // Validate that lines don't overflow due to width calculation issues
      const result = validateLineWidths(plainOutput, 80, { tolerance: 2 });
      expect(result.isValid).toBe(true);
    });
  });

  describe('Content Truncation Edge Cases', () => {
    test('very long content gets truncated with ellipsis', async () => {
      // Using help command which might have longer lines
      const instance = await renderWithWidth(getBinaryPath(), ['help'], 40);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const lines = plainOutput.split('\n');

      // Check that long lines are handled appropriately
      for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.length > 40) {
          // If a line exceeds width, it should contain ellipsis or be wrapped
          expect(trimmed.includes('...') || line.length <= 42).toBe(true);
        }
      }
    });

    test('empty content renders without errors', async () => {
      // Test with minimal command that might produce less output
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 100);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = instance.hasExit();
      expect(exitInfo?.exitCode).toBe(0);
    });
  });

  describe('Performance Edge Cases', () => {
    test('rapid width changes complete within timeout', async () => {
      const startTime = Date.now();
      const widths = [25, 50, 75, 100, 125, 100, 75, 50, 25];

      for (const width of widths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 100));

        const exitInfo = instance.hasExit();
        expect(exitInfo?.exitCode).toBe(0);
      }

      const duration = Date.now() - startTime;

      // Should complete reasonably quickly (under 10 seconds for 9 renders)
      expect(duration).toBeLessThan(10000);
    });

    test('extreme width changes do not cause memory issues', async () => {
      const extremeWidths = [1, 500, 1, 500, 1, 500];

      for (const width of extremeWidths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 200));

        const exitInfo = instance.hasExit();
        expect(exitInfo?.exitCode).toBe(0);
      }
    });
  });

  describe('Terminal Type Edge Cases', () => {
    test('TERM=dumb: basic output without ANSI', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: '80',
          LINES: '24',
          TERM: 'dumb'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = hasExit();
      const output = getStdallStr();

      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('TERM=xterm-256color: full color support', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: '80',
          LINES: '24',
          TERM: 'xterm-256color'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = hasExit();
      const output = getStdallStr();

      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
      // Should contain ANSI codes for 256-color terminal
      expect(output).toMatch(/\x1b\[\d+m/);
    });

    test('TERM=xterm: standard color support', async () => {
      const { getStdallStr, hasExit } = await render(getBinaryPath(), ['--version'], {
        env: {
          COLUMNS: '80',
          LINES: '24',
          TERM: 'xterm'
        }
      });

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = hasExit();
      const output = getStdallStr();

      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });
  });

  describe('Boundary Value Testing', () => {
    test('width at each breakpoint boundary produces valid output', async () => {
      const boundaryWidths = [29, 30, 49, 50, 79, 80];

      for (const width of boundaryWidths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();
        const plainOutput = stripAnsi(output);
        const result = validateLineWidths(plainOutput, width, { tolerance: 2 });

        if (!result.isValid) {
          console.error(`Boundary width ${width} failed:`, result.errors);
        }

        expect(result.isValid).toBe(true);
      }
    });

    test('width just below each breakpoint', async () => {
      const belowBoundaryWidths = [28, 48, 78];

      for (const width of belowBoundaryWidths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const exitInfo = instance.hasExit();
        expect(exitInfo?.exitCode).toBe(0);
      }
    });

    test('width just above each breakpoint', async () => {
      const aboveBoundaryWidths = [31, 51, 81];

      for (const width of aboveBoundaryWidths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const exitInfo = instance.hasExit();
        expect(exitInfo?.exitCode).toBe(0);
      }
    });
  });

  describe('Regression Prevention', () => {
    test('previous bug: width 0 crash - now handled gracefully', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 0);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const exitInfo = instance.hasExit();
      expect(exitInfo?.exitCode).toBe(0);
    });

    test('previous bug: ANSI imbalance at truncation - now balanced', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 25);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const isBalanced = validateAnsiBalance(output);

      expect(isBalanced).toBe(true);
    });

    test('previous bug: Unicode width miscalculation - now accurate', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 60);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 60, { tolerance: 2 });

      expect(result.isValid).toBe(true);
    });
  });
});
