use crate::contracts::Capability;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Help,
    CommandHelp(String),
    List,
    Check,
    Tui,
    Current,
    Version {
        verbose: bool,
    },
    Use(String),
    Show(Option<String>),
    Plan {
        harness: Option<String>,
        capability: Capability,
    },
    Run(Vec<String>),
    Direct {
        harness: String,
        extra: Vec<String>,
    },
    Install(Option<String>),
    Uninstall(Option<String>),
    SelfUpdate {
        dry_run: bool,
    },
    Update(Option<String>),
    Auth(Vec<String>),
    Config(Vec<String>),
    Cache(Vec<String>),
    Security(Vec<String>),
    Gate(Vec<String>),
    Legacy(String),
}
