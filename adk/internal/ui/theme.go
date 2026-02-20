// Package ui provides terminal rendering utilities for Terminal Jarvis.
package ui

// ANSI colour constants — mirrors src/theme/theme_config.rs TJarvis theme.
const (
	Cyan   = "\033[38;2;0;255;255m"
	BoldW  = "\033[1;38;2;255;255;255m"
	LightB = "\033[38;2;200;230;255m"
	Dim    = "\033[2;38;2;120;140;160m"
	ThinkG = "\033[38;2;130;145;160m"
	Green  = "\033[38;2;0;255;150m"
	Red    = "\033[38;2;255;100;100m"
	Reset  = "\033[0m"

	Version = "v0.0.77"
)
