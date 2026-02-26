package repl

import (
	"bufio"
	"context"
	"fmt"
	"io"
	"os"
	"strings"
	"time"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/chat"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
)

// IsHeadless returns true when running in non-interactive mode.
// Checks JARVIS_HEADLESS env var or if stdin is not a terminal.
func IsHeadless() bool {
	if os.Getenv("JARVIS_HEADLESS") == "1" || os.Getenv("JARVIS_HEADLESS") == "true" {
		return true
	}
	fi, err := os.Stdin.Stat()
	if err != nil {
		return false
	}
	// If stdin is a pipe or file (not a char device), we're headless
	return fi.Mode()&os.ModeCharDevice == 0
}

// runHeadless is the non-interactive REPL for piped/scripted input.
// No liner, no spinner, no ANSI codes. Reads lines from stdin, sends to LLM.
func runHeadless(chain []providers.Provider) {
	if len(chain) == 0 {
		fmt.Fprintln(os.Stderr, "[ERROR] No providers configured. Run setup first.")
		os.Exit(1)
	}

	currentProvider := chain[0]
	session := chat.NewSession(chat.SystemPrompt)
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		input := strings.TrimSpace(scanner.Text())
		if input == "" {
			continue
		}

		// Handle slash commands minimally
		if strings.HasPrefix(input, "/") {
			lower := strings.ToLower(input)
			if lower == "/exit" || lower == "/quit" {
				return
			}
			if lower == "/help" {
				fmt.Println("Commands: /exit, /quit, /help")
				continue
			}
			fmt.Fprintf(os.Stderr, "[WARN] Unknown command: %s\n", input)
			continue
		}

		// Send to LLM
		ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
		reply, err := chat.Send(ctx, session, currentProvider, input)
		cancel()

		if err != nil {
			fmt.Fprintf(os.Stderr, "[ERROR] %v\n", err)
			continue
		}

		fmt.Println(reply)
	}

	if err := scanner.Err(); err != nil && err != io.EOF {
		fmt.Fprintf(os.Stderr, "[ERROR] reading stdin: %v\n", err)
	}
}
