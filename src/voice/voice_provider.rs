// Voice Input Provider Trait
// Defines the interface for voice-to-text service implementations

use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// Result from voice recognition containing transcribed text and confidence score
#[derive(Debug, Clone)]
pub struct VoiceRecognitionResult {
    /// The transcribed text from voice input
    pub text: String,

    /// Confidence score (0.0 to 1.0) indicating recognition certainty
    pub confidence: f32,

    /// Duration of the audio segment processed
    pub duration: Duration,

    /// Optional metadata from the recognition service
    pub metadata: Option<VoiceMetadata>,
}

/// Additional metadata from voice recognition
#[derive(Debug, Clone)]
pub struct VoiceMetadata {
    /// Language detected or used for recognition
    pub language: Option<String>,

    /// Token count or credits used (for rate limiting/billing)
    pub tokens_used: Option<u64>,

    /// Service-specific data
    pub extra: std::collections::HashMap<String, String>,
}

/// Configuration for voice input providers
#[derive(Debug, Clone)]
pub struct VoiceProviderConfig {
    /// Minimum confidence threshold to accept recognition (0.0 to 1.0)
    pub min_confidence: f32,

    /// Maximum recording duration allowed
    pub max_duration: Duration,

    /// Language code for recognition (e.g., "en-US", "es-ES")
    pub language: String,

    /// Whether to enable continuous listening mode
    pub continuous_mode: bool,

    /// Custom wake word for activation (optional)
    pub wake_word: Option<String>,
}

impl Default for VoiceProviderConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.7,
            max_duration: Duration::from_secs(30),
            language: "en-US".to_string(),
            continuous_mode: false,
            wake_word: None,
        }
    }
}

/// Trait defining the interface for voice-to-text providers
///
/// Implementations can use different services (Whisper, Google Speech, etc.)
/// while maintaining a consistent interface for the application.
///
/// # Example Implementation Pattern
///
/// ```rust,ignore
/// struct WhisperProvider {
///     config: VoiceProviderConfig,
///     client: WhisperClient,
/// }
///
/// impl VoiceInputProvider for WhisperProvider {
///     async fn listen(&self) -> Result<VoiceRecognitionResult> {
///         // Record audio and send to Whisper API
///         // Return transcribed text with confidence
///     }
/// }
/// ```
pub trait VoiceInputProvider: Send + Sync {
    /// Start listening and return transcribed text when complete
    ///
    /// This method should:
    /// 1. Start audio recording
    /// 2. Stop when silence is detected or max_duration is reached
    /// 3. Send audio to recognition service
    /// 4. Return transcribed text with confidence score
    fn listen(&self) -> Pin<Box<dyn Future<Output = Result<VoiceRecognitionResult>> + Send + '_>>;

    /// Check if the provider is properly configured and ready
    ///
    /// Should verify:
    /// - API keys/credentials are present
    /// - Network connectivity (if needed)
    /// - Audio device availability
    fn is_ready(&self) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>>;

    /// Get the current configuration
    fn config(&self) -> &VoiceProviderConfig;

    /// Test the provider with a brief recording
    /// Used for setup and validation
    fn test_connection(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async {
            if self.is_ready().await? {
                Ok(())
            } else {
                Err(anyhow::anyhow!("Voice provider not ready"))
            }
        })
    }

    /// Get provider name for display/logging
    fn provider_name(&self) -> &str;

    /// Get token/credit usage estimate for intelligent provider selection
    /// Returns None if provider doesn't track usage
    fn estimated_tokens_per_second(&self) -> Option<u64> {
        None
    }
}

/// Factory for creating voice input providers based on configuration
///
/// This allows the application to intelligently select providers based on:
/// - Available tokens/credits
/// - User preferences
/// - Network availability
pub struct VoiceProviderFactory;

impl VoiceProviderFactory {
    /// Select the best available provider based on current conditions
    ///
    /// Future implementation will consider:
    /// - Token limits for each service
    /// - User's request complexity
    /// - Network conditions
    /// - Cost optimization
    #[allow(dead_code)]
    pub async fn select_optimal_provider(
        _available_providers: Vec<Box<dyn VoiceInputProvider>>,
        _request_duration_estimate: Duration,
    ) -> Result<Box<dyn VoiceInputProvider>> {
        // Placeholder for intelligent provider selection
        // Will be implemented when concrete providers are added
        Err(anyhow::anyhow!(
            "Provider selection not yet implemented - waiting for concrete provider implementations"
        ))
    }
}
