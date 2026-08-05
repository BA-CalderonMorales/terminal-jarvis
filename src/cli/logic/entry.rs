use super::{args::Action, json, style};

pub fn parse_failure(args: &[String], error: &str) -> i32 {
    if before_boundary(args, "--json") {
        print!(
            "{}",
            json::failure("parse", 2, "usage", error, "terminal-jarvis --help",)
        );
    } else {
        let plain = before_boundary(args, "--plain");
        let no_color = plain || before_boundary(args, "--no-color");
        let previous = style::set(plain, no_color);
        eprint!("{}", style::error(error));
        style::restore(previous);
    }
    2
}

pub fn action_name(action: &Action) -> String {
    match action {
        Action::CommandHelp(command) => format!("help {command}"),
        other => format!("{other:?}")
            .split([' ', '{', '('])
            .next()
            .unwrap_or("unknown")
            .to_ascii_lowercase(),
    }
}

fn before_boundary(args: &[String], flag: &str) -> bool {
    args.iter()
        .skip(1)
        .take_while(|word| *word != "--")
        .any(|word| word == flag)
}
