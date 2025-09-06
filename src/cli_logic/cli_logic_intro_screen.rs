use crate::theme::theme_global_config;
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind},
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};

/// Display the intro screen with living animations
pub async fn display_intro_screen() -> Result<bool> {
    // Initialize theme configuration
    let theme = theme_global_config::current_theme();

    // Clear screen and setup full terminal mode
    print!("\x1b[2J\x1b[H");

    // Display living intro interface
    display_living_intro_interface(&theme).await
}

/// Display full-screen living intro interface with smoother animations
async fn display_living_intro_interface(theme: &crate::theme::Theme) -> Result<bool> {
    // Enable raw mode for input handling and animations
    enable_raw_mode()?;

    let result = {
        let mut stdout = io::stdout();
        queue!(stdout, Hide)?; // Hide cursor

        // Get terminal dimensions
        let (term_width, term_height) = size()?;

        // Animation state
        let start_time = Instant::now();
        let mut frame_count = 0u64;

        loop {
            // Clear screen for each frame
            queue!(stdout, MoveTo(0, 0))?;

            // Calculate smoother animation phases
            let elapsed = start_time.elapsed().as_millis() as f32 / 1000.0;
            let pulse_phase = (elapsed * 1.8).sin() * 0.5 + 0.5; // Slightly faster, smoother pulse
            let wave_phase = elapsed * 1.2; // Smoother wave speed

            // Draw living background
            draw_living_background(
                &mut stdout,
                term_width,
                term_height,
                pulse_phase,
                wave_phase,
            )?;

            // Draw centered ASCII logo with breathing effect
            draw_breathing_logo(&mut stdout, term_width, term_height, theme, pulse_phase)?;

            // Draw living status and version info
            draw_living_status(
                &mut stdout,
                term_width,
                term_height,
                theme,
                wave_phase,
                frame_count,
            )?;

            // Draw animated instructions
            draw_animated_instructions(&mut stdout, term_width, term_height, theme, pulse_phase)?;

            stdout.flush()?;
            frame_count += 1;

            // Check for input with higher frame rate (~30 FPS)
            if event::poll(Duration::from_millis(33))? {
                match event::read()? {
                    Event::Key(key_event) => {
                        if key_event.kind != KeyEventKind::Press {
                            continue;
                        }

                        match key_event.code {
                            KeyCode::Enter => {
                                break Ok(true); // Continue to main menu
                            }
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                // Enter screensaver mode
                                screensaver_mode().await?;
                                break Ok(true); // After screensaver, go to menu
                            }
                            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                                // Exit application
                                queue!(stdout, ResetColor, MoveTo(0, term_height - 1))?;
                                stdout.flush()?;
                                println!("\nGoodbye!");
                                std::process::exit(0);
                            }
                            _ => {
                                // Other keys ignored
                            }
                        }
                    }
                    _ => {
                        // Other events ignored
                    }
                }
            }

            // Smooth frame rate (~30 FPS)
            std::thread::sleep(Duration::from_millis(33));
        }
    };

    // Clean up terminal state
    let mut stdout = io::stdout();
    queue!(stdout, Show, ResetColor)?;
    stdout.flush()?;
    disable_raw_mode()?;

    result
}

/// Draw living background with rich ASCII animations and dynamic colors
fn draw_living_background(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    pulse_phase: f32,
    wave_phase: f32,
) -> Result<()> {
    // More dynamic colors that pulse with life (with overflow protection)
    let base_blue = (8.0 + (pulse_phase * 8.0).sin().abs() * 8.0).min(255.0) as u8; // Safe 8-16 range
    let accent_purple = (6.0 + (pulse_phase * 1.5 + wave_phase * 0.7).sin().abs() * 6.0).min(255.0) as u8; // Safe 6-12 range
    let accent_cyan = (4.0 + (wave_phase * 2.0).cos().abs() * 4.0).min(255.0) as u8; // Safe 4-8 range
    
    // Expanded set of living ASCII characters
    let bg_chars = ['·', '˙', '•', '∘', '○', '◦', '⋅', '‧', '∴', '∵', '⁖', '⁘'];
    
    for row in 0..height {
        queue!(stdout, MoveTo(0, row))?;
        
        for col in 0..width {
            // Multiple overlapping wave patterns for organic feel
            let primary_wave = (col as f32 * 0.04 + wave_phase * 1.2).sin() 
                * (row as f32 * 0.025 + wave_phase * 0.8).cos();
            let secondary_wave = (col as f32 * 0.07 + wave_phase * 0.6).cos() 
                * (row as f32 * 0.03 + wave_phase * 1.1).sin();
            let tertiary_wave = (col as f32 * 0.02 + wave_phase * 1.8).sin() 
                * (row as f32 * 0.015 + wave_phase * 0.4).cos();
            
            let combined_wave = (primary_wave + secondary_wave * 0.6 + tertiary_wave * 0.3) / 3.0;
            let wave_intensity = combined_wave * 0.25 + 0.75; // More pronounced waves
            
            // Dynamic animated character probability
            let char_trigger = (((col as f32 * 0.12 + wave_phase * 1.3).sin() 
                              * (row as f32 * 0.09 + wave_phase * 1.7).cos() 
                              + (pulse_phase * 2.5).sin() * 0.3) + 1.0) / 2.0;
            
            if char_trigger > 0.88 { // 12% of cells get animated chars
                // Pick animated background character with more variation
                let char_phase = (col + row) as f32 * 0.4 + wave_phase * 1.5 + pulse_phase * 0.8;
                let char_index = char_phase.sin().abs();
                let char_idx = (char_index * bg_chars.len() as f32) as usize % bg_chars.len();
                
                // Dynamic animated character colors with overflow protection
                let char_intensity = ((pulse_phase * 40.0 + wave_phase * 20.0 + 20.0).max(0.0).min(255.0)) as u8;
                let char_hue_shift = (char_phase * 0.5).sin();
                
                queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                    r: ((char_intensity as f32 * (1.0 + char_hue_shift * 0.3)).max(0.0).min(255.0)) as u8,
                    g: ((char_intensity as f32 * (1.0 + char_hue_shift * 0.1)).max(0.0).min(255.0)) as u8,
                    b: char_intensity.saturating_add(15),
                }))?;
                queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset))?;
                queue!(stdout, Print(bg_chars[char_idx]))?;
            } else {
                // More dynamic background colors with multiple hues
                let mixed_intensity = wave_intensity * 0.7 + 0.3;
                let color_selector = (col + row) as f32 * 0.1 + wave_phase * 0.5;
                
                if color_selector.sin() > 0.6 { // Purple zones
                    let bg_color = (accent_purple as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: bg_color / 2, 
                        g: 0, 
                        b: bg_color 
                    }))?;
                } else if color_selector.cos() > 0.3 { // Cyan zones
                    let bg_color = (accent_cyan as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: 0, 
                        g: bg_color, 
                        b: bg_color + 5 
                    }))?;
                } else { // Blue zones
                    let bg_color = (base_blue as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: 0, 
                        g: bg_color / 10, 
                        b: bg_color 
                    }))?;
                }
                queue!(stdout, Print(" "))?;
            }
        }
    }
    
    Ok(())
}

/// Draw breathing ASCII logo with dynamic life
fn draw_breathing_logo(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    _theme: &crate::theme::Theme,
    pulse_phase: f32,
) -> Result<()> {
    // Simple, highly readable text-based logo
    let logo_lines = vec![
        "                                  ",
        "                                  ",
        "         T . J A R V I S          ",
        "                                  ",
        "       Terminal AI Assistant      ",
        "                                  ",
        "                                  ",
    ];

    let logo_start_row = (height / 2).saturating_sub(logo_lines.len() as u16 / 2 + 1);

    for (i, line) in logo_lines.iter().enumerate() {
        let row = logo_start_row + i as u16;
        let col = (width / 2).saturating_sub(line.len() as u16 / 2);
        
        queue!(stdout, MoveTo(col, row))?;
        
        // Much more alive text with dynamic colors and breathing
        for (char_i, ch) in line.chars().enumerate() {
            // Multi-layered animation effects
            let primary_shimmer = (char_i as f32 * 0.15 + pulse_phase * 2.0).sin() * 0.12 + 0.88;
            let secondary_pulse = (pulse_phase * 3.5 + i as f32 * 0.5).cos() * 0.08;
            let character_wave = (char_i as f32 * 0.3 + pulse_phase * 1.2).sin() * 0.05;
            
            let combined_intensity = primary_shimmer + secondary_pulse + character_wave;
            let base_intensity = 240; // Very bright base
            let intensity = ((base_intensity as f32 * combined_intensity).max(180.0).min(255.0)) as u8;
            
            // Dynamic color shifting for alive feeling
            let color_shift = (pulse_phase * 1.8 + char_i as f32 * 0.1).sin();
            let red_component = intensity;
            let green_component = (intensity as f32 * (0.95 + color_shift * 0.1)) as u8;
            let blue_component = (intensity as f32 * (1.0 + color_shift * 0.15).min(1.1)) as u8;
            
            queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                r: red_component,
                g: green_component,
                b: blue_component,
            }))?;
            
            queue!(stdout, Print(ch))?;
        }
    }
    
    Ok(())
}

/// Draw living status with dynamic life and energy
fn draw_living_status(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    _theme: &crate::theme::Theme,
    wave_phase: f32,
    frame_count: u64,
) -> Result<()> {
    let status_row = (height / 2) + 4;
    
    // Dynamic status indicators with alive animations
    let status_indicators = vec![
        ("◉", "SESSION ACTIVE"),
        ("◈", "SYSTEMS ONLINE"),
        ("▲", "AI TOOLS READY"), 
        ("★", "WELCOME ABOARD")
    ];
    
    for (i, (icon, text)) in status_indicators.iter().enumerate() {
        let row = status_row + i as u16;
        let col = (width / 2).saturating_sub((icon.len() + text.len() + 1) as u16 / 2);
        
        queue!(stdout, MoveTo(col, row))?;
        
        // Animated icon with pulsing colors
        let icon_pulse = (frame_count as f32 * 0.15 + i as f32 * 0.8).sin() * 0.3 + 0.7;
        let icon_shimmer = (wave_phase * 2.5 + i as f32 * 1.2).cos() * 0.2 + 0.8;
        let combined_icon_intensity = icon_pulse * icon_shimmer;
        
        let icon_base = 200;
        let icon_intensity = (icon_base as f32 * combined_icon_intensity) as u8;
        
        // Dynamic icon colors that shift with time
        let hue_shift = (wave_phase * 1.5 + i as f32 * 0.5).sin();
        queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
            r: (icon_intensity as f32 * (1.0 + hue_shift * 0.2)) as u8,
            g: (icon_intensity as f32 * (1.1 + hue_shift * 0.1)) as u8,
            b: (icon_intensity as f32 * (0.9 + hue_shift * 0.3)) as u8,
        }))?;
        queue!(stdout, Print(icon))?;
        queue!(stdout, Print(" "))?;
        
        // Animated text with wave effects
        for (char_i, ch) in text.chars().enumerate() {
            let char_wave = (char_i as f32 * 0.3 + wave_phase * 1.8 + i as f32 * 0.4).sin() * 0.15 + 0.85;
            let char_pulse = (frame_count as f32 * 0.08 + char_i as f32 * 0.2).cos() * 0.1 + 0.9;
            let combined_char_intensity = char_wave * char_pulse;
            
            let text_base = 190;
            let char_intensity = (text_base as f32 * combined_char_intensity) as u8;
            
            queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                r: char_intensity,
                g: (char_intensity as f32 * 1.05) as u8,
                b: (char_intensity as f32 * 0.95) as u8,
            }))?;
            
            queue!(stdout, Print(ch))?;
        }
    }
    
    Ok(())
}

/// Draw animated instructions with dynamic life and energy
fn draw_animated_instructions(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    _theme: &crate::theme::Theme,
    pulse_phase: f32,
) -> Result<()> {
    let instructions = vec![
        ("►", "Press ENTER to continue"),
        ("◐", "Press S for screensaver mode"),
        ("◢", "Press ESC to exit")
    ];
    
    let start_row = height.saturating_sub(instructions.len() as u16 + 2);
    
    for (i, (icon, text)) in instructions.iter().enumerate() {
        let row = start_row + i as u16;
        let col = (width / 2).saturating_sub((icon.len() + text.len() + 1) as u16 / 2);
        
        queue!(stdout, MoveTo(col, row))?;
        
        // Animated icon with dynamic pulsing
        let icon_pulse = (pulse_phase * 3.0 + i as f32 * 1.2).sin() * 0.4 + 0.6;
        let icon_rotation = (pulse_phase * 1.8 + i as f32 * 0.8).cos() * 0.3 + 0.7;
        let combined_icon_intensity = icon_pulse * icon_rotation;
        
        let icon_base = 220;
        let icon_intensity = (icon_base as f32 * combined_icon_intensity) as u8;
        
        // Dynamic icon colors with hue shifting
        let hue_cycle = (pulse_phase * 2.0 + i as f32 * 0.7).sin();
        queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
            r: (icon_intensity as f32 * (1.0 + hue_cycle * 0.25)) as u8,
            g: (icon_intensity as f32 * (0.95 + hue_cycle * 0.15)) as u8,
            b: (icon_intensity as f32 * (1.1 + hue_cycle * 0.2)) as u8,
        }))?;
        queue!(stdout, Print(icon))?;
        queue!(stdout, Print(" "))?;
        
        // Animated text with character-by-character wave effects
        for (char_i, ch) in text.chars().enumerate() {
            let char_pulse = (pulse_phase * 2.2 + char_i as f32 * 0.4 + i as f32 * 0.6).sin() * 0.2 + 0.8;
            let char_wave = (pulse_phase * 1.5 + char_i as f32 * 0.2).cos() * 0.15 + 0.85;
            let combined_char_intensity = char_pulse * char_wave;
            
            let text_base = 200;
            let char_intensity = (text_base as f32 * combined_char_intensity) as u8;
            
            // Subtle color variation per character
            let char_hue = (char_i as f32 * 0.1 + pulse_phase * 0.8).sin() * 0.1;
            queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                r: char_intensity,
                g: char_intensity,
                b: (char_intensity as f32 * (1.2 + char_hue)).min(255.0) as u8,
            }))?;
            
            queue!(stdout, Print(ch))?;
        }
    }
    
    Ok(())
}

/// Enhanced screensaver mode with smoother animations
async fn screensaver_mode() -> Result<()> {
    let mut stdout = io::stdout();
    let (term_width, term_height) = size()?;
    let theme = theme_global_config::current_theme();
    
    // Enhanced screensaver with slower, smoother animations
    let start_time = Instant::now();
    let mut frame_count = 0u64;
    
    // Display screensaver mode message briefly
    queue!(stdout, MoveTo(0, term_height - 1))?;
    queue!(stdout, SetForegroundColor(Color::Yellow))?;
    queue!(stdout, Print("SCREENSAVER MODE - Press any key to continue or Esc to exit"))?;
    stdout.flush()?;
    
    std::thread::sleep(Duration::from_millis(1500));
    
    loop {
        // Calculate slower, smoother animation phase for screensaver
        let elapsed = start_time.elapsed().as_millis() as f32 / 2500.0; // Even slower
        let pulse_phase = (elapsed * 0.8).sin() * 0.5 + 0.5; // Gentler pulse
        let wave_phase = elapsed * 0.6; // Slower wave
        
        // Clear and redraw living screensaver
        queue!(stdout, MoveTo(0, 0))?;
        
        // Even more subtle background for screensaver
        draw_screensaver_background(&mut stdout, term_width, term_height, pulse_phase, wave_phase)?;
        
        // Gentle breathing logo
        draw_breathing_logo(&mut stdout, term_width, term_height, &theme, pulse_phase)?;
        
        // Minimal living status for screensaver
        draw_screensaver_status(&mut stdout, term_width, term_height, &theme, wave_phase, frame_count)?;
        
        stdout.flush()?;
        frame_count += 1;
        
        // Check for input
        if event::poll(Duration::from_millis(80))? {  // Slower polling for screensaver
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.kind != KeyEventKind::Press {
                        continue;
                    }
                    
                    match key_event.code {
                        KeyCode::Esc => {
                            queue!(stdout, ResetColor, MoveTo(0, term_height - 1))?;
                            stdout.flush()?;
                            println!("\nGoodbye!");
                            std::process::exit(0);
                        }
                        _ => {
                            // Any other key - exit screensaver mode
                            break Ok(());
                        }
                    }
                }
                _ => {
                    // Ignore other events
                }
            }
        }
        
        // Slower frame rate for screensaver (~12 FPS)
        std::thread::sleep(Duration::from_millis(80));
    }
}

/// Draw living screensaver background with rich animations
fn draw_screensaver_background(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    pulse_phase: f32,
    wave_phase: f32,
) -> Result<()> {
    // More vibrant screensaver colors (with overflow protection)
    let base_blue = (6.0 + (pulse_phase * 6.0).sin().abs() * 6.0).min(255.0) as u8; // Safe 6-12 range
    let accent_purple = (4.0 + (wave_phase * 1.8 + pulse_phase * 0.5).sin().abs() * 4.0).min(255.0) as u8; // Safe 4-8 range  
    let accent_cyan = (3.0 + (wave_phase * 2.5).cos().abs() * 3.0).min(255.0) as u8; // Safe 3-6 range
    
    // Rich variety of screensaver animation characters
    let bg_chars = ['˙', '·', '∘', '◦', '⋅', '‧', '∴', '∵'];
    
    for row in 0..height {
        queue!(stdout, MoveTo(0, row))?;
        
        for col in 0..width {
            // Complex overlapping wave patterns for mesmerizing effect
            let primary_wave = (col as f32 * 0.05 + wave_phase * 0.8).sin() 
                * (row as f32 * 0.03 + wave_phase * 0.6).cos();
            let secondary_wave = (col as f32 * 0.08 + wave_phase * 0.4).cos() 
                * (row as f32 * 0.025 + wave_phase * 0.9).sin();
            let tertiary_wave = (col as f32 * 0.02 + wave_phase * 1.2).sin() 
                * (row as f32 * 0.01 + wave_phase * 0.3).cos();
            
            let combined_wave = (primary_wave + secondary_wave * 0.7 + tertiary_wave * 0.4) / 3.0;
            let wave_intensity = combined_wave * 0.2 + 0.8;
            
            // More frequent animated characters in screensaver
            let char_trigger = ((col as f32 * 0.15 + wave_phase * 1.5).sin() 
                              * (row as f32 * 0.12 + wave_phase * 1.8).cos() 
                              + (pulse_phase * 3.0).sin() * 0.4 + 1.0) / 2.0;
            
            if char_trigger > 0.85 { // 15% of cells get animated chars
                let char_phase = (col + row) as f32 * 0.5 + wave_phase * 1.8 + pulse_phase * 1.2;
                let char_index = char_phase.sin().abs();
                let char_idx = (char_index * bg_chars.len() as f32) as usize % bg_chars.len();
                
                // More vibrant animated characters for screensaver with overflow protection
                let char_intensity = ((pulse_phase * 50.0 + wave_phase * 30.0 + 25.0).max(0.0).min(255.0)) as u8;
                let char_hue_shift = (char_phase * 0.7).cos();
                
                queue!(stdout, crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                    r: ((char_intensity as f32 * (1.0 + char_hue_shift * 0.4)).max(0.0).min(255.0)) as u8,
                    g: ((char_intensity as f32 * (1.0 + char_hue_shift * 0.2)).max(0.0).min(255.0)) as u8,
                    b: char_intensity.saturating_add(20),
                }))?;
                queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Reset))?;
                queue!(stdout, Print(bg_chars[char_idx]))?;
            } else {
                // More dynamic screensaver background colors
                let mixed_intensity = wave_intensity * 0.8 + 0.2;
                let color_selector = (col + row) as f32 * 0.12 + wave_phase * 0.8;
                
                if color_selector.sin() > 0.5 { // Purple zones
                    let bg_color = (accent_purple as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: bg_color / 2, 
                        g: 0, 
                        b: bg_color 
                    }))?;
                } else if color_selector.cos() > 0.2 { // Cyan zones
                    let bg_color = (accent_cyan as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: 0, 
                        g: bg_color, 
                        b: bg_color.saturating_add(8) 
                    }))?;
                } else { // Blue zones
                    let bg_color = (base_blue as f32 * mixed_intensity) as u8;
                    queue!(stdout, crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { 
                        r: 0, 
                        g: bg_color / 8, 
                        b: bg_color 
                    }))?;
                }
                queue!(stdout, Print(" "))?;
            }
        }
    }
    
    Ok(())
}

/// Draw minimal status for screensaver
fn draw_screensaver_status(
    stdout: &mut io::Stdout,
    width: u16,
    height: u16,
    _theme: &crate::theme::Theme,
    wave_phase: f32,
    _frame_count: u64,
) -> Result<()> {
    let status_row = (height / 2) + 8;
    
    // Just a gentle "STANDBY" indicator
    let status_text = "● STANDBY ●";
    let col = (width / 2).saturating_sub(status_text.len() as u16 / 2);
    
    queue!(stdout, MoveTo(col, status_row))?;
    
    // Very gentle pulsing
    let pulse = (wave_phase * 1.2).sin() * 0.3 + 0.7;
    let color_intensity = (pulse * 80.0) as u8 + 40; // Very dim
    
    queue!(stdout, SetForegroundColor(Color::Rgb {
        r: color_intensity,
        g: color_intensity,
        b: color_intensity + 30
    }))?;
    
    queue!(stdout, Print(status_text))?;
    
    Ok(())
}