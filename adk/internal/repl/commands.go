// Package repl implements the interactive REPL loop and slash command handlers.
package repl

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/auth"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/tools"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
	"github.com/peterh/liner"
)

// handleSlash dispatches a "/" command without involving the LLM.
// Returns (exit, refreshProviders).
func handleSlash(input string, envPath string, replLine *liner.State) (exit bool, refreshProviders bool) {
	parts := strings.Fields(input)
	if len(parts) == 0 {
		return false, false
	}
	cmd := strings.ToLower(parts[0])
	rest := parts[1:]

	switch cmd {
	case "/exit", "/quit":
		fmt.Printf("\n   %sGoodbye.%s\n\n", ui.Cyan, ui.Reset)
		os.Exit(0)

	case "/help":
		ui.PrintHelp()

	case "/tools":
		fmt.Println(tools.Run("list"))

	case "/status":
		fmt.Println(tools.Run("status"))

	case "/config":
		fmt.Println(tools.Run("config", "show"))

	case "/install":
		if len(rest) > 0 {
			fmt.Println(tools.Run(append([]string{"install"}, rest...)...))
		} else {
			fmt.Printf("   %sUsage: /install <tool-name>%s\n", ui.LightB, ui.Reset)
		}

	case "/update":
		if len(rest) > 0 {
			fmt.Println(tools.Run(append([]string{"update"}, rest...)...))
		} else {
			fmt.Println(tools.Run("update"))
		}

	case "/auth":
		if len(rest) > 0 {
			fmt.Println(tools.Run(append([]string{"auth", "help"}, rest...)...))
		} else {
			fmt.Println(tools.Run("auth", "manage"))
		}

	case "/setup":
		var configured bool
		if replLine != nil {
			configured = auth.RunWizardWithPrompt(envPath, replLine.Prompt)
		} else {
			configured = auth.RunWizard(envPath)
		}
		return false, configured

	case "/logout":
		target := ""
		if len(rest) > 0 {
			target = rest[0]
		}
		provider, err := auth.LogoutProvider(envPath, target)
		if err != nil {
			fmt.Printf("   %sCould not log out provider: %v%s\n", ui.LightB, err, ui.Reset)
		} else {
			fmt.Printf("   %sLogged out %s credentials. Run /setup to switch providers.%s\n", ui.Green, provider, ui.Reset)
		}
		return false, true

	default:
		fmt.Printf("   %sUnknown command '%s'. Type /help for options.%s\n", ui.LightB, cmd, ui.Reset)
	}
	return false, false
}

// findEnvPath resolves the adk/.env path relative to the binary location.
func findEnvPath() string {
	exe, err := os.Executable()
	if err != nil {
		return "adk/.env"
	}
	// adk binary is at <repo>/adk/jarvis (or similar)
	// Walk up to find adk/.env
	dir := filepath.Dir(exe)
	for i := 0; i < 5; i++ {
		candidate := filepath.Join(dir, "adk", ".env")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		candidate = filepath.Join(dir, ".env")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	return "adk/.env"
}
