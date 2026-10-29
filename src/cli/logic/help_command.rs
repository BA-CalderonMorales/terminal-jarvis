use super::{style, table};

#[path = "help_entry.rs"]
mod entry_table;

use entry_table::entry;

pub fn text(name: &str) -> String {
    let (usage, purpose) = entry(name);
    if style::plain() {
        return format!("terminal-jarvis {name}\n{purpose}\n\nusage: {usage}\n");
    }
    format!(
        "{}{}",
        style::banner(&format!("terminal-jarvis {name}"), purpose),
        table::fields("Usage", &[("COMMAND", usage.to_string())])
    )
}

#[cfg(test)]
#[path = "../tests/help_command_test.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/help_command_rich_test.rs"]
mod rich_tests;
