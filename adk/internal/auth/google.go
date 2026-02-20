package auth

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

const googleKeyURL = "https://aistudio.google.com/app/apikey"

// SetupGoogle opens AI Studio in the browser and prompts for a key paste.
// Returns the key if obtained, or "" if skipped.
func SetupGoogle(envPath string) string {
	return setupGoogleWithPrompt(envPath, nil)
}

func setupGoogleWithPrompt(envPath string, promptFn TextPrompt) string {
	fmt.Println()
	fmt.Printf("   %s► %sGoogle Gemini Setup%s\n", ui.Cyan, ui.BoldW, ui.Reset)
	fmt.Printf("   %sFree tier available -- no credit card required.%s\n", ui.Dim, ui.Reset)
	fmt.Println()
	fmt.Printf("   %sOpening Google AI Studio in your browser...%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s%s%s\n", ui.Dim, googleKeyURL, ui.Reset)
	fmt.Println()

	openBrowser(googleKeyURL)

	fmt.Printf("   %sSteps:%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s  1. Sign in with your Google account%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s  2. Click \"Create API key\"%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %s  3. Copy the key and paste it below%s\n", ui.LightB, ui.Reset)
	fmt.Println()
	fmt.Printf("   %sPaste your API key below.%s\n", ui.LightB, ui.Reset)

	reader := bufio.NewReader(os.Stdin)
	raw, _ := readInput(reader, promptFn, "   Paste GOOGLE_API_KEY (Enter to skip): ")
	if raw == "" {
		return ""
	}

	if !strings.HasPrefix(raw, "AIza") {
		fmt.Printf("   %sNote: key doesn't look like a Gemini key (expected prefix 'AIza').%s\n", ui.LightB, ui.Reset)
		ans, _ := readInput(reader, promptFn, "   Save anyway? [y/N] ")
		ans = strings.ToLower(strings.TrimSpace(ans))
		if ans != "y" && ans != "yes" {
			return ""
		}
	}

	writeEnvKey(envPath, "GOOGLE_API_KEY", raw)
	return raw
}
