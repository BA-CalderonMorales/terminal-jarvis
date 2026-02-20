# Manual Testing Guide for Tool Detection Improvements

This document lists the commands to manually verify that each supported tool can be correctly detected and executed.

For each tool, you can try:
1.  **Run Directly:** Launches the tool (interactive mode).
2.  **Help Output:** Verifies the tool executes and prints help (non-interactive).

## Testing Commands by Tool

### aider
```bash
cargo run -- aider
cargo run -- run aider -- --help
```

### amp
```bash
cargo run -- amp
cargo run -- run amp -- --help
```

### claude
```bash
cargo run -- claude
cargo run -- run claude -- --help
```

### code (VS Code CLI)
```bash
cargo run -- code
cargo run -- run code -- --help
```

### codex
```bash
cargo run -- codex
cargo run -- run codex -- --help
```

### copilot
```bash
cargo run -- copilot
cargo run -- run copilot -- --help
```

### crush
```bash
cargo run -- crush
cargo run -- run crush -- --help
```

### cursor-agent
```bash
cargo run -- cursor-agent
cargo run -- run cursor-agent -- --help
```

### droid
```bash
cargo run -- droid
cargo run -- run droid -- --help
```

### eca
```bash
cargo run -- eca
cargo run -- run eca -- --help
```

### forge
```bash
cargo run -- forge
cargo run -- run forge -- --help
```

### gemini
```bash
cargo run -- gemini
cargo run -- run gemini -- --help
```

### goose
```bash
cargo run -- goose
cargo run -- run goose -- --help
```

### jules
```bash
cargo run -- jules
cargo run -- run jules -- --help
```

### kilocode
```bash
cargo run -- kilocode
cargo run -- run kilocode -- --help
```

### letta
```bash
cargo run -- letta
cargo run -- run letta -- --help
```

### llxprt
```bash
cargo run -- llxprt
cargo run -- run llxprt -- --help
```

### nanocoder
```bash
cargo run -- nanocoder
cargo run -- run nanocoder -- --help
```

### ollama
```bash
cargo run -- ollama
cargo run -- run ollama -- --help
```

### opencode
```bash
cargo run -- opencode
cargo run -- run opencode -- --help
```

### pi
```bash
cargo run -- pi
cargo run -- run pi -- --help
```

### qwen
```bash
cargo run -- qwen
cargo run -- run qwen -- --help
```

### vibe
```bash
cargo run -- vibe
cargo run -- run vibe -- --help
```

## Notes

-   **`cargo run -- [tool]`**: Attempts to launch the tool interactively. If not installed, it should prompt for installation.
-   **`cargo run -- run [tool] -- --help`**: Executes the tool with the `--help` flag. The extra `--` is required so `--help` is forwarded to the tool instead of being consumed by `terminal-jarvis run`.
-   If any of these fail for an *installed* tool, it indicates a detection or execution path issue.
