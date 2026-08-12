use super::option_parser::{Options, OutputMode};

impl Default for Options {
    fn default() -> Self {
        Self {
            output: OutputMode::default(),
            no_color: false,
            verbose: false,
            dry_run: false,
            no_input: false,
            confirm: None,
            allow_dangerous: false,
            narrate: true,
        }
    }
}
