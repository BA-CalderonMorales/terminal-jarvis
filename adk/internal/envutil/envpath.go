// Package envutil provides shared environment-related utilities for the ADK.
package envutil

import (
	"fmt"
	"os"
	"path/filepath"
)

// FindEnvPath locates the adk/.env file relative to the running binary.
// It walks up from the binary's directory looking for adk/.env or .env,
// then falls back to the current working directory.
func FindEnvPath() string {
	exe, err := os.Executable()
	if err != nil {
		return "adk/.env"
	}

	dir := filepath.Dir(exe)
	for i := 0; i < 5; i++ {
		// Check <dir>/adk/.env (when binary is in project root or subdir)
		candidate := filepath.Join(dir, "adk", ".env")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		// Check <dir>/.env (when binary is inside adk/ itself)
		candidate = filepath.Join(dir, ".env")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		// Check <dir>/../.env (sibling of adk/)
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

	// Fallback: look in current working directory
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
