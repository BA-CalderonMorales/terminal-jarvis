// Linux Platform Voice Provider Implementation
// Uses ALSA/PulseAudio for audio capture + Whisper for transcription
//
// This module provides Linux-specific speech recognition functionality
// by capturing audio and transcribing it using available Whisper providers.

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;

/// Capture audio on Linux using available audio tools
pub async fn capture_audio(duration: Duration, output_path: &PathBuf) -> Result<()> {
    let duration_secs = duration.as_secs();

    println!("Listening... ({}s)", duration_secs);

    // Try arecord (ALSA) first - most common on Linux
    if is_command_available("arecord").await {
        return capture_with_arecord(duration_secs, output_path).await;
    }

    // Try parecord (PulseAudio) as fallback
    if is_command_available("parecord").await {
        return capture_with_parecord(duration_secs, output_path).await;
    }

    // Try ffmpeg as another fallback
    if is_command_available("ffmpeg").await {
        return capture_with_ffmpeg(duration_secs, output_path).await;
    }

    Err(anyhow!(
        "No audio capture tool found on Linux.\n\
         Please install one of the following:\n\
         - ALSA: sudo apt-get install alsa-utils\n\
         - PulseAudio: sudo apt-get install pulseaudio-utils\n\
         - FFmpeg: sudo apt-get install ffmpeg"
    ))
}

/// Capture audio using arecord (ALSA)
async fn capture_with_arecord(duration_secs: u64, output_path: &PathBuf) -> Result<()> {
    let output = tokio::process::Command::new("arecord")
        .args(&[
            "-f",
            "S16_LE", // 16-bit signed little-endian
            "-c",
            "1", // Mono
            "-r",
            "16000", // 16kHz sample rate (good for speech)
            "-d",
            &duration_secs.to_string(),
            output_path.to_str().unwrap(),
        ])
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run arecord: {}", e))?;

    if !output.status.success() {
        return Err(anyhow!(
            "arecord failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Capture audio using parecord (PulseAudio)
async fn capture_with_parecord(duration_secs: u64, output_path: &PathBuf) -> Result<()> {
    // parecord doesn't have a built-in duration limit, so we'll use timeout
    let output = tokio::process::Command::new("timeout")
        .args(&[
            &format!("{}s", duration_secs),
            "parecord",
            "--format=s16le",
            "--rate=16000",
            "--channels=1",
            output_path.to_str().unwrap(),
        ])
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run parecord: {}", e))?;

    // timeout returns 124 when it times out (expected behavior)
    if !output.status.success() && output.status.code() != Some(124) {
        return Err(anyhow!(
            "parecord failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Capture audio using ffmpeg
async fn capture_with_ffmpeg(duration_secs: u64, output_path: &PathBuf) -> Result<()> {
    let output = tokio::process::Command::new("ffmpeg")
        .args(&[
            "-f",
            "pulse", // Use PulseAudio
            "-i",
            "default", // Default audio input
            "-t",
            &duration_secs.to_string(),
            "-ar",
            "16000", // 16kHz sample rate
            "-ac",
            "1",  // Mono
            "-y", // Overwrite output file
            output_path.to_str().unwrap(),
        ])
        .output()
        .await
        .map_err(|e| anyhow!("Failed to run ffmpeg: {}", e))?;

    if !output.status.success() {
        return Err(anyhow!(
            "ffmpeg failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Check if a command is available on the system
async fn is_command_available(command: &str) -> bool {
    tokio::process::Command::new("which")
        .arg(command)
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if Linux audio capture is available
pub async fn is_ready() -> Result<bool> {
    // Check if any audio capture tool is available
    Ok(is_command_available("arecord").await
        || is_command_available("parecord").await
        || is_command_available("ffmpeg").await)
}

/// Cleanup temporary audio file
pub async fn cleanup_audio_file(path: &PathBuf) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).await?;
    }
    Ok(())
}
