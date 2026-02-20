package providers

import (
	"fmt"
	"os"
	"strings"
)

// BuildChain returns a priority-ordered list of configured providers.
//
// Priority (mirrors Python providers.py):
//  1. JARVIS_MODEL env var — if set, use it exclusively (no litellm required
//     in Go; we handle openrouter/... and gemini/... natively).
//  2. GOOGLE_API_KEY / GEMINI_API_KEY → gemini-2.0-flash
//  3. OPENROUTER_API_KEY → openrouter/google/gemini-flash-1.5
//  4. Ollama reachable at localhost → ollama/llama3.2
func BuildChain() ([]Provider, error) {
	// Explicit model override.
	if explicit := os.Getenv("JARVIS_MODEL"); explicit != "" {
		return buildExplicit(explicit)
	}

	var chain []Provider

	// Google Gemini
	if key := firstOf("GOOGLE_API_KEY", "GEMINI_API_KEY"); key != "" {
		p, err := NewGemini(key, "gemini-2.0-flash")
		if err == nil {
			chain = append(chain, p)
		}
	}

	// OpenRouter
	if key := os.Getenv("OPENROUTER_API_KEY"); key != "" {
		chain = append(chain, NewOpenRouter(key, "google/gemini-flash-1.5"))
	}

	// Ollama (local)
	ollamaHost := os.Getenv("OLLAMA_HOST")
	if ollamaHost == "" {
		ollamaHost = "http://localhost:11434"
	}
	if OllamaReachable(ollamaHost) {
		chain = append(chain, NewOllama(ollamaHost, "llama3.2"))
	}

	if len(chain) == 0 {
		return nil, fmt.Errorf("no provider configured — set GOOGLE_API_KEY, OPENROUTER_API_KEY, or start Ollama")
	}

	return chain, nil
}

// buildExplicit constructs a chain for a single JARVIS_MODEL value.
func buildExplicit(model string) ([]Provider, error) {
	lower := strings.ToLower(model)

	switch {
	case strings.HasPrefix(lower, "openrouter/"):
		key := os.Getenv("OPENROUTER_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("JARVIS_MODEL=%q requires OPENROUTER_API_KEY", model)
		}
		// Strip the "openrouter/" prefix — NewOpenRouter handles it.
		return []Provider{NewOpenRouter(key, strings.TrimPrefix(model, "openrouter/"))}, nil

	case strings.HasPrefix(lower, "ollama/"):
		host := os.Getenv("OLLAMA_HOST")
		modelName := strings.TrimPrefix(model, "ollama/")
		return []Provider{NewOllama(host, modelName)}, nil

	case strings.HasPrefix(lower, "gemini"):
		key := firstOf("GOOGLE_API_KEY", "GEMINI_API_KEY")
		if key == "" {
			return nil, fmt.Errorf("JARVIS_MODEL=%q requires GOOGLE_API_KEY or GEMINI_API_KEY", model)
		}
		p, err := NewGemini(key, model)
		if err != nil {
			return nil, err
		}
		return []Provider{p}, nil

	default:
		return nil, fmt.Errorf("unrecognised JARVIS_MODEL=%q (prefix with openrouter/, ollama/, or gemini)", model)
	}
}

func firstOf(keys ...string) string {
	for _, k := range keys {
		if v := os.Getenv(k); v != "" {
			return v
		}
	}
	return ""
}
