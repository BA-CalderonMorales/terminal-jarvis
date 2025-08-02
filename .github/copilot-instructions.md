# Terminal Jarvis - AI Coding Tools Wrapper

## What This Project Does

Terminal Jarvis is a thin Rust wrapper that provides a unified interface for managing and running AI coding tools like claude-code, gemini-cli, qwen-code, and opencode. Think of it as a package manager and runner for AI coding assistants.

The project follows Orhun Parmaksız's approach for packaging Rust applications via NPM, making it easy to install with `npm install -g terminal-jarvis`.

## How The Code Is Organized

The repository has two main parts:

**Rust Application** (`/src/`):
- `main.rs` - Entry point that starts the CLI
- `cli.rs` - Command definitions using clap (run, update, list, info, templates)
- `cli_logic.rs` - The actual business logic for each command
- `services.rs` - PackageService and GitHubService for managing tools
- `config.rs` - TOML configuration management
- `api.rs`, `api_client.rs`, `api_base.rs` - Future API framework (currently unused)

**NPM Package** (`/npm/terminal-jarvis/`):
- `src/index.ts` - Simple TypeScript wrapper that calls the Rust binary
- `package.json` - NPM package configuration
- `biome.json` - Biome linting configuration (we use Biome, not ESLint)

## Version Numbers Are Important

We use semantic versioning with **NO EMOJIS** and **NO DECORATIONS**. Just clean version numbers:

- `0.0.1` - Bug fixes, docs, small improvements
- `0.1.0` - New features that don't break existing functionality  
- `1.0.0` - Breaking changes that require users to update their code

Always update BOTH `Cargo.toml` and `npm/terminal-jarvis/package.json` at the same time.

## How To Write Commit Messages

Keep them simple and clear:

```
fix: resolve clippy warnings in api module
feat: add support for qwen-code tool
break: change cli argument structure for templates command
docs: update installation instructions
```

Types to use: `fix`, `feat`, `break`, `docs`, `style`, `refactor`, `test`, `chore`

## Code Quality Rules

**Rust Code:**
- Must pass `cargo clippy --all-targets --all-features -- -D warnings`
- Must be formatted with `cargo fmt --all`
- Use `anyhow::Result` for error handling
- Add doc comments for public functions

**TypeScript Code:**
- Use Biome for linting and formatting, NOT ESLint
- Run `npm run lint` and `npm run format` before committing

## File Sync Requirements

The README.md needs to be the same in both the root directory and `npm/terminal-jarvis/`. Before publishing to NPM, always run:

```bash
cd npm/terminal-jarvis
npm run sync-readme
```

## What Not To Do

- No emojis anywhere (commits, code, documentation)
- No vague commit messages like "fix stuff" or "update things"
- No combining unrelated changes in one commit
- No force pushing to main or develop branches
- No `.unwrap()` without good error handling
- No magic numbers - use named constants

## How To Release

1. Update version numbers in both `Cargo.toml` and `npm/terminal-jarvis/package.json`
2. Update version display in `npm/terminal-jarvis/src/index.ts`
3. Update version display in `src/cli_logic.rs` (interactive mode version)
4. Update CHANGELOG.md with new version and changes
5. Update version references in README.md (root and NPM package will sync automatically)
6. Run `npm run sync-readme` to sync the README
7. Commit with clear message: `feat: add new feature X`
8. Create tag: `git tag v0.0.6`
9. Push to GitHub: `git push origin develop --tags`
10. Publish to NPM: `cd npm/terminal-jarvis && npm publish`

## Local CD with Agent Mode

When conducting local continuous deployment with agents, **ALWAYS** follow this order:

1. **Update All Version References:**
   - `Cargo.toml` - version field
   - `npm/terminal-jarvis/package.json` - version field
   - `npm/terminal-jarvis/src/index.ts` - console.log version display
   - `src/cli_logic.rs` - interactive mode version display
   - `CHANGELOG.md` - add new version entry with changes
   - `README.md` - version reference in the note section

2. **Build and Test:**
   ```bash
   cd npm/terminal-jarvis
   npm run build
   ```

3. **Commit and Push to GitHub FIRST:**
   ```bash
   git add .
   git commit -m "version: bump to vX.X.X with description"
   git tag vX.X.X
   git push origin develop
   git push origin vX.X.X
   ```

4. **Then Publish to NPM Registry:**
   ```bash
   cd npm/terminal-jarvis
   npm publish --access public
   ```

**Why This Order Matters:**
- GitHub serves as source of truth for version history
- Git tags provide audit trail for releases
- NPM registry reflects committed code, not uncommitted changes
- Allows rollback if NPM publish fails
- Ensures consistency between repository and published package

## Technical Notes

- The API modules (`api.rs`, `api_client.rs`, etc.) are framework code for future use
- They have `#[allow(dead_code)]` attributes since they're not used yet
- Configuration system uses TOML files for per-tool settings
- NPM package is just a thin wrapper around the Rust binary