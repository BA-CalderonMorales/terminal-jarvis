// OpenAI Whisper Voice Provider Implementation
// Provides voice-to-text using OpenAI's Whisper API

use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
use anyhow::{anyhow, Result};
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

/// OpenAI Whisper provider implementation
pub struct WhisperProvider {
    config: VoiceProviderConfig,
    api_key: String,
    temp_dir: PathBuf,
}

impl WhisperProvider {
    /// Create a new Whisper provider
    pub fn new(config: VoiceProviderConfig) -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .or_else(|_| std::env::var("ANTHROPIC_API_KEY"))
            .map_err(|_| {
                anyhow!(
                    "No API key found. Set OPENAI_API_KEY environment variable.\n\
                     You can configure it in Terminal Jarvis authentication menu."
                )
            })?;

        let temp_dir = std::env::temp_dir();

        Ok(Self {
            config,
            api_key,
            temp_dir,
        })
    }

    /// Record audio to a temporary file
    async fn record_audio(&self) -> Result<PathBuf> {
        let audio_file = self.temp_dir.join("voice_input.wav");
        let duration_secs = self.config.max_duration.as_secs();

        println!("\n[LISTENING] Speak now... ({}s max)", duration_secs);

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
                    "Failed to record audio: {}\n\
                     Make sure recording tools are installed:\n\
                     - Linux: 'sudo apt-get install alsa-utils'\n\
                     - macOS: 'brew install sox'\n\
                     - Windows: Install FFmpeg",
                    e
                )
            })?;

        if !output.status.success() {
            return Err(anyhow!(
                "Recording failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        println!("[RECORDED] Processing audio...");
        Ok(audio_file)
    }

    /// Send audio to OpenAI Whisper API
    async fn transcribe_audio(&self, audio_path: &PathBuf) -> Result<VoiceRecognitionResult> {
        let client = reqwest::Client::new();

        // Read audio file
        let audio_data = tokio::fs::read(audio_path).await?;

        // Build multipart form
        let form = reqwest::multipart::Form::new()
            .text("model", "whisper-1")
            .text("language", self.config.language.clone())
            .part(
                "file",
                reqwest::multipart::Part::bytes(audio_data)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?,
            );

        // Send request
        let response = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Whisper API error: {}", error_text));
        }

        let result: WhisperResponse = response.json().await?;

        println!("[TRANSCRIBED] \"{}\"", result.text);

        Ok(VoiceRecognitionResult {
            text: result.text,
            confidence: 0.9, // Whisper doesn't provide confidence, use high default
            duration: self.config.max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(self.config.language.clone()),
                tokens_used: None,
                extra: std::collections::HashMap::new(),
            }),
        })
    }

    /// Clean up temporary audio file
    async fn cleanup(&self, audio_path: &PathBuf) -> Result<()> {
        if audio_path.exists() {
            tokio::fs::remove_file(audio_path).await?;
        }
        Ok(())
    }
}

impl VoiceInputProvider for WhisperProvider {
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

                    // If "no soundcards found" is in stdout or stderr, no devices available
                    if stdout_str.contains("no soundcards found")
                        || stderr_str.contains("no soundcards found")
                    {
                        return Ok(false);
                    }

                    // Also check if the command failed
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
        "OpenAI Whisper"
    }

    fn estimated_tokens_per_second(&self) -> Option<u64> {
        Some(10) // Approximate estimate
    }
}

/// Whisper API response format
#[derive(Debug, serde::Deserialize)]
struct WhisperResponse {
    text: String,
}
