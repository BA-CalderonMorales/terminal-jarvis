# Benchmark Scenarios

This directory contains TOML-based benchmark scenarios for automated testing of AI coding tools.

## Directory Structure

```
scenarios/
├── code-completion/     # Code completion benchmarks
│   └── basic-001.toml  # Basic function completion
├── refactoring/        # Code refactoring benchmarks (future)
├── debugging/          # Bug fixing benchmarks (future)
└── README.md           # This file
```

## Scenario Format

Each scenario is defined in a TOML file with the following structure:

```toml
[metadata]
id = "unique-scenario-id"           # Unique identifier
name = "Human-readable name"        # Display name
category = "category-name"          # Category (code-completion, refactoring, etc.)
version = "1.0.0"                   # Semantic version
difficulty = "basic"                # Difficulty level (basic, intermediate, advanced)

[prompt]
template = "Prompt for the AI tool"  # Template for the test prompt

[validation]
type = "pattern-match"               # Validation type
expected_patterns = ["regex1", "regex2"]  # Expected patterns in response

[scoring]
points_possible = 10                 # Maximum points
pass_threshold = 0.75                # Minimum score to pass (0.0-1.0)
```

## Usage

The `BenchmarkRegistry` automatically loads all scenarios from this directory:

```rust
use terminal_jarvis::evals::benchmarks::BenchmarkRegistry;

let registry = BenchmarkRegistry::from_directory("config/benchmarks/scenarios")?;
let scenarios = registry.list_scenarios();
```

## Adding New Scenarios

1. Create a new TOML file in the appropriate category directory
2. Follow the format shown above
3. Choose a unique ID (format: `category-difficulty-number`)
4. Add validation patterns and scoring criteria
5. Test your scenario with the benchmark runner

## Categories

- **code-completion**: Testing code completion capabilities
- **refactoring**: Code restructuring and improvement (planned)
- **debugging**: Bug identification and fixing (planned)
- **documentation**: Code documentation generation (planned)
- **testing**: Test generation capabilities (planned)

## Difficulty Levels

- **basic**: Simple, straightforward tasks
- **intermediate**: More complex scenarios with multiple steps
- **advanced**: Complex, real-world challenges
