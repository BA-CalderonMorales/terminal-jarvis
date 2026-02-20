// Terminal Jarvis ADK - Go edition.
//
// Replaces the Python/ADK home screen with a statically-compiled Go binary
// for millisecond startup times. No Python interpreter, no litellm, no venv.
//
// Architecture (DDD layers, mirroring src/ Rust structure):
//   providers/ -- LLM backend detection + API clients (Gemini, OpenRouter, Ollama)
//   tools/     -- terminal-jarvis binary wrappers + LLM tool schemas
//   chat/      -- conversation history + LLM dispatch loop
//   auth/      -- provider auth wizards (PKCE OAuth, browser-guided key paste)
//   repl/      -- interactive REPL + slash command handlers
//   ui/        -- ANSI theme, spinner, home screen, help
package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/joho/godotenv"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/repl"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

func main() {
	// Load .env for the Go home screen.
	envPath := findEnvPath()
	_ = godotenv.Load(envPath)

	// Start the startup spinner immediately -- before any blocking work.
	spin := ui.StartSpinner()

	// Detect providers from environment.
	chain, err := providers.BuildChain()
	spin.Stop()

	if err != nil {
		// No provider configured -- run the interactive setup wizard.
		repl.RunWizardAndRetry(envPath)
		return
	}

	repl.Run(chain, envPath)
}

// findEnvPath locates adk/.env relative to the binary.
func findEnvPath() string {
	exe, err := os.Executable()
	if err != nil {
		return "adk/.env"
	}

	dir := filepath.Dir(exe)
	// The binary typically lives in adk/ inside the project root.
	// Walk up to find adk/.env.
	for i := 0; i < 5; i++ {
		// Check <dir>/adk/.env (when binary is in project root or subdir)
		candidate := filepath.Join(dir, "adk", ".env")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		// Check <dir>/.env (when binary is inside adk/ itself)
		candidate = filepath.Join(dir, "..", ".env")
		if abs, err := filepath.Abs(candidate); err == nil {
			if _, err := os.Stat(abs); err == nil {
				return abs
			}
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}

	// Fallback: look for .env in current working directory.
	if cwd, err := os.Getwd(); err == nil {
		for _, rel := range []string{"adk/.env", ".env"} {
			p := filepath.Join(cwd, rel)
			if _, err := os.Stat(p); err == nil {
				return p
			}
		}
	}

	fmt.Fprintf(os.Stderr, "Warning: could not locate adk/.env; set GOOGLE_API_KEY or OPENROUTER_API_KEY in environment.\n")
	return "adk/.env"
}
