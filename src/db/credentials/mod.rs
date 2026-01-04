// Credentials Domain Module
//
// Secure credential storage for AI tool API keys.
// Supports encryption at rest and migration from TOML-based storage.

mod entities;
mod repository;

pub use entities::{Credential, CredentialBuilder};
pub use repository::CredentialsRepository;
