pub use super::action::Action;

pub use option_parser::{Options, OutputMode, Parsed};

#[path = "args_action.rs"]
mod action_parser;
#[path = "args_child.rs"]
mod child_parser;
#[path = "args_default.rs"]
mod defaults;
#[path = "args_help.rs"]
mod help_parser;
#[path = "args_options.rs"]
mod option_parser;
#[path = "args_validate.rs"]
mod validators;
#[path = "args_values.rs"]
mod values;

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
    if action_parser::bare_launch(&words, &options) {
        action = Action::Tui;
    }
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
    Ok(match parse_cli(args)?.action {
        Action::CommandHelp(_) => Action::Help,
        action => action,
    })
}

#[cfg(test)]
#[path = "../tests/args_matrix.rs"]
mod matrix;
#[cfg(test)]
#[path = "../tests/args_red.rs"]
mod matrix_red;
#[cfg(test)]
#[path = "../tests/mutation_test.rs"]
mod mutation_tests;
#[cfg(test)]
#[path = "../tests/args_props.rs"]
mod props;
#[cfg(test)]
#[path = "../tests/args_red_green.rs"]
mod red_green;
#[cfg(test)]
#[path = "../tests/args_rules.rs"]
mod rules;
#[cfg(test)]
#[path = "../tests/args_test.rs"]
mod tests;
#[cfg(test)]
#[path = "../tests/args_test_extra.rs"]
mod tests_extra;
#[cfg(test)]
#[path = "../tests/args_test_gate.rs"]
mod tests_gate;
