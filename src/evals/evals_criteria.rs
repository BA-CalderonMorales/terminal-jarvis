// Evals Criteria Domain
// Defines standard evaluation criteria and loads custom X-factor criteria

use super::evals_data::{EvaluationCriterion, MetricDefinition, MetricType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Standard evaluation criteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriteriaConfig {
    pub version: String,
    pub last_updated: String,
    pub criteria: Vec<EvaluationCriterion>,
}

/// X-factor (custom) criteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XFactorConfig {
    pub enabled: bool,
    pub custom_criteria: Vec<EvaluationCriterion>,
}

/// Criteria loader and manager
pub struct CriteriaManager {
    standard_criteria: Vec<EvaluationCriterion>,
    custom_criteria: Vec<EvaluationCriterion>,
}

impl Default for CriteriaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CriteriaManager {
    /// Create new criteria manager with standard and custom criteria
    pub fn new() -> Self {
        let mut manager = Self {
            standard_criteria: Self::get_builtin_criteria(),
            custom_criteria: Vec::new(),
        };

        // Try to load custom criteria
        if let Err(e) = manager.load_custom_criteria() {
            eprintln!("Warning: Failed to load custom X-factor criteria: {}", e);
        }

        manager
    }

    /// Get all standard built-in evaluation criteria
    fn get_builtin_criteria() -> Vec<EvaluationCriterion> {
        vec![
            // 1. Authentication & Setup
            EvaluationCriterion {
                id: "auth_setup".to_string(),
                name: "Authentication & Setup".to_string(),
                description: "Ease of authentication, API key management, initial setup complexity".to_string(),
                weight: 1.0,
                metrics: vec![
                    MetricDefinition {
                        id: "auth_method".to_string(),
                        name: "Authentication Method".to_string(),
                        description: "How does the tool authenticate? (API key, OAuth, browser, etc.)".to_string(),
                        metric_type: MetricType::Categorical,
                        evaluation_guide: "Document the authentication flow and complexity".to_string(),
                    },
                    MetricDefinition {
                        id: "setup_time".to_string(),
                        name: "Setup Time".to_string(),
                        description: "Time from installation to first successful use".to_string(),
                        metric_type: MetricType::Numeric,
                        evaluation_guide: "Measure in minutes for a new user".to_string(),
                    },
                    MetricDefinition {
                        id: "env_var_management".to_string(),
                        name: "Environment Variable Management".to_string(),
                        description: "How well does it handle API keys and environment variables?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate ease of configuration (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 2. Invocation Interface
            EvaluationCriterion {
                id: "invocation".to_string(),
                name: "Invocation Interface".to_string(),
                description: "CLI ergonomics, command structure, ease of use".to_string(),
                weight: 1.2,
                metrics: vec![
                    MetricDefinition {
                        id: "command_structure".to_string(),
                        name: "Command Structure".to_string(),
                        description: "How intuitive is the CLI command syntax?".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "Assess verb-noun patterns, flags, subcommands".to_string(),
                    },
                    MetricDefinition {
                        id: "help_system".to_string(),
                        name: "Help System Quality".to_string(),
                        description: "Quality of --help, documentation, examples".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate completeness and clarity (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "interactive_mode".to_string(),
                        name: "Interactive Mode Support".to_string(),
                        description: "Does it support interactive/REPL mode?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe capabilities if yes".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 3. Model/Provider Support
            EvaluationCriterion {
                id: "model_support".to_string(),
                name: "Model/Provider Support".to_string(),
                description: "Which LLMs/providers are supported, multi-provider capability".to_string(),
                weight: 1.1,
                metrics: vec![
                    MetricDefinition {
                        id: "supported_providers".to_string(),
                        name: "Supported Providers".to_string(),
                        description: "List of supported AI providers (OpenAI, Anthropic, etc.)".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "Enumerate all supported providers".to_string(),
                    },
                    MetricDefinition {
                        id: "multi_provider".to_string(),
                        name: "Multi-Provider Support".to_string(),
                        description: "Can switch between providers easily?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe switching mechanism".to_string(),
                    },
                    MetricDefinition {
                        id: "model_selection".to_string(),
                        name: "Model Selection Flexibility".to_string(),
                        description: "Can users choose specific models within a provider?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate flexibility (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 4. Extensibility
            EvaluationCriterion {
                id: "extensibility".to_string(),
                name: "Extensibility".to_string(),
                description: "Plugin system, customization, configuration options".to_string(),
                weight: 0.9,
                metrics: vec![
                    MetricDefinition {
                        id: "plugin_system".to_string(),
                        name: "Plugin System".to_string(),
                        description: "Does it have a plugin/extension system?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe plugin architecture".to_string(),
                    },
                    MetricDefinition {
                        id: "config_options".to_string(),
                        name: "Configuration Options".to_string(),
                        description: "Depth and breadth of configuration possibilities".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate customizability (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "custom_prompts".to_string(),
                        name: "Custom Prompt Support".to_string(),
                        description: "Can users define custom system prompts or templates?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe mechanism".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 5. User Experience
            EvaluationCriterion {
                id: "user_experience".to_string(),
                name: "User Experience".to_string(),
                description: "Conversation flow, error messages, help system, overall UX".to_string(),
                weight: 1.3,
                metrics: vec![
                    MetricDefinition {
                        id: "conversation_quality".to_string(),
                        name: "Conversation Quality".to_string(),
                        description: "How natural and effective is the interaction?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate conversation flow (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "error_handling".to_string(),
                        name: "Error Handling".to_string(),
                        description: "Quality of error messages and recovery".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate clarity and helpfulness (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "context_awareness".to_string(),
                        name: "Context Awareness".to_string(),
                        description: "How well does it understand project context?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate context understanding (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 6. Privacy & Security
            EvaluationCriterion {
                id: "privacy_security".to_string(),
                name: "Privacy & Security".to_string(),
                description: "Data handling, local-first capabilities, telemetry, security practices".to_string(),
                weight: 1.4,
                metrics: vec![
                    MetricDefinition {
                        id: "data_handling".to_string(),
                        name: "Data Handling Policy".to_string(),
                        description: "What data is sent to cloud services? Is local mode available?".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "Document data flows and storage".to_string(),
                    },
                    MetricDefinition {
                        id: "telemetry".to_string(),
                        name: "Telemetry & Analytics".to_string(),
                        description: "Does it collect usage data? Can it be disabled?".to_string(),
                        metric_type: MetricType::Categorical,
                        evaluation_guide: "None/Opt-in/Opt-out/Always-on".to_string(),
                    },
                    MetricDefinition {
                        id: "credential_security".to_string(),
                        name: "Credential Security".to_string(),
                        description: "How are API keys and credentials stored?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate security practices (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 7. Documentation Quality
            EvaluationCriterion {
                id: "documentation".to_string(),
                name: "Documentation Quality".to_string(),
                description: "Completeness, examples, API docs, tutorials".to_string(),
                weight: 1.0,
                metrics: vec![
                    MetricDefinition {
                        id: "completeness".to_string(),
                        name: "Completeness".to_string(),
                        description: "Coverage of features and use cases".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate documentation coverage (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "examples".to_string(),
                        name: "Example Quality".to_string(),
                        description: "Quality and quantity of examples".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate examples (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "api_docs".to_string(),
                        name: "API Documentation".to_string(),
                        description: "Technical API documentation availability".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - assess quality if yes".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 8. Community & Support
            EvaluationCriterion {
                id: "community_support".to_string(),
                name: "Community & Support".to_string(),
                description: "GitHub activity, Discord/forums, issue response time, community size".to_string(),
                weight: 0.8,
                metrics: vec![
                    MetricDefinition {
                        id: "github_activity".to_string(),
                        name: "GitHub Activity".to_string(),
                        description: "Stars, forks, recent commits, active contributors".to_string(),
                        metric_type: MetricType::Numeric,
                        evaluation_guide: "Count stars, assess commit frequency".to_string(),
                    },
                    MetricDefinition {
                        id: "support_channels".to_string(),
                        name: "Support Channels".to_string(),
                        description: "Available support channels (Discord, forums, email)".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "List all available channels".to_string(),
                    },
                    MetricDefinition {
                        id: "response_time".to_string(),
                        name: "Issue Response Time".to_string(),
                        description: "Average time to first response on issues".to_string(),
                        metric_type: MetricType::Numeric,
                        evaluation_guide: "Measure in hours/days".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 9. Licensing
            EvaluationCriterion {
                id: "licensing".to_string(),
                name: "Licensing".to_string(),
                description: "Open source vs proprietary, license type, restrictions".to_string(),
                weight: 0.7,
                metrics: vec![
                    MetricDefinition {
                        id: "license_type".to_string(),
                        name: "License Type".to_string(),
                        description: "What license governs the tool?".to_string(),
                        metric_type: MetricType::Categorical,
                        evaluation_guide: "MIT/Apache/GPL/Proprietary/etc.".to_string(),
                    },
                    MetricDefinition {
                        id: "open_source".to_string(),
                        name: "Open Source".to_string(),
                        description: "Is the source code publicly available?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - link to repository".to_string(),
                    },
                    MetricDefinition {
                        id: "commercial_use".to_string(),
                        name: "Commercial Use Restrictions".to_string(),
                        description: "Any restrictions on commercial usage?".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "Document any limitations".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 10. Performance
            EvaluationCriterion {
                id: "performance".to_string(),
                name: "Performance".to_string(),
                description: "Speed, resource usage, streaming capabilities".to_string(),
                weight: 1.0,
                metrics: vec![
                    MetricDefinition {
                        id: "response_time".to_string(),
                        name: "Response Time".to_string(),
                        description: "Time to first token / completion".to_string(),
                        metric_type: MetricType::Numeric,
                        evaluation_guide: "Measure in seconds".to_string(),
                    },
                    MetricDefinition {
                        id: "streaming".to_string(),
                        name: "Streaming Support".to_string(),
                        description: "Does it support streaming responses?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe quality".to_string(),
                    },
                    MetricDefinition {
                        id: "resource_usage".to_string(),
                        name: "Resource Usage".to_string(),
                        description: "CPU/memory footprint".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate efficiency (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 11. Integration Capabilities
            EvaluationCriterion {
                id: "integration".to_string(),
                name: "Integration Capabilities".to_string(),
                description: "Git integration, IDE plugins, workflow tool compatibility".to_string(),
                weight: 0.9,
                metrics: vec![
                    MetricDefinition {
                        id: "git_integration".to_string(),
                        name: "Git Integration".to_string(),
                        description: "How well does it integrate with Git workflows?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate Git awareness (1-10)".to_string(),
                    },
                    MetricDefinition {
                        id: "ide_support".to_string(),
                        name: "IDE Support".to_string(),
                        description: "Available IDE plugins or extensions".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "List supported IDEs".to_string(),
                    },
                    MetricDefinition {
                        id: "workflow_tools".to_string(),
                        name: "Workflow Tool Integration".to_string(),
                        description: "Integration with CI/CD, project management, etc.".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "Document integrations".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 12. Unique Differentiators
            EvaluationCriterion {
                id: "differentiators".to_string(),
                name: "Unique Differentiators".to_string(),
                description: "What makes this tool special? Killer features, unique approaches".to_string(),
                weight: 1.1,
                metrics: vec![
                    MetricDefinition {
                        id: "killer_features".to_string(),
                        name: "Killer Features".to_string(),
                        description: "Standout features not found elsewhere".to_string(),
                        metric_type: MetricType::Qualitative,
                        evaluation_guide: "List and describe unique capabilities".to_string(),
                    },
                    MetricDefinition {
                        id: "innovation".to_string(),
                        name: "Innovation Score".to_string(),
                        description: "How innovative is the approach?".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate novelty and innovation (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },

            // 13. Cost Structure
            EvaluationCriterion {
                id: "cost".to_string(),
                name: "Cost Structure".to_string(),
                description: "Free tier, pricing model, usage limits".to_string(),
                weight: 0.8,
                metrics: vec![
                    MetricDefinition {
                        id: "free_tier".to_string(),
                        name: "Free Tier Availability".to_string(),
                        description: "Is there a free tier or trial?".to_string(),
                        metric_type: MetricType::Boolean,
                        evaluation_guide: "Yes/No - describe limits".to_string(),
                    },
                    MetricDefinition {
                        id: "pricing_model".to_string(),
                        name: "Pricing Model".to_string(),
                        description: "How is usage charged? (API calls, tokens, subscription)".to_string(),
                        metric_type: MetricType::Categorical,
                        evaluation_guide: "Free/Pay-as-you-go/Subscription/Hybrid".to_string(),
                    },
                    MetricDefinition {
                        id: "cost_efficiency".to_string(),
                        name: "Cost Efficiency".to_string(),
                        description: "Value for money compared to alternatives".to_string(),
                        metric_type: MetricType::Scale,
                        evaluation_guide: "Rate cost-effectiveness (1-10)".to_string(),
                    },
                ],
                is_custom: false,
            },
        ]
    }

    /// Load custom X-factor criteria from configuration
    fn load_custom_criteria(&mut self) -> Result<()> {
        let config_paths = vec![
            dirs::config_dir().map(|p| p.join("terminal-jarvis").join("evals").join("x-factor.toml")),
            Some(PathBuf::from("./config/evals/x-factor.toml")),
        ];

        for path in config_paths.into_iter().flatten() {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let x_factor: XFactorConfig = toml::from_str(&content)?;

                if x_factor.enabled {
                    self.custom_criteria = x_factor.custom_criteria;
                }
                break;
            }
        }

        Ok(())
    }

    /// Get all criteria (standard + custom)
    pub fn get_all_criteria(&self) -> Vec<EvaluationCriterion> {
        let mut all_criteria = self.standard_criteria.clone();
        all_criteria.extend(self.custom_criteria.clone());
        all_criteria
    }

    /// Get only standard criteria
    pub fn get_standard_criteria(&self) -> &[EvaluationCriterion] {
        &self.standard_criteria
    }

    /// Get only custom X-factor criteria
    pub fn get_custom_criteria(&self) -> &[EvaluationCriterion] {
        &self.custom_criteria
    }

    /// Get criterion by ID
    pub fn get_criterion(&self, id: &str) -> Option<&EvaluationCriterion> {
        self.get_all_criteria().iter().find(|c| c.id == id)
    }

    /// Get criteria as HashMap for quick lookup
    pub fn get_criteria_map(&self) -> HashMap<String, EvaluationCriterion> {
        self.get_all_criteria()
            .into_iter()
            .map(|c| (c.id.clone(), c))
            .collect()
    }
}

/// Global criteria manager instance
static CRITERIA_MANAGER: std::sync::OnceLock<CriteriaManager> = std::sync::OnceLock::new();

/// Get global criteria manager
pub fn get_criteria_manager() -> &'static CriteriaManager {
    CRITERIA_MANAGER.get_or_init(CriteriaManager::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criteria_manager_creation() {
        let manager = CriteriaManager::new();
        let criteria = manager.get_standard_criteria();

        // Should have 13 standard criteria
        assert_eq!(criteria.len(), 13);

        // Check that all have unique IDs
        let ids: std::collections::HashSet<_> = criteria.iter().map(|c| &c.id).collect();
        assert_eq!(ids.len(), 13);
    }

    #[test]
    fn test_get_criterion_by_id() {
        let manager = CriteriaManager::new();
        let auth_criterion = manager.get_criterion("auth_setup");

        assert!(auth_criterion.is_some());
        assert_eq!(auth_criterion.unwrap().name, "Authentication & Setup");
    }

    #[test]
    fn test_criteria_weights() {
        let manager = CriteriaManager::new();
        let criteria = manager.get_standard_criteria();

        // All weights should be positive
        for criterion in criteria {
            assert!(criterion.weight > 0.0);
        }
    }
}
