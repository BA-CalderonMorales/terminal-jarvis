// Windows Platform Voice Provider Implementation
// Uses Windows Speech Recognition via PowerShell
//
// This module provides Windows-specific speech recognition functionality
// using the built-in System.Speech.Recognition engine.

use anyhow::{anyhow, Result};
use std::time::Duration;

/// Listen for speech directly using Windows Speech Recognition (no audio file needed)
pub async fn listen_windows_direct(duration: Duration) -> Result<String> {
    let duration_secs = duration.as_secs();
    
    println!("Listening... ({}s)", duration_secs);
    
    // Use Windows Speech Recognition with better accuracy and fuzzy matching
    let ps_script = format!(
        r#"
Add-Type -AssemblyName System.Speech

$recognizer = New-Object System.Speech.Recognition.SpeechRecognitionEngine
$recognizer.SetInputToDefaultAudioDevice()

# Load dictation grammar for flexibility
$recognizer.LoadGrammar((New-Object System.Speech.Recognition.DictationGrammar))

# Set confidence threshold
$recognizer.UpdateRecognizerSetting("CFGConfidenceRejectionThreshold", 30)

$timeout = {}
$result = $recognizer.Recognize([TimeSpan]::FromSeconds($timeout))

if ($result) {{
    # Get the recognized text
    $text = $result.Text.ToLower().Trim()
    
    # Fuzzy match common misheard words - order matters, check most specific first
    # "help" is commonly misheard as many words
    $text = $text -replace "^he'll$|^hell$|^heel$|^who$|^hope$|^cope$|^cold$|^hold$|^told$|^help$", "help"
    $text = $text -replace "^helps$|^he'll s$", "help"
    
    # Navigation commands
    $text = $text -replace "^open\s+", "open "
    $text = $text -replace "^opened\s+", "open "
    $text = $text -replace "ai\s+tools|a\s+i\s+tools|8\s+tools", "AI tools"
    $text = $text -replace "authentication|authentications", "authentication"
    $text = $text -replace "settings|setting", "settings"
    $text = $text -replace "eval|equals|evaluations", "evals"
    
    # Tool commands  
    $text = $text -replace "^list\s+tools|^list\s+two", "list tools"
    $text = $text -replace "^install\s+|^in\s+stall\s+", "install "
    $text = $text -replace "^update\s+|^up\s+date\s+", "update "
    $text = $text -replace "^remove\s+|^re\s+move\s+", "remove "
    
    # General commands
    $text = $text -replace "^exit$|^exist$|^exits$", "exit"
    $text = $text -replace "^quit$|^quick$|^quite$", "quit"  
    $text = $text -replace "^back$|^bad$|^bat$|^bag$", "back"
    $text = $text -replace "^commands$|^command$", "commands"
    
    $text
}} else {{
    ""
}}
"#,
        duration_secs
    );

    let output = tokio::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_script])
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run Windows Speech Recognition: {}", e))?;

    if !output.status.success() {
        return Err(anyhow!(
            "Windows Speech Recognition failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let transcription = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if transcription.is_empty() {
        return Err(anyhow!("No speech detected"));
    }

    Ok(transcription)
}

/// Check if Windows speech recognition is available
pub async fn is_ready() -> Result<bool> {
    // Check for PowerShell (always available on Windows)
    let ps_check = tokio::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", "exit 0"])
        .status()
        .await;
    Ok(ps_check.is_ok())
}
