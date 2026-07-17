pub use super::action::Action;

#[path = "args_action.rs"]
mod action_parser;
#[path = "args_child.rs"]
mod child_parser;
#[path = "args_help.rs"]
mod help_parser;
#[path = "args_options.rs"]
mod option_parser;
#[path = "args_validate.rs"]
mod validators;
#[path = "args_values.rs"]
mod values;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OutputMode {
    #[default]
    Rich,
    Plain,
    Json,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Options {
    pub output: OutputMode,
    pub no_color: bool,
    pub verbose: bool,
    pub dry_run: bool,
    pub no_input: bool,
    pub confirm: Option<String>,
    pub allow_dangerous: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parsed {
    pub action: Action,
    pub options: Options,
}

pub fn parse_cli<I>(args: I) -> Result<Parsed, String>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let words = args.into_iter().map(Into::into).skip(1).collect();
    let option_parser::Extracted {
        words,
        child,
        boundary,
        options,
    } = option_parser::extract(words)?;
    let mut action = action_parser::parse(&words, child, boundary)?;
    validators::validate_options(&action, &options)?;
    if let Action::Version { verbose } = &mut action {
        *verbose |= options.verbose;
    }
    if let Action::SelfUpdate { dry_run } = &mut action {
        *dry_run = options.dry_run;
    }
    Ok(Parsed { action, options })
}

pub fn parse<I>(args: I) -> Result<Action, String>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    parse_cli(args).map(|parsed| match parsed.action {
        Action::CommandHelp(_) => Action::Help,
        action => action,
    })
}

#[cfg(test)]
#[path = "mutation_test.rs"]
mod mutation_tests;
#[cfg(test)]
#[path = "args_test.rs"]
mod tests;
#[cfg(test)]
#[path = "args_test_extra.rs"]
mod tests_extra;
#[cfg(test)]
#[path = "args_test_gate.rs"]
mod tests_gate;
