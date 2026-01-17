// Native Platform Voice Provider Implementation
// Uses platform-native speech recognition with platform-specific implementations
//
// This provider delegates to platform-specific modules:
// - Windows: Windows Speech Recognition via PowerShell (src/voice/platforms/windows.rs)
// - Linux: Audio capture + Whisper transcription (src/voice/platforms/linux.rs)
// - macOS: Future implementation (currently uses fallback)

use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
use anyhow::{anyhow, Result};
use std::future::Future;
use std::pin::Pin;

// Import platform-specific modules
#[cfg(target_os = "windows")]
use super::platforms::windows;

#[cfg(target_os = "linux")]
use super::platforms::linux;

// Dev environment support (always available for Codespaces detection)
use super::platforms::dev;

/// Native platform voice provider (no external dependencies)
pub struct NativeVoiceProvider {
    config: VoiceProviderConfig,
}

impl NativeVoiceProvider {
    /// Create a new native voice provider
    pub fn new(config: VoiceProviderConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Check if platform supports native voice recognition
    pub fn is_supported() -> bool {
        // Windows and Linux have speech recognition support
        // Dev environments (Codespaces) always supported with text-based simulation
        // macOS can be added later
        true // All platforms now supported (dev mode for Codespaces/containers)
    }

    /// Listen for speech on Linux using audio capture + Whisper transcription
    #[cfg(target_os = "linux")]
    async fn listen_linux(&self) -> Result<String> {
        // Create temp directory for audio file
        let temp_dir = std::env::temp_dir();
        let audio_path = temp_dir.join("terminal_jarvis_voice.wav");

        // Capture audio using platform-specific Linux implementation
        linux::capture_audio(self.config.max_duration, &audio_path).await?;

        // Transcribe the audio file using Whisper
        // This will use whatever Whisper provider is available (API or local)
        let transcription = self.transcribe_audio_file(&audio_path).await?;

        // Cleanup temp file
        let _ = linux::cleanup_audio_file(&audio_path).await;

        Ok(transcription)
    }

    /// Transcribe an audio file using available Whisper provider
    #[cfg(target_os = "linux")]
    async fn transcribe_audio_file(&self, audio_path: &std::path::PathBuf) -> Result<String> {
        // Try to use OpenAI Whisper API if available
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            if !api_key.is_empty() {
                return self.transcribe_with_openai(audio_path).await;
            }
        }

        // Try Groq API if available (free tier)
        if let Ok(api_key) = std::env::var("GROQ_API_KEY") {
            if !api_key.is_empty() {
                return self.transcribe_with_openai(audio_path).await;
            }
        }

        Err(anyhow!(
            "No transcription service available.\n\
             Set one of these environment variables:\n\
             - OPENAI_API_KEY (recommended, best accuracy)\n\
             - GROQ_API_KEY (free tier available)"
        ))
    }

    /// Transcribe audio using OpenAI Whisper API
    #[cfg(target_os = "linux")]
    async fn transcribe_with_openai(&self, audio_path: &std::path::PathBuf) -> Result<String> {
        // Direct API call to OpenAI Whisper - simpler than using WhisperProvider
        // which expects to handle its own audio recording

        // Read audio file
        let audio_data = tokio::fs::read(audio_path).await?;

        // The WhisperProvider expects to handle its own recording, so we'll use a workaround
        // by temporarily saving the audio and letting it process
        // For now, we'll use a direct API call

        let client = reqwest::Client::new();
        let api_key = std::env::var("OPENAI_API_KEY")?;

        let form = reqwest::multipart::Form::new()
            .text("model", "whisper-1")
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_data)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?,
            );

        let response = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {api_key}"))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "OpenAI Whisper API error: {}",
                response.text().await?
            ));
        }

        let result: serde_json::Value = response.json().await?;
        let text = result["text"]
            .as_str()
            .ok_or_else(|| anyhow!("No text in response"))?
            .to_string();

        Ok(text)
    }
}

impl VoiceInputProvider for NativeVoiceProvider {
    fn listen(&self) -> Pin<Box<dyn Future<Output = Result<VoiceRecognitionResult>> + Send + '_>> {
        Box::pin(async move {
            // FIRST: Check if we're in a dev environment (Codespaces, Docker, etc.)
            if dev::is_dev_environment().await {
                let transcription = dev::simulate_voice_input(self.config.max_duration).await?;

                return Ok(VoiceRecognitionResult {
                    text: transcription,
                    confidence: 1.0, // Text input is always "confident"
                    duration: self.config.max_duration,
                    metadata: Some(VoiceMetadata {
                        language: Some(self.config.language.clone()),
                        tokens_used: None,
                        extra: std::collections::HashMap::new(),
                    }),
                });
            }

            // Platform-specific implementations for environments with audio hardware
            #[cfg(target_os = "windows")]
            {
                // Windows: Use direct speech recognition via PowerShell (no audio file)
                let transcription =
                    windows::listen_windows_direct(self.config.max_duration).await?;

                return Ok(VoiceRecognitionResult {
                    text: transcription,
                    confidence: 0.8,
                    duration: self.config.max_duration,
                    metadata: Some(VoiceMetadata {
                        language: Some(self.config.language.clone()),
                        tokens_used: None,
                        extra: std::collections::HashMap::new(),
                    }),
                });
            }

            #[cfg(target_os = "linux")]
            {
                // Linux: Capture audio and transcribe using Whisper
                let transcription = self.listen_linux().await?;

                Ok(VoiceRecognitionResult {
                    text: transcription,
                    confidence: 0.8,
                    duration: self.config.max_duration,
                    metadata: Some(VoiceMetadata {
                        language: Some(self.config.language.clone()),
                        tokens_used: None,
                        extra: std::collections::HashMap::new(),
                    }),
                })
            }

            #[cfg(target_os = "macos")]
            {
                // macOS doesn't have simple built-in CLI speech recognition yet
                Err(anyhow!(
                    "macOS native speech recognition requires additional setup.\n\
                     Use cloud API: Set OPENAI_API_KEY or GROQ_API_KEY environment variable"
                ))
            }
        })
    }

    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        Box::pin(async move {
            // Check if platform is supported
            if !Self::is_supported() {
                return Ok(false);
            }

            // FIRST: Check if we're in a dev environment (always ready with text input)
            if dev::is_ready().await? {
                return Ok(true);
            }

            // Check if required tools are available using platform-specific checks
            #[cfg(target_os = "windows")]
            {
                windows::is_ready().await
            }

            #[cfg(target_os = "linux")]
            {
                linux::is_ready().await
            }

            #[cfg(target_os = "macos")]
            {
                Ok(false) // macOS not yet implemented
            }
        })
    }

    fn config(&self) -> &VoiceProviderConfig {
        &self.config
    }

    fn provider_name(&self) -> &str {
        #[cfg(target_os = "windows")]
        {
            "Windows Native Speech Recognition"
        }

        #[cfg(target_os = "linux")]
        {
            "Linux Audio Capture + Whisper"
        }

        #[cfg(target_os = "macos")]
        {
            "macOS (Native Not Yet Supported)"
        }
    }
}
