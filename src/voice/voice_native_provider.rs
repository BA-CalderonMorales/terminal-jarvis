// Native Platform Voice Provider Implementation
// Uses platform-native speech recognition (no external dependencies)
//
// This provider shells out to native OS speech recognition tools:
// - Windows: Windows Speech Recognition via PowerShell
// - macOS: Built-in dictation
// - Linux: Fallback to simple command matching (no speech-to-text)

use super::voice_provider::{
    VoiceInputProvider, VoiceMetadata, VoiceProviderConfig, VoiceRecognitionResult,
};
use anyhow::{anyhow, Result};
use std::future::Future;
use std::pin::Pin;

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
        // Windows and macOS have built-in speech recognition
        cfg!(target_os = "windows") || cfg!(target_os = "macos")
    }

    /// Listen for speech directly using Windows Speech Recognition (no audio file needed)
    #[cfg(target_os = "windows")]
    async fn listen_windows_direct(&self) -> Result<String> {
        let duration_secs = self.config.max_duration.as_secs();
        
        println!("\n[LISTENING] Speak now... ({}s max)", duration_secs);
        
        // Use Windows Speech Recognition in real-time (no audio file needed)
        let ps_script = format!(
            r#"
Add-Type -AssemblyName System.Speech
$recognizer = New-Object System.Speech.Recognition.SpeechRecognitionEngine
$recognizer.SetInputToDefaultAudioDevice()
$recognizer.LoadGrammar((New-Object System.Speech.Recognition.DictationGrammar))

$timeout = {}
$result = $recognizer.Recognize([TimeSpan]::FromSeconds($timeout))

if ($result) {{
    $result.Text
}} else {{
    ""
}}
"#,
            duration_secs
        );

        let output = tokio::process::Command::new("powershell")
            .args(&["-NoProfile", "-Command", &ps_script])
            .output()
            .await
            .map_err(|e| anyhow!("Failed to run Windows Speech Recognition: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!(
                "Windows Speech Recognition failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let transcription = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if transcription.is_empty() {
            return Err(anyhow!("No speech detected"));
        }

        Ok(transcription)
    }
}

impl VoiceInputProvider for NativeVoiceProvider {
    fn listen(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<VoiceRecognitionResult>> + Send + '_>> {
        Box::pin(async move {
            #[cfg(target_os = "windows")]
            {
                // Windows: Use direct speech recognition (no audio file)
                let transcription = self.listen_windows_direct().await?;
                
                println!("[TRANSCRIBED] \"{}\"", transcription);
                
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

            #[cfg(target_os = "macos")]
            {
                // macOS doesn't have simple built-in CLI speech recognition
                return Err(anyhow!(
                    "macOS native speech recognition requires additional setup.\n\
                     Options:\n\
                     1. Use cloud API: Set OPENAI_API_KEY environment variable\n\
                     2. Build with local-voice feature: cargo install terminal-jarvis --features local-voice"
                ));
            }

            #[cfg(target_os = "linux")]
            {
                // Linux doesn't have built-in speech recognition
                return Err(anyhow!(
                    "Linux does not have built-in speech recognition.\n\
                     Options:\n\
                     1. Use cloud API: Set OPENAI_API_KEY environment variable\n\
                     2. Build with local-voice feature: cargo install terminal-jarvis --features local-voice\n\
                     3. Install and use Vosk or other offline engines"
                ));
            }
        })
    }

    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        Box::pin(async move {
            // Check if platform is supported
            if !Self::is_supported() {
                return Ok(false);
            }

            // Check if required tools are available
            #[cfg(target_os = "windows")]
            {
                // Check for PowerShell (always available on Windows)
                let ps_check = tokio::process::Command::new("powershell")
                    .args(&["-NoProfile", "-Command", "exit 0"])
                    .status()
                    .await;
                return Ok(ps_check.is_ok());
            }

            #[cfg(target_os = "macos")]
            {
                // Check for rec command
                let rec_check = tokio::process::Command::new("which")
                    .arg("rec")
                    .status()
                    .await;
                return Ok(rec_check.is_ok() && rec_check.unwrap().success());
            }

            #[cfg(target_os = "linux")]
            {
                return Ok(false); // Linux not supported without external tools
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

        #[cfg(target_os = "macos")]
        {
            "macOS Native Speech Recognition"
        }

        #[cfg(target_os = "linux")]
        {
            "Linux (Native Not Supported)"
        }
    }
}
