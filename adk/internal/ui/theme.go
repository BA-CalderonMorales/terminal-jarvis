// Package ui provides terminal rendering utilities for Terminal Jarvis.
package ui

// ANSI colour constants — mirrors src/theme/theme_config.rs TJarvis theme.
// Using \x1b (hex) instead of \033 (octal) for better cross-platform compatibility.
const (
	Cyan   = "\x1b[38;2;0;255;255m"
	BoldW  = "\x1b[1;38;2;255;255;255m"
	LightB = "\x1b[38;2;200;230;255m"
	Dim    = "\x1b[2;38;2;120;140;160m"
	ThinkG = "\x1b[38;2;130;145;160m"
	Green  = "\x1b[38;2;0;255;150m"
	Red    = "\x1b[38;2;255;100;100m"
	Reset  = "\x1b[0m"

	Version = "v0.0.78"
)
