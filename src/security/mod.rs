// Security Module - Zero-Trust Architecture for Terminal Jarvis
// Implements defense-in-depth against supply chain attacks

pub mod core;
pub mod supply_chain;
pub mod crypto;

pub use core::{SecurityValidator, SecurityConfig, SecurityError};
pub use supply_chain::{SecureModelLoader, ModelAllowlist};
pub use crypto::{IntegrityVerifier};



/// Main security manager that coordinates all security components
pub struct SecurityManager {
    validator: SecurityValidator,
    model_loader: SecureModelLoader,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            validator: SecurityValidator::new(),
            model_loader: SecureModelLoader::new(),
        }
    }

    /// Validate any external input before processing
    pub fn validate_input(&self, input: &str, context: &str) -> Result<bool, SecurityError> {
        // Only log security details when DEBUG_SECURITY env var is set
        if std::env::var("DEBUG_SECURITY").is_ok() {
            eprintln!("[SECURITY] Validating input: {} for context: {}", input, context);
        }
        
        let is_valid = self.validator.validate_input(input, context)?;
        
        if !is_valid {
            eprintln!("[SECURITY BLOCKED] Input validation failed: {} for context: {}", input, context);
        } else if std::env::var("DEBUG_SECURITY").is_ok() {
            eprintln!("[SECURITY] Input validation passed");
        }
        
        Ok(is_valid)
    }

    /// Securely load a model with full verification
    pub async fn secure_load_model(&self, model_name: &str) -> Result<std::path::PathBuf, SecurityError> {
        eprintln!("[SECURITY] Model access attempt: {}", model_name);
        
        let result = self.model_loader.load_model(model_name).await;
        
        match &result {
            Ok(_) => eprintln!("[SECURITY] Model access successful: {}", model_name),
            Err(e) => eprintln!("[SECURITY BLOCKED] Model access denied: {} - {}", model_name, e),
        }
        
        result
    }

    /// Check if a command execution is allowed
    pub fn validate_command_execution(&self, command: &str, args: &[String]) -> Result<bool, SecurityError> {
        eprintln!("[SECURITY] Command execution attempt: {} {:?}", command, args);
        
        let is_allowed = self.validator.validate_command(command, args)?;
        
        if !is_allowed {
            eprintln!("[SECURITY BLOCKED] Command execution denied: {} {:?}", command, args);
        } else {
            eprintln!("[SECURITY] Command execution allowed");
        }
        
        Ok(is_allowed)
    }

    /// Get security status for monitoring
    pub fn get_security_status(&self) -> serde_json::Value {
        serde_json::json!({
            "security_module": "active",
            "validator": "enabled",
            "model_loader": "enabled",
            "auto_download": "disabled"
        })
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}
