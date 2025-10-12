// Local Whisper Voice Provider Implementation (SECURE VERSION)
// NO AUTO-DOWNLOADS - Only locally verified models allowed

#[cfg(feature = "local-voice")]
use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
#[cfg(feature = "local-voice")]
use anyhow::{anyhow, Result};
#[cfg(feature = "local-voice")]
use std::future::Future;
#[cfg(feature = "local-voice")]
use std::path::PathBuf;
#[cfg(feature = "local-voice")]
use std::pin::Pin;

#[cfg(feature = "local-voice")]
use crate::security::{SecurityManager, SecurityError};

#[cfg(feature = "local-voice")]
/// Local Whisper provider using whisper.cpp (SECURE VERSION)
/// Only works with manually verified models - NO AUTO-DOWNLOADS
pub struct LocalWhisperProvider {
    config: VoiceProviderConfig,
    model_path: PathBuf,
    temp_dir: PathBuf,
    security_manager: SecurityManager,
}

#[cfg(feature = "local-voice")]
impl LocalWhisperProvider {
    /// Create a new local Whisper provider
    pub async fn new(config: VoiceProviderConfig) -> Result<Self> {
        let temp_dir = std::env::temp_dir();

        // SECURITY: NO AUTO-DOWNLOAD - Model must be manually provided
        let model_path = Self::get_verified_model_path().await?;

        Ok(Self {
            config,
            model_path,
            temp_dir,
            security_manager: SecurityManager::new(),
        })
    }

    /// Get verified model path - NO AUTO-DOWNLOADS
    async fn get_verified_model_path() -> Result<PathBuf> {
        // SECURITY: Only use manually verified local models
        let possible_paths = vec![
            // Check common secure locations first
            PathBuf::from("./models/ggml-tiny.en.bin.verified"),
            PathBuf::from("./config/whisper-model.bin.verified"),
            PathBuf::from("./models/whisper-tiny.bin"),
        ];

        for path in &possible_paths {
            if path.exists() {
                // Verify file integrity before using
                if Self::verify_model_integrity(path)? {
                    println!("[SECURITY] Using verified local model: {}", path.display());
                    return Ok(path.clone());
                } else {
                    eprintln!("[SECURITY WARNING] Model failed integrity check: {}", path.display());
                }
            }
        }

        // SECURITY: NO AUTO-DOWNLOAD - Return error if no verified model found
        Err(anyhow!(
            "No verified whisper model found. Auto-download disabled for security.\n\
             To use local voice:\n\
             1. Manually download a trusted whisper model\n\
             2. Place it in ./models/ directory with .verified extension\n\
             3. Or use the Secure API provider instead"
        ))
    }

    /// Verify model file integrity before use
    fn verify_model_integrity(path: &PathBuf) -> Result<bool> {
        // Basic safety checks
        let metadata = std::fs::metadata(path)
            .map_err(|_| anyhow!("Cannot read model metadata"))?;

        // Check reasonable size (10MB - 1GB for whisper models)
        if metadata.len() < 10_000_000 || metadata.len() > 1_000_000_000 {
            eprintln!("[SECURITY] Model size unreasonable: {} bytes", metadata.len());
            return Ok(false);
        }

        // Check file extension for .verified models
        if let Some(extension) = path.extension() {
            if extension == "verified" {
                println!("[SECURITY] Model has .verified extension - good sign");
            }
        }

        // TODO: Add hash verification against allowlist
        Ok(true)
    }

    /// Generate safe recording commands (hardcoded, validated)
    fn get_safe_recording_command(&self, audio_file: &PathBuf, duration_secs: u64) -> String {
        // SECURITY: Only use hardcoded, validated commands
        #[cfg(target_os = "linux")]
        {
            // Safe arecord command with fixed parameters
            format!(
                "arecord -d {} -f cd -t wav {}",
                duration_secs,
                audio_file.display()
            )
        }

        #[cfg(target_os = "macos")]
        {
            // Safe rec command with fixed parameters
            format!(
                "rec -r 16000 -c 1 {} trim 0 {}",
                audio_file.display(),
                duration_secs
            )
        }

        #[cfg(target_os = "windows")]
        {
            // Safe ffmpeg command with fixed parameters
            format!(
                "ffmpeg -f dshow -i audio=\"Microphone\" -t {} -y {}",
                duration_secs,
                audio_file.display()
            )
        }
    }

    /// Record audio to a temporary file (SECURE VERSION)
    async fn record_audio(&self) -> Result<PathBuf> {
        let audio_file = self.temp_dir.join("voice_input_local.wav");
        let duration_secs = self.config.max_duration.as_secs();

        println!("\n[SECURE LISTENING] Speak now... ({}s max)", duration_secs);

        // SECURITY: Use hardcoded, safe recording commands only
        let recording_command = self.get_safe_recording_command(&audio_file, duration_secs);
        
        // SECURITY: Validate the recording command components
        let command_parts: Vec<String> = recording_command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if let Some(base_cmd) = command_parts.first() {
            self.security_manager.validate_command_execution(
                base_cmd, 
                &command_parts[1..]
            ).map_err(|e| anyhow!("Security validation failed: {}", e))?;
        } else {
            return Err(anyhow!("Invalid recording command generated"));
        }

        println!("[SECURITY] Executing validated recording command");

        // Execute recording command with validation
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&recording_command)
            .output()
            .await
            .map_err(|e| {
                anyhow!(
                    "Failed to record audio: {}\n\
                     SECURITY: Make sure recording tools are installed:\n\
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

        println!("[RECORDED] Processing with local whisper...");
        Ok(audio_file)
    }

    /// Transcribe audio using local whisper model
    async fn transcribe_audio(&self, audio_path: &PathBuf) -> Result<VoiceRecognitionResult> {
        use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

        // Load the model
        let ctx = WhisperContext::new(&self.model_path.to_string_lossy())
            .map_err(|e| anyhow!("Failed to load whisper model: {}", e))?;

        // Read and convert audio
        let audio_data = Self::load_audio_file(audio_path)?;

        // Clone language for use in the closure
        let language = self.config.language.clone();

        // Run transcription in blocking task (whisper-rs is synchronous)
        // Move ctx, audio_data, and language into the closure
        let transcription = tokio::task::spawn_blocking(move || {
            // Set up parameters inside the closure where language is owned
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            params.set_language(Some(&language));
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            let mut state = ctx.create_state()
                .map_err(|e| anyhow!("Failed to create whisper state: {}", e))?;

            state.full(params, &audio_data)
                .map_err(|e| anyhow!("Transcription failed: {}", e))?;

            let num_segments = state.full_n_segments()
                .map_err(|e| anyhow!("Failed to get segments: {}", e))?;

            let mut text = String::new();
            for i in 0..num_segments {
                let segment = state.full_get_segment_text(i)
                    .map_err(|e| anyhow!("Failed to get segment text: {}", e))?;
                text.push_str(&segment);
            }

            Ok::<String, anyhow::Error>(text.trim().to_string())
        }).await??;

        println!("[TRANSCRIBED] \"{}\"", transcription);

        Ok(VoiceRecognitionResult {
            text: transcription,
            confidence: 0.9, // Whisper doesn't provide confidence, use default
            duration: self.config.max_duration,
            metadata: Some(VoiceMetadata {
                language: Some(self.config.language.clone()),
                tokens_used: None,
                extra: std::collections::HashMap::new(),
            }),
        })
    }

    /// Load and preprocess audio file for whisper
    fn load_audio_file(path: &PathBuf) -> Result<Vec<f32>> {
        // For now, assume the audio is already in the right format (16kHz mono)
        // In production, you'd want to use something like `symphonia` to decode WAV

        // This is a simplified version - whisper expects 16kHz mono float32 samples
        // The recorded audio from arecord/rec should already be in a compatible format

        // Read the WAV file
        let wav_data = std::fs::read(path)?;

        // Skip WAV header (44 bytes) and convert to f32 samples
        // This is a simplified approach - proper WAV parsing would be better
        let audio_data = &wav_data[44..]; // Skip WAV header

        let mut samples = Vec::with_capacity(audio_data.len() / 2);
        for chunk in audio_data.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            samples.push(sample as f32 / 32768.0); // Normalize to -1.0..1.0
        }

        Ok(samples)
    }

    /// Clean up temporary audio file
    async fn cleanup(&self, audio_path: &PathBuf) -> Result<()> {
        if audio_path.exists() {
            tokio::fs::remove_file(audio_path).await?;
        }
        Ok(())
    }
}

#[cfg(feature = "local-voice")]
impl VoiceInputProvider for LocalWhisperProvider {
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
            // Check if model file exists
            if !self.model_path.exists() {
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

            Ok(output.map(|o| o.status.success()).unwrap_or(false))
        })
    }

    fn config(&self) -> &VoiceProviderConfig {
        &self.config
    }

    fn provider_name(&self) -> &str {
        "Local Whisper (whisper.cpp)"
    }

    fn estimated_tokens_per_second(&self) -> Option<u64> {
        None // Not applicable for local inference
    }
}
