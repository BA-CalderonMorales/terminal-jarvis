# Phase 10: Modern Architecture - Database & Voice Simplification

**Status**: PENDING  
**Priority**: HIGH  
**Estimated Sessions**: 3-4

## The Problem

### Current Issues

1. **Voice Complexity**: whisper-rs requires compiling C++ (whisper.cpp), causing build failures in containerized/cloud environments. The local voice feature adds significant build complexity.

2. **TOML Sprawl**: Configuration spread across 15+ TOML files:
   - `/config/tools/*.toml` (10 files)
   - `/config/api.toml`, `metadata.toml`, `templates.toml`, `user-preferences.toml`
   - `/config/evals/*.toml`, `/config/benchmarks/*.toml`
   - No sync capability, no versioning, no cloud backup

## The Solution

### 1. Voice: Cloud-First API Approach

Replace whisper-rs with simple HTTP calls to cloud transcription APIs:

| Option | Pros | Cons |
|--------|------|------|
| **OpenAI Whisper API** | Best accuracy, simple API | Cost ($0.006/min) |
| **Deepgram** | Fast, streaming support | Cost |
| **AssemblyAI** | Good accuracy, speaker diarization | Cost |
| **Groq Whisper** | Fast, free tier | Limited free tier |

**Recommendation**: Start with OpenAI Whisper API (users already have OPENAI_API_KEY for other tools).

### 2. Database: Turso/libSQL

Replace TOML files with embedded SQLite (libSQL) that can optionally sync to Turso cloud:

```
Local: ~/.terminal-jarvis/jarvis.db (libSQL embedded)
Cloud: turso.io sync (optional, for multi-device)
```

**Benefits**:
- Single source of truth
- Query capabilities (find tools by feature, etc.)
- Versioning/migrations built-in
- Optional cloud sync for multi-device setups
- Better performance for large configs

## Tasks

### Phase 10.1: Database Foundation

#### 1. Add libSQL Dependency
```toml
[dependencies]
libsql = "0.9"
```

#### 2. Create Database Schema
```sql
-- Tools configuration
CREATE TABLE tools (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    cli_command TEXT NOT NULL,
    description TEXT,
    homepage TEXT,
    documentation TEXT,
    requires_npm BOOLEAN DEFAULT FALSE,
    requires_sudo BOOLEAN DEFAULT FALSE,
    status TEXT DEFAULT 'stable',
    enabled BOOLEAN DEFAULT TRUE,
    auto_update BOOLEAN DEFAULT TRUE,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Tool installation commands
CREATE TABLE tool_install (
    tool_id TEXT PRIMARY KEY REFERENCES tools(id),
    command TEXT NOT NULL,
    args TEXT, -- JSON array
    verify_command TEXT,
    post_install_message TEXT
);

-- Tool authentication
CREATE TABLE tool_auth (
    tool_id TEXT PRIMARY KEY REFERENCES tools(id),
    env_vars TEXT, -- JSON array
    setup_url TEXT,
    browser_auth BOOLEAN DEFAULT FALSE,
    auth_instructions TEXT
);

-- User preferences
CREATE TABLE preferences (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Credentials (encrypted)
CREATE TABLE credentials (
    tool_id TEXT PRIMARY KEY,
    env_var TEXT NOT NULL,
    encrypted_value TEXT,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
```

#### 3. Create Database Manager
- `src/db/mod.rs` - Database module entry point
- `src/db/db_connection.rs` - Connection management
- `src/db/db_migrations.rs` - Schema migrations
- `src/db/db_tools.rs` - Tool CRUD operations
- `src/db/db_preferences.rs` - Preferences CRUD

#### 4. Migration from TOML
- [ ] Write migration script to import existing TOML configs
- [ ] Preserve backward compatibility during transition
- [ ] Add `--use-db` flag for testing

### Phase 10.2: Voice Simplification

#### 1. Remove whisper-rs Dependency
- [ ] Remove from Cargo.toml
- [ ] Remove `local-voice` feature flag
- [ ] Remove whisper provider code

#### 2. Create Cloud Voice Provider
```rust
// src/voice/voice_cloud_provider.rs
pub struct CloudVoiceProvider {
    api_key: String,
    provider: VoiceCloudService,
}

pub enum VoiceCloudService {
    OpenAI,
    Deepgram,
    Groq,
}

impl CloudVoiceProvider {
    pub async fn transcribe(&self, audio: &[u8]) -> Result<String> {
        // Simple HTTP POST to API
    }
}
```

#### 3. Simplify Audio Capture
- Use `cpal` for cross-platform audio capture (already lightweight)
- Record to WAV in memory
- Send to cloud API

### Phase 10.3: Integration

#### 1. Update Config Manager
- [ ] Make ConfigManager use database as primary source
- [ ] Fall back to TOML for read-only scenarios
- [ ] Update all config reads to use new DB layer

#### 2. Update Tool Manager
- [ ] Query tools from database
- [ ] Cache hot paths in memory

#### 3. Cloud Sync (Optional)
- [ ] Add Turso remote URL configuration
- [ ] Implement sync on startup/shutdown
- [ ] Handle offline-first scenarios

## Agent Instructions

### Session 1: Database Foundation
```bash
# Add libsql dependency
cargo add libsql

# Create database module structure
mkdir -p src/db

# Start with connection and migrations
```

### Session 2: Tool Migration
```bash
# Write TOML-to-DB migration
# Update ToolManager to use DB
# Test with --use-db flag
```

### Session 3: Voice Simplification
```bash
# Remove whisper-rs
# Implement cloud provider
# Update voice module
```

### Session 4: Integration & Cleanup
```bash
# Remove TOML loading code (keep files as backup)
# Full testing
# Documentation update
```

## Success Criteria

- [ ] `cargo build` works without libclang/whisper.cpp
- [ ] All tool configs stored in SQLite
- [ ] Voice commands work via cloud API
- [ ] Build time reduced (no C++ compilation)
- [ ] Optional Turso sync configured
- [ ] Existing functionality preserved

## Migration Path

1. **Phase 1**: Add DB alongside TOML (dual-write)
2. **Phase 2**: Read from DB, fall back to TOML
3. **Phase 3**: Remove TOML reading code
4. **Phase 4**: Optional: Remove TOML files from repo

## Dependencies

```toml
# Add to Cargo.toml
libsql = "0.9"
# Remove:
# whisper-rs = "..." 
```

## References

- [Turso Documentation](https://docs.turso.tech/)
- [libSQL Rust SDK](https://github.com/tursodatabase/libsql)
- [OpenAI Whisper API](https://platform.openai.com/docs/guides/speech-to-text)
