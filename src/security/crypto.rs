// Cryptographic Security Module - Integrity Verification and Digital Signatures
// Provides cryptographic verification for all external assets

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityErrorCode {
    UnallowedModel,
    IntegrityCheckFailed,
    SignatureVerificationFailed,
    DownloadBlocked,
    CryptographicError,
    HashMismatch,
    InvalidCertificate,
}

#[derive(Debug, Clone)]
pub struct IntegrityVerifier {
    algorithm: HashAlgorithm,
}

#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    SHA256,
    SHA512,
}

impl IntegrityVerifier {
    pub fn new() -> Self {
        Self {
            algorithm: HashAlgorithm::SHA256,
        }
    }

    pub fn with_algorithm(algorithm: HashAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Calculate hash of file data
    pub fn calculate_hash(&self, data: &[u8]) -> Result<String> {
        match self.algorithm {
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                Digest::update(&mut hasher, data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            HashAlgorithm::SHA512 => {
                let mut hasher = Sha512::new();
                Digest::update(&mut hasher, data);
                Ok(format!("{:x}", hasher.finalize()))
            }
        }
    }

    /// Calculate hash of file by path
    pub fn calculate_file_hash(&self, path: &std::path::Path) -> Result<String> {
        use std::io::Read;
        
        let hash = match self.algorithm {
            HashAlgorithm::SHA256 => {
                let mut hasher = Sha256::new();
                let mut file = std::fs::File::open(path)?;
                let mut buffer = [0; 8192];
                
                loop {
                    let bytes_read = file.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }
                    Digest::update(&mut hasher, &buffer[..bytes_read]);
                }
                
                format!("{:x}", hasher.finalize())
            }
            HashAlgorithm::SHA512 => {
                let mut hasher = Sha512::new();
                let mut file = std::fs::File::open(path)?;
                let mut buffer = [0; 8192];
                
                loop {
                    let bytes_read = file.read(&mut buffer)?;
                    if bytes_read == 0 {
                        break;
                    }
                    Digest::update(&mut hasher, &buffer[..bytes_read]);
                }
                
                format!("{:x}", hasher.finalize())
            }
        };

        Ok(hash)
    }

    /// Verify data against expected hash
    pub fn verify_hash(&self, data: &[u8], expected_hash: &str) -> bool {
        if let Ok(calculated_hash) = self.calculate_hash(data) {
            calculated_hash == expected_hash.to_lowercase()
        } else {
            false
        }
    }

    /// Verify file against expected hash
    pub fn verify_file_hash(&self, path: &std::path::Path, expected_hash: &str) -> bool {
        if let Ok(calculated_hash) = self.calculate_file_hash(path) {
            calculated_hash == expected_hash.to_lowercase()
        } else {
            false
        }
    }
}

impl Default for IntegrityVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Model signature information for verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSignature {
    pub model_name: String,
    pub hash: String,
    pub signature: String,
    pub public_key: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub signer: String,
}

impl ModelSignature {
    /// Create new signature (in production, this would use proper crypto)
    pub fn new(model_name: &str, hash: &str, public_key: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
            hash: hash.to_string(),
            signature: "placeholder_signature".to_string(), // TODO: Implement real signing
            public_key: public_key.to_string(),
            timestamp: chrono::Utc::now(),
            signer: "terminal-jarvis".to_string(),
        }
    }

    /// Verify signature (in production, this would use proper cryptographic verification)
    pub fn verify(&self, _data: &[u8]) -> bool {
        // Placeholder for real signature verification
        // In production, this would use Ed25519 or similar
        !self.signature.is_empty() && !self.public_key.is_empty()
    }

    /// Serialize signature for storage
    pub fn serialize(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Deserialize signature from storage
    pub fn deserialize(serialized: &str) -> Result<Self> {
        Ok(serde_json::from_str(serialized)?)
    }
}

/// Certificate management for trusted repositories
#[derive(Debug, Clone)]
pub struct CertificateStore {
    trusted_keys: Vec<String>,
}

impl CertificateStore {
    pub fn new() -> Self {
        Self {
            trusted_keys: vec![
                // In production, this would contain actual trusted public keys
                // For now, empty to force manual verification only
            ],
        }
    }

    pub fn add_trusted_key(&mut self, public_key: String) {
        self.trusted_keys.push(public_key);
    }

    pub fn is_key_trusted(&self, public_key: &str) -> bool {
        self.trusted_keys.contains(&public_key.to_string())
    }

    pub fn clear_trusted_keys(&mut self) {
        self.trusted_keys.clear();
    }

    pub fn get_trusted_keys_count(&self) -> usize {
        self.trusted_keys.len()
    }
}

impl Default for CertificateStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_calculation() {
        let verifier = IntegrityVerifier::new();
        let data = b"test data";
        
        let hash1 = verifier.calculate_hash(data).unwrap();
        let hash2 = verifier.calculate_hash(data).unwrap();
        
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex length
    }

    #[test]
    fn test_hash_verification() {
        let verifier = IntegrityVerifier::new();
        let data = b"test data";
        
        let hash = verifier.calculate_hash(data).unwrap();
        
        assert!(verifier.verify_hash(data, &hash));
        assert!(!verifier.verify_hash(data, "wrong_hash"));
    }

    #[test]
    fn test_model_signature() {
        let signature = ModelSignature::new("test_model", "abc123", "public_key");
        
        let serialized = signature.serialize().unwrap();
        let deserialized = ModelSignature::deserialize(&serialized).unwrap();
        
        assert_eq!(signature.model_name, deserialized.model_name);
        assert_eq!(signature.hash, deserialized.hash);
    }

    #[test]
    fn test_certificate_store() {
        let mut store = CertificateStore::new();
        
        assert_eq!(store.get_trusted_keys_count(), 0);
        assert!(!store.is_key_trusted("test_key"));
        
        store.add_trusted_key("test_key".to_string());
        assert_eq!(store.get_trusted_keys_count(), 1);
        assert!(store.is_key_trusted("test_key"));
        
        store.clear_trusted_keys();
        assert_eq!(store.get_trusted_keys_count(), 0);
    }
}
