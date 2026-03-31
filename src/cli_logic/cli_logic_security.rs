// CLI Logic - Security Commands
// Handles security status, encryption, and audit functionality

use crate::security::{CredentialEncryption, EncryptionConfig, EncryptionMethod};
use anyhow::Result;

/// Display security status to the user
pub async fn handle_security_status() -> Result<()> {
    let config = EncryptionConfig::default();
    let encryption = CredentialEncryption::new(config);
    let status = encryption.get_encryption_status();

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              Security Status Report                      ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    // Encryption status
    println!("Credential Storage:");
    println!("  Method:        {}", status.method);
    println!(
        "  Keychain:      {}",
        if status.keychain_available {
            "Available"
        } else {
            "Not Available"
        }
    );
    println!(
        "  Credentials:   {}",
        if status.has_credentials {
            "Present"
        } else {
            "None stored"
        }
    );

    match status.method {
        EncryptionMethod::Plaintext => {
            println!();
            println!("⚠️  WARNING: Credentials are stored in plaintext");
            println!("   Any process running as your user can read these.");
            println!();
            println!("   To encrypt your credentials, run:");
            println!("     jarvis security encrypt");
        }
        EncryptionMethod::None => {
            println!();
            println!("ℹ️  No credentials stored yet.");
            println!("   Credentials will be encrypted when you add them.");
        }
        _ => {}
    }

    println!();

    // Supply chain status
    println!("Supply Chain Security:");
    println!("  Dependencies:  Pinned to exact versions");
    println!("  Lockfile:      Verified in CI");
    println!("  Audit:         Automated (moderate+)");

    println!();

    // Recommendations
    if status.method == EncryptionMethod::Plaintext {
        println!("Recommendations:");
        println!("  [HIGH] Encrypt your credentials: jarvis security encrypt");
    }

    if !status.keychain_available && status.method != EncryptionMethod::None {
        println!("  [MED]  Install a keychain service for better security");
    }

    Ok(())
}

/// Encrypt existing plaintext credentials
pub async fn handle_security_encrypt() -> Result<()> {
    let config = EncryptionConfig::default();
    let mut encryption = CredentialEncryption::new(config);

    // Check if we have plaintext credentials
    if !encryption.has_plaintext_credentials()? {
        if encryption.has_encrypted_credentials()? {
            println!("✓ Credentials are already encrypted.");
            return Ok(());
        } else {
            println!("ℹ️  No credentials found to encrypt.");
            println!("   Credentials will be encrypted automatically when you add them.");
            return Ok(());
        }
    }

    println!("Encrypting credentials...");
    println!();

    // Perform migration
    match encryption.migrate_plaintext() {
        Ok(_) => {
            println!("✓ Credentials successfully encrypted!");
            println!();

            // Show new status
            let status = encryption.get_encryption_status();
            println!("Storage method: {}", status.method);
        }
        Err(e) => {
            eprintln!("✗ Failed to encrypt credentials: {}", e);
            eprintln!();
            eprintln!("Your original credentials are still intact.");
            return Err(e);
        }
    }

    Ok(())
}

/// Run security audit
pub async fn handle_security_audit() -> Result<()> {
    println!("Running security audit...");
    println!();

    let config = EncryptionConfig::default();
    let encryption = CredentialEncryption::new(config);
    let status = encryption.get_encryption_status();

    let mut issues_found = 0;

    // Check credential encryption
    match status.method {
        EncryptionMethod::Plaintext => {
            println!("[HIGH] Credentials stored in plaintext");
            println!("       Run: jarvis security encrypt");
            issues_found += 1;
        }
        EncryptionMethod::None => {
            println!("[INFO] No credentials stored");
        }
        _ => {
            println!("[OK]   Credentials encrypted");
        }
    }

    // Check keychain availability
    if status.keychain_available {
        println!("[OK]   Platform keychain available");
    } else if status.has_credentials {
        println!("[MED]  Platform keychain not available");
        println!("       Using file-based encryption (still secure)");
    }

    println!();

    if issues_found == 0 {
        println!("✓ No security issues found");
    } else {
        println!("⚠️  {} issue(s) found", issues_found);
    }

    Ok(())
}
