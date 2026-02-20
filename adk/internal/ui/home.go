package ui

import (
	"fmt"
	"os"
	"path/filepath"
)

// Clear clears the terminal screen.
func Clear() {
	fmt.Print("\033[2J\033[H")
}

// PrintHome renders the home screen matching the Rust TUI layout.
func PrintHome(providerLabel string) {
	cwd, _ := os.Getwd()
	// Use the binary's working directory if cwd is empty.
	if cwd == "" {
		cwd, _ = filepath.Abs(".")
	}

	Clear()

	fmt.Printf("%s   \u250c\u2500\u2500\u2500\u2500\u2500\u2510%s  %sTerminal Jarvis%s\n", Cyan, Reset, BoldW, Reset)
	fmt.Printf("%s   \u2502 T.J \u2502%s  %s%s%s\n", Cyan, Reset, LightB, Version, Reset)
	fmt.Printf("%s   \u2502 \u2550 \u2550 \u2502%s  %sProvider: %s%s\n", Cyan, Reset, LightB, providerLabel, Reset)
	fmt.Printf("%s   \u2502     \u2502%s  %s%s%s\n", Cyan, Reset, LightB, cwd, Reset)
	fmt.Printf("%s   \u2514\u2500\u2500\u2500\u2500\u2500\u2518%s  %sType /help to see available commands%s\n", Cyan, Reset, Cyan, Reset)
	fmt.Println()
	fmt.Printf("   %sOr describe what you want in plain English.%s\n", LightB, Reset)
	fmt.Println()
}

// PrintAuthGuide prints a clear provider setup guide when authentication fails.
func PrintAuthGuide(failedLabel string) {
	if failedLabel != "" {
		fmt.Printf("\n   %s[auth failed]%s %s%s rejected the API key.%s\n", Cyan, Reset, LightB, failedLabel, Reset)
	}
	fmt.Println()
	fmt.Printf("   %sChoose a provider and add it to:%s %sadk/.env%s\n", BoldW, Reset, LightB, Reset)
	fmt.Println()
	fmt.Printf("   %s►%s %sOption 1 — Google Gemini%s  %s(recommended · free tier available)%s\n", Cyan, Reset, BoldW, Reset, Dim, Reset)
	fmt.Printf("   %s    GOOGLE_API_KEY=your-key-here%s\n", LightB, Reset)
	fmt.Printf("   %s    https://aistudio.google.com/app/apikey%s\n", Dim, Reset)
	fmt.Println()
	fmt.Printf("   %s►%s %sOption 2 — OpenRouter%s  %s(100+ cloud models)%s\n", Cyan, Reset, BoldW, Reset, Dim, Reset)
	fmt.Printf("   %s    OPENROUTER_API_KEY=your-key-here%s\n", LightB, Reset)
	fmt.Printf("   %s    https://openrouter.ai/keys%s\n", Dim, Reset)
	fmt.Println()
	fmt.Printf("   %s►%s %sOption 3 — Ollama%s  %s(local · no API key required)%s\n", Cyan, Reset, BoldW, Reset, Dim, Reset)
	fmt.Printf("   %s    Install: https://ollama.com/download%s\n", Dim, Reset)
	fmt.Printf("   %s    Then:    ollama pull llama3.2%s\n", Dim, Reset)
	fmt.Println()
	fmt.Printf("   %sRun /setup now (no restart needed), or edit adk/.env manually.%s\n\n", LightB, Reset)
}
