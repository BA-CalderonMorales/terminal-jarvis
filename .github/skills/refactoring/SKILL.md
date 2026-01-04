# Skill: Code Refactoring

**Name**: refactoring
**Description**: Domain-based module extraction and code organization
**Trigger**: "Refactor this file", large files (>200 lines), code organization

---

## Objective

Break large files (>200 lines) into focused domain modules.

## Domain-Based Folder Structure

```
src/
  large_module/
    mod.rs              # Public interface, re-exports
    domain_a.rs         # First domain (focused responsibility)
    domain_b.rs         # Second domain (focused responsibility)
    domain_c.rs         # Third domain (focused responsibility)
```

## Refactoring Workflow

1. **Identify domains** - Group related functions/structs by responsibility
2. **Create new files** - One file per domain
3. **Move code** - Cut from old file, paste into new domain files
4. **Update mod.rs** - Add `mod domain_a;` and `pub use domain_a::*;`
5. **Fix imports** - Update `use` statements across codebase
6. **Validate** - `cargo check` after each domain extraction
7. **Quality gates** - clippy, fmt, tests

## Example: cli_logic.rs Refactoring

**Before**: 1,358 lines in `cli_logic.rs`

**After**: 10 focused modules
- `cli_logic/mod.rs` - Public interface (50 lines)
- `cli_logic/cli_logic_entry_point.rs` - Main coordination (150 lines)
- `cli_logic/cli_logic_interactive.rs` - Interactive interface (200 lines)
- `cli_logic/cli_logic_tool_execution.rs` - Tool execution (120 lines)
- `cli_logic/cli_logic_update_operations.rs` - Update logic (130 lines)
- `cli_logic/cli_logic_info_operations.rs` - Info display (90 lines)
- `cli_logic/cli_logic_list_operations.rs` - List operations (80 lines)

## Benefits

- Each file has single, clear responsibility
- Easy to navigate and understand
- Easier to test in isolation
- Reduced merge conflicts
- Better code organization

## Validation After Each Step

```bash
cargo check        # Must compile
cargo clippy -- -D warnings  # No warnings
cargo fmt --all    # Properly formatted
cargo test         # Tests pass
```
