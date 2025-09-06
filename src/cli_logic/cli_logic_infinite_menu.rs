use crate::theme::theme_global_config;
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

/// Custom hybrid menu widget with infinite cycling + tab completion + search
pub struct InfiniteHybridMenu {
    options: Vec<String>,
    filtered_options: Vec<(usize, String)>, // (original_index, option)
    current_selection: usize,
    search_text: String,
    preserve_header: bool, // New field to control header preservation
    last_terminal_size: Option<(u16, u16)>, // Track terminal size for static display
    menu_start_row: u16,   // Track where the menu should start
}

impl InfiniteHybridMenu {
    pub fn new(_prompt: &str, options: Vec<String>) -> Self {
        let filtered_options = options
            .iter()
            .enumerate()
            .map(|(i, opt)| (i, opt.clone()))
            .collect();

        Self {
            options,
            filtered_options,
            current_selection: 0,
            search_text: String::new(),
            preserve_header: false,
            last_terminal_size: None,
            menu_start_row: 0,
        }
    }

    /// Create a new menu that preserves header content (for main menu with logo)
    pub fn new_with_header_preserved(_prompt: &str, options: Vec<String>) -> Self {
        let filtered_options = options
            .iter()
            .enumerate()
            .map(|(i, opt)| (i, opt.clone()))
            .collect();

        Self {
            options,
            filtered_options,
            current_selection: 0,
            search_text: String::new(),
            preserve_header: true,
            last_terminal_size: None,
            menu_start_row: 0,
        }
    }

    /// Update filtered options based on search text
    fn update_filter(&mut self) {
        if self.search_text.is_empty() {
            self.filtered_options = self
                .options
                .iter()
                .enumerate()
                .map(|(i, opt)| (i, opt.clone()))
                .collect();
        } else {
            let search_lower = self.search_text.to_lowercase();
            self.filtered_options = self
                .options
                .iter()
                .enumerate()
                .filter(|(_, opt)| opt.to_lowercase().contains(&search_lower))
                .map(|(i, opt)| (i, opt.clone()))
                .collect();
        }

        // Reset selection to first item if current selection is out of bounds
        if self.current_selection >= self.filtered_options.len()
            && !self.filtered_options.is_empty()
        {
            self.current_selection = 0;
        }
    }

    /// Move selection with infinite cycling
    fn move_selection(&mut self, direction: isize) {
        if self.filtered_options.is_empty() {
            return;
        }

        let len = self.filtered_options.len() as isize;
        let new_selection = (self.current_selection as isize + direction + len) % len;
        self.current_selection = new_selection as usize;
    }

    /// Get tab completion suggestion
    fn get_tab_completion(&self) -> Option<String> {
        if self.search_text.is_empty() || self.filtered_options.is_empty() {
            return None;
        }

        // If only one match, complete to it
        if self.filtered_options.len() == 1 {
            return Some(self.filtered_options[0].1.clone());
        }

        // Find common prefix among filtered options
        let candidates: Vec<_> = self
            .filtered_options
            .iter()
            .map(|(_, opt)| opt.as_str())
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Find the longest common prefix that extends beyond search text
        let first = candidates[0];
        let mut common_len = first.len();

        for candidate in candidates.iter().skip(1) {
            let mut len = 0;
            for (a, b) in first.chars().zip(candidate.chars()) {
                if a.to_lowercase().eq(b.to_lowercase()) {
                    len += a.len_utf8();
                } else {
                    break;
                }
            }
            common_len = common_len.min(len);
        }

        if common_len > self.search_text.len() {
            Some(first.chars().take(common_len / 4).collect())
        } else {
            None
        }
    }

    /// Draw the menu interface
    fn draw(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        let theme = theme_global_config::current_theme();

        // Check if terminal size changed (for header preservation menus)
        let current_size = crossterm::terminal::size().unwrap_or((80, 24));
        let size_changed = if let Some(last_size) = self.last_terminal_size {
            current_size != last_size
        } else {
            // First time - initialize size but don't trigger redraw
            self.last_terminal_size = Some(current_size);
            self.calculate_menu_position(current_size);
            false // Don't consider first time as size changed
        };

        if self.preserve_header && size_changed {
            // Terminal size actually changed - need complete redraw
            self.last_terminal_size = Some(current_size);
            self.calculate_menu_position(current_size);

            // Return error to trigger complete interface redraw
            return Err(anyhow::anyhow!("SIZE_CHANGE_REDRAW"));
        }

        // Clear screen to prevent resize corruption
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            MoveTo(0, 0)
        )?;

        // Hide cursor to prevent blinking
        queue!(stdout, Hide)?;

        if self.preserve_header {
            // For main menu with logo - move to calculated menu position after clearing
            queue!(stdout, MoveTo(0, self.menu_start_row))?;
        }

        // Draw prompt and search text with proper theme colors
        queue!(
            stdout,
            Print(theme.primary("Search or Choose from the options below: ")),
            Print(theme.secondary(&self.search_text)),
            Print(theme.primary(" (Tab to complete, ↑/↓ infinite cycle, Enter to select)\n\r\n\r")),
        )?;

        // Draw filtered options with theme colors
        if self.filtered_options.is_empty() {
            queue!(
                stdout,
                Print(theme.primary("No matches found.")),
                Print("\n\r"),
            )?;
        } else {
            for (index, (_, option)) in self.filtered_options.iter().enumerate() {
                if index == self.current_selection {
                    queue!(
                        stdout,
                        Print(theme.accent("▶ ")),
                        Print(theme.primary(option)),
                        Print("\n\r"),
                    )?;
                } else {
                    queue!(
                        stdout,
                        Print("  "),
                        Print(theme.secondary(option)),
                        Print("\n\r"),
                    )?;
                }
            }
        }

        // Flush all queued commands at once
        stdout
            .flush()
            .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;
        Ok(())
    }

    /// Calculate where the menu should start based on terminal size
    fn calculate_menu_position(&mut self, terminal_size: (u16, u16)) {
        let (_width, height) = terminal_size;

        // Simple header count:
        // - Empty line: 1 line
        // - Title line: 1 line  
        // - Version line: 1 line
        // - NPM warning (if shown): 2 lines
        // - Empty line: 1 line
        // Total: ~5-6 lines, use 6 for spacing
        let estimated_header_height = 6u16;

        // Position menu right after the header with minimal spacing
        self.menu_start_row = estimated_header_height.min(height.saturating_sub(8));
    }

    /// Run the interactive menu
    pub fn interact(mut self) -> Result<String> {
        // Enable raw mode with explicit echo control
        enable_raw_mode()?;

        // Initialize menu position for header-preserved menus
        if self.preserve_header {
            let terminal_size = crossterm::terminal::size().unwrap_or((80, 24));
            self.calculate_menu_position(terminal_size);
            self.last_terminal_size = Some(terminal_size);
        }

        let result = self.run_menu_loop();

        // Cleanup - show cursor and disable raw mode
        let mut stdout = io::stdout();
        queue!(stdout, Show)?;
        stdout.flush()?;
        disable_raw_mode()?;
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;

        result
    }

    /// Helper to handle draw with size change detection
    fn safe_draw(&mut self) -> Result<()> {
        self.draw().map_err(|e| {
            if e.to_string().contains("SIZE_CHANGE_REDRAW") {
                anyhow::anyhow!("SIZE_CHANGE_REDRAW") // Propagate size change error
            } else {
                anyhow::anyhow!("Draw error: {:?}", e)
            }
        })
    }

    fn run_menu_loop(&mut self) -> Result<String> {
        // Initial draw
        self.safe_draw()?;

        loop {
            // Wait for events (both key events and resize events)
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    // Only process key press events, not key release events
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }

                    match key_event.code {
                        KeyCode::Up => {
                            self.move_selection(-1);
                            self.safe_draw()?;
                        }
                        KeyCode::Down => {
                            self.move_selection(1);
                            self.safe_draw()?;
                        }
                        KeyCode::Tab => {
                            // Tab completion
                            if let Some(completion) = self.get_tab_completion() {
                                self.search_text = completion;
                                self.update_filter();
                                self.safe_draw()?;
                            }
                        }
                        KeyCode::Enter => {
                            // Enter - select current option
                            if !self.filtered_options.is_empty() {
                                let selected =
                                    self.filtered_options[self.current_selection].1.clone();
                                return Ok(selected);
                            }
                        }
                        KeyCode::Char(c) => {
                            // Handle Ctrl+C
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                                return Err(anyhow::anyhow!("Cancelled by user"));
                            }

                            // Add character to search
                            self.search_text.push(c);
                            self.update_filter();
                            self.safe_draw()?;
                        }
                        KeyCode::Backspace => {
                            // Remove character from search
                            self.search_text.pop();
                            self.update_filter();
                            self.safe_draw()?;
                        }
                        KeyCode::Esc => {
                            return Err(anyhow::anyhow!("Cancelled by user"));
                        }
                        _ => {
                            // Ignore other keys
                        }
                    }
                }
                Ok(Event::Resize(_, _)) => {
                    // OPTION A: Completely ignore all resize events to prevent interface corruption
                    // This eliminates the "stupid" horizontal resizing behavior entirely
                    continue;
                }
                _ => {
                    // Ignore other events (mouse, etc.)
                }
            }
        }
    }
}

/// Public function to create and run infinite hybrid menu
pub async fn infinite_hybrid_menu_select(prompt: &str, options: Vec<String>) -> Result<String> {
    let menu = InfiniteHybridMenu::new(prompt, options);
    menu.interact()
}

/// Public function to create and run infinite hybrid menu with header preserved (for main menu)
pub async fn infinite_hybrid_menu_select_with_header(
    prompt: &str,
    options: Vec<String>,
) -> Result<String> {
    let menu = InfiniteHybridMenu::new_with_header_preserved(prompt, options);
    menu.interact()
}
