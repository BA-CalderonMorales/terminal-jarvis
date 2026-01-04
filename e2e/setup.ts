import { expect } from 'vitest';
import 'cli-testing-library/extend-expect';

// Global test configuration
process.env.CI = 'true'; // Ensure non-interactive mode for tests
process.env.NO_COLOR = '1'; // Disable color output for easier assertions
