# Skill: NPM Distribution

**Name**: npm
**Description**: NPM package publishing and distribution
**Trigger**: "NPM publish", npm release, JavaScript/TypeScript distribution

---

## NPM Authentication

```bash
# Check authentication status
npm whoami

# Login if needed
npm login

# Verify credentials
npm whoami  # Should show your username
```

## Distribution Tags

| Tag | Purpose | Command |
|-----|---------|---------|
| `latest` | Stable release (default) | `npm publish` or `npm publish --tag latest` |
| `next` | Beta/preview release | `npm publish --tag next` |
| `alpha` | Early development | `npm publish --tag alpha` |

## NPM Publishing Workflow

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

## NPM Package Structure

Following Orhun Parmaksiz pattern:
1. Build Rust binary for target platform
2. Package binary in NPM with TypeScript wrapper
3. Wrapper calls binary via child_process
4. Cross-platform support via platform-specific binaries

## Package Location

`npm/terminal-jarvis/`
- `package.json` - NPM package manifest
- `src/` - TypeScript wrapper source
- `bin/` - Binary entry point
- `scripts/` - Build and installation scripts

## TypeScript Standards

- Use Biome for linting/formatting (NOT ESLint)
- Run `npm run lint` and `npm run format` before committing
