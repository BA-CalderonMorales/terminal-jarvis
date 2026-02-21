package auth

import (
	"fmt"
	"os"
	"strings"
)

const (
	geminiModel     = "gemini-2.0-flash"
	openRouterModel = "openrouter/google/gemini-flash-1.5"
	ollamaModel     = "ollama/llama3.2"
)

var providerEnvKeys = map[string][]string{
	"gemini":     {"GOOGLE_API_KEY", "GEMINI_API_KEY"},
	"openrouter": {"OPENROUTER_API_KEY"},
	"ollama":     {"OLLAMA_HOST"},
}

// ActivateProvider persists and exports the selected provider model so switching
// providers is immediate in the running session.
func ActivateProvider(envPath, provider string) (string, error) {
	provider = normalizeProviderChoice(provider)

	var model string
	switch provider {
	case "gemini":
		model = geminiModel
	case "openrouter":
		model = openRouterModel
	case "ollama":
		model = ollamaModel
	default:
		return "", fmt.Errorf("unknown provider: %s", provider)
	}

	writeEnvKeyToFile(envPath, "JARVIS_MODEL", model)
	_ = os.Setenv("JARVIS_MODEL", model)
	return model, nil
}

// LogoutProvider clears provider credentials from current process env and .env.
// If provider is empty, the currently active provider is used.
func LogoutProvider(envPath, provider string) (string, error) {
	provider = normalizeProviderChoice(provider)
	if provider == "" {
		provider = DetectActiveProvider()
	}
	if provider == "" {
		return "", fmt.Errorf("no active provider found")
	}
	if provider != "all" {
		if _, ok := providerEnvKeys[provider]; !ok {
			return "", fmt.Errorf("unknown provider: %s", provider)
		}
	}

	keysToClear := make([]string, 0)
	if provider == "all" {
		keysToClear = append(keysToClear,
			"GOOGLE_API_KEY", "GEMINI_API_KEY", "OPENROUTER_API_KEY", "OLLAMA_HOST", "JARVIS_MODEL",
		)
	} else {
		keysToClear = append(keysToClear, providerEnvKeys[provider]...)
		if DetectActiveProvider() == provider {
			keysToClear = append(keysToClear, "JARVIS_MODEL")
		}
	}

	for _, key := range keysToClear {
		_ = os.Unsetenv(key)
	}
	clearEnvKeysFromFile(envPath, keysToClear...)
	return provider, nil
}

// DetectActiveProvider infers the active provider from JARVIS_MODEL first,
// then falls back to configured API keys.
func DetectActiveProvider() string {
	model := strings.ToLower(strings.TrimSpace(os.Getenv("JARVIS_MODEL")))
	switch {
	case strings.HasPrefix(model, "openrouter/"):
		return "openrouter"
	case strings.HasPrefix(model, "ollama/"):
		return "ollama"
	case strings.HasPrefix(model, "gemini"):
		return "gemini"
	}

	if os.Getenv("GOOGLE_API_KEY") != "" || os.Getenv("GEMINI_API_KEY") != "" {
		return "gemini"
	}
	if os.Getenv("OPENROUTER_API_KEY") != "" {
		return "openrouter"
	}
	if os.Getenv("OLLAMA_HOST") != "" {
		return "ollama"
	}
	return ""
}

func normalizeProviderChoice(input string) string {
	switch strings.ToLower(strings.TrimSpace(input)) {
	case "google", "google-gemini", "gemini":
		return "gemini"
	case "openrouter", "or":
		return "openrouter"
	case "ollama", "local":
		return "ollama"
	case "all", "*":
		return "all"
	default:
		return strings.ToLower(strings.TrimSpace(input))
	}
}
