# Phase 10: Modern Architecture - Database & Voice Simplification

**Status**: IN PROGRESS (Session 1 Complete)  
**Priority**: HIGH  
**Estimated Sessions**: 3-4  
**Last Updated**: 2026-01-03

## Session Progress

### Session 1: Database Foundation - COMPLETE (2026-01-03)

**Commits:**
- `ca842ff` - feat(error): add centralized error module and remove production unwrap calls
- `229cc5d` - feat(db): wire ToolManager to use database with TOML fallback
- `3240857` - refactor(db): eliminate hardcoded SQL strings in repository.rs

**Completed:**
- [x] libsql dependency added (0.9)
- [x] Database schema created (`src/db/schema.rs`)
- [x] QueryBuilder pattern implemented (`src/db/core/query_builder.rs`)
- [x] Repository pattern established (`src/db/core/repository.rs`)
- [x] ToolsRepository, CredentialsRepository, PreferencesRepository created
- [x] TomlImporter for TOML-to-DB migration
- [x] DatabaseManager with connection pooling
- [x] Hybrid tool loading (DB first, TOML fallback)
- [x] `cargo run -- db status` command working
- [x] Error module for graceful error handling (`src/error/`)
- [x] All hardcoded CRUD SQL eliminated (QueryBuilder only)

**Key Files Created/Modified:**
- `src/db/` - Full database module structure
- `src/error/` - Centralized error handling
- `src/tools/tools_db_bridge.rs` - Hybrid DB/TOML bridge functions
- `src/tools/tools_entry_point.rs` - Async ToolManager methods

### Session 2: Voice Simplification - NOT STARTED

### Session 3: Integration & Cleanup - NOT STARTED

---

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

### Phase 10.1: Database Foundation - COMPLETE

#### 1. Add libSQL Dependency - DONE
```toml
[dependencies]
libsql = "0.9"
```

#### 2. Create Database Schema - DONE
Schema implemented in `src/db/schema.rs` with QueryBuilder pattern.
Tables: tools, tool_install, tool_auth, preferences, credentials, schema_migrations

#### 3. Create Database Manager - DONE
- `src/db/mod.rs` - Database module entry point
- `src/db/core/connection.rs` - Connection management (DatabaseManager)
- `src/db/core/migrations.rs` - Schema migrations with versioning
- `src/db/tools/repository.rs` - Tool CRUD operations (ToolsRepository)
- `src/db/preferences/repository.rs` - Preferences CRUD (PreferencesRepository)
- `src/db/credentials/repository.rs` - Credentials CRUD (CredentialsRepository)

#### 4. Migration from TOML - DONE
- [x] TomlImporter script imports existing TOML configs
- [x] Backward compatibility preserved (TOML fallback when DB empty)
- [x] Hybrid loading via `tools_db_bridge.rs`

### Phase 10.2: Voice Simplification - TODO

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

### Phase 10.3: Integration - PARTIAL

#### 1. Update Config Manager - PARTIAL
- [x] ConfigManager can use database as source
- [x] TOML fallback for read-only scenarios preserved
- [ ] Full config reads migration to DB layer (pending)

#### 2. Update Tool Manager - DONE
- [x] Query tools from database via ToolsRepository
- [x] Hybrid lookup in tools_db_bridge.rs
- [ ] Memory caching for hot paths (optional, not critical)

#### 3. Cloud Sync (Optional) - NOT STARTED
- [ ] Add Turso remote URL configuration
- [ ] Implement sync on startup/shutdown
- [ ] Handle offline-first scenarios

---

## Next Session Pickup Guide

### Recommended Next Steps (Priority Order)

1. **Phase 10.2: Voice Simplification** (High Priority)
   - Remove whisper-rs C++ dependency from Cargo.toml
   - Remove `local-voice` feature flag
   - Implement CloudVoiceProvider with OpenAI Whisper API
   - This eliminates the main build complexity issue

2. **Phase 10.3: Full Integration** (Medium Priority)
   - Migrate remaining config reads to DB layer
   - Consider removing TOML loading code (keep files as backup)
   - Full testing across all tools

3. **Optional Enhancements** (Low Priority)
   - Turso cloud sync for multi-device
   - Memory caching for performance
   - `db import` / `db export` CLI commands

### Key Commands to Verify Current State
```bash
cargo run -- db status          # Check database initialization
cargo run -- list               # Verify tools load from DB/TOML
cargo check && cargo clippy -- -D warnings  # Quality gates
```

---

## Agent Instructions

### Session 1: Database Foundation - COMPLETE
See "Session Progress" section above for details.

### Session 2: Voice Simplification - NEXT
```bash
# Remove whisper-rs from Cargo.toml
# Remove local-voice feature flag
# Create src/voice/voice_cloud_provider.rs
# Implement OpenAI Whisper API transcription
# Update voice module to use CloudVoiceProvider
# Test: cargo build (should work without libclang/whisper.cpp)
```

### Session 3: Integration & Cleanup
```bash
# Migrate remaining config reads to DB
# Remove TOML loading code (keep files as backup)
# Full testing
# Documentation update
```

## Success Criteria

- [x] All tool configs stored in SQLite (with TOML fallback)
- [x] QueryBuilder pattern eliminates hardcoded SQL
- [x] Error handling prevents production panics
- [ ] `cargo build` works without libclang/whisper.cpp (pending whisper-rs removal)
- [ ] Voice commands work via cloud API (pending)
- [ ] Build time reduced (pending whisper-rs removal)
- [ ] Optional Turso sync configured (deferred)
- [x] Existing functionality preserved

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
