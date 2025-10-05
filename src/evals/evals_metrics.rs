// Evals Metrics Domain
// Real-world verifiable metrics for tool evaluation

use serde::{Deserialize, Serialize};

/// Real-world metrics that help researchers and companies make decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetrics {
    /// GitHub repository information
    pub github: Option<GitHubMetrics>,

    /// Package manager statistics
    pub package: Option<PackageMetrics>,

    /// Community engagement metrics
    pub community: Option<CommunityMetrics>,

    /// Documentation quality indicators
    pub documentation: Option<DocumentationMetrics>,

    /// Platform and compatibility information
    pub platform: PlatformMetrics,

    /// Team and organization transparency
    pub team: Option<TeamMetrics>,

    /// Support and response metrics
    pub support: Option<SupportMetrics>,
}

/// GitHub repository metrics (verifiable via API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubMetrics {
    pub repo_url: String,
    pub stars: Option<u32>,
    pub forks: Option<u32>,
    pub open_issues: Option<u32>,
    pub last_commit_date: Option<String>,
    pub commit_frequency: Option<String>, // "Daily", "Weekly", "Monthly"
    pub contributors: Option<u32>,
    pub license: Option<String>,
    pub created_date: Option<String>,
    pub archived: bool,
}

/// Package manager statistics (NPM, PyPI, Cargo, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetrics {
    pub package_name: String,
    pub registry: String, // "NPM", "PyPI", "Crates.io"
    pub weekly_downloads: Option<u64>,
    pub total_downloads: Option<u64>,
    pub latest_version: Option<String>,
    pub last_publish_date: Option<String>,
    pub version_count: Option<u32>,
}

/// Community engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityMetrics {
    pub discord_members: Option<u32>,
    pub discord_url: Option<String>,
    pub reddit_subscribers: Option<u32>,
    pub reddit_url: Option<String>,
    pub slack_members: Option<u32>,
    pub twitter_followers: Option<u32>,
    pub twitter_url: Option<String>,
    pub forum_url: Option<String>,
    pub active_discussions: bool,
}

/// Documentation quality indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationMetrics {
    pub docs_url: String,

    // Primary documentation flags
    #[serde(default)]
    pub has_readme: bool,
    #[serde(default)]
    pub has_contributing_guide: bool,
    #[serde(default)]
    pub has_examples: bool,

    // Quality indicators
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_freshness: Option<String>, // "< 1 week", "< 1 month", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_docs_quality: Option<String>, // "Excellent", "Good", "Adequate", etc.

    // Optional extended fields (for backward compatibility)
    #[serde(default)]
    pub has_getting_started: bool,
    #[serde(default)]
    pub has_api_reference: bool,
    #[serde(default)]
    pub has_troubleshooting: bool,
    #[serde(default)]
    pub has_video_tutorials: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_docs_update: Option<String>,
    #[serde(default)]
    pub search_available: bool,
    #[serde(default)]
    pub multilingual: bool,
    #[serde(default)]
    pub languages: Vec<String>,
}

/// Platform and OS support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformMetrics {
    pub supported_os: Vec<String>,  // "macOS", "Linux", "Windows", "BSD"
    pub architectures: Vec<String>, // "x86_64", "ARM64", "ARM"
    pub requires_docker: bool,
    pub web_based: bool,
    pub mobile_support: bool,
    pub cloud_only: bool,

    // Extended platform support fields
    #[serde(default)]
    pub supported_languages: Vec<String>, // Programming languages supported
    #[serde(default)]
    pub docker_support: bool,
    #[serde(default)]
    pub ci_cd_setup: bool,
    #[serde(default)]
    pub cloud_deployment_guides: bool,
}

/// Team and organization transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMetrics {
    pub organization_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_size: Option<String>, // "Solo", "Small (2-10)", "Medium (11-50)", "Large (50+)"
    pub public_team: bool, // Are team members publicly listed?
    #[serde(default)]
    pub backed_by: Vec<String>, // Investors, companies, foundations
    pub funding_disclosed: bool,
    pub security_policy: bool,
    pub responsible_disclosure: bool,

    // Extended team transparency fields
    #[serde(default)]
    pub doxxed_team: bool, // Team members publicly identifiable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_funding: Option<String>, // Funding round/amount
    #[serde(default)]
    pub public_roadmap: bool,
    #[serde(default)]
    pub security_audits: bool,
}

/// Support and response metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportMetrics {
    // Response time fields (support both naming conventions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_response_time: Option<String>, // Backward compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_issue_response_time: Option<String>, // "< 24 hours", "1-3 days", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_pr_review_time: Option<String>, // Average PR review time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_issue_close_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_vs_closed_ratio: Option<f64>,

    // Support availability
    pub has_paid_support: bool,
    #[serde(default)]
    pub commercial_support: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_support: Option<String>, // Description of community support
    #[serde(default)]
    pub support_channels: Vec<String>, // "GitHub Issues", "Discord", "Email", "Forum"
    pub sla_available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime_percentage: Option<f64>,

    // Team metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_maintainers: Option<u32>,
}
