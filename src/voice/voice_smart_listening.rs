// Smart Voice Listening with Professional Feedback (SECURE VERSION)
// Provides wake word detection, animated indicators, and enhanced UX
// SECURITY: All inputs validated and logged

use super::voice_provider::{VoiceInputProvider, VoiceProviderConfig};
use super::voice_command::{VoiceCommandParser, VoiceCommand};
use crate::security::SecurityManager;
use anyhow::Result;
use std::io::{self, Write};

/// Types of voice feedback for different states
#[derive(Debug, Clone)]
pub enum VoiceFeedbackType {
    /// Initial listening state
    Listening,
    /// Processing voice input
    Processing,
    /// Success feedback
    Success(String),
    /// Error feedback
    Error(String),
    /// Warning feedback  
    Warning(String),
    /// Information feedback
    Info(String),
}

/// Professional voice feedback system with visual indicators
pub struct VoiceFeedback {
    theme: crate::theme::Theme,
}

impl VoiceFeedback {
    pub fn new() -> Self {
        let theme = crate::theme::theme_global_config::current_theme();
        Self { theme }
    }

    /// Display feedback with professional styling
    pub fn show(&self, feedback_type: VoiceFeedbackType) {
        match feedback_type {
            VoiceFeedbackType::Listening => {
                self.show_listening_indicator();
            }
            VoiceFeedbackType::Processing => {
                self.show_processing_indicator();
            }
            VoiceFeedbackType::Success(message) => {
                self.show_success(&message);
            }
            VoiceFeedbackType::Error(message) => {
                self.show_error(&message);
            }
            VoiceFeedbackType::Warning(message) => {
                self.show_warning(&message);
            }
            VoiceFeedbackType::Info(message) => {
                self.show_info(&message);
            }
        }
    }

    /// Animated listening indicator
    fn show_listening_indicator(&self) {
        println!();

        // Simple static indicator for now (animation would need async support)
        print!(
            "{}[● LISTENING] Say a command... ",
            self.theme.primary("")
        );
        io::stdout().flush().unwrap();
    }

    /// Processing indicator
    fn show_processing_indicator(&self) {
        println!(
            "\n{}[PROCESSING] {}",
            self.theme.primary("[●]"),
            self.theme.secondary("Transcribing voice input...")
        );
    }

    /// Success message with checkmark-like indicator
    fn show_success(&self, _message: &str) {
        // Success is implicit - don't show verbose messages
        // Command will be handled by the main system
    }

    /// Error message with warning indicator
    fn show_error(&self, message: &str) {
        println!(
            "\n{} {}",
            self.theme.accent("Could not understand:"),
            self.theme.primary(message)
        );
    }

    /// Warning message with info indicator
    fn show_warning(&self, message: &str) {
        println!(
            "\n{}[WARNING] {}",
            self.theme.accent("[!]"),
            self.theme.secondary(message)
        );
    }

    /// Information message with info indicator
    fn show_info(&self, message: &str) {
        println!(
            "{}[INFO] {}",
            self.theme.accent("[i]"),
            self.theme.secondary(message)
        );
    }

    /// Show available voice commands help
    pub fn show_commands_help(&self) {
        println!(
            "\n{}[VOICE COMMANDS]\n",
            self.theme.primary("=").repeat(50)
        );
        
        let commands = vec![
            ("Navigation", "open ai tools", "Navigate to AI tools menu"),
            ("Navigation", "open authentication", "Navigate to authentication"),
            ("Navigation", "open settings", "Navigate to settings"),
            ("Navigation", "open evals", "Navigate to benchmarks"),
            ("Navigation", "open links", "Navigate to documentation"),
            ("Navigation", "back / main menu", "Navigate back or to main menu"),
            
            ("Tool Management", "install [tool]", "Install a specific tool"),
            ("Tool Management", "update [tool]", "Update a specific tool"),
            ("Tool Management", "status [tool]", "Check tool status"),
            ("Tool Management", "remove [tool]", "Uninstall a tool"),
            ("Tool Management", "list tools", "Show all available tools"),
            ("Tool Management", "list installed tools", "Show installed tools only"),
            ("Tool Management", "update all", "Update all installed tools"),
            
            ("General", "help / commands", "Show this help"),
            ("General", "cancel", "Cancel current operation"),
            ("General", "exit", "Exit the application"),
        ];

        for (_category, example, description) in commands {
            println!(
                "  • {}{}: {} - {}",
                self.theme.accent("["),
                self.theme.primary(example),
                self.theme.accent("]"),
                self.theme.secondary(description)
            );
        }
    }
}

/// Enhanced voice listener with smart features (SECURE VERSION)
pub struct SmartVoiceListener {
    provider: Box<dyn VoiceInputProvider>,
    parser: VoiceCommandParser,
    feedback: VoiceFeedback,
    wake_word: String,
    security_manager: SecurityManager,
}

impl SmartVoiceListener {
    /// Create new smart voice listener
    pub fn new(provider: Box<dyn VoiceInputProvider>) -> Self {
        Self {
            provider,
            parser: VoiceCommandParser::new(),
            feedback: VoiceFeedback::new(),
            wake_word: "jarvis".to_string(),
            security_manager: SecurityManager::new(),
        }
    }

    /// Set custom wake word
    pub fn with_wake_word(mut self, wake_word: String) -> Self {
        self.wake_word = wake_word.to_lowercase();
        self
    }

    /// Listen for single command with enhanced feedback (SECURE VERSION)
    pub async fn listen_for_command(&self) -> Result<Option<VoiceCommand>> {
        self.feedback.show(VoiceFeedbackType::Listening);

        // Record and transcribe
        self.feedback.show(VoiceFeedbackType::Processing);
        let result = self.provider.listen().await;

        match result {
            Ok(recognition) => {
                // Validate transcribed text before processing
                let transcribed_text = &recognition.text;
                if let Err(_e) = self.security_manager.validate_input(transcribed_text, "voice_command") {
                    self.feedback.show(VoiceFeedbackType::Error(
                        "Could not validate voice input".to_string()
                    ));
                    return Ok(None);
                }

                println!("Heard: \"{}\"", transcribed_text);

                // Parse command
                match self.parser.parse(transcribed_text, recognition.confidence) {
                    Ok(command) => {
                        self.feedback.show(VoiceFeedbackType::Success(
                            format!("Command recognized: {:?}", command)
                        ));
                        Ok(Some(command))
                    }
                    Err(_e) => {
                        self.feedback.show(VoiceFeedbackType::Error(
                            "Could not understand command".to_string()
                        ));
                        Ok(None)
                    }
                }
            }
            Err(e) => {
                self.feedback.show(VoiceFeedbackType::Error(
                    format!("Voice recognition failed: {}", e)
                ));
                Ok(None)
            }
        }
    }

    /// Listen with wake word activation (placeholder for future implementation)
    pub async fn listen_with_wake_word(&self) -> Result<Option<VoiceCommand>> {
        // For now, just prompt for the wake word
        println!(
            "{}[WAKE WORD] Say \"{}\" to start voice command...",
            self.feedback.theme.primary("[●]"),
            self.feedback.theme.primary(&self.wake_word)
        );

        // This would be enhanced with continuous listening in Phase 2
        // For now, just proceed to regular command listening
        self.listen_for_command().await
    }

    /// Check if voice system is ready
    pub async fn check_ready(&self) -> Result<bool> {
        self.provider.is_ready().await
    }

    /// Show voice system status
    pub async fn show_status(&self) -> Result<()> {
        println!(
            "\n{}[VOICE SYSTEM STATUS]\n",
            self.feedback.theme.primary("=").repeat(40)
        );

        // Provider status
        println!(
            "  {}: {}",
            self.feedback.theme.primary("Provider:"),
            self.feedback.theme.secondary(self.provider.provider_name())
        );

        // Configuration
        let config = self.provider.config();
        println!(
            "  {}: {}",
            self.feedback.theme.primary("Language:"),
            self.feedback.theme.secondary(&config.language)
        );
        println!(
            "  {}: {}",
            self.feedback.theme.primary("Min Confidence:"),
            self.feedback.theme.secondary(&format!("{:.1}", config.min_confidence))
        );
        println!(
            "  {}: {}",
            self.feedback.theme.primary("Max Duration:"),
            self.feedback.theme.secondary(&format!("{:.0}s", config.max_duration.as_secs()))
        );

        // Ready status
        let ready = self.provider.is_ready().await?;
        println!(
            "  {}: {}",
            self.feedback.theme.primary("Status:"),
            if ready {
                self.feedback.theme.accent("READY")
            } else {
                self.feedback.theme.accent("NOT READY")
            }
        );

        println!();
        Ok(())
    }
}

/// Factory methods for creating voice listeners
pub struct VoiceListenerFactory;

impl VoiceListenerFactory {
    /// Create smart voice listener with Whisper API provider
    pub async fn create_whisper_listener(config: VoiceProviderConfig) -> Result<SmartVoiceListener> {
        let provider = super::voice_whisper_provider::WhisperProvider::new(config)?;
        let listener = SmartVoiceListener::new(Box::new(provider));
        Ok(listener)
    }

    /// Create smart voice listener with local Whisper provider (no API key required)
    #[cfg(feature = "local-voice")]
    pub async fn create_local_whisper_listener(config: VoiceProviderConfig) -> Result<SmartVoiceListener> {
        let provider = super::voice_local_whisper_provider::LocalWhisperProvider::new(config).await?;
        let listener = SmartVoiceListener::new(Box::new(provider));
        Ok(listener)
    }

    /// Create smart voice listener with native platform provider (Windows/macOS built-in)
    pub async fn create_native_listener(config: VoiceProviderConfig) -> Result<SmartVoiceListener> {
        let provider = super::voice_native_provider::NativeVoiceProvider::new(config)?;
        let listener = SmartVoiceListener::new(Box::new(provider));
        Ok(listener)
    }

    /// Create listener with default configuration
    /// Priority: Native (Windows/macOS) → Local Whisper → Cloud API
    pub async fn create_default_listener() -> Result<SmartVoiceListener> {
        let config = VoiceProviderConfig::default();

        // Try native platform voice first (Windows/macOS built-in)
        if super::voice_native_provider::NativeVoiceProvider::is_supported() {
            println!("[NATIVE VOICE] Using platform built-in speech recognition (no API keys)...");
            match Self::create_native_listener(config.clone()).await {
                Ok(listener) => {
                    // Check if it's actually ready
                    if listener.check_ready().await.unwrap_or(false) {
                        println!("[SUCCESS] Native voice recognition ready.");
                        return Ok(listener);
                    } else {
                        println!("[INFO] Native voice not configured properly.");
                    }
                }
                Err(e) => {
                    println!("[INFO] Native voice not available: {}", e);
                }
            }
        }

        // Try local whisper next if feature is enabled
        #[cfg(feature = "local-voice")]
        {
            println!("[PRIVACY MODE] Attempting local SLM voice recognition (no API keys, no cloud)...");
            match Self::create_local_whisper_listener(config.clone()).await {
                Ok(listener) => {
                    println!("[SUCCESS] Local voice recognition ready. Your speech stays on-device.");
                    return Ok(listener);
                }
                Err(e) => {
                    println!("[INFO] Local voice not available: {}", e);
                    println!("[FALLBACK] Attempting cloud-based voice recognition...");
                }
            }
        }

        // Fall back to OpenAI Whisper API
        Self::create_whisper_listener(config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_creation() {
        let feedback = VoiceFeedback::new();
        // Just test that it doesn't panic
        feedback.show(VoiceFeedbackType::Info("Test".to_string()));
    }

    #[test]
    fn test_wake_word_normalization() {
        let config = VoiceProviderConfig::default();
        let provider = crate::voice::WhisperProvider::new(config);
        if provider.is_ok() {
            let listener = SmartVoiceListener::new(Box::new(provider.unwrap()));
            assert_eq!(listener.wake_word, "jarvis");
        }
    }
}
