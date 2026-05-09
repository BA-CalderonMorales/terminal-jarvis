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
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/tools"
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
			action := parseHeadlessSlash(input)
			if action.exit {
				return
			}
			if action.stdout != "" {
				fmt.Println(action.stdout)
				continue
			}
			if action.stderr != "" {
				fmt.Fprintln(os.Stderr, action.stderr)
				continue
			}
			if len(action.toolArgs) > 0 {
				fmt.Println(tools.Run(action.toolArgs...))
				continue
			}
			fmt.Fprintf(os.Stderr, "[WARN] Unknown command: %s\n", strings.TrimSpace(input))
			continue
		}

		// Send to LLM
		ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
		reply, _, err := chat.Send(ctx, session, currentProvider, input)
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

type headlessSlashAction struct {
	exit     bool
	stdout   string
	stderr   string
	toolArgs []string
}

func parseHeadlessSlash(input string) headlessSlashAction {
	fields := strings.Fields(strings.TrimSpace(input))
	if len(fields) == 0 {
		return headlessSlashAction{stderr: "[WARN] Empty command"}
	}

	cmd := strings.ToLower(fields[0])
	rest := fields[1:]

	switch cmd {
	case "/exit", "/quit":
		return headlessSlashAction{exit: true}
	case "/help":
		return headlessSlashAction{
			stdout: "Commands: /exit, /quit, /help, /tools, /status, /config, /auth [tool], /install <tool>, /update [tool]",
		}
	case "/tools":
		return headlessSlashAction{toolArgs: []string{"list"}}
	case "/status":
		return headlessSlashAction{toolArgs: []string{"status"}}
	case "/config":
		return headlessSlashAction{toolArgs: []string{"config", "show"}}
	case "/auth":
		if len(rest) > 0 {
			return headlessSlashAction{toolArgs: append([]string{"auth", "help"}, rest...)}
		}
		return headlessSlashAction{toolArgs: []string{"auth", "manage"}}
	case "/install":
		if len(rest) == 0 {
			return headlessSlashAction{stderr: "[WARN] Usage: /install <tool-name>"}
		}
		return headlessSlashAction{toolArgs: append([]string{"install"}, rest...)}
	case "/update":
		if len(rest) == 0 {
			return headlessSlashAction{toolArgs: []string{"update"}}
		}
		return headlessSlashAction{toolArgs: append([]string{"update"}, rest...)}
	default:
		return headlessSlashAction{}
	}
}
