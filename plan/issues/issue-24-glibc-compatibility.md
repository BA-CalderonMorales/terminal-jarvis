# Issue #24: GLIBC 2.39 Hard Dependency

## Problem

Terminal-jarvis requires GLIBC 2.39, but many systems (Debian 12, Ubuntu 22.04, older containers) have older versions.

**Error seen:**
```
version `GLIBC_2.39' not found
```

## Root Cause

The Rust binary is dynamically linked against glibc, picking up the build machine's version.

## Solutions

### Option 1: musl Static Linking (Recommended)

Build with `x86_64-unknown-linux-musl` target for fully static binary.

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

**Pros:**
- Works on any Linux
- No glibc dependency
- Single binary

**Cons:**
- Slightly larger binary
- May have minor performance impact

### Option 2: Multiple Binary Variants

Publish separate binaries:
- `terminal-jarvis-linux-glibc` (dynamic, smaller)
- `terminal-jarvis-linux-musl` (static, portable)

Postinstall script detects glibc version and downloads appropriate binary.

### Option 3: Build on Older System

Build on Ubuntu 20.04 or Debian 11 to get lower glibc requirement.

## Implementation Plan

1. Add musl target to CI build matrix
2. Create `terminal-jarvis-linux-musl.tar.gz` release artifact
3. Update postinstall.js to detect glibc and choose binary
4. Test in qa/env-debian-bookworm

## QA Branch

Test environment: `qa/env-debian-bookworm`

```bash
git checkout qa/env-debian-bookworm
# Open in Codespace/devcontainer
npm run test:glibc
```

## Status

- [x] QA environment created
- [x] **QA TESTING COMPLETE (2026-01-04)**
- [ ] musl build added to CI
- [ ] Postinstall glibc detection
- [ ] Release with musl binary

## QA Test Results

### Tested Environments

| Environment | GLIBC | Install | Execution |
|-------------|-------|---------|-----------|
| Ubuntu 22.04 | 2.35 | ✅ PASS (31s) | ❌ FAIL |
| Debian 12 | 2.36 | ✅ PASS (31s) | ❌ FAIL |

### Binary Analysis

Required GLIBC symbols (highest version dependencies):
```
GLIBC_2.34
GLIBC_2.39  ← Blocker
```

### Error Output
```
./terminal-jarvis: /lib/x86_64-linux-gnu/libc.so.6: 
version `GLIBC_2.39' not found (required by ./terminal-jarvis)
```

### Impact Assessment

**Critical:** This issue blocks:
- All usage on Ubuntu 22.04 and older
- All usage on Debian 12 and older
- All usage on RHEL/CentOS 9 and older
- Issue #27 auth flow testing

**Only compatible with:**
- Ubuntu 24.04+ (GLIBC 2.39)
- Fedora 40+ 
- Arch Linux (rolling)

### Recommended Fix Priority

**P0 - Fix immediately.** Ship a musl-linked binary in next release.
