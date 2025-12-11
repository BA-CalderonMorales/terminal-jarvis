# Phase 2: Voice Feature

**Status**: DEFERRED

## Objective

Implement voice command support for Terminal Jarvis, allowing hands-free navigation and tool control.

## Why Deferred

- GitHub Codespaces lacks audio hardware
- Voice feature requires microphone access
- Current implementation has 3 failing tests in voice_command.rs
- Lower priority than core CLI functionality

## Prerequisites for Future Work

1. Development environment with audio hardware (local machine or VM with audio)
2. Install audio tools:
   - Linux: `sudo apt-get install alsa-utils`
   - macOS: `brew install sox`
   - Windows: FFmpeg
3. API key for cloud transcription (OPENAI_API_KEY) OR local whisper setup

## Remaining Tasks

### 1. Fix Voice Command Parser Tests
- [ ] Fix `test_parse_update_all_command` - parsing "upgrade all" as ListTools instead of UpdateAllTools
- [ ] Fix `test_parse_list_installed_command` - similar parsing issue
- [ ] Fix `test_parse_back_command` - parsing issue

**Files**: `src/voice/voice_command.rs`

### 2. Re-enable /voice Command
- [ ] Add `/voice` back to `display_available_commands()` in cli_logic_interactive.rs
- [ ] Restore `/voice` handler block
- [ ] Test voice recognition flow end-to-end

### 3. Improve Voice UX
- [ ] Add voice activation indicator (visual feedback when listening)
- [ ] Implement wake word detection ("hey jarvis")
- [ ] Add voice command confirmation before execution
- [ ] Support for cancelling voice commands

### 4. Local Voice Option
- [ ] Test `--features local-voice` build
- [ ] Document local whisper setup process
- [ ] Validate whisper model loading from cache

## Agent Instructions

When resuming this phase:

1. First, run voice tests to understand current failures:
   ```bash
   cargo test voice 2>&1 | head -50
   ```

2. Read the voice command parser implementation:
   ```bash
   rg "UpdateAllTools|ListTools" src/voice/voice_command.rs -C 5
   ```

3. Fix the command parsing logic (likely priority/ordering issue in pattern matching)

4. Re-run tests to verify fixes:
   ```bash
   cargo test voice
   ```

5. Only proceed to re-enabling /voice after tests pass
