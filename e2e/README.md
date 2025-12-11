# E2E Tests

End-to-end tests for the terminal-jarvis CLI using [cli-testing-library](https://github.com/gmrchk/cli-testing-library) and Vitest.

## Structure

```
e2e/
├── index.test.ts    # Entry point - validates test structure
├── flows/           # Core CLI behavior tests
│   ├── help.test.ts
│   ├── version.test.ts
│   ├── error-handling.test.ts
│   └── list.test.ts
├── menus/           # Menu structure tests
│   ├── main-menu.test.ts
│   ├── auth-menu.test.ts
│   ├── config-menu.test.ts
│   └── templates-menu.test.ts
├── utils/           # Test utilities
│   ├── binary.ts    # Binary path resolution
│   └── output.ts    # Output normalization
└── config/          # Test configuration
```

## Usage

```bash
# Build the CLI first
cargo build --release

# Run tests
cd e2e
npm install
npm test
```

## vs tests/

| Folder | Technology | Purpose |
|--------|------------|---------|
| `e2e/` | TypeScript + Vitest | Tests CLI behavior from user perspective |
| `tests/` | Rust + cargo test | Tests internal Rust code units |

**e2e/** = "Does the CLI work correctly for users?"  
**tests/** = "Does the Rust code work correctly internally?"
