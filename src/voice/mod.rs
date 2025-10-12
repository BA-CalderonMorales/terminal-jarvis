// Voice Input Domain Module
// Provides voice-to-text capabilities with swappable provider architecture
//
// This module follows the domain-based architecture pattern and allows
// for different voice recognition services to be plugged in without
// affecting the core application logic.

mod voice_command;
mod voice_provider;
mod voice_smart_listening;
mod voice_whisper_provider;

#[cfg(feature = "local-voice")]
mod voice_local_whisper_provider;

// Re-export public interfaces
pub use voice_command::{VoiceCommand, VoiceCommandParser};
pub use voice_provider::{VoiceInputProvider, VoiceProviderConfig, VoiceRecognitionResult};
pub use voice_smart_listening::{SmartVoiceListener, VoiceFeedback, VoiceFeedbackType, VoiceListenerFactory};
pub use voice_whisper_provider::WhisperProvider;

#[cfg(feature = "local-voice")]
pub use voice_local_whisper_provider::LocalWhisperProvider;
