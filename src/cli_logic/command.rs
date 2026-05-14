use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlashCommand {
    Tools,
    Auth,
    Links,
    Settings,
    Config,
    Db,
    Theme,
    Dashboard,
    Help,
    Exit,
}

impl SlashCommand {
    pub const fn all() -> &'static [Self] {
        &[
            Self::Tools,
            Self::Auth,
            Self::Links,
            Self::Settings,
            Self::Config,
            Self::Db,
            Self::Theme,
            Self::Dashboard,
            Self::Help,
            Self::Exit,
        ]
    }

    pub const fn command(self) -> &'static str {
        match self {
            Self::Tools => "/tools",
            Self::Auth => "/auth",
            Self::Links => "/links",
            Self::Settings => "/settings",
            Self::Config => "/config",
            Self::Db => "/db",
            Self::Theme => "/theme",
            Self::Dashboard => "/dashboard",
            Self::Help => "/help",
            Self::Exit => "/exit",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Tools => "AI CLI Tools",
            Self::Auth => "Authentication",
            Self::Links => "Important Links",
            Self::Settings => "Settings",
            Self::Config => "Configuration Path",
            Self::Db => "Database Management",
            Self::Theme => "Change UI Theme",
            Self::Dashboard => "Tool Health Dashboard",
            Self::Help => "Show help",
            Self::Exit => "Exit",
        }
    }

    pub const fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Dashboard => &["/status"],
            Self::Exit => &["/quit"],
            _ => &[],
        }
    }

    pub const fn accepts_arguments(self) -> bool {
        matches!(self, Self::Config)
    }

    pub fn parse_input(input: &str) -> Result<(Self, &str), String> {
        let trimmed = input.trim();
        let Some((command, args)) = trimmed.split_once(char::is_whitespace) else {
            return trimmed.parse().map(|command| (command, ""));
        };

        command.parse().map(|command| (command, args.trim()))
    }
}

impl fmt::Display for SlashCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.command())
    }
}

impl FromStr for SlashCommand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim().to_lowercase();
        for command in Self::all() {
            if input == command.command() || command.aliases().contains(&input.as_str()) {
                return Ok(*command);
            }
        }

        Err(format!("Unknown command: {input}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_commands() {
        assert_eq!(
            "/tools".parse::<SlashCommand>().unwrap(),
            SlashCommand::Tools
        );
        assert_eq!(
            "/config".parse::<SlashCommand>().unwrap(),
            SlashCommand::Config
        );
    }

    #[test]
    fn parses_aliases_to_canonical_variants() {
        assert_eq!(
            "/status".parse::<SlashCommand>().unwrap(),
            SlashCommand::Dashboard
        );
        assert_eq!("/quit".parse::<SlashCommand>().unwrap(), SlashCommand::Exit);
    }

    #[test]
    fn parse_input_preserves_arguments() {
        let (command, args) = SlashCommand::parse_input("/config ./custom.toml").unwrap();

        assert_eq!(command, SlashCommand::Config);
        assert_eq!(args, "./custom.toml");
    }

    #[test]
    fn only_config_accepts_arguments() {
        assert!(SlashCommand::Config.accepts_arguments());
        assert!(!SlashCommand::Help.accepts_arguments());
        assert!(!SlashCommand::Tools.accepts_arguments());
    }

    #[test]
    fn all_commands_have_descriptions() {
        assert!(SlashCommand::all()
            .iter()
            .all(|command| !command.description().is_empty()));
    }
}
