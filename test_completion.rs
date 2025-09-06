use dialoguer::{Completion, Input, theme::ColorfulTheme};

struct TestCompletion {
    options: Vec<String>,
}

impl TestCompletion {
    fn new() -> Self {
        Self {
            options: vec!["claude".to_string(), "codex".to_string(), "gemini".to_string()],
        }
    }
}

impl Completion for TestCompletion {
    fn get(&self, input: &str) -> Option<String> {
        println!("DEBUG: get() called with input: '{}'", input);
        let matches: Vec<_> = self
            .options
            .iter()
            .filter(|option| option.starts_with(&input.to_lowercase()))
            .collect();

        if matches.len() == 1 {
            println!("DEBUG: Found single match: {}", matches[0]);
            Some(matches[0].to_string())
        } else {
            println!("DEBUG: Found {} matches", matches.len());
            None
        }
    }
}

fn main() {
    let completion = TestCompletion::new();

    println!("Test tab completion. Type 'cl' and press Tab:");
    
    match Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Tool name")
        .completion_with(&completion)
        .interact_text()
    {
        Ok(input) => println!("You entered: '{}'", input),
        Err(e) => println!("Error: {}", e),
    }
}
