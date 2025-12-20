# AGENTS.md - AI Assistant Guidelines for Terminal Jarvis

**Single Source of Truth for All AI Coding Assistants**

---

## CRITICAL RULES - READ FIRST

| Rule | What | Why | When Violated |
|------|------|-----|---------------|
| **NO EMOJIS** | Zero emojis in code, commits, docs, output | Professional appearance, accessibility | Use "[INSTALLED]", "►", "•" instead |
| **NO HARDCODED SQL** | All SQL via QueryBuilder or schema.rs | Single point of maintenance | Scattered SQL, migration nightmares |
| **CHANGELOG FIRST** | Update CHANGELOG.md BEFORE deployment scripts | Prevents version confusion | Deployment fails or creates wrong version |
| **Formula BEFORE Release** | Commit Homebrew Formula BEFORE GitHub release | URL matching for brew install | Broken Homebrew installations |
| **Test-Driven Bugs** | Write failing test FIRST, then fix | Prevents regression | Bug may reappear later |
| **Version Sync** | Update Cargo.toml, package.json, Formula together | Multi-platform consistency | Installation failures across platforms |
| **NO Premature Docs** | Document ONLY after feature verified on all platforms | User trust, accuracy | False documentation, broken examples |
| **Proactive Agents** | Invoke specialized agents immediately without asking | Quality, efficiency | Suboptimal work, longer sessions |
| **Token Budget** | Lead max 1000, Agents max 750 tokens | Session longevity (12+ hours) | Session ends prematurely |
| **Descriptive Headers** | "## Deployment Steps" not "Steps:" | Clear references | User confusion on "what's step 4?" |
| **Git Full Paths** | Use `/usr/bin/git` not `git` | Avoid alias issues | Unexpected git behavior |

---

## QUICK START

| User Says | Go To | Critical Info |
|-----------|-------|---------------|
| "Let's deploy" / "Run local-cd.sh" | [DEPLOYMENT](#deployment-guide) | Pre-flight checks, version sync, CHANGELOG first |
| "Fix this bug" | [TEST-DRIVEN](#test-driven-bugfixes-mandatory) | Failing test first, then fix |
| "Add new AI tool" | [TOOL CONFIG](#tool-configuration-consistency-critical) | config/tools/*.toml pattern |
| "Refactor this file" | [REFACTORING](#refactoring-best-practices-critical) | Domain-based module extraction |
| "Update version" | [VERSIONING](#version-management) | Sync Cargo.toml, package.json, Formula |
| "Before I commit" | [PRE-COMMIT](#pre-commit-checklist) | Quality gates, version check, Formula |
| "Maximize session" | [TOKEN BUDGET](#token-budget-management) | Lead orchestrator, parallel agents |
| "Homebrew release" | [HOMEBREW](#homebrew-integration) | Archive -> Formula -> Commit -> Release |
| "NPM publish" | [NPM DIST](#npm-distribution) | `npm login`, tags, version sync |

---

## PREFERRED TOOLING

**Use these tools instead of defaults:**

| Instead of | Use | Why |
|-----------|-----|-----|
| `grep` | `rg` (ripgrep) | Faster, respects .gitignore, better defaults |
| `pip` | `uv` | 10-100x faster Python package management |
| `find` + `grep` | `rg --files \| rg pattern` | Single tool, much faster |

**Common patterns:**
```bash
rg "pattern" --type rust          # Search in Rust files
rg "function_name" -C 3           # Search with context
rg --files | rg "config"          # Find files by name
uv pip install package-name       # Python packages
```

---

## TOKEN EFFICIENCY

**DO NOT create documentation files unless explicitly requested.**

**DO leverage specialized agents proactively** - they bring expertise without wasting tokens.

**DO use concise responses** - skip verbose explanations when action is clear.

---

## BRANCHING AND MERGE STRATEGY (CRITICAL)

**NEVER commit directly to main or develop** - Always use feature branches.

### Branch Flow

```
feature/*, bugfix/*, security/*, pipeline/*
                    |
                    v
                develop
                    |
                    v
                  main
```

### Branch Naming

| Prefix | Use Case | Example |
|--------|----------|--------|
| `feature/` | New functionality | `feature/add-aider-support` |
| `bugfix/` | Bug fixes | `bugfix/auth-flow-codex` |
| `security/` | Security patches | `security/api-key-exposure` |
| `pipeline/` | CI/CD changes | `pipeline/update-gh-actions` |

### Merge Rules

**Contributors:** `feature/* -> develop` (via PR)

**Admin Release:** `develop -> main` (direct merge after PR reviews complete)

---

## PROJECT OVERVIEW

**Terminal Jarvis** = Unified command center for AI coding tools (claude-code, gemini-cli, qwen-code, opencode, llxprt, codex, goose, amp, aider, crush).

**Core Innovation**: Session Continuation System (prevents auth workflow interruptions).

**Distribution**: NPM, Cargo, Homebrew (follows Orhun Parmaksiz NPM packaging pattern).

**Current Version**: 0.0.70

**Installation**:
```bash
npm install -g terminal-jarvis        # NPM
cargo install terminal-jarvis         # Cargo  
brew tap ba-calderonmorales/terminal-jarvis && brew install terminal-jarvis  # Homebrew
```

**Architecture**:
- `/src/` - Rust application (domain-based modules: cli_logic/, tools/, config/, services/)
- `/config/` - Global + modular tool configs (config/tools/*.toml)
- `/npm/terminal-jarvis/` - TypeScript wrapper for NPM distribution
- `/homebrew/` - Formula + release archives
- `/scripts/cicd/` - Deployment automation (local-ci.sh, local-cd.sh)

---

## DEPLOYMENT GUIDE

### When User Says "Let's Deploy"

**Standard Workflow**:
```bash
# 1. Pre-flight (MANDATORY)
git status                                    # Must be clean
./scripts/cicd/local-cd.sh --check-versions  # Must pass

# 2. Update CHANGELOG.md (REQUIRED FIRST)
## [X.X.X] - YYYY-MM-DD
### Added / Enhanced / Fixed / Technical

# 3. Deploy
./scripts/cicd/local-ci.sh      # Validate (no commits)
./scripts/cicd/local-cd.sh      # Deploy (commits, tags, pushes)
```

**Version Update Workflow**:
```bash
# Determine increment: 0.0.X (fix) | 0.X.0 (feature) | X.0.0 (breaking)
./scripts/cicd/local-cd.sh --update-version X.X.X
./scripts/cicd/local-cd.sh --check-versions
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh
```

**Homebrew Release Workflow**:
```bash
# 1. Complete standard deployment first
# 2. Generate archives
./scripts/utils/generate-homebrew-release.sh --stage
git add homebrew/release/ && git commit -m "feat: Homebrew archives vX.X.X" && git push

# 3. Create GitHub release
gh release create vX.X.X \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release vX.X.X" --notes "..." --latest

# 4. Verify
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/vX.X.X/terminal-jarvis-mac.tar.gz
```

### Deployment Failure Prevention

| Failure | Prevention | Fix |
|---------|-----------|-----|
| Uncommitted changes | `git status` first | Commit or stash |
| Version mismatch | `--check-versions` first | `--update-version X.X.X` |
| Missing CHANGELOG | Update before scripts | Add entry manually |
| Formula after release | Commit Formula before `gh release` | Delete release, fix, recreate |
| NPM auth failure | `npm whoami` check | `npm login` |

---

## VERSION MANAGEMENT

**Semantic Versioning**: `MAJOR.MINOR.PATCH`
- `0.0.X` - Bug fixes, docs, small improvements
- `0.X.0` - New features (no breaking changes)
- `X.0.0` - Breaking changes

**CRITICAL**: Always update ALL THREE files simultaneously:
1. `Cargo.toml`
2. `npm/terminal-jarvis/package.json`
3. `homebrew/Formula/terminal-jarvis.rb` ← COMMONLY FORGOTTEN

**CHANGELOG.md Structure**:
```markdown
## [X.X.X] - YYYY-MM-DD
### Added
- New user-visible features
### Enhanced
- Improvements to existing features
### Fixed
- Bug fixes and corrections
### Technical
- Internal changes (refactoring, tests, infrastructure)
```

**Rules**:
- Update CHANGELOG.md BEFORE running deployment scripts
- One release = one cohesive feature set
- Match actual work timeline (don't mix unrelated features)

---

## TOKEN BUDGET MANAGEMENT

**OBJECTIVE**: Maximize session duration beyond 12 hours through intelligent orchestration.

### Budget Caps

| Agent Type | Model | Max Tokens | Responsibilities |
|------------|-------|------------|------------------|
| **Lead** | Sonnet 4.5 | 1000 | Planning, architecture, orchestration, integration, validation |
| **Spawned** | Haiku Latest | 750 | File edits, docs, tests, refactoring, specialized tasks |

### Lead Orchestrator Pattern

**Phase 1: Planning (Lead - 1000 tokens)**
1. Analyze request scope
2. Break into discrete tasks
3. Identify parallelization opportunities
4. Select appropriate agents
5. Define validation criteria

**Phase 2: Delegation (Agents - 750 tokens each)**
- Spawn agents in parallel for independent tasks
- Sequential for dependencies
- Focused context (only what's needed)
- Clear success criteria

**Phase 3: Integration (Lead - 1000 tokens)**
- Collect results
- Run quality checks (cargo check, clippy, fmt, test)
- Coordinate commits
- Update documentation

### Token Conservation Techniques

| Technique | Good | Bad |
|-----------|------|-----|
| **Delegation** | Lead plans, 5 agents implement | Lead implements everything |
| **Parallelization** | 3 agents simultaneously | Sequential when independent |
| **File reads** | Read specific file | Read entire directory tree |
| **Validation** | cargo check after each agent | Accumulate, validate at end |
| **Context** | "Update lines 45-60 in file X" | "Read file, understand, update" |

**Target Metrics**:
- Agent utilization: >80% (most work by agents)
- Parallelization: >40% (many concurrent)
- Session duration: >12 hours

---

## PROACTIVE AGENT USAGE (MANDATORY)

**AI assistants MUST invoke specialized agents immediately without waiting to be asked.**

| Scenario | Agent | When |
|----------|-------|------|
| Documentation | @documentation-specialist | README, CHANGELOG, API docs, guides |
| Testing | @qa-automation-engineer, @test-automation-expert | Test suites, TDD, fixtures |
| Code review | @code-reviewer | PR review, quality gates, clippy/fmt |
| Security | @security-specialist | Auth, credentials, encryption, sensitive data |
| Infrastructure | @infrastructure-expert, @devops-engineer | CI/CD, deployment scripts, Docker |
| Architecture | @software-architect, @api-architect | System design, refactoring, structural decisions |
| Performance | @performance-specialist | Profiling, optimization, bottlenecks |
| Frontend/UI | @frontend-specialist, @ui-ux-designer | CLI interface, menus, UX |

**Commit Attribution** (when agents contribute):
```bash
docs(readme): update installation - @documentation-specialist
feat(auth): implement OAuth - @security-specialist @software-engineering-expert
refactor(cli): extract domains - @software-architect @code-reviewer @documentation-specialist
```

---

## CODE QUALITY STANDARDS

### Quality Gates (MANDATORY before commits)

```bash
cargo check                                                  # Must compile
cargo clippy -- -D warnings                                  # Must pass (no --all-features: avoids C++ deps)
cargo fmt --all                                              # Must be formatted
cargo test                                                   # Must pass (if tests exist)
```

### Commit Message Format

```
<type>(<scope>): <description>

Types: fix, feat, break, docs, style, refactor, test, chore
Examples:
  fix: resolve clippy warnings in api module
  feat: add support for qwen-code tool
  break: change cli argument structure for templates command
  docs: update installation instructions
```

### Rust Code Standards

- Use `anyhow::Result` for error handling
- Add doc comments for public functions
- Keep files under 200 lines (extract domains if larger)
- Follow domain-based module organization
- No unwrap() or expect() in production code (use proper error handling)

### TypeScript Code Standards

- Use Biome for linting/formatting (NOT ESLint)
- Run `npm run lint` and `npm run format` before committing
- Follow existing patterns in npm/terminal-jarvis/

### Test Structure

| Location | Technology | Purpose | Command |
|----------|------------|---------|---------|
| `tests/` | Rust | Unit and integration tests | `cargo test` |
| `e2e/` | TypeScript (Vitest + cli-testing-library) | End-to-end CLI tests | `cd e2e && npm test` |

**E2E Test Setup:**
```bash
cd e2e
npm install
npm test          # Run all E2E tests
npm test -- --watch  # Watch mode
```

**Key E2E Test Files:**
- `e2e/helpers.ts` - CLI rendering and spawn utilities
- `e2e/helpers/` - ANSI parsing, layout validation, width simulation
- `e2e/*.test.ts` - Test suites (help, version, installation, themes, etc.)

---

## PRE-COMMIT CHECKLIST

### Version Consistency

- [ ] Cargo.toml version matches target release
- [ ] npm/terminal-jarvis/package.json version matches
- [ ] homebrew/Formula/terminal-jarvis.rb version matches

### Documentation

- [ ] CHANGELOG.md updated with new version entry
- [ ] README.md reflects new features (if user-facing)
- [ ] Inline documentation updated for changed APIs

### Quality Checks

- [ ] `cargo check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --all` applied
- [ ] `cargo test` passes (all Rust tests green)
- [ ] E2E tests pass (if modified): `cd e2e && npm test`

### Homebrew Integration (if updating version)

- [ ] Homebrew Formula version synchronized
- [ ] Formula committed BEFORE creating GitHub release
- [ ] Release archives generated and staged

### NPM Distribution (if publishing)

- [ ] `npm whoami` confirms authenticated
- [ ] `npm run lint` passes
- [ ] `npm run format` applied
- [ ] package.json version matches Cargo.toml

---

## TEST-DRIVEN BUGFIXES (MANDATORY)

**ABSOLUTE REQUIREMENT**: ALL bug fixes must follow this workflow:

1. **Write failing test** - Reproduce the bug with a test that fails
2. **Verify test fails** - Run `cargo test` and confirm failure
3. **Implement fix** - Make minimal changes to fix the bug
4. **Verify test passes** - Run `cargo test` and confirm success
5. **Run quality gates** - clippy, fmt, full test suite
6. **Commit both** - Test and fix together in same commit

**Example workflow**:
```bash
# 1. Create test that reproduces bug
# tests/cli_integration_tests.rs
#[test]
fn test_tool_launch_single_enter() {
    let output = launch_tool("claude");
    assert!(output.prompts_count <= 1);  // This will fail if 3 prompts
}

# 2. Verify failure
cargo test test_tool_launch_single_enter  # Should fail

# 3. Implement fix in src/cli_logic/cli_logic_tool_execution.rs

# 4. Verify success
cargo test test_tool_launch_single_enter  # Should pass
cargo test                                # All tests pass

# 5. Commit together
git add tests/cli_integration_tests.rs src/cli_logic/cli_logic_tool_execution.rs
git commit -m "fix(cli): streamline tool launch flow - @test-automation-expert"
```

---

## REFACTORING BEST PRACTICES (CRITICAL)

**OBJECTIVE**: Break large files (>200 lines) into focused domain modules.

### Domain-Based Folder Structure

```
src/
  large_module/
    mod.rs              # Public interface, re-exports
    domain_a.rs         # First domain (focused responsibility)
    domain_b.rs         # Second domain (focused responsibility)
    domain_c.rs         # Third domain (focused responsibility)
```

### Refactoring Workflow

1. **Identify domains** - Group related functions/structs by responsibility
2. **Create new files** - One file per domain
3. **Move code** - Cut from old file, paste into new domain files
4. **Update mod.rs** - Add `mod domain_a;` and `pub use domain_a::*;`
5. **Fix imports** - Update `use` statements across codebase
6. **Validate** - `cargo check` after each domain extraction
7. **Quality gates** - clippy, fmt, tests

### Example: cli_logic.rs Refactoring

**Before**: 1,358 lines in `cli_logic.rs`

**After**: 10 focused modules
- `cli_logic/mod.rs` - Public interface (50 lines)
- `cli_logic/cli_logic_entry_point.rs` - Main coordination (150 lines)
- `cli_logic/cli_logic_interactive.rs` - Interactive interface (200 lines)
- `cli_logic/cli_logic_tool_execution.rs` - Tool execution (120 lines)
- `cli_logic/cli_logic_update_operations.rs` - Update logic (130 lines)
- `cli_logic/cli_logic_info_operations.rs` - Info display (90 lines)
- `cli_logic/cli_logic_list_operations.rs` - List operations (80 lines)
- ...and more

**Benefits**:
- Each file has single, clear responsibility
- Easy to navigate and understand
- Easier to test in isolation
- Reduced merge conflicts
- Better code organization

---

## DATABASE ARCHITECTURE (CRITICAL)

**OBJECTIVE**: All database operations use schema.rs and QueryBuilder - NO hardcoded SQL strings.

### Architecture Overview

```
src/db/
  schema.rs           # Single source of truth for table/column definitions
  query_builder.rs    # Fluent API for SQL construction
  repository.rs       # Base repository pattern
  migrations.rs       # Version-controlled schema changes (uses QueryBuilder)
  *_repository.rs     # Entity-specific data access
```

### Rules

1. **Schema is Truth**: All table/column definitions in `schema.rs`
2. **QueryBuilder for Queries**: Never write raw SQL strings
3. **Repository Pattern**: Each entity has its own repository
4. **Migrations via Schema**: Use `table.create_table_sql()`, never hardcode DDL

### Example - Correct Pattern

```rust
// GOOD: Using QueryBuilder
let sql = QueryBuilder::select(&TOOLS_TABLE)
    .columns(&["id", "display_name"])
    .where_eq("enabled")
    .order_by("display_name", true)
    .build();

// GOOD: Schema-driven DDL
let ddl = TOOLS_TABLE.create_table_sql();
```

### Anti-Patterns to Avoid

```rust
// BAD: Hardcoded SQL
db.execute("SELECT * FROM tools WHERE id = ?", [id]).await?;

// BAD: Hardcoded DDL
db.execute("CREATE TABLE tools (id TEXT PRIMARY KEY)", ()).await?;

// BAD: SQL in migrations
db.execute("INSERT INTO schema_migrations...", params).await?;
```

---

## TOOL CONFIGURATION CONSISTENCY (CRITICAL)

**OBJECTIVE**: All AI tools follow consistent modular configuration pattern.

### Modular Tool Configuration System

**Location**: `/config/tools/<tool-name>.toml`

**Template**:
```toml
[tool]
name = "Tool Name"
command = "tool-command"
description = "Brief description"
category = "ai-coding-assistant"

[installation]
npm = "npm install -g package-name"
cargo = "cargo install package-name"
other = "curl -fsSL install-script.sh | sh"

[authentication]
required = true
method = "api_key"
env_var = "TOOL_API_KEY"
instructions = "Get your API key from https://provider.com/keys"

[features]
supports_chat = true
supports_code_generation = true
supports_refactoring = true
supports_testing = false
```

### Adding New Tool Workflow

1. **Create config file**: `/config/tools/newtool.toml`
2. **Define tool metadata**: name, command, description
3. **Specify installation**: npm, cargo, or custom script
4. **Document authentication**: API keys, env vars, instructions
5. **List features**: capabilities for UI display
6. **Test detection**: `cargo run -- list` should show new tool
7. **Test execution**: `cargo run -- run newtool` should work
8. **Update CHANGELOG**: Add under `### Added`

---

## HOMEBREW INTEGRATION

### Formula Structure

**Location**: `homebrew/Formula/terminal-jarvis.rb`

**Key components**:
- Version must match Cargo.toml, package.json
- URL points to GitHub release archive
- SHA256 matches archive checksum
- Install script copies binary to bin/

### Release Archive Generation

```bash
# Generate platform-specific archives
./scripts/utils/generate-homebrew-release.sh --stage

# Output:
# homebrew/release/terminal-jarvis-mac.tar.gz
# homebrew/release/terminal-jarvis-linux.tar.gz
```

### Complete Homebrew Release Workflow

```bash
# 1. Standard deployment (includes version updates, CHANGELOG, Formula)
./scripts/cicd/local-ci.sh && ./scripts/cicd/local-cd.sh

# 2. Generate release archives
./scripts/utils/generate-homebrew-release.sh --stage

# 3. Commit archives
git add homebrew/release/
git commit -m "feat: Homebrew release archives vX.X.X"
git push

# 4. Create GitHub release with archives
gh release create vX.X.X \
  homebrew/release/terminal-jarvis-mac.tar.gz \
  homebrew/release/terminal-jarvis-linux.tar.gz \
  --title "Release vX.X.X" \
  --notes "Release notes from CHANGELOG.md" \
  --latest

# 5. Verify archive accessibility
curl -I https://github.com/BA-CalderonMorales/terminal-jarvis/releases/download/vX.X.X/terminal-jarvis-mac.tar.gz

# 6. Test installation
brew uninstall terminal-jarvis
brew install terminal-jarvis
terminal-jarvis --version  # Should show vX.X.X
```

### Common Homebrew Pitfalls

| Issue | Cause | Fix |
|-------|-------|-----|
| SHA256 mismatch | Formula SHA doesn't match archive | Regenerate archive or update Formula SHA |
| Archive naming | Incorrect filename pattern | Use generate-homebrew-release.sh script |
| Formula syntax error | Ruby syntax mistakes | Run `brew install --dry-run` to validate |
| Binary permissions | Binary not executable | Ensure chmod +x in install script |
| Cross-platform issue | macOS/Linux binary differences | Test on both platforms |
| Formula after release | Committed Formula changes too late | Always commit Formula BEFORE `gh release` |

---

## NPM DISTRIBUTION

### NPM Authentication

```bash
# Check authentication status
npm whoami

# Login if needed
npm login

# Verify credentials
npm whoami  # Should show your username
```

### NPM Distribution Tags

| Tag | Purpose | Command |
|-----|---------|---------|
| `latest` | Stable release (default) | `npm publish` or `npm publish --tag latest` |
| `next` | Beta/preview release | `npm publish --tag next` |
| `alpha` | Early development | `npm publish --tag alpha` |

### NPM Publishing Workflow

```bash
# 1. Ensure authenticated
npm whoami

# 2. Update version (via deployment script)
./scripts/cicd/local-cd.sh --update-version X.X.X

# 3. Build TypeScript wrapper
cd npm/terminal-jarvis
npm run lint
npm run format
npm run build

# 4. Publish (happens automatically via local-cd.sh)
# Or manually: npm publish --tag latest

# 5. Verify
npm info terminal-jarvis
npm install -g terminal-jarvis@X.X.X
terminal-jarvis --version
```

---

## COMMUNICATION GUIDELINES

### Reference Clarity (CRITICAL)

**ALWAYS use descriptive headers**:
- GOOD: "## Deployment Steps"
- BAD: "Steps:"

**ALWAYS include context in references**:
- GOOD: "In the Homebrew release workflow above, step 3 refers to..."
- BAD: "Step 3 means..."

**When user asks "what's step 4?"**:
1. Apologize for ambiguity
2. Quote the specific numbered list being referenced
3. Explain which workflow/section it belongs to
4. Provide the clear answer

---

## DETAILED EXAMPLES

### Example 1: Adding New AI Tool Integration

```
User Request: "Add support for new-ai-tool CLI"

Lead Planning (1000 tokens):
- Analyze: Need config file, command mapping, detection
- Breakdown:
  * Task 1: Create config/tools/newtool.toml (sequential)
  * Task 2: Add to tools_command_mapping.rs (sequential after Task 1)
  * Task 3: Test detection and execution (sequential after Task 2)
  * Task 4: Update CHANGELOG.md (parallel with Task 3)
- Parallelization: Tasks 1-3 sequential (dependencies), Task 4 parallel

Lead Delegation:
  Agent 1 (750 tokens): Create config/tools/newtool.toml
  [Wait for completion]
  Agent 2 (750 tokens): Add command mapping entry
  [Wait for completion]
  Agent 3 (750 tokens): Test with cargo run -- list && cargo run -- info newtool
  Agent 4 (750 tokens): Update CHANGELOG.md (parallel with Agent 3)

Lead Integration (1000 tokens):
- cargo check (verify compilation)
- cargo test (verify tests pass)
- Manual test: cargo run -- list
- Prepare commit message
```

### Example 2: Refactoring Large Module

```
User Request: "Refactor cli_logic.rs (800 lines) into focused modules"

Lead Planning (1000 tokens):
- Analyze current structure: 4 domains identified
  * Tool execution (200 lines)
  * List operations (150 lines)
  * Update operations (200 lines)
  * Interactive menu (250 lines)
- Breakdown:
  * Task 1: Create cli_logic/cli_logic_tool_execution.rs
  * Task 2: Create cli_logic/cli_logic_list_operations.rs
  * Task 3: Create cli_logic/cli_logic_update_operations.rs
  * Task 4: Create cli_logic/cli_logic_interactive.rs
  * Task 5: Update cli_logic/mod.rs to re-export
- Parallelization: Tasks 1-4 parallel, Task 5 after all complete

Lead Delegation:
  Agent 1-4 (750 tokens each): Extract each domain in parallel
  [After agents complete]
  Agent 5 (750 tokens): Update mod.rs with re-exports

Lead Integration (1000 tokens):
- cargo check --all-targets
- cargo clippy -- -D warnings
- cargo fmt --all
- Prepare commit
```

### Example 3: Bug Fix with Test-Driven Development

```
User Request: "Fix: Tool launches require too many Enter presses"

Lead Planning (1000 tokens):
- Analyze: UX friction in tool launch flow
- Root cause hypothesis: Unnecessary confirmation prompts
- Breakdown:
  * Task 1: Write test capturing current behavior
  * Task 2: Identify and remove unnecessary prompts in cli_logic_tool_execution.rs
  * Task 3: Verify streamlined flow
- Parallelization: Sequential (TDD requirement)

Lead Delegation:
  Agent 1 (750 tokens): Create test for tool launch flow
  [After test documents current behavior]
  Agent 2 (750 tokens): Streamline prompts in tool execution
  [After fix implemented]
  Agent 3 (750 tokens): Verify improved UX

Lead Integration (1000 tokens):
- cargo test (verify fix)
- Manual test: cargo run, select tool, count prompts
- Update CHANGELOG.md with UX improvement
- Prepare commit
```

---

## SESSION CONTINUATION SYSTEM

**Key Feature**: Prevents users from being kicked out during authentication workflows.

**How It Works**:
1. User launches AI tool (e.g., `terminal-jarvis run claude`)
2. Tool requires authentication (redirects to browser)
3. Traditional approach: User returns, session gone
4. Terminal Jarvis: Session preserved, resumes automatically

**Implementation**: `src/tools/tools_execution_engine.rs`
- Detects authentication workflows
- Maintains session state during external auth
- Resumes execution after auth completion

**Debugging Session Issues**:
```bash
# Check session state
cargo run -- run <tool> --debug

# View logs
tail -f ~/.terminal-jarvis/logs/session.log

# Reset session state
rm -rf ~/.terminal-jarvis/sessions/
```

---

## TECHNICAL NOTES

### Rust Dependencies

Key crates:
- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `anyhow` - Error handling
- `reqwest` - HTTP client
- `toml` - Configuration parsing

### NPM Packaging Approach

Following Orhun Parmaksız pattern:
1. Build Rust binary for target platform
2. Package binary in NPM with TypeScript wrapper
3. Wrapper calls binary via child_process
4. Cross-platform support via platform-specific binaries

### Multi-Platform Build

```bash
# Build for current platform
cargo build --release

# Cross-compile (requires setup)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
```

### Environment Variables

| Variable | Purpose | Default |
|----------|---------|---------|
| `TERMINAL_JARVIS_CONFIG` | Config file path | `~/.terminal-jarvis/config.toml` |
| `TERMINAL_JARVIS_LOG_LEVEL` | Logging verbosity | `info` |
| `TERMINAL_JARVIS_SESSION_DIR` | Session state directory | `~/.terminal-jarvis/sessions/` |

---

## APPENDIX: ORIGINAL CHANGELOG EXAMPLE

```markdown
## [0.0.47] - 2025-08-09

### Added
- **Homebrew Integration**: Complete multi-platform distribution system
  - Formula-based installation via `brew install terminal-jarvis`
  - Automated release archive generation for macOS and Linux
  - Platform-specific binary packaging with proper permissions
  - SHA256 checksum validation for security
  - GitHub Release integration for archive hosting

- **Testing Infrastructure**: Comprehensive validation scripts
  - Homebrew installation testing without requiring GitHub repository
  - Platform-specific test coverage (macOS, Linux)
  - Archive generation and SHA256 verification
  - End-to-end Homebrew workflow validation

- **Multi-Platform Distribution Documentation**
  - Complete Homebrew publishing workflow guide
  - Platform-specific installation instructions
  - Testing protocols for release validation
  - Common pitfalls and troubleshooting guide

### Enhanced
- **Deployment Workflow**: Streamlined version management
  - `--update-version` flag for synchronized version updates across all files
  - `--check-versions` validation for Cargo.toml, package.json, and Homebrew Formula
  - Improved error messaging for version mismatches
  - Automated version synchronization in CI/CD pipeline

- **README Organization**: Improved clarity and navigation
  - Multi-channel installation badges (NPM, Crates.io, Homebrew)
  - Clear distribution channel separation
  - Platform-specific installation instructions
  - Enhanced feature list organization

### Fixed
- **Homebrew Formula Synchronization**: Version consistency across platforms
  - Fixed issue where Formula version could drift from Cargo.toml
  - Added validation checks to prevent deployment with mismatched versions
  - Improved deployment script to update all version files atomically

### Technical
- **Refactoring**: Homebrew integration scripts
  - Modular shell script architecture for release generation
  - Improved error handling and validation
  - Better logging and debug output
  - Cross-platform compatibility improvements
```

---

**END OF AGENTS.MD**

Navigation: Use Ctrl+F to search | All headers are anchor links | Critical sections at top
