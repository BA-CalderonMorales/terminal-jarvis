# Tool Architecture & Integration Pattern

## The "Why" Behind Tool Integration

Terminal Jarvis uses a **hybrid configuration architecture** designed to balance ease of contribution with robust runtime performance.

### 1. The "Seed Data" Pattern (TOML Files)
We define tools in `config/tools/*.toml` rather than hardcoding them in Rust or requiring manual database entry.

*   **Why?**:
    *   **Accessibility**: Anyone can add a tool by copying an existing TOML file. No Rust knowledge required.
    *   **Portability**: TOML files are distributed with the NPM and Cargo packages, ensuring all users get the latest tool definitions on install.
    *   **Seed Strategy**: On first run, the application reads these TOML files and "seeds" the local SQLite database.

### 2. The Database-First Runtime
While configuration starts in TOML, runtime execution relies on the SQLite database (`src/db/tools/`).

*   **Why?**:
    *   **Performance**: Querying a relational DB is faster than parsing 20+ TOML files on every command.
    *   **User Overrides**: Users can modify tool settings (like `requires_sudo`) locally without editing read-only system files.
    *   **State Tracking**: The DB tracks installation status, last used dates, and successful/failed execution metrics.

### 3. The Command Mapping Layer
`src/tools/tools_command_mapping.rs` provides the glue between user input and the configuration system.

*   **Why?**:
    *   **Aliasing**: Maps `run gpt` to the `codex` configuration.
    *   **Sanitization**: Ensures we have a static registry of known tools before hitting the database.
    *   **Compile-Time Safety**: Provides a partial check that tools referenced in code actually exist.

## How to Add a New Tool (The "Golden Path")

1.  **Create Config**: Add `config/tools/<tool_name>.toml` (Follow existing schema).
2.  **Register Mapping**: Add entry to `src/tools/tools_command_mapping.rs`.
3.  **Sync Distribution**: Copy TOML to `npm/terminal-jarvis/config/tools/`.

This pattern ensures that a new tool is immediately available to all users across all platforms (Rust/NPM/Homebrew) without complex migration scripts.
