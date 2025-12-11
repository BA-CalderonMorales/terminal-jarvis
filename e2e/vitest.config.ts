import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    setupFiles: ['./setup.ts'],
    testTimeout: 30000,
    hookTimeout: 30000,
    teardownTimeout: 10000,
    include: [
      'index.test.ts',
      'flows/**/*.test.ts',
      'menus/**/*.test.ts',
    ],
    exclude: ['node_modules'],
  },
  resolve: {
    alias: {
      '@e2e': resolve(__dirname, '.'),
    },
  },
});
