use crate::contracts::Capability;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Help,
    CommandHelp(String),
    List,
    Check,
    Current,
    Version {
        verbose: bool,
    },
    Use(String),
    Show(String),
    Plan {
        harness: Option<String>,
        capability: Capability,
    },
    Run(Vec<String>),
    Direct {
        harness: String,
        extra: Vec<String>,
    },
    Install(String),
    SelfUpdate {
        dry_run: bool,
    },
    Update(Option<String>),
    Auth(Vec<String>),
    Config(Vec<String>),
    Cache(Vec<String>),
    Security(Vec<String>),
    Gate(Vec<String>),
    Experimental(Vec<String>),
    Legacy(String),
}
