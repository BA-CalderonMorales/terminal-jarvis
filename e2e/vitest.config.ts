import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    setupFiles: ['./setup.ts'],
    testTimeout: 30000, // 30s timeout for CLI tests
    hookTimeout: 30000,
    teardownTimeout: 10000,
    include: ['**/*.test.ts'],
    exclude: ['node_modules'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['**/*.ts'],
      exclude: ['**/*.test.ts', 'setup.ts'],
    },
  },
  resolve: {
    alias: {
      '@e2e': resolve(__dirname, '.'),
    },
  },
});
