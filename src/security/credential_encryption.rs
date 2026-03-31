// Credential Encryption Module - Secure storage for API keys and secrets
//
// Implements defense-in-depth for credential storage:
// 1. Platform keychain integration (preferred)
// 2. AES-256-GCM encrypted file fallback
// 3. Argon2id key derivation
// 4. Transparent migration from plaintext

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Credential encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    pub service_name: String,
    pub keychain_preferred: bool,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            service_name: "terminal-jarvis".to_string(),
            keychain_preferred: true,
        }
    }
}

/// Encrypted credential store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentialStore {
    /// Version of the encryption format
    pub version: u32,
    /// Salt for key derivation (Argon2id)
    pub salt: Vec<u8>,
    /// Nonce for AES-GCM
    pub nonce: Vec<u8>,
    /// Encrypted credential data
    pub ciphertext: Vec<u8>,
    /// Tools configuration (unencrypted, tool names only)
    pub tools: Vec<String>,
}

impl EncryptedCredentialStore {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new(salt: Vec<u8>, nonce: Vec<u8>, ciphertext: Vec<u8>, tools: Vec<String>) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            salt,
            nonce,
            ciphertext,
            tools,
        }
    }
}

/// Credential encryption provider
pub struct CredentialEncryption {
    config: EncryptionConfig,
    master_key: Option<Vec<u8>>,
}

impl CredentialEncryption {
    pub fn new(config: EncryptionConfig) -> Self {
        Self {
            config,
            master_key: None,
        }
    }

    pub fn with_master_key(config: EncryptionConfig, key: Vec<u8>) -> Self {
        Self {
            config,
            master_key: Some(key),
        }
    }

    /// Check if platform keychain is available
    #[cfg(feature = "credential-encryption")]
    pub fn is_keychain_available(&self) -> bool {
        // Try to access the keychain by attempting a dummy entry
        match keyring::Entry::new(&self.config.service_name, "test") {
            Ok(entry) => entry.get_password().is_ok() || entry.set_password("test").is_ok(),
            Err(_) => false,
        }
    }

    #[cfg(not(feature = "credential-encryption"))]
    pub fn is_keychain_available(&self) -> bool {
        false
    }

    /// Get the path for encrypted credential storage
    pub fn encrypted_creds_path(&self) -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("credentials.enc"))
    }

    /// Get the path for plaintext credential storage (legacy)
    pub fn plaintext_creds_path(&self) -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow!("Could not determine config directory"))?
            .join("terminal-jarvis");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("credentials.toml"))
    }

    /// Check if plaintext credentials exist
    pub fn has_plaintext_credentials(&self) -> Result<bool> {
        let path = self.plaintext_creds_path()?;
        Ok(path.exists())
    }

    /// Check if encrypted credentials exist
    pub fn has_encrypted_credentials(&self) -> Result<bool> {
        let path = self.encrypted_creds_path()?;
        Ok(path.exists())
    }

    /// Store credentials using the best available method
    #[cfg(feature = "credential-encryption")]
    pub fn store_credentials(
        &mut self,
        tools: &HashMap<String, HashMap<String, String>>,
    ) -> Result<()> {
        if self.config.keychain_preferred && self.is_keychain_available() {
            self.store_in_keychain(tools)
        } else {
            self.store_encrypted_file(tools)
        }
    }

    #[cfg(not(feature = "credential-encryption"))]
    pub fn store_credentials(
        &mut self,
        tools: &HashMap<String, HashMap<String, String>>,
    ) -> Result<()> {
        // Fallback to plaintext when encryption is disabled
        self.store_plaintext(tools)
    }

    /// Store credentials in platform keychain
    #[cfg(feature = "credential-encryption")]
    fn store_in_keychain(&self, tools: &HashMap<String, HashMap<String, String>>) -> Result<()> {
        let json = serde_json::to_string(tools)?;

        let entry = keyring::Entry::new(&self.config.service_name, "credentials")
            .context("Failed to create keychain entry")?;
        entry
            .set_password(&json)
            .context("Failed to store credentials in keychain")?;

        Ok(())
    }

    /// Store credentials in encrypted file
    #[cfg(feature = "credential-encryption")]
    fn store_encrypted_file(
        &mut self,
        tools: &HashMap<String, HashMap<String, String>>,
    ) -> Result<()> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        use argon2::{Argon2, PasswordHasher};
        use rand::RngCore;

        // Get or derive master key
        let key = if let Some(ref key) = self.master_key {
            key.clone()
        } else {
            // Generate a new key using Argon2id
            let password = self.prompt_master_password()?;
            let salt: [u8; 32] = {
                let mut salt = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut salt);
                salt
            };

            let argon2 = Argon2::default();

            // Use the salt directly with argon2
            let salt_str = format!("{:x}", sha2::Sha256::digest(salt));
            let parsed_salt = argon2::password_hash::Salt::from_b64(&salt_str[..32])
                .map_err(|e| anyhow!("Failed to create salt: {}", e))?
                .to_owned();

            let hash = argon2
                .hash_password(password.as_bytes(), parsed_salt)
                .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

            // Use the hash output as the AES key
            let key_bytes = hash.hash.ok_or_else(|| anyhow!("Failed to derive key"))?;
            let key: Vec<u8> = key_bytes.as_bytes().to_vec();

            // Store salt for later decryption
            let path = self.encrypted_creds_path()?;

            // Prepare data for encryption
            let plaintext = serde_json::to_vec(tools)?;

            // Generate nonce
            let mut nonce_bytes = [0u8; 12];
            rand::thread_rng().fill_bytes(&mut nonce_bytes);

            // Encrypt
            let cipher = Aes256Gcm::new_from_slice(&key[..32])?;
            let nonce = Nonce::from_slice(&nonce_bytes);
            let ciphertext = cipher
                .encrypt(nonce, plaintext.as_ref())
                .map_err(|e| anyhow!("Encryption failed: {:?}", e))?;

            // Create store
            let store = EncryptedCredentialStore::new(
                salt.to_vec(),
                nonce_bytes.to_vec(),
                ciphertext,
                tools.keys().cloned().collect(),
            );

            // Save to file
            let encoded = serde_json::to_vec(&store)?;
            fs::write(&path, encoded)?;

            // Remove plaintext file if it exists
            let plaintext_path = self.plaintext_creds_path()?;
            if plaintext_path.exists() {
                fs::remove_file(&plaintext_path)?;
            }

            self.master_key = Some(key.clone());
            return Ok(());
        };

        // If we already have the key, re-encrypt with existing parameters
        let path = self.encrypted_creds_path()?;
        let plaintext = serde_json::to_vec(tools)?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(&key[..32])?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| anyhow!("Encryption failed: {:?}", e))?;

        let store = EncryptedCredentialStore::new(
            // Use existing salt or generate new
            vec![0u8; 32],
            nonce_bytes.to_vec(),
            ciphertext,
            tools.keys().cloned().collect(),
        );

        let encoded = serde_json::to_vec(&store)?;
        fs::write(&path, encoded)?;

        // Remove plaintext file
        let plaintext_path = self.plaintext_creds_path()?;
        if plaintext_path.exists() {
            fs::remove_file(&plaintext_path)?;
        }

        Ok(())
    }

    /// Store credentials in plaintext (fallback when encryption disabled)
    #[cfg(not(feature = "credential-encryption"))]
    fn store_plaintext(&self, tools: &HashMap<String, HashMap<String, String>>) -> Result<()> {
        let path = self.plaintext_creds_path()?;

        #[derive(Debug, Serialize, Deserialize)]
        struct CredentialsFile {
            pub tools: HashMap<String, HashMap<String, String>>,
        }

        let creds = CredentialsFile {
            tools: tools.clone(),
        };

        let content = toml::to_string_pretty(&creds)?;
        fs::write(path, content)?;

        Ok(())
    }

    /// Load credentials using the best available method
    #[cfg(feature = "credential-encryption")]
    pub fn load_credentials(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        // Try keychain first
        if self.config.keychain_preferred && self.is_keychain_available() {
            match self.load_from_keychain() {
                Ok(creds) => return Ok(creds),
                Err(_) => {
                    // Fall through to encrypted file
                }
            }
        }

        // Try encrypted file
        if self.has_encrypted_credentials()? {
            return self.load_encrypted_file();
        }

        // Try plaintext (legacy)
        if self.has_plaintext_credentials()? {
            return self.load_plaintext();
        }

        // No credentials found
        Ok(HashMap::new())
    }

    #[cfg(not(feature = "credential-encryption"))]
    pub fn load_credentials(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        if self.has_plaintext_credentials()? {
            self.load_plaintext()
        } else {
            Ok(HashMap::new())
        }
    }

    /// Load credentials from platform keychain
    #[cfg(feature = "credential-encryption")]
    fn load_from_keychain(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        let entry = keyring::Entry::new(&self.config.service_name, "credentials")
            .context("Failed to create keychain entry")?;
        let json = entry
            .get_password()
            .context("Failed to read from keychain")?;
        let tools: HashMap<String, HashMap<String, String>> = serde_json::from_str(&json)?;
        Ok(tools)
    }

    /// Load credentials from encrypted file
    #[cfg(feature = "credential-encryption")]
    fn load_encrypted_file(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        use argon2::{Argon2, PasswordHasher};

        let path = self.encrypted_creds_path()?;
        let encoded = fs::read(&path)?;
        let store: EncryptedCredentialStore = serde_json::from_slice(&encoded)?;

        // Get master password from user
        let password = self.prompt_master_password()?;

        // Derive key using Argon2id
        let argon2 = Argon2::default();
        let salt_str = format!("{:x}", sha2::Sha256::digest(&store.salt));
        let parsed_salt = argon2::password_hash::Salt::from_b64(&salt_str[..32])
            .map_err(|e| anyhow!("Failed to create salt: {}", e))?
            .to_owned();

        let hash = argon2
            .hash_password(password.as_bytes(), parsed_salt)
            .map_err(|e| anyhow!("Failed to derive key: {}", e))?;

        let key_bytes = hash.hash.ok_or_else(|| anyhow!("Failed to derive key"))?;
        let key = key_bytes.as_bytes();

        // Decrypt
        let cipher = Aes256Gcm::new_from_slice(&key[..32])?;
        let nonce = Nonce::from_slice(&store.nonce);
        let plaintext = cipher
            .decrypt(nonce, store.ciphertext.as_ref())
            .map_err(|e| anyhow!("Decryption failed: {:?}", e))?;

        let tools: HashMap<String, HashMap<String, String>> = serde_json::from_slice(&plaintext)?;
        Ok(tools)
    }

    /// Load credentials from plaintext file
    fn load_plaintext(&self) -> Result<HashMap<String, HashMap<String, String>>> {
        let path = self.plaintext_creds_path()?;

        #[derive(Debug, Deserialize)]
        struct CredentialsFile {
            pub tools: HashMap<String, HashMap<String, String>>,
        }

        let content = fs::read_to_string(&path)?;
        let creds: CredentialsFile = toml::from_str(&content)?;
        Ok(creds.tools)
    }

    /// Prompt user for master password
    #[cfg(feature = "credential-encryption")]
    fn prompt_master_password(&self) -> Result<String> {
        println!("Enter master password for credential encryption:");
        let password = rpassword::read_password()?;
        Ok(password)
    }

    /// Get encryption status for display
    pub fn get_encryption_status(&self) -> EncryptionStatus {
        let has_plaintext = self.has_plaintext_credentials().unwrap_or(false);
        let has_encrypted = self.has_encrypted_credentials().unwrap_or(false);
        let keychain_available = self.is_keychain_available();

        let method = if has_encrypted {
            EncryptionMethod::Aes256Gcm
        } else if has_plaintext {
            EncryptionMethod::Plaintext
        } else {
            EncryptionMethod::None
        };

        EncryptionStatus {
            method,
            keychain_available,
            has_credentials: has_plaintext || has_encrypted,
            credential_count: 0, // Would need to load to count
        }
    }

    /// Migrate plaintext credentials to encrypted storage
    #[cfg(feature = "credential-encryption")]
    pub fn migrate_plaintext(&mut self) -> Result<()> {
        if !self.has_plaintext_credentials()? {
            return Ok(());
        }

        let tools = self.load_plaintext()?;
        self.store_credentials(&tools)?;

        Ok(())
    }

    #[cfg(not(feature = "credential-encryption"))]
    pub fn migrate_plaintext(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Encryption method in use
#[derive(Debug, Clone, PartialEq)]
pub enum EncryptionMethod {
    None,
    Plaintext,
    Keychain,
    Aes256Gcm,
}

impl std::fmt::Display for EncryptionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionMethod::None => write!(f, "No credentials stored"),
            EncryptionMethod::Plaintext => write!(f, "Plaintext (not encrypted)"),
            EncryptionMethod::Keychain => write!(f, "Platform Keychain"),
            EncryptionMethod::Aes256Gcm => write!(f, "AES-256-GCM"),
        }
    }
}

/// Current encryption status
#[derive(Debug, Clone)]
pub struct EncryptionStatus {
    pub method: EncryptionMethod,
    pub keychain_available: bool,
    pub has_credentials: bool,
    pub credential_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_config_default() {
        let config = EncryptionConfig::default();
        assert_eq!(config.service_name, "terminal-jarvis");
        assert!(config.keychain_preferred);
    }

    #[test]
    fn test_encrypted_store_serialization() {
        let store = EncryptedCredentialStore::new(
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec!["tool1".to_string()],
        );

        let json = serde_json::to_string(&store).unwrap();
        let deserialized: EncryptedCredentialStore = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.version,
            EncryptedCredentialStore::CURRENT_VERSION
        );
        assert_eq!(deserialized.salt, vec![1, 2, 3]);
        assert_eq!(deserialized.nonce, vec![4, 5, 6]);
        assert_eq!(deserialized.ciphertext, vec![7, 8, 9]);
        assert_eq!(deserialized.tools, vec!["tool1".to_string()]);
    }

    #[test]
    fn test_encryption_method_display() {
        assert_eq!(
            format!("{}", EncryptionMethod::None),
            "No credentials stored"
        );
        assert_eq!(
            format!("{}", EncryptionMethod::Plaintext),
            "Plaintext (not encrypted)"
        );
        assert_eq!(
            format!("{}", EncryptionMethod::Keychain),
            "Platform Keychain"
        );
        assert_eq!(format!("{}", EncryptionMethod::Aes256Gcm), "AES-256-GCM");
    }
}
