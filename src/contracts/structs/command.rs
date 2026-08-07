#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandPlan {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandPlan {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }

    pub fn render(&self) -> String {
        let mut parts = Vec::with_capacity(self.args.len() + 1);
        parts.push(shell_word(&self.command));
        parts.extend(self.args.iter().map(|arg| shell_word(arg)));
        parts.join(" ")
    }
}

fn shell_word(value: &str) -> String {
    if value
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || "-_./@:=+".contains(char))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}
