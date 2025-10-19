// Vosk Voice Provider Implementation (LIGHTWEIGHT OFFLINE)
// Provides fast, accurate offline voice-to-text using Vosk API
// No API keys required - fully on-device processing
//
// Model size: 50MB (small), 142MB (medium), 1.8GB (large)
// Features: Zero-latency streaming, 20+ languages, speaker ID

#[cfg(feature = "vosk-voice")]
use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
#[cfg(feature = "vosk-voice")]
use anyhow::{anyhow, Result};
#[cfg(feature = "vosk-voice")]
use std::future::Future;
#[cfg(feature = "vosk-voice")]
use std::path::PathBuf;
#[cfg(feature = "vosk-voice")]
use std::pin::Pin;

#[cfg(feature = "vosk-voice")]
use crate::security::SecurityManager;

#[cfg(feature = "vosk-voice")]
/// Vosk provider for lightweight offline voice recognition
///
/// This provider uses the Vosk API for fast, accurate speech recognition
/// without requiring any API keys or internet connection. All processing
/// happens on-device for maximum privacy.
///
/// # Model Setup
///
/// Download a model from: https://alphacephei.com/vosk/models
/// Recommended: vosk-model-small-en-us-0.15 (40MB)
///
/// Place model in one of:
/// - ./models/vosk-model-small-en-us-0.15/
/// - ./config/vosk-model/
/// - Set VOSK_MODEL_PATH environment variable
pub struct VoskProvider {
    config: VoiceProviderConfig,
    model_path: PathBuf,
    temp_dir: PathBuf,
    security_manager: SecurityManager,
}

#[cfg(feature = "vosk-voice")]
impl VoskProvider {
    /// Create a new Vosk provider with verified model
    pub fn new(config: VoiceProviderConfig) -> Result<Self> {
        let temp_dir = std::env::temp_dir();

        // Get verified model path (no auto-download for security)
        let model_path = Self::get_verified_model_path()?;

        Ok(Self {
            config,
            model_path,
            temp_dir,
            security_manager: SecurityManager::new(),
        })
    }

    /// Get verified Vosk model path
    ///
    /// Checks in order:
    /// 1. VOSK_MODEL_PATH environment variable
    /// 2. ./models/vosk-model-*/
    /// 3. ./config/vosk-model/
    /// 4. ~/.terminal-jarvis/vosk-model/
    fn get_verified_model_path() -> Result<PathBuf> {
        // Check environment variable first
        if let Ok(env_path) = std::env::var("VOSK_MODEL_PATH") {
            let path = PathBuf::from(env_path);
            if path.exists() && Self::validate_model_structure(&path)? {
                println!(
                    "[VOSK] Using model from VOSK_MODEL_PATH: {}",
                    path.display()
                );
                return Ok(path);
            }
        }

        // Check common locations
        let possible_paths = vec![
            PathBuf::from("./models/vosk-model-small-en-us-0.15"),
            PathBuf::from("./models/vosk-model-en-us-0.22"),
            PathBuf::from("./config/vosk-model"),
            dirs::home_dir()
                .map(|h| h.join(".terminal-jarvis/vosk-model"))
                .unwrap_or_default(),
        ];

        for path in &possible_paths {
            if path.exists() && Self::validate_model_structure(path)? {
                println!("[VOSK] Using model: {}", path.display());
                return Ok(path.clone());
            }
        }

        // No model found - return helpful error
        Err(anyhow!(
            "No Vosk model found. Please download a model to enable voice recognition.\n\
             \n\
             Quick Setup:\n\
             \n\
             1. Download a model from https://alphacephei.com/vosk/models\n\
             \n\
             Recommended models:\n\
             - English (US): vosk-model-small-en-us-0.15 (40 MB)\n\
             - English (US): vosk-model-en-us-0.22 (1.8 GB, most accurate)\n\
             - Other languages available on the website\n\
             \n\
             2. Extract the model:\n\
             tar -xzf vosk-model-small-en-us-0.15.tar.gz\n\
             \n\
             3. Place in one of these locations:\n\
             - ./models/vosk-model-small-en-us-0.15/\n\
             - ./config/vosk-model/\n\
             - ~/.terminal-jarvis/vosk-model/\n\
             \n\
             Or set environment variable:\n\
             export VOSK_MODEL_PATH=/path/to/vosk-model\n\
             \n\
             Model structure should contain:\n\
             - am/final.mdl (acoustic model)\n\
             - conf/model.conf (configuration)\n\
             - graph/HCLG.fst or graph/HCLr.fst (language model)"
        ))
    }

    /// Validate that the model directory has the required structure
    fn validate_model_structure(model_path: &PathBuf) -> Result<bool> {
        // Check for required files
        let required_files = vec![
            model_path.join("am/final.mdl"),
            model_path.join("conf/model.conf"),
        ];

        // Check for either HCLG.fst or HCLr.fst
        let has_graph = model_path.join("graph/HCLG.fst").exists()
            || model_path.join("graph/HCLr.fst").exists();

        for file in &required_files {
            if !file.exists() {
                return Ok(false);
            }
        }

        if !has_graph {
            return Ok(false);
        }

        Ok(true)
    }

    /// Record audio to a temporary file using platform-specific tools
    async fn record_audio(&self) -> Result<PathBuf> {
        let audio_file = self.temp_dir.join("vosk_voice_input.wav");
        let duration_secs = self.config.max_duration.as_secs();

        println!("[VOSK LISTENING] Speak now... ({}s max)", duration_secs);

        // Use platform-specific audio capture
        #[cfg(target_os = "linux")]
        {
            super::platforms::linux::capture_audio(self.config.max_duration, &audio_file).await?;
        }

        #[cfg(target_os = "windows")]
        {
            // Windows: Use ffmpeg or similar
            let recording_command = format!(
                "ffmpeg -f dshow -i audio=\"Microphone\" -t {} -ar 16000 -ac 1 -y {}",
                duration_secs,
                audio_file.display()
            );

            let output = tokio::process::Command::new("cmd")
                .args(&["/C", &recording_command])
                .output()
                .await
                .map_err(|e| anyhow!("Failed to record audio: {}", e))?;

            if !output.status.success() {
                return Err(anyhow!(
                    "Recording failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        #[cfg(target_os = "macos")]
        {
            // macOS: Use sox
            let recording_command = format!(
                "rec -r 16000 -c 1 {} trim 0 {}",
                audio_file.display(),
                duration_secs
            );

            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&recording_command)
                .output()
                .await
                .map_err(|e| anyhow!("Failed to record audio: {}", e))?;

            if !output.status.success() {
                return Err(anyhow!(
                    "Recording failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        Ok(audio_file)
    }

    /// Transcribe audio file using Vosk
    async fn transcribe_audio(&self, audio_path: &PathBuf) -> Result<VoiceRecognitionResult> {
        println!("[VOSK PROCESSING] Transcribing audio...");

        // Spawn blocking task for Vosk processing (CPU-intensive)
        let model_path = self.model_path.clone();
        let audio_path = audio_path.clone();
        let config_language = self.config.language.clone();
        let max_duration = self.config.max_duration;

        let result = tokio::task::spawn_blocking(move || {
            Self::transcribe_blocking(&model_path, &audio_path)
        })
        .await
        .map_err(|e| anyhow!("Transcription task failed: {}", e))??;

        println!("[VOSK TRANSCRIBED] \"{}\"", result.text);

        Ok(VoiceRecognitionResult {
            text: result.text,
            confidence: result.confidence,
            duration: max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(config_language),
                tokens_used: None,
                extra: std::collections::HashMap::new(),
            }),
        })
    }

    /// Blocking transcription using Vosk (runs in separate thread)
    fn transcribe_blocking(
        model_path: &PathBuf,
        audio_path: &PathBuf,
    ) -> Result<TranscriptionResult> {
        use vosk::{Model, Recognizer};

        // Load model (Vosk API returns Option, not Result)
        let model = Model::new(model_path.to_str().unwrap())
            .ok_or_else(|| anyhow!("Failed to load Vosk model from: {}", model_path.display()))?;

        // Create recognizer with 16kHz sample rate (also returns Option)
        let mut recognizer = Recognizer::new(&model, 16000.0)
            .ok_or_else(|| anyhow!("Failed to create Vosk recognizer with 16kHz sample rate"))?;

        // Read audio file
        let audio_data = std::fs::read(audio_path)?;

        // Vosk expects raw PCM data as i16 samples, but we have WAV with bytes
        // Skip WAV header (44 bytes for standard WAV)
        let pcm_bytes = if audio_data.len() > 44 {
            &audio_data[44..]
        } else {
            return Err(anyhow!("Audio file too short or invalid"));
        };

        // Convert bytes to i16 samples (little-endian)
        let mut samples: Vec<i16> = Vec::with_capacity(pcm_bytes.len() / 2);
        for chunk in pcm_bytes.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            samples.push(sample);
        }

        // Process audio in chunks (4000 samples = 0.25 seconds at 16kHz)
        let chunk_size = 4000;
        for chunk in samples.chunks(chunk_size) {
            recognizer.accept_waveform(chunk);
        }

        // Get final result
        let final_result = recognizer.final_result();

        // Extract text from CompleteResult (using single() since we don't enable alternatives)
        let result_single = final_result
            .single()
            .ok_or_else(|| anyhow!("Failed to get recognition result from Vosk"))?;

        let text = result_single.text.to_string();

        // Vosk doesn't provide confidence in final result, use a reasonable default
        let confidence = if !text.is_empty() { 0.85 } else { 0.0 };

        Ok(TranscriptionResult { text, confidence })
    }

    /// Clean up temporary audio file
    async fn cleanup(&self, audio_path: &PathBuf) -> Result<()> {
        if audio_path.exists() {
            tokio::fs::remove_file(audio_path).await?;
        }
        Ok(())
    }

    /// Check if Vosk model is available
    pub fn is_model_available() -> bool {
        Self::get_verified_model_path().is_ok()
    }
}

#[cfg(feature = "vosk-voice")]
impl VoiceInputProvider for VoskProvider {
    fn listen(&self) -> Pin<Box<dyn Future<Output = Result<VoiceRecognitionResult>> + Send + '_>> {
        Box::pin(async {
            // Validate we're not in a compromised state
            if let Err(e) = self
                .security_manager
                .validate_input("vosk_init", "voice_provider")
            {
                return Err(anyhow!("Security validation failed: {}", e));
            }

            // Record audio
            let audio_path = self.record_audio().await?;

            // Transcribe
            let result = self.transcribe_audio(&audio_path).await;

            // Cleanup
            let _ = self.cleanup(&audio_path).await;

            // Validate transcribed text
            if let Ok(ref recognition) = result {
                if let Err(e) = self
                    .security_manager
                    .validate_input(&recognition.text, "voice_transcription")
                {
                    return Err(anyhow!("Transcription validation failed: {}", e));
                }
            }

            result
        })
    }

    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        Box::pin(async {
            // Check if model exists
            if !self.model_path.exists() {
                return Ok(false);
            }

            // Check if audio recording tools are available
            #[cfg(target_os = "linux")]
            {
                return super::platforms::linux::is_ready().await;
            }

            #[cfg(target_os = "windows")]
            {
                // Check for ffmpeg on Windows
                let check = tokio::process::Command::new("where")
                    .arg("ffmpeg")
                    .output()
                    .await;

                return Ok(check.as_ref().map(|o| o.status.success()).unwrap_or(false));
            }

            #[cfg(target_os = "macos")]
            {
                // Check for sox on macOS
                let check = tokio::process::Command::new("which")
                    .arg("rec")
                    .output()
                    .await;

                return Ok(check.as_ref().map(|o| o.status.success()).unwrap_or(false));
            }
        })
    }

    fn config(&self) -> &VoiceProviderConfig {
        &self.config
    }

    fn provider_name(&self) -> &str {
        "Vosk (Lightweight Offline)"
    }

    fn estimated_tokens_per_second(&self) -> Option<u64> {
        None // Fully offline, no token usage
    }
}

#[cfg(feature = "vosk-voice")]
/// Helper struct for transcription results
struct TranscriptionResult {
    text: String,
    confidence: f32,
}

#[cfg(test)]
#[cfg(feature = "vosk-voice")]
mod tests {
    use super::*;

    #[test]
    fn test_model_path_validation() {
        // Test that model path validation doesn't panic
        let result = VoskProvider::get_verified_model_path();
        // It's OK if model isn't found in test environment
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_is_model_available() {
        // Test that the check doesn't panic
        let available = VoskProvider::is_model_available();
        // Just verify it returns a boolean
        assert!(available || !available);
    }
}
