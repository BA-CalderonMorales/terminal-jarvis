use super::ToolHandler;
use crate::auth_manager::AuthManager;
use anyhow::Result;
use std::io::IsTerminal;
use std::process::Command;

pub struct GooseHandler;

impl ToolHandler for GooseHandler {
    fn validate_auth(&self, cmd: &mut Command) -> Result<()> {
        validate_goose_gemini_key(cmd)
    }

    fn prepare_env(&self, cmd: &mut Command) -> Result<()> {
        hydrate_goose_credentials(cmd);
        Ok(())
    }

    fn uses_host_auth_environment(&self) -> bool {
        true
    }
}

fn hydrate_goose_credentials(cmd: &mut Command) {
    if let Ok(saved) = AuthManager::get_tool_credentials("goose") {
        for (key, value) in saved {
            cmd.env(&key, &value);
        }
    }

    if let Ok(saved_gemini) = AuthManager::get_tool_credentials("gemini") {
        for (key, value) in saved_gemini {
            if key == "GOOGLE_API_KEY" || key == "GEMINI_API_KEY" {
                cmd.env(&key, &value);
            }
        }
    }
}

fn validate_goose_gemini_key(cmd: &mut Command) -> Result<()> {
    let Some(candidate_key) = candidate_gemini_key() else {
        return Ok(());
    };

    if is_valid_gemini_api_key(&candidate_key) {
        return Ok(());
    }

    let theme = crate::theme::theme_global_config::current_theme();
    println!(
        "{}",
        theme.primary("Gemini provider requires a valid API key, not an OAuth token.")
    );
    println!(
        "{}",
        theme.secondary(
            "Get a key from Google AI Studio and set GOOGLE_API_KEY (or GEMINI_API_KEY).\nDocs: https://ai.google.dev/gemini-api/docs/api-key"
        )
    );

    if !std::io::stdin().is_terminal() {
        return Err(invalid_gemini_credentials_error());
    }

    let input = inquire::Password::new("Enter a valid GOOGLE_API_KEY (leave blank to cancel):")
        .without_confirmation()
        .prompt();
    let Ok(input) = input else {
        return Ok(());
    };

    let new_key = input.trim().to_string();
    if new_key.is_empty() {
        return Err(invalid_gemini_credentials_error());
    }

    if !looks_like_gemini_api_key(&new_key) {
        return Err(anyhow::anyhow!(
            "The provided key does not look like a valid Gemini API key."
        ));
    }

    cmd.env("GOOGLE_API_KEY", &new_key);
    cmd.env("GEMINI_API_KEY", &new_key);
    save_gemini_credentials(new_key);
    Ok(())
}

fn candidate_gemini_key() -> Option<String> {
    std::env::var("GOOGLE_API_KEY")
        .ok()
        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
        .or_else(|| saved_key_for("goose").or_else(|| saved_key_for("gemini")))
}

fn saved_key_for(tool: &str) -> Option<String> {
    AuthManager::get_tool_credentials(tool)
        .ok()
        .and_then(|credentials| {
            credentials
                .get("GOOGLE_API_KEY")
                .cloned()
                .or_else(|| credentials.get("GEMINI_API_KEY").cloned())
        })
}

fn save_gemini_credentials(new_key: String) {
    let mut credentials = std::collections::HashMap::new();
    credentials.insert("GOOGLE_API_KEY".to_string(), new_key.clone());
    credentials.insert("GEMINI_API_KEY".to_string(), new_key);
    let _ = AuthManager::save_tool_credentials("gemini", &credentials);
    let _ = AuthManager::save_tool_credentials("goose", &credentials);
}

fn invalid_gemini_credentials_error() -> anyhow::Error {
    anyhow::anyhow!(
        "Invalid Gemini credentials detected. Update your GOOGLE_API_KEY and try again."
    )
}

fn is_valid_gemini_api_key(key: &str) -> bool {
    !looks_like_oauth_token(key) && looks_like_gemini_api_key(key)
}

fn looks_like_gemini_api_key(key: &str) -> bool {
    let key = key.trim();
    (key.starts_with("AIza") || key.starts_with("AI"))
        && key.len() >= 25
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn looks_like_oauth_token(token: &str) -> bool {
    let token = token.trim();
    token.starts_with("4/") || token.starts_with("ya29.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gemini_api_key_allows_expected_prefixes() {
        assert!(is_valid_gemini_api_key("AIza1234567890abcdefghi_jklmn"));
        assert!(is_valid_gemini_api_key("AI1234567890abcdefghi-jklmno"));
    }

    #[test]
    fn gemini_api_key_rejects_oauth_tokens() {
        assert!(!is_valid_gemini_api_key("4/oauth-token"));
        assert!(!is_valid_gemini_api_key("ya29.oauth-token"));
    }

    #[test]
    fn gemini_api_key_rejects_short_or_malformed_values() {
        assert!(!is_valid_gemini_api_key("AIza-short"));
        assert!(!is_valid_gemini_api_key("AIza1234567890abcdefghi+jklmn"));
    }

    #[test]
    fn goose_handler_uses_host_auth_environment() {
        assert!(GooseHandler.uses_host_auth_environment());
    }
}
