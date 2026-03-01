// Terminal Jarvis ADK - Go edition.
//
// Replaces the Python/ADK home screen with a statically-compiled Go binary
// for millisecond startup times. No Python interpreter, no litellm, no venv.
//
// Architecture (DDD layers, mirroring src/ Rust structure):
//
//	providers/ -- LLM backend detection + API clients (Gemini, OpenRouter, Ollama)
//	tools/     -- terminal-jarvis binary wrappers + LLM tool schemas
//	chat/      -- conversation history + LLM dispatch loop
//	auth/      -- provider auth wizards (PKCE OAuth, browser-guided key paste)
//	repl/      -- interactive REPL + slash command handlers
//	ui/        -- ANSI theme, spinner, home screen, help
package main

import (
	"github.com/joho/godotenv"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/envutil"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/repl"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

func main() {
	// Load .env for the Go home screen.
	envPath := envutil.FindEnvPath()
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
