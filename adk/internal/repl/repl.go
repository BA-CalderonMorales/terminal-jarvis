package repl

import (
	"context"
	"fmt"
	"io"
	"strings"
	"time"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/auth"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/chat"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
	"github.com/peterh/liner"
)

const llmTimeout = 60 * time.Second
const promptText = "   > "
const exitPromptText = "   Exit Terminal Jarvis? [y/N] "
const setupNowPromptText = "   Run setup wizard now? [Y/n] "

// Run starts the REPL loop.
// chain is the ordered list of providers to try; the first that responds wins.
func Run(chain []providers.Provider, envPath string) {
	if restoreTTY := attachControllingTTY(); restoreTTY != nil {
		defer restoreTTY()
	}

	if envPath == "" {
		envPath = findEnvPath()
	}

	providerIdx := 0
	currentProvider := chain[providerIdx]
	session := chat.NewSession(chat.SystemPrompt)

	ui.PrintHome(currentProvider.Label())

	// liner provides arrow-key history (equivalent to Python's readline).
	line := liner.NewLiner()
	defer line.Close()
	line.SetCtrlCAborts(true)

	for {
		// liner rejects control/ANSI sequences in prompt text.
		input, err := line.Prompt(promptText)
		if err != nil {
			// liner returns liner.ErrPromptAborted on Ctrl-C, io.EOF on Ctrl-D.
			if err == liner.ErrPromptAborted {
				fmt.Printf("\r%-60s\r", "")
				ans, _ := line.Prompt(exitPromptText)
				if strings.ToLower(strings.TrimSpace(ans)) == "y" {
					fmt.Printf("\n   %sGoodbye.%s\n\n", ui.Cyan, ui.Reset)
					return
				}
				continue
			}
			if err == io.EOF {
				fmt.Printf("\n   %sGoodbye.%s\n\n", ui.Cyan, ui.Reset)
				return
			}
			fmt.Printf("\n   %s[ERROR]%s input error: %v\n\n", ui.Cyan, ui.Reset, err)
			continue
		}

		input = strings.TrimSpace(input)
		if input == "" {
			continue
		}
		line.AppendHistory(input)

		if strings.HasPrefix(input, "/") {
			shouldExit, refreshProviders := handleSlash(input, envPath, line)
			if shouldExit {
				return
			}
			if refreshProviders {
				newChain, err := providers.BuildChain()
				if err != nil || len(newChain) == 0 {
					fmt.Printf("\n   %s[setup]%s Provider update saved, but no active provider is ready yet. Run /setup again.\n\n", ui.Cyan, ui.Reset)
				} else {
					chain = newChain
					providerIdx = 0
					currentProvider = chain[providerIdx]
					session = chat.NewSession(chat.SystemPrompt)
					fmt.Printf("\n   %sActive provider switched to %s.%s\n\n", ui.Green, currentProvider.Label(), ui.Reset)
				}
			}
			continue
		}
		if maybeHandleDirectLaunchIntent(input) {
			continue
		}

		// Plain English -- send to LLM with provider fallback.
		replied := false
		for !replied && providerIdx < len(chain) {
			spin := ui.StartThinkingSpinner()

			ctx, cancel := context.WithTimeout(context.Background(), llmTimeout)
			reply, err := chat.Send(ctx, session, currentProvider, input)
			cancel()
			spin.Stop()

			if err == nil {
				fmt.Println()
				ui.PrintResponse(reply)
				replied = true
				continue
			}

			// On error, try the next provider.
			errStr := err.Error()
			nextIdx := providerIdx + 1

			if nextIdx < len(chain) {
				nextLabel := chain[nextIdx].Label()
				if isAuthError(errStr) {
					fmt.Printf("\n   %s[auth]%s %s%s: bad key -- trying %s...%s\n\n",
						ui.Cyan, ui.Reset, ui.LightB, currentProvider.Label(), nextLabel, ui.Reset)
				} else if isTimeout(errStr) {
					fmt.Printf("\n   %s[timeout]%s %s%s took too long -- trying %s...%s\n\n",
						ui.Cyan, ui.Reset, ui.LightB, currentProvider.Label(), nextLabel, ui.Reset)
				} else {
					fmt.Printf("\n   %s[%s failed]%s %sFalling back to %s...%s\n\n",
						ui.Cyan, currentProvider.Label(), ui.Reset, ui.LightB, nextLabel, ui.Reset)
				}
				providerIdx = nextIdx
				currentProvider = chain[providerIdx]
				session = chat.NewSession(chat.SystemPrompt)
			} else {
				if isAuthError(errStr) {
					if runSetupNow(line, envPath) {
						// Rebuild provider chain and retry the same user message.
						newChain, buildErr := providers.BuildChain()
						if buildErr == nil && len(newChain) > 0 {
							chain = newChain
							providerIdx = 0
							currentProvider = chain[providerIdx]
							session = chat.NewSession(chat.SystemPrompt)
							fmt.Printf("\n   %sSetup complete.%s Retrying your request...\n\n", ui.Green, ui.Reset)
							continue
						}
						fmt.Printf("\n   %s[setup]%s Could not load providers after setup. Try /setup again.\n\n", ui.Cyan, ui.Reset)
					}
					ui.PrintAuthGuide(currentProvider.Label())
				} else {
					fmt.Printf("\n   %s[ERROR]%s All providers failed. Last error: %v\n\n",
						ui.Cyan, ui.Reset, err)
				}
				replied = true
			}
		}
	}
}

// RunWizardAndRetry runs the auth wizard and, if a key is obtained, rebuilds
// the provider chain and starts the REPL. Used from main when no provider is configured.
func RunWizardAndRetry(envPath string) {
	configured := auth.RunWizard(envPath)
	if !configured {
		ui.PrintAuthGuide("")
		return
	}
	chain, err := providers.BuildChain()
	if err != nil {
		ui.PrintAuthGuide("")
		return
	}
	Run(chain, envPath)
}

func isAuthError(s string) bool {
	s = strings.ToLower(s)
	for _, sig := range []string{"autenticationerror", "401", "403", "unauthorized",
		"invalid_api_key", "api key not valid", "no auth credentials"} {
		if strings.Contains(s, sig) {
			return true
		}
	}
	return false
}

func isTimeout(s string) bool {
	return strings.Contains(strings.ToLower(s), "context deadline exceeded") ||
		strings.Contains(strings.ToLower(s), "timeout")
}

func runSetupNow(line *liner.State, envPath string) bool {
	fmt.Printf("\n   %sAuthentication failed.%s You can fix it now without restarting.\n", ui.Cyan, ui.Reset)
	ans, err := line.Prompt(setupNowPromptText)
	if err != nil {
		return false
	}
	choice := strings.ToLower(strings.TrimSpace(ans))
	if choice == "n" || choice == "no" {
		return false
	}
	return auth.RunWizardWithPrompt(envPath, line.Prompt)
}
