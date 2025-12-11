import { describe, test, expect } from 'vitest';
import {
  getBinaryPath,
  normalizeOutput,
  renderWithWidth,
  testAcrossBreakpoints,
  BREAKPOINTS,
  validateLineWidths,
  validateCentering,
  stripAnsi,
  getVisualWidth
} from './helpers';

describe('Responsive Display Breakpoints', () => {
  describe('Logo Rendering by Terminal Width', () => {
    test('ultra-wide terminal (100+ chars): full spaced branding', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 120);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Ultra-wide should show full spaced branding
      expect(output).toContain('T E R M I N A L');
      expect(output).toContain('J A R V I S');
      expect(output).toContain('AI Coding Assistant Command Center');
    });

    test('wide terminal (80-99 chars): full spaced branding', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Wide terminal should show full spaced branding
      expect(output).toContain('T E R M I N A L');
      expect(output).toContain('J A R V I S');
    });

    test('medium terminal (50-79 chars): compact branding', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 60);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Medium should show compact branding
      expect(output).toContain('TERMINAL');
      expect(output).toContain('JARVIS');
      expect(output).toContain('AI Coding Assistant');
    });

    test('narrow terminal (30-49 chars): minimal branding', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 40);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Narrow should show minimal branding
      expect(output).toContain('TERMINAL JARVIS');
      expect(output).toContain('AI Coding Tools');
    });

    test('very narrow terminal (<30 chars): essential only', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 25);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());

      // Very narrow should show essential branding
      expect(output).toContain('T.JARVIS');
      // Should not show tagline at this width
      expect(output).not.toContain('AI Coding Assistant Command Center');
    });
  });

  describe('Breakpoint Boundary Testing', () => {
    test('exactly 80 chars: triggers wide layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('T E R M I N A L');
    });

    test('exactly 79 chars: triggers medium layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 79);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('TERMINAL');
      expect(output).not.toContain('T E R M I N A L');
    });

    test('exactly 50 chars: triggers medium layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 50);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('TERMINAL');
      expect(output).toContain('JARVIS');
    });

    test('exactly 49 chars: triggers narrow layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 49);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('TERMINAL JARVIS');
      expect(output).toContain('AI Coding Tools');
    });

    test('exactly 30 chars: triggers narrow layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 30);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('TERMINAL JARVIS');
    });

    test('exactly 29 chars: triggers very narrow layout', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 29);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = normalizeOutput(instance.getStdallStr());
      expect(output).toContain('T.JARVIS');
    });
  });

  describe('Line Width Validation', () => {
    test('all output lines respect terminal width constraints', async () => {
      await testAcrossBreakpoints(getBinaryPath(), ['--version'], (output, width) => {
        const plainOutput = stripAnsi(output);
        const result = validateLineWidths(plainOutput, width, { tolerance: 2 });

        if (!result.isValid) {
          console.error(`Width ${width} validation failed:`, result.errors);
        }

        expect(result.isValid).toBe(true);
      });
    });

    test('ultra-wide terminal: lines fit within 120 chars', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 120);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 120, { tolerance: 2 });

      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    test('narrow terminal: lines fit within 40 chars', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 40);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 40, { tolerance: 2 });

      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    test('very narrow terminal: lines fit within 25 chars', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 25);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 25, { tolerance: 2 });

      expect(result.isValid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });
  });

  describe('Content Centering Validation', () => {
    test('logo text is properly centered at wide width', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 100);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const lines = output.split('\n');

      // Find logo lines
      const logoLines = lines.filter(line => {
        const plain = stripAnsi(line);
        return plain.includes('TERMINAL') || plain.includes('JARVIS');
      });

      expect(logoLines.length).toBeGreaterThan(0);

      // Each logo line should be centered (has padding on both sides)
      for (const line of logoLines) {
        const plain = stripAnsi(line);
        const trimmed = plain.trim();

        if (trimmed.length > 0) {
          // Line should have symmetric or near-symmetric padding
          const isCentered = validateCentering(plain, 100);
          expect(isCentered).toBe(true);
        }
      }
    });

    test('version text is properly centered', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const lines = output.split('\n');

      // Find version line
      const versionLines = lines.filter(line => {
        const plain = stripAnsi(line);
        return /\d+\.\d+\.\d+/.test(plain);
      });

      expect(versionLines.length).toBeGreaterThan(0);

      for (const line of versionLines) {
        const plain = stripAnsi(line);
        const trimmed = plain.trim();

        if (trimmed.length > 0) {
          const isCentered = validateCentering(plain, 80);
          expect(isCentered).toBe(true);
        }
      }
    });
  });

  describe('ANSI Code Handling Across Widths', () => {
    test('ANSI codes present in all terminal widths', async () => {
      const widths = [25, 40, 60, 80, 100, 120];

      for (const width of widths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();

        // Should contain ANSI escape sequences for theming
        expect(output).toMatch(/\x1b\[\d+m/);
      }
    });

    test('ANSI codes do not affect width calculations', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 80);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const lines = output.split('\n');

      for (const line of lines) {
        const plainText = stripAnsi(line);
        const visualWidth = getVisualWidth(plainText);

        // Visual width should be reasonable for 80-char terminal
        // Using content_width = 80 - 10 = 70 for wide terminals
        expect(visualWidth).toBeLessThanOrEqual(82); // Allow small tolerance
      }
    });
  });

  describe('Unicode Character Support', () => {
    test('separator symbols render correctly across widths', async () => {
      const widths = [40, 60, 80, 100];

      for (const width of widths) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();
        const plainOutput = stripAnsi(output);

        // Check for separator character (─)
        expect(plainOutput).toMatch(/[─-]/);
      }
    });

    test('menu symbols render at supported widths', async () => {
      // Note: This test would require interactive mode, which is harder to test
      // For now, we verify the binary doesn't crash with menu commands
      const instance = await renderWithWidth(getBinaryPath(), ['help'], 60);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      expect(output).toBeDefined();
      expect(output.length).toBeGreaterThan(0);
    });
  });

  describe('Extreme Width Edge Cases', () => {
    test('minimum width (10 chars): graceful degradation', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 10);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const exitInfo = instance.hasExit();

      // Should not crash
      expect(exitInfo).toBeTruthy();
      expect(exitInfo?.exitCode).toBe(0);
      expect(output).toBeDefined();
    });

    test('extremely wide (200 chars): no overflow', async () => {
      const instance = await renderWithWidth(getBinaryPath(), ['--version'], 200);

      await new Promise(resolve => setTimeout(resolve, 1000));

      const output = instance.getStdallStr();
      const plainOutput = stripAnsi(output);
      const result = validateLineWidths(plainOutput, 200, { tolerance: 5 });

      expect(result.isValid).toBe(true);
    });
  });

  describe('Cross-Command Consistency', () => {
    test('help command respects terminal width', async () => {
      await testAcrossBreakpoints(getBinaryPath(), ['help'], (output, width) => {
        const plainOutput = stripAnsi(output);
        const result = validateLineWidths(plainOutput, width, { tolerance: 2 });

        expect(result.isValid).toBe(true);
      });
    });

    test('list command respects terminal width', async () => {
      await testAcrossBreakpoints(getBinaryPath(), ['list'], (output, width) => {
        const plainOutput = stripAnsi(output);
        const result = validateLineWidths(plainOutput, width, { tolerance: 2 });

        expect(result.isValid).toBe(true);
      });
    });

    test('version command respects terminal width', async () => {
      await testAcrossBreakpoints(getBinaryPath(), ['--version'], (output, width) => {
        const plainOutput = stripAnsi(output);
        const result = validateLineWidths(plainOutput, width, { tolerance: 2 });

        expect(result.isValid).toBe(true);
      });
    });
  });

  describe('Standard Breakpoint Constants', () => {
    test('BREAKPOINTS constants match expected values', () => {
      expect(BREAKPOINTS.VERY_NARROW).toBe(25);
      expect(BREAKPOINTS.NARROW).toBe(40);
      expect(BREAKPOINTS.MEDIUM).toBe(60);
      expect(BREAKPOINTS.WIDE).toBe(100);
      expect(BREAKPOINTS.ULTRA_WIDE).toBe(120);
    });

    test('all breakpoints produce valid output', async () => {
      const breakpointValues = Object.values(BREAKPOINTS);

      for (const width of breakpointValues) {
        const instance = await renderWithWidth(getBinaryPath(), ['--version'], width);
        await new Promise(resolve => setTimeout(resolve, 500));

        const output = instance.getStdallStr();
        const exitInfo = instance.hasExit();

        expect(exitInfo?.exitCode).toBe(0);
        expect(output).toBeDefined();
        expect(output.length).toBeGreaterThan(0);
      }
    });
  });
});
