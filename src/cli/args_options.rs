use super::{Options, OutputMode};

pub(super) struct Extracted {
    pub words: Vec<String>,
    pub child: Vec<String>,
    pub boundary: bool,
    pub options: Options,
}

pub(super) fn extract(input: Vec<String>) -> Result<Extracted, String> {
    let mut words = Vec::new();
    let mut options = Options::default();
    let mut index = 0;
    while index < input.len() {
        let word = &input[index];
        if word == "--" {
            return finish(words, input[index + 1..].to_vec(), true, options);
        }
        match word.as_str() {
            "--plain" => set_output(&mut options, OutputMode::Plain)?,
            "--json" => set_output(&mut options, OutputMode::Json)?,
            "--no-color" => options.no_color = true,
            "--verbose" => options.verbose = true,
            "--dry-run" => options.dry_run = true,
            "--no-input" => options.no_input = true,
            "--allow-dangerous" => options.allow_dangerous = true,
            _ if word.starts_with("--confirm=") => set_confirm(&mut options, word)?,
            _ => words.push(word.clone()),
        }
        index += 1;
    }
    finish(words, Vec::new(), false, options)
}

fn finish(
    words: Vec<String>,
    child: Vec<String>,
    boundary: bool,
    options: Options,
) -> Result<Extracted, String> {
    Ok(Extracted {
        words,
        child,
        boundary,
        options,
    })
}

fn set_output(options: &mut Options, output: OutputMode) -> Result<(), String> {
    if options.output != OutputMode::Rich && options.output != output {
        return Err("--plain and --json are mutually exclusive".into());
    }
    options.output = output;
    Ok(())
}

fn set_confirm(options: &mut Options, word: &str) -> Result<(), String> {
    let token = word.trim_start_matches("--confirm=");
    let valid = token
        .split_once(':')
        .is_some_and(|(operation, target)| !operation.is_empty() && !target.is_empty());
    if !valid {
        return Err("--confirm requires <operation>:<target>".into());
    }
    if options.confirm.replace(token.to_string()).is_some() {
        return Err("--confirm may be supplied only once".into());
    }
    Ok(())
}
