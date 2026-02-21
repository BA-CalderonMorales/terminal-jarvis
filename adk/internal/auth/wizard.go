// Package auth provides interactive provider authentication flows.
package auth

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

// RunWizard runs the interactive provider setup wizard.
// Returns true when the caller should retry provider detection immediately.
func RunWizard(envPath string) bool {
	return runWizard(envPath, nil)
}

// RunWizardWithPrompt runs the setup wizard using a shared prompt function.
// Use this from REPL flows to avoid mixing multiple stdin readers.
func RunWizardWithPrompt(envPath string, promptFn TextPrompt) bool {
	return runWizard(envPath, promptFn)
}

func runWizard(envPath string, promptFn TextPrompt) bool {
	if envPath == "" {
		envPath = defaultEnvPath()
	}

	fmt.Println()
	fmt.Printf("   %s\u250c\u2500\u2500\u2500\u2500\u2500\u2510%s  %sTerminal Jarvis -- Provider Setup%s\n", ui.Cyan, ui.Reset, ui.BoldW, ui.Reset)
	fmt.Printf("   %s\u2502 T.J \u2502%s  %sNo provider configured. Let's fix that.%s\n", ui.Cyan, ui.Reset, ui.LightB, ui.Reset)
	fmt.Printf("   %s\u2514\u2500\u2500\u2500\u2500\u2500\u2518%s\n", ui.Cyan, ui.Reset)
	fmt.Println()
	fmt.Printf("   %sWhich provider do you want to use?%s\n", ui.LightB, ui.Reset)
	fmt.Println()
	fmt.Printf("   %s1. %sGoogle Gemini%s  %s-- recommended, free tier, browser-guided key creation%s\n", ui.Cyan, ui.BoldW, ui.Reset, ui.Dim, ui.Reset)
	fmt.Printf("   %s2. %sOpenRouter%s     %s-- 100+ models, paste API key from browser%s\n", ui.Cyan, ui.BoldW, ui.Reset, ui.Dim, ui.Reset)
	fmt.Printf("   %s3. %sOllama%s         %s-- local, no API key required, prints setup instructions%s\n", ui.Cyan, ui.BoldW, ui.Reset, ui.Dim, ui.Reset)
	fmt.Printf("   %s4. %sSkip%s           %s-- I'll edit %s manually%s\n", ui.Cyan, ui.BoldW, ui.Reset, ui.Dim, envPath, ui.Reset)
	reader := bufio.NewReader(os.Stdin)
	choice, _ := readInput(reader, promptFn, "   > ")
	choice = strings.TrimSpace(choice)

	switch choice {
	case "1":
		key := setupGoogleWithPrompt(envPath, promptFn)
		if key != "" {
			os.Setenv("GOOGLE_API_KEY", key)
			if _, err := ActivateProvider(envPath, "gemini"); err != nil {
				fmt.Printf("   %sWarning: could not set active provider: %v%s\n", ui.LightB, err, ui.Reset)
			}
			fmt.Println()
			fmt.Printf("   %sGOOGLE_API_KEY saved to .env%s\n", ui.Green, ui.Reset)
			fmt.Printf("   %sActive provider set to Google Gemini.%s\n\n", ui.Green, ui.Reset)
			return true
		}
	case "2":
		key := setupOpenRouterWithPrompt(envPath, promptFn)
		if key != "" {
			os.Setenv("OPENROUTER_API_KEY", key)
			if _, err := ActivateProvider(envPath, "openrouter"); err != nil {
				fmt.Printf("   %sWarning: could not set active provider: %v%s\n", ui.LightB, err, ui.Reset)
			}
			fmt.Println()
			fmt.Printf("   %sOPENROUTER_API_KEY saved to .env%s\n", ui.Green, ui.Reset)
			fmt.Printf("   %sActive provider set to OpenRouter.%s\n\n", ui.Green, ui.Reset)
			return true
		}
	case "3":
		SetupOllama()
		if _, err := ActivateProvider(envPath, "ollama"); err != nil {
			fmt.Printf("   %sWarning: could not set active provider: %v%s\n", ui.LightB, err, ui.Reset)
		}
		host := os.Getenv("OLLAMA_HOST")
		if host == "" {
			host = "http://localhost:11434"
		}
		if providers.OllamaReachable(host) {
			fmt.Printf("   %sOllama detected at %s. Active provider set to Ollama.%s\n\n", ui.Green, host, ui.Reset)
			return true
		}
		fmt.Printf("   %sOllama is not reachable yet. Start it, then run /setup again.%s\n\n", ui.LightB, ui.Reset)
	default:
		fmt.Println()
		fmt.Printf("   %sSkipped. Edit %s%s%s and run /setup anytime to retry.%s\n\n",
			ui.LightB, ui.Cyan, envPath, ui.LightB, ui.Reset)
	}
	return false
}

func defaultEnvPath() string {
	exe, _ := os.Executable()
	// Walk up from binary location to find adk/.env
	dir := exe
	for i := 0; i < 5; i++ {
		dir = strings.TrimRight(dir, "/"+lastSegment(dir))
		candidate := dir + "/adk/.env"
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
	}
	return "adk/.env"
}

func lastSegment(path string) string {
	parts := strings.Split(path, "/")
	if len(parts) == 0 {
		return ""
	}
	return parts[len(parts)-1]
}
