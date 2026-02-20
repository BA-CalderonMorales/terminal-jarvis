// Package tools wraps terminal-jarvis subcommands for use by the LLM agent.
package tools

import (
	"bytes"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// FindBinary locates the terminal-jarvis binary.
//
// Search order:
//  1. PATH (globally installed)
//  2. <project root>/target/release/terminal-jarvis
//  3. <project root>/target/debug/terminal-jarvis
//  4. CARGO_HOME/bin/terminal-jarvis
func FindBinary() string {
	if found, err := exec.LookPath("terminal-jarvis"); err == nil {
		return found
	}

	// adk/ lives inside the project repo. Walk upward and probe for local builds.
	self, _ := filepath.Abs(os.Args[0])
	dir := filepath.Dir(self)
	var candidates []string
	for i := 0; i < 8; i++ {
		candidates = append(candidates,
			filepath.Join(dir, "target", "release", "terminal-jarvis"),
			filepath.Join(dir, "target", "debug", "terminal-jarvis"),
		)
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}

	cargoHome := os.Getenv("CARGO_HOME")
	if cargoHome == "" {
		if home, err := os.UserHomeDir(); err == nil {
			cargoHome = filepath.Join(home, ".cargo")
		}
	}
	if cargoHome != "" {
		candidates = append(candidates, filepath.Join(cargoHome, "bin", "terminal-jarvis"))
	}

	for _, c := range candidates {
		if _, err := os.Stat(c); err == nil {
			return c
		}
	}

	return "terminal-jarvis" // surfaces a clear error on exec failure
}

// Run executes terminal-jarvis with args and returns captured stdout+stderr.
func Run(args ...string) string {
	binary := FindBinary()
	cmd := exec.Command(binary, args...)
	var out bytes.Buffer
	cmd.Stdout = &out
	cmd.Stderr = &out
	if err := cmd.Run(); err != nil {
		if out.Len() == 0 {
			if strings.Contains(err.Error(), "executable file not found") {
				return "terminal-jarvis binary not found. Install: cargo install terminal-jarvis"
			}
			return err.Error()
		}
	}
	result := strings.TrimSpace(out.String())
	if result == "" {
		return "(no output)"
	}
	return result
}

// Launch runs terminal-jarvis interactively (tool owns the terminal).
// Returns after the user exits the launched tool.
func Launch(toolName string) string {
	binary := FindBinary()
	// Use direct invocation path (<tool>) to mirror `cargo run <tool>`
	// behavior in the Rust CLI (external subcommand forwarding).
	cmd := exec.Command(binary, toolName)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		return "Tool session ended with error: " + err.Error()
	}
	return "Returned from " + toolName + ". Back in Terminal Jarvis home."
}
