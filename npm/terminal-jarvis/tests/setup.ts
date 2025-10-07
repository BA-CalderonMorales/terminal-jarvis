import { expect } from 'vitest';
import * as matchers from 'cli-testing-library/vitest';

// Extend Vitest's expect with cli-testing-library matchers
expect.extend(matchers);

// Global test configuration
process.env.CI = 'true'; // Ensure non-interactive mode for tests
process.env.NO_COLOR = '1'; // Disable color output for easier assertions
