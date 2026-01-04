// Voice Input Domain Module
// Provides voice-to-text capabilities with swappable provider architecture
//
// This module follows the domain-based architecture pattern and allows
// for different voice recognition services to be plugged in without
// affecting the core application logic.
//
// Cloud-first approach: Uses cloud transcription APIs (OpenAI Whisper, Deepgram, Groq)
// for reliable, cross-platform voice recognition without C++ compilation dependencies.
//
// Platform-specific implementations are in src/voice/platforms/

mod voice_cloud_provider;
mod voice_command;
mod voice_native_provider;
mod voice_provider;
mod voice_smart_listening;
mod voice_whisper_provider;

// Platform-specific implementations
mod platforms;

// Re-export public interfaces
pub use voice_cloud_provider::{CloudVoiceProvider, VoiceCloudService};
pub use voice_command::{VoiceCommand, VoiceCommandParser};
pub use voice_native_provider::NativeVoiceProvider;
pub use voice_provider::{VoiceInputProvider, VoiceProviderConfig, VoiceRecognitionResult};
pub use voice_smart_listening::{
    SmartVoiceListener, VoiceFeedback, VoiceFeedbackType, VoiceListenerFactory,
};
pub use voice_whisper_provider::WhisperProvider;
