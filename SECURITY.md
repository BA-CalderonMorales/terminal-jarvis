# Security Policy

> **Status:** Active monitoring with automated vulnerability scanning

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.0.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not open a public issue**
2. Email: [security@ba-calderonmorales.dev](mailto:security@ba-calderonmorales.dev)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We aim to respond within 48 hours and will keep you updated on the remediation progress.

---

## Supply Chain Security

This project implements defense-in-depth measures to protect against supply chain attacks.

### Dependency Management

#### NPM (JavaScript/TypeScript)

- **Pinned Versions:** All dependencies use exact versions (no `^` or `~`)
- **Lockfile Verification:** CI verifies `package-lock.json` is in sync with `package.json`
- **Clean Install:** CI uses `npm ci` instead of `npm install` for reproducible builds
- **Audit Level:** CI fails on moderate or higher severity vulnerabilities

```bash
# Verify lockfile sync locally
cd npm/terminal-jarvis
npm install --package-lock-only
git diff package-lock.json

# Run security audit
npm audit --audit-level=moderate
```

#### Cargo (Rust)

- **Locked Builds:** CI uses `cargo build --locked` to ensure `Cargo.lock` is current
- **Cargo.lock Committed:** Lockfile is tracked in version control
- **cargo-audit:** Automated security scanning for Rust dependencies
- **cargo-deny:** License and advisory checking

```bash
# Verify locked build
cargo build --locked

# Run security audit
cargo audit

# Run cargo-deny checks
cargo deny check advisories bans sources
```

### CI Security Checks

Our continuous integration pipeline includes:

| Check              | Tool          | Severity Threshold |
|--------------------|---------------|--------------------|
| NPM Audit          | npm audit     | Moderate+          |
| Rust Audit         | cargo-audit   | Warnings+          |
| License Check      | cargo-deny    | Banned licenses    |
| Secret Scanning    | Gitleaks      | All secrets        |
| Shell Script Check | ShellCheck    | Style issues       |
| SBOM Scan          | Anchore       | Critical           |

### Install Scripts

The NPM package includes a `postinstall` script that downloads platform-specific binaries:

**File:** `npm/terminal-jarvis/scripts/postinstall.js`

- Downloads from GitHub releases (HTTPS only)
- No automatic execution of downloaded binaries
- Graceful fallback if download fails
- Checksums not yet implemented (planned for v0.1.0)

### Binary Verification

Pre-built binaries are available for:
- macOS (x64, arm64)
- Linux (x64, arm64)
- Windows (x64, arm64)

Download from: [GitHub Releases](https://github.com/BA-CalderonMorales/terminal-jarvis/releases)

**Verification (manual):**
```bash
# TODO: Add checksum verification in v0.1.0
# sha256sum -c terminal-jarvis-linux.tar.gz.sha256
```

---

## Credential Security

### Storage

API keys and credentials are stored with encryption:

| Feature                  | Status      | Version |
|--------------------------|-------------|---------|
| Platform Keychain        | Planned     | 0.0.79  |
| AES-256-GCM Fallback     | Planned     | 0.0.79  |
| Argon2 Key Derivation    | Planned     | 0.0.79  |
| Plaintext Migration      | Supported   | Current |

### Current Behavior

- Credentials stored at: `~/.config/terminal-jarvis/credentials.toml`
- Encryption: **None** (plaintext TOML)
- Migration: Automatic encryption on first write after upgrade

### Planned Improvements

See Issue #59 for credential encryption implementation:
- Master password protection
- Platform-native keychain integration
- Transparent migration from plaintext

---

## Security Features

### Zero-Trust Model Loading

The `SecureModelLoader` implements:
- Allowlist-only model loading
- SHA-256 hash verification
- Auto-download disabled
- Cache verification on each access

### Input Validation

All external input passes through `SecurityValidator`:
- Path traversal detection
- Command injection prevention
- Shell escape detection

### Browser Security

See `src/security/browser_attack_tests.rs` for:
- Localhost attack prevention
- Browser cache poisoning tests
- CORS misconfiguration detection

---

## Security Score Targets

| Metric                  | Tool       | Target | Current |
|-------------------------|------------|--------|---------|
| Supply Chain Security   | Socket.dev | 100/100| In Progress |
| Vulnerability Alerts    | GitHub     | 0      | 0       |
| Dependency Age          | Dependabot | Current| Weekly  |
| Secrets in Code         | Gitleaks   | 0      | 0       |

---

## Related Documentation

- [CONTRIBUTING.md](./CONTRIBUTING.md) - Development practices
- [README.md](./README.md) - General project information
- Issue #62 - Supply chain hardening
- Issue #59 - Credential encryption
