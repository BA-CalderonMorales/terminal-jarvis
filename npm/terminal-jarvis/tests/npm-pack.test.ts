import { describe, test, expect, beforeAll, afterAll } from 'vitest';
import { render } from 'cli-testing-library';
import { createPackageTestEnvironment, type PackageTestContext } from './helpers';

describe('terminal-jarvis via npm pack (package installation)', () => {
  let ctx: PackageTestContext;

  beforeAll(async () => {
    ctx = createPackageTestEnvironment();
  }, 60000); // 60s timeout for npm pack + install

  afterAll(() => {
    if (ctx) {
      ctx.cleanup();
    }
  });

  test('installs successfully via npm pack', () => {
    expect(ctx.testDir).toBeTruthy();
    expect(ctx.packagePath).toBeTruthy();
  });

  test('can execute jarvis command after installation', async () => {
    const { findByText } = await render('npx', ['terminal-jarvis', '--version'], {
      cwd: ctx.testDir
    });

    expect(await findByText(/\d+\.\d+\.\d+/)).toBeInTheConsole();
  });

  test('help command works from installed package', async () => {
    const { findByText } = await render('npx', ['terminal-jarvis', '--help'], {
      cwd: ctx.testDir
    });

    expect(await findByText('Usage:')).toBeInTheConsole();
    expect(await findByText('Commands:')).toBeInTheConsole();
  });

  test('installed version matches package.json', async () => {
    const { findByText } = await render('npx', ['terminal-jarvis', '--version'], {
      cwd: ctx.testDir
    });

    expect(await findByText(new RegExp(ctx.version))).toBeInTheConsole();
  });

  test('package includes all necessary files', async () => {
    const { findByText } = await render('npx', ['terminal-jarvis', '--help'], {
      cwd: ctx.testDir
    });

    // If help works, it means the binary and configs are properly packaged
    expect(await findByText('Usage:')).toBeInTheConsole();
  });
});
