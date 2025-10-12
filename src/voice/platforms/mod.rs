// Platform-Specific Voice Provider Implementations
//
// This module contains platform-specific implementations for voice recognition:
// - Windows: Native Speech Recognition via PowerShell
// - Linux: Audio capture + Whisper transcription
// - Dev: Text-based simulation for Codespaces and environments without audio
// - macOS: Future implementation (currently uses fallback)

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

// Dev environment support (Codespaces, containers without audio)
pub mod dev;

// macOS support can be added here in the future
// #[cfg(target_os = "macos")]
// pub mod macos;
