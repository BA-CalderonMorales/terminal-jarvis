//! Menu Component
//!
//! A reusable menu component for terminal interfaces.
//! Supports multiple menu items with descriptions and selection.

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(title: String) -> Self {
        Self {
            title,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, label: String, value: String, description: Option<String>) {
        self.items.push(MenuItem {
            label,
            value,
            description,
        });
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn render(&self) -> String {
        let mut output = format!("{}\n", self.title);
        output.push_str(&"=".repeat(self.title.len()));
        output.push_str("\n\n");

        for (index, item) in self.items.iter().enumerate() {
            output.push_str(&format!("{}. {}", index + 1, item.label));

            if let Some(desc) = &item.description {
                output.push_str(&format!(" - {desc}"));
            }

            output.push('\n');
        }

        output
    }
}
