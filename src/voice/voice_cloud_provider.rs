// Cloud Voice Provider Implementation
// Provides voice-to-text using cloud transcription APIs
//
// Supported services:
// - OpenAI Whisper API (recommended, users often have OPENAI_API_KEY)
// - Deepgram (fast, streaming support)
// - Groq Whisper (fast, free tier available)
//
// This replaces local whisper-rs to avoid C++ compilation dependencies.

use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

/// Cloud voice service options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceCloudService {
    /// OpenAI Whisper API - Best accuracy, simple API, $0.006/min
    OpenAI,
    /// Deepgram - Fast, streaming support
    Deepgram,
    /// Groq Whisper - Fast inference, free tier available
    Groq,
}

impl VoiceCloudService {
    /// Get the API endpoint for this service
    fn endpoint(&self) -> &'static str {
        match self {
            VoiceCloudService::OpenAI => "https://api.openai.com/v1/audio/transcriptions",
            VoiceCloudService::Deepgram => "https://api.deepgram.com/v1/listen",
            VoiceCloudService::Groq => "https://api.groq.com/openai/v1/audio/transcriptions",
        }
    }

    /// Get the environment variable name for the API key
    fn api_key_env(&self) -> &'static str {
        match self {
            VoiceCloudService::OpenAI => "OPENAI_API_KEY",
            VoiceCloudService::Deepgram => "DEEPGRAM_API_KEY",
            VoiceCloudService::Groq => "GROQ_API_KEY",
        }
    }

    /// Get display name for this service
    pub fn display_name(&self) -> &'static str {
        match self {
            VoiceCloudService::OpenAI => "OpenAI Whisper",
            VoiceCloudService::Deepgram => "Deepgram",
            VoiceCloudService::Groq => "Groq Whisper",
        }
    }

    /// Get the model name to use for this service
    fn model(&self) -> &'static str {
        match self {
            VoiceCloudService::OpenAI => "whisper-1",
            VoiceCloudService::Deepgram => "nova-2", // Best Deepgram model
            VoiceCloudService::Groq => "whisper-large-v3",
        }
    }
}

impl std::fmt::Display for VoiceCloudService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Cloud voice provider that sends audio to transcription APIs
pub struct CloudVoiceProvider {
    config: VoiceProviderConfig,
    service: VoiceCloudService,
    api_key: String,
    temp_dir: PathBuf,
}

impl CloudVoiceProvider {
    /// Create a new cloud voice provider with a specific service
    pub fn new(config: VoiceProviderConfig, service: VoiceCloudService) -> Result<Self> {
        let api_key = std::env::var(service.api_key_env()).map_err(|_| {
            anyhow!(
                "No API key found for {}. Set {} environment variable.\n\
                 You can configure it in Terminal Jarvis authentication menu.",
                service.display_name(),
                service.api_key_env()
            )
        })?;

        let temp_dir = std::env::temp_dir();

        Ok(Self {
            config,
            service,
            api_key,
            temp_dir,
        })
    }

    /// Create a provider with automatic service detection
    /// Tries services in order: OpenAI -> Groq -> Deepgram
    pub fn auto_detect(config: VoiceProviderConfig) -> Result<Self> {
        // Try OpenAI first (most common)
        if std::env::var("OPENAI_API_KEY").is_ok() {
            return Self::new(config, VoiceCloudService::OpenAI);
        }

        // Try Groq (free tier available)
        if std::env::var("GROQ_API_KEY").is_ok() {
            return Self::new(config, VoiceCloudService::Groq);
        }

        // Try Deepgram
        if std::env::var("DEEPGRAM_API_KEY").is_ok() {
            return Self::new(config, VoiceCloudService::Deepgram);
        }

        Err(anyhow!(
            "No cloud voice API key found.\n\n\
             Set one of these environment variables:\n\
             - OPENAI_API_KEY (recommended, best accuracy)\n\
             - GROQ_API_KEY (free tier available)\n\
             - DEEPGRAM_API_KEY (fast, streaming)\n\n\
             You can configure API keys in Terminal Jarvis authentication menu."
        ))
    }

    /// Record audio to a temporary file
    async fn record_audio(&self) -> Result<PathBuf> {
        let audio_file = self.temp_dir.join("voice_input.wav");
        let duration_secs = self.config.max_duration.as_secs();

        println!(
            "\n[LISTENING] Speak now... ({}s max, using {})",
            duration_secs,
            self.service.display_name()
        );

        // Detect platform and use appropriate recording tool
        #[cfg(target_os = "linux")]
        let recording_command = format!(
            "arecord -d {} -f cd -t wav {}",
            duration_secs,
            audio_file.display()
        );

        #[cfg(target_os = "macos")]
        let recording_command = format!(
            "rec -r 16000 -c 1 {} trim 0 {}",
            audio_file.display(),
            duration_secs
        );

        #[cfg(target_os = "windows")]
        let recording_command = format!(
            "ffmpeg -f dshow -i audio=\"Microphone\" -t {} -y {}",
            duration_secs,
            audio_file.display()
        );

        // Execute recording command
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&recording_command)
            .output()
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to record audio: {e}\n\
                     Make sure recording tools are installed:\n\
                     - Linux: 'sudo apt-get install alsa-utils'\n\
                     - macOS: 'brew install sox'\n\
                     - Windows: Install FFmpeg"
                )
            })?;

        if !output.status.success() {
            return Err(anyhow!(
                "Recording failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("[RECORDED] Processing audio with {}...", self.service);
        Ok(audio_file)
    }

    /// Transcribe audio using OpenAI Whisper API
    async fn transcribe_openai(&self, audio_data: Vec<u8>) -> Result<VoiceRecognitionResult> {
        let client = reqwest::Client::new();

        let form = reqwest::multipart::Form::new()
            .text("model", self.service.model())
            .text("language", self.config.language.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_data)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?,
            );

        let response = client
            .post(self.service.endpoint())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("{} API error: {}", self.service, error_text));
        }

        let result: OpenAIResponse = response.json().await?;

        Ok(VoiceRecognitionResult {
            text: result.text,
            confidence: 0.9, // Whisper doesn't provide confidence
            duration: self.config.max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(self.config.language.clone()),
                tokens_used: None,
                extra: HashMap::from([("service".to_string(), self.service.to_string())]),
            }),
        })
    }

    /// Transcribe audio using Deepgram API
    async fn transcribe_deepgram(&self, audio_data: Vec<u8>) -> Result<VoiceRecognitionResult> {
        let client = reqwest::Client::new();

        let url = format!(
            "{}?model={}&language={}",
            self.service.endpoint(),
            self.service.model(),
            self.config.language
        );

        let response = client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "audio/wav")
            .body(audio_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Deepgram API error: {error_text}"));
        }

        let result: DeepgramResponse = response.json().await?;

        // Extract text and confidence from nested response
        let (text, confidence) = result
            .results
            .as_ref()
            .and_then(|r| r.channels.first())
            .and_then(|c| c.alternatives.first())
            .map(|a| (a.transcript.clone(), a.confidence))
            .unwrap_or_else(|| (String::new(), 0.0));

        Ok(VoiceRecognitionResult {
            text,
            confidence,
            duration: self.config.max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(self.config.language.clone()),
                tokens_used: None,
                extra: HashMap::from([("service".to_string(), "Deepgram".to_string())]),
            }),
        })
    }

    /// Transcribe audio using Groq API (OpenAI-compatible)
    async fn transcribe_groq(&self, audio_data: Vec<u8>) -> Result<VoiceRecognitionResult> {
        // Groq uses OpenAI-compatible API format
        let client = reqwest::Client::new();

        let form = reqwest::multipart::Form::new()
            .text("model", self.service.model())
            .text("language", self.config.language.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_data)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?,
            );

        let response = client
            .post(self.service.endpoint())
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Groq API error: {error_text}"));
        }

        let result: OpenAIResponse = response.json().await?;

        Ok(VoiceRecognitionResult {
            text: result.text,
            confidence: 0.9,
            duration: self.config.max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(self.config.language.clone()),
                tokens_used: None,
                extra: HashMap::from([("service".to_string(), "Groq Whisper".to_string())]),
            }),
        })
    }

    /// Transcribe audio using the configured service
    async fn transcribe_audio(&self, audio_path: &PathBuf) -> Result<VoiceRecognitionResult> {
        let audio_data = tokio::fs::read(audio_path).await?;

        let result = match self.service {
            VoiceCloudService::OpenAI => self.transcribe_openai(audio_data).await,
            VoiceCloudService::Deepgram => self.transcribe_deepgram(audio_data).await,
            VoiceCloudService::Groq => self.transcribe_groq(audio_data).await,
        }?;

        println!("[TRANSCRIBED] \"{}\"", result.text);
        Ok(result)
    }

    /// Clean up temporary audio file
    async fn cleanup(&self, audio_path: &PathBuf) -> Result<()> {
        if audio_path.exists() {
            tokio::fs::remove_file(audio_path).await?;
        }
        Ok(())
    }
}

impl VoiceInputProvider for CloudVoiceProvider {
    fn listen(&self) -> Pin<Box<dyn Future<Output = Result<VoiceRecognitionResult>> + Send + '_>> {
        Box::pin(async {
            // Record audio
            let audio_path = self.record_audio().await?;

            // Transcribe
            let result = self.transcribe_audio(&audio_path).await;

            // Cleanup
            let _ = self.cleanup(&audio_path).await;

            result
        })
    }

    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        Box::pin(async {
            // Check if API key is present
            if self.api_key.is_empty() {
                return Ok(false);
            }

            // Check if recording tools are available
            #[cfg(target_os = "linux")]
            let check_cmd = "which arecord";

            #[cfg(target_os = "macos")]
            let check_cmd = "which rec";

            #[cfg(target_os = "windows")]
            let check_cmd = "where ffmpeg";

            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(check_cmd)
                .output()
                .await;

            if !output.as_ref().map(|o| o.status.success()).unwrap_or(false) {
                return Ok(false);
            }

            // Check if audio devices are available (Linux only)
            #[cfg(target_os = "linux")]
            {
                let devices_check = tokio::process::Command::new("arecord")
                    .arg("--list-devices")
                    .output()
                    .await;

                if let Ok(output) = devices_check {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                    // If "no soundcards found" in stdout/stderr, no devices available
                    if stdout_str.contains("no soundcards found")
                        || stderr_str.contains("no soundcards found")
                    {
                        return Ok(false);
                    }

                    if !output.status.success() {
                        return Ok(false);
                    }
                }
            }

            Ok(true)
        })
    }

    fn config(&self) -> &VoiceProviderConfig {
        &self.config
    }

    fn provider_name(&self) -> &str {
        self.service.display_name()
    }

    fn estimated_tokens_per_second(&self) -> Option<u64> {
        Some(10) // Approximate estimate
    }
}

/// OpenAI/Groq API response format
#[derive(Debug, serde::Deserialize)]
struct OpenAIResponse {
    text: String,
}

/// Deepgram API response format
#[derive(Debug, serde::Deserialize)]
struct DeepgramResponse {
    results: Option<DeepgramResults>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct DeepgramResults {
    channels: Vec<DeepgramChannel>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct DeepgramChannel {
    alternatives: Vec<DeepgramAlternative>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct DeepgramAlternative {
    transcript: String,
    confidence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_endpoints() {
        assert!(VoiceCloudService::OpenAI.endpoint().contains("openai.com"));
        assert!(VoiceCloudService::Deepgram
            .endpoint()
            .contains("deepgram.com"));
        assert!(VoiceCloudService::Groq.endpoint().contains("groq.com"));
    }

    #[test]
    fn test_service_display_names() {
        assert_eq!(VoiceCloudService::OpenAI.display_name(), "OpenAI Whisper");
        assert_eq!(VoiceCloudService::Deepgram.display_name(), "Deepgram");
        assert_eq!(VoiceCloudService::Groq.display_name(), "Groq Whisper");
    }

    #[test]
    fn test_service_api_key_env() {
        assert_eq!(VoiceCloudService::OpenAI.api_key_env(), "OPENAI_API_KEY");
        assert_eq!(
            VoiceCloudService::Deepgram.api_key_env(),
            "DEEPGRAM_API_KEY"
        );
        assert_eq!(VoiceCloudService::Groq.api_key_env(), "GROQ_API_KEY");
    }
}
