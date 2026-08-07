use super::Action;

pub(super) fn run(words: &[String], child: Vec<String>, boundary: bool) -> Result<Action, String> {
    validate_flags(words, boundary, run_help(words))?;
    Ok(Action::Run(join(words, child)))
}

pub(super) fn direct(
    harness: &str,
    words: &[String],
    child: Vec<String>,
    boundary: bool,
) -> Result<Action, String> {
    validate_flags(words, boundary, words.len() == 1 && is_help(&words[0]))?;
    Ok(Action::Direct {
        harness: harness.to_string(),
        extra: join(words, child),
    })
}

fn validate_flags(
    words: &[String],
    boundary: bool,
    compatibility_help: bool,
) -> Result<(), String> {
    if !boundary && compatibility_help {
        return Ok(());
    }
    if let Some(flag) = words.iter().find(|word| word.starts_with('-')) {
        return Err(format!(
            "child flag '{flag}' requires an explicit `--` boundary"
        ));
    }
    Ok(())
}

fn run_help(words: &[String]) -> bool {
    words.len() == 2 && !words[0].starts_with('-') && is_help(&words[1])
}

fn is_help(word: &str) -> bool {
    matches!(word, "--help" | "-h")
}

fn join(words: &[String], child: Vec<String>) -> Vec<String> {
    words.iter().cloned().chain(child).collect()
}
