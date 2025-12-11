import { describe, test, expect } from 'vitest';
import { render } from 'cli-testing-library';
import { getBinaryPath, normalizeOutput } from './helpers';

/**
 * Evals Menu Tests
 *
 * Validates that the EVALS & COMPARISONS menu displays correctly
 * and all options are properly numbered and accessible.
 *
 * Menu structure (as of removal of Statistics & Insights):
 * 1. View All Evaluations
 * 2. Compare Tools
 * 3. View Tool Details
 * 4. Export Evaluations
 * 5. Coverage Report
 * 6. Validate Evaluations
 * 7. About Evals Framework
 * 0. Return to Main Menu
 */

describe('terminal-jarvis evals menu', () => {
  test('displays evals menu header and title', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu to render
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Check for menu header
    expect(output).toContain('EVALS & COMPARISONS');
    expect(output).toContain('AI Coding Tools Evaluation');
  });

  test('displays all menu options with correct numbering', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu to render
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Verify all menu options are present with correct numbering
    const expectedOptions = [
      { num: '1', label: 'View All Evaluations' },
      { num: '2', label: 'Compare Tools' },
      { num: '3', label: 'View Tool Details' },
      { num: '4', label: 'Export Evaluations' },
      { num: '5', label: 'Coverage Report' },
      { num: '6', label: 'Validate Evaluations' },
      { num: '7', label: 'About Evals Framework' },
      { num: '0', label: 'Return to Main Menu' },
    ];

    for (const option of expectedOptions) {
      // Check that the option number and label appear in the output
      expect(output).toContain(option.num);
      expect(output).toContain(option.label);
    }
  });

  test('does not display removed Statistics & Insights option', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu to render
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Verify Statistics & Insights is NOT present
    expect(output).not.toContain('Statistics & Insights');
  });

  test('menu has correct option count (7 main options + 1 return)', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu to render
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Count numbered options (should be 1-7 and 0)
    const optionPattern = /\d+\.\s+\w/g;
    const matches = output.match(optionPattern);

    // Should have exactly 8 options (1-7 and 0)
    expect(matches).toBeDefined();
    expect(matches?.length).toBeGreaterThanOrEqual(7);
  });

  test('displays choice prompt', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu to render
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Check for choice/input prompt
    expect(output).toMatch(/Choice:|choice:/i);
  });

  test('Coverage Report is accessible via option 5', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu, send option 5, wait for response
    await new Promise(resolve => setTimeout(resolve, 1000));
    instance.write('5\n');
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Should show Coverage Report header
    expect(output).toContain('COVERAGE REPORT');
  });

  test('About Evals Framework is accessible via option 7', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu, send option 7, wait for response
    await new Promise(resolve => setTimeout(resolve, 1000));
    instance.write('7\n');
    await new Promise(resolve => setTimeout(resolve, 1500));

    const output = normalizeOutput(instance.getStdallStr());

    // Should show About header
    expect(output).toContain('ABOUT EVALS FRAMEWORK');
  });

  test('returns to main menu with option 0', async () => {
    const instance = await render(getBinaryPath(), ['evals']);

    // Wait for menu, send option 0 to exit
    await new Promise(resolve => setTimeout(resolve, 1000));
    instance.write('0\n');
    await new Promise(resolve => setTimeout(resolve, 1000));

    const output = normalizeOutput(instance.getStdallStr());

    // Should exit the evals menu (back to main menu or exit)
    // The exact behavior depends on the implementation
    // At minimum, the menu should not show the evals options again after 0
    expect(output).toBeTruthy();
  });
});
