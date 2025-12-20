// CLI Command Autocomplete System
//
// Provides efficient prefix-based autocomplete for slash commands.
// Uses a simple prefix tree (trie-like) structure with caching for
// fast lookups during interactive input.

use std::collections::HashMap;

/// Command definition with metadata
#[derive(Debug, Clone)]
pub struct CommandDef {
    pub command: &'static str,
    pub description: &'static str,
    pub aliases: &'static [&'static str],
}

/// Static command definitions
pub static COMMANDS: &[CommandDef] = &[
    CommandDef {
        command: "/tools",
        description: "AI CLI Tools",
        aliases: &["/t"],
    },
    CommandDef {
        command: "/evals",
        description: "Evals & Comparisons",
        aliases: &["/e"],
    },
    CommandDef {
        command: "/auth",
        description: "Authentication",
        aliases: &["/a"],
    },
    CommandDef {
        command: "/links",
        description: "Important Links",
        aliases: &["/l"],
    },
    CommandDef {
        command: "/settings",
        description: "Settings",
        aliases: &["/s"],
    },
    CommandDef {
        command: "/db",
        description: "Database Management",
        aliases: &[],
    },
    CommandDef {
        command: "/theme",
        description: "Change UI Theme",
        aliases: &[],
    },
    CommandDef {
        command: "/help",
        description: "Show help",
        aliases: &["/h", "/?"],
    },
    CommandDef {
        command: "/exit",
        description: "Exit Terminal Jarvis",
        aliases: &["/quit", "/q"],
    },
];

/// Autocomplete suggestion with match info
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub command: String,
    pub description: String,
    pub match_score: u8, // Higher = better match
}

/// Command autocomplete engine with prefix caching
pub struct CommandAutocomplete {
    /// Prefix cache: prefix -> matching commands
    cache: HashMap<String, Vec<Suggestion>>,
    /// All commands flattened (includes aliases)
    all_commands: Vec<(String, String)>, // (command, description)
}

impl Default for CommandAutocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandAutocomplete {
    /// Create new autocomplete engine
    pub fn new() -> Self {
        let mut all_commands = Vec::new();

        // Add all commands and their aliases
        for cmd in COMMANDS {
            all_commands.push((cmd.command.to_string(), cmd.description.to_string()));
            for alias in cmd.aliases {
                all_commands.push((alias.to_string(), cmd.description.to_string()));
            }
        }

        // Sort for consistent ordering
        all_commands.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            cache: HashMap::new(),
            all_commands,
        }
    }

    /// Get suggestions for a prefix
    pub fn suggest(&mut self, prefix: &str) -> Vec<Suggestion> {
        let prefix = prefix.to_lowercase();

        // Empty or non-slash returns nothing
        if prefix.is_empty() || !prefix.starts_with('/') {
            return Vec::new();
        }

        // Check cache first
        if let Some(cached) = self.cache.get(&prefix) {
            return cached.clone();
        }

        // Find matching commands
        let mut suggestions: Vec<Suggestion> = self
            .all_commands
            .iter()
            .filter(|(cmd, _)| cmd.starts_with(&prefix))
            .map(|(cmd, desc)| {
                // Score: exact match = 100, prefix match = length similarity
                let score = if cmd == &prefix {
                    100
                } else {
                    (prefix.len() * 10 / cmd.len()) as u8
                };

                Suggestion {
                    command: cmd.clone(),
                    description: desc.clone(),
                    match_score: score,
                }
            })
            .collect();

        // Sort by score (descending), then alphabetically
        suggestions.sort_by(|a, b| {
            b.match_score
                .cmp(&a.match_score)
                .then(a.command.cmp(&b.command))
        });

        // Cache the result
        self.cache.insert(prefix, suggestions.clone());

        suggestions
    }

    /// Get top suggestion (for tab completion)
    pub fn top_suggestion(&mut self, prefix: &str) -> Option<Suggestion> {
        self.suggest(prefix).into_iter().next()
    }

    /// Check if input is a complete valid command
    pub fn is_valid_command(&self, input: &str) -> bool {
        let input = input.to_lowercase();
        self.all_commands.iter().any(|(cmd, _)| cmd == &input)
    }

    /// Resolve alias to canonical command
    pub fn resolve_command(&self, input: &str) -> Option<&'static str> {
        let input_lower = input.to_lowercase();

        for cmd in COMMANDS {
            if cmd.command == input_lower {
                return Some(cmd.command);
            }
            for alias in cmd.aliases {
                if *alias == input_lower {
                    return Some(cmd.command);
                }
            }
        }
        None
    }

    /// Get all available commands (for help display)
    pub fn all_commands(&self) -> Vec<(&'static str, &'static str)> {
        COMMANDS
            .iter()
            .map(|c| (c.command, c.description))
            .collect()
    }

    /// Clear the cache (if commands change dynamically)
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

/// Format suggestions for display
pub fn format_suggestions(suggestions: &[Suggestion], theme: &crate::theme::Theme) -> String {
    if suggestions.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for (i, s) in suggestions.iter().take(5).enumerate() {
        if i > 0 {
            output.push_str("  ");
        }
        output.push_str(&format!(
            "{} {}",
            theme.accent(&s.command),
            theme.secondary(&format!("({})", s.description))
        ));
    }
    output
}

/// Global autocomplete instance (lazy initialized)
use std::sync::OnceLock;
static AUTOCOMPLETE: OnceLock<std::sync::Mutex<CommandAutocomplete>> = OnceLock::new();

/// Get or initialize global autocomplete
pub fn get_autocomplete() -> &'static std::sync::Mutex<CommandAutocomplete> {
    AUTOCOMPLETE.get_or_init(|| std::sync::Mutex::new(CommandAutocomplete::new()))
}

/// Inquire Autocomplete implementation for slash commands
#[derive(Clone)]
pub struct SlashCommandSuggester;

impl inquire::Autocomplete for SlashCommandSuggester {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        // Show suggestions when user types "/" (shows all commands) or more
        if input.starts_with('/') {
            if let Ok(mut ac) = get_autocomplete().lock() {
                let suggestions = ac.suggest(input);
                return Ok(suggestions
                    .into_iter()
                    .map(|s| format!("{} - {}", s.command, s.description))
                    .collect());
            }
        }
        Ok(Vec::new())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        // If user selected a suggestion, extract the command part
        if let Some(suggestion) = highlighted_suggestion {
            // Format is "command - description", extract command
            let command = suggestion.split(" - ").next().unwrap_or(&suggestion);
            return Ok(Some(command.to_string()));
        }

        // Tab completion - complete to top match
        if input.starts_with('/') {
            if let Ok(mut ac) = get_autocomplete().lock() {
                if let Some(top) = ac.top_suggestion(input) {
                    return Ok(Some(top.command));
                }
            }
        }

        Ok(None)
    }
}

/// Get all commands as formatted strings for display
pub fn get_all_command_options() -> Vec<String> {
    COMMANDS
        .iter()
        .map(|c| format!("{} - {}", c.command, c.description))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_suggestions() {
        let mut ac = CommandAutocomplete::new();

        let suggestions = ac.suggest("/t");
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.command == "/tools"));
    }

    #[test]
    fn test_exact_match() {
        let mut ac = CommandAutocomplete::new();

        let suggestions = ac.suggest("/tools");
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].command, "/tools");
        assert_eq!(suggestions[0].match_score, 100);
    }

    #[test]
    fn test_alias_resolution() {
        let ac = CommandAutocomplete::new();

        assert_eq!(ac.resolve_command("/t"), Some("/tools"));
        assert_eq!(ac.resolve_command("/q"), Some("/exit"));
        assert_eq!(ac.resolve_command("/h"), Some("/help"));
    }

    #[test]
    fn test_cache_efficiency() {
        let mut ac = CommandAutocomplete::new();

        // First call populates cache
        let _ = ac.suggest("/e");
        assert!(ac.cache.contains_key("/e"));

        // Second call uses cache
        let suggestions = ac.suggest("/e");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_empty_input() {
        let mut ac = CommandAutocomplete::new();

        assert!(ac.suggest("").is_empty());
        assert!(ac.suggest("hello").is_empty()); // No slash
    }

    #[test]
    fn test_valid_command() {
        let ac = CommandAutocomplete::new();

        assert!(ac.is_valid_command("/tools"));
        assert!(ac.is_valid_command("/t")); // Alias
        assert!(!ac.is_valid_command("/invalid"));
    }

    #[test]
    fn test_multiple_matches() {
        let mut ac = CommandAutocomplete::new();

        // "/" should match all commands
        let suggestions = ac.suggest("/");
        assert!(suggestions.len() > 5);
    }

    #[test]
    fn test_top_suggestion() {
        let mut ac = CommandAutocomplete::new();

        let top = ac.top_suggestion("/ex");
        assert!(top.is_some());
        assert_eq!(top.unwrap().command, "/exit");
    }
}
