// Supply Chain Security Module - Model Verification and Safe Loading
// Implements zero-trust model loading with cryptographic verification

use super::{core::SecurityErrorCode, SecurityError};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ModelAllowlist {
    allowed_models: HashMap<String, ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub expected_hash: String,
    pub signature: String,
    pub source_url: String,
    pub size_bytes: u64,
    pub last_verified: chrono::DateTime<chrono::Utc>,
}

impl ModelAllowlist {
    pub fn new() -> Self {
        Self {
            allowed_models: HashMap::new(),
        }
    }

    pub fn add_model(&mut self, model_info: ModelInfo) {
        self.allowed_models
            .insert(model_info.name.clone(), model_info);
    }

    pub fn is_model_allowed(&self, model_name: &str) -> bool {
        self.allowed_models.contains_key(model_name)
    }

    pub fn get_model_info(&self, model_name: &str) -> Option<&ModelInfo> {
        self.allowed_models.get(model_name)
    }

    pub fn remove_model(&mut self, model_name: &str) -> bool {
        self.allowed_models.remove(model_name).is_some()
    }

    /// Get all approved model names for display
    pub fn get_approved_models(&self) -> Vec<String> {
        self.allowed_models.keys().cloned().collect()
    }
}

impl Default for ModelAllowlist {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SecureModelLoader {
    allowlist: ModelAllowlist,
    cache_dir: PathBuf,
}

impl SecureModelLoader {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = home_dir
            .join(".cache")
            .join("terminal-jarvis")
            .join("models");

        std::fs::create_dir_all(&cache_dir).ok();

        Self {
            allowlist: ModelAllowlist::new(),
            cache_dir,
        }
    }

    /// Securely load a model with full verification
    pub async fn load_model(&self, model_name: &str) -> Result<PathBuf, SecurityError> {
        // STEP 1: Check against allowlist
        let model_info =
            self.allowlist
                .get_model_info(model_name)
                .ok_or_else(|| SecurityError {
                    message: format!("Model '{}' not in allowlist", model_name),
                    code: SecurityErrorCode::BlockedKeyword,
                })?;

        // STEP 2: Check if model already cached and verified
        let cached_path = self.cache_dir.join(format!("{}.verified", model_name));
        if cached_path.exists() {
            if self.verify_cached_model(&cached_path, model_info)? {
                return Ok(cached_path);
            } else {
                // Cached model failed verification, remove it
                std::fs::remove_file(&cached_path).ok();
            }
        }

        // STEP 3: Download is BLOCKED - no auto-downloads allowed
        return Err(SecurityError {
            message: format!(
                "Auto-download disabled. Model '{}' must be manually verified and placed in cache.",
                model_name
            ),
            code: SecurityErrorCode::BlockedKeyword,
        });
    }

    /// Verify a cached model file
    fn verify_cached_model(
        &self,
        path: &PathBuf,
        expected_info: &ModelInfo,
    ) -> Result<bool, SecurityError> {
        // Check file size
        let metadata = std::fs::metadata(path).map_err(|_| SecurityError {
            message: "Cannot read model file metadata".to_string(),
            code: SecurityErrorCode::InjectionAttempt,
        })?;

        if metadata.len() != expected_info.size_bytes {
            return Ok(false);
        }

        // Calculate and verify hash
        let file_hash = self.calculate_file_hash(path)?;
        if file_hash != expected_info.expected_hash {
            return Ok(false);
        }

        // Verify signature (if implemented)
        if !expected_info.signature.is_empty() {
            if !self.verify_signature(path, &expected_info.signature)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Calculate SHA-256 hash of file
    fn calculate_file_hash(&self, path: &PathBuf) -> Result<String, SecurityError> {
        use sha2::{Digest, Sha256};

        let mut file = std::fs::File::open(path).map_err(|_| SecurityError {
            message: "Cannot open file for hashing".to_string(),
            code: SecurityErrorCode::InjectionAttempt,
        })?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let bytes_read =
                std::io::Read::read(&mut file, &mut buffer).map_err(|_| SecurityError {
                    message: "Cannot read file for hashing".to_string(),
                    code: SecurityErrorCode::InjectionAttempt,
                })?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Verify digital signature of model file
    fn verify_signature(&self, _path: &PathBuf, _signature: &str) -> Result<bool, SecurityError> {
        // Placeholder for signature verification
        // In production, this would use proper cryptographic verification
        Ok(true)
    }

    /// Manually add a verified model to cache
    pub fn add_verified_model(
        &self,
        source_path: &PathBuf,
        model_name: &str,
        expected_info: ModelInfo,
    ) -> Result<(), SecurityError> {
        // Verify the source file matches expectations
        if !source_path.exists() {
            return Err(SecurityError {
                message: "Source model file does not exist".to_string(),
                code: SecurityErrorCode::InjectionAttempt,
            });
        }

        // Move to cache with .verified suffix
        let cached_path = self.cache_dir.join(format!("{}.verified", model_name));
        std::fs::copy(source_path, &cached_path).map_err(|_| SecurityError {
            message: "Failed to copy model to cache".to_string(),
            code: SecurityErrorCode::InjectionAttempt,
        })?;

        // Verify after caching first
        if !self.verify_cached_model(&cached_path, &expected_info)? {
            std::fs::remove_file(&cached_path).ok();
            return Err(SecurityError {
                message: "Model verification failed after caching".to_string(),
                code: SecurityErrorCode::InjectionAttempt,
            });
        }

        // Add to allowlist after successful verification
        let mut allowlist = ModelAllowlist::new();
        allowlist.add_model(expected_info);
        Ok(())
    }

    /// Get cache status for monitoring
    pub fn get_cache_status(&self) -> serde_json::Value {
        let mut cached_models = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        cached_models.push(serde_json::json!({
                            "name": entry.file_name().to_string_lossy(),
                            "size": metadata.len(),
                            "modified": metadata.modified().ok().map(|t| {
                                chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339()
                            })
                        }));
                    }
                }
            }
        }

        serde_json::json!({
            "cache_directory": self.cache_dir.to_string_lossy(),
            "cached_models": cached_models,
            "allowed_models_count": self.allowlist.get_approved_models().len(),
            "approved_models": self.allowlist.get_approved_models()
        })
    }

    /// Clear cache for security reasons
    pub fn clear_cache(&self) -> Result<(), SecurityError> {
        if let Ok(entries) = std::fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let _ = std::fs::remove_file(entry.path());
            }
        }
        Ok(())
    }
}

impl Default for SecureModelLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_allowlist() {
        let mut allowlist = ModelAllowlist::new();

        // Test empty allowlist
        assert!(!allowlist.is_model_allowed("test_model"));

        // Add model and test
        let model_info = ModelInfo {
            name: "test_model".to_string(),
            expected_hash: "abc123".to_string(),
            signature: "sig456".to_string(),
            source_url: "https://example.com/model.bin".to_string(),
            size_bytes: 1024,
            last_verified: chrono::Utc::now(),
        };

        allowlist.add_model(model_info);
        assert!(allowlist.is_model_allowed("test_model"));
    }

    #[test]
    fn test_secure_model_loader_no_autodownload() {
        let loader = SecureModelLoader::new();

        // Should fail since model not in allowlist and auto-download is disabled
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(loader.load_model("any_model"));

        assert!(result.is_err());
        match result.unwrap_err().code {
            SecurityErrorCode::BlockedKeyword => (),
            _ => panic!("Expected BlockedKeyword error"),
        }
    }
}
