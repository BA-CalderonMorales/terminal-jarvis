package auth

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestActivateProviderSetsModel(t *testing.T) {
	envPath := filepath.Join(t.TempDir(), ".env")
	model, err := ActivateProvider(envPath, "openrouter")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if model != openRouterModel {
		t.Fatalf("expected %s, got %s", openRouterModel, model)
	}

	raw, err := os.ReadFile(envPath)
	if err != nil {
		t.Fatalf("failed to read env file: %v", err)
	}
	if !strings.Contains(string(raw), "JARVIS_MODEL="+openRouterModel) {
		t.Fatalf("JARVIS_MODEL not written to env file: %s", string(raw))
	}
}

func TestLogoutProviderRejectsUnknownProvider(t *testing.T) {
	envPath := filepath.Join(t.TempDir(), ".env")
	if _, err := LogoutProvider(envPath, "unknown-provider"); err == nil {
		t.Fatal("expected unknown provider error")
	}
}

func TestLogoutProviderClearsKeysAndModel(t *testing.T) {
	envPath := filepath.Join(t.TempDir(), ".env")
	writeEnvKeyToFile(envPath, "OPENROUTER_API_KEY", "sk-or-test")
	writeEnvKeyToFile(envPath, "JARVIS_MODEL", openRouterModel)
	_ = os.Setenv("OPENROUTER_API_KEY", "sk-or-test")
	_ = os.Setenv("JARVIS_MODEL", openRouterModel)
	t.Cleanup(func() {
		_ = os.Unsetenv("OPENROUTER_API_KEY")
		_ = os.Unsetenv("JARVIS_MODEL")
	})

	provider, err := LogoutProvider(envPath, "openrouter")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if provider != "openrouter" {
		t.Fatalf("expected openrouter, got %s", provider)
	}
	if os.Getenv("OPENROUTER_API_KEY") != "" || os.Getenv("JARVIS_MODEL") != "" {
		t.Fatal("expected process env to be cleared")
	}

	raw, err := os.ReadFile(envPath)
	if err != nil {
		t.Fatalf("failed to read env file: %v", err)
	}
	text := string(raw)
	if strings.Contains(text, "OPENROUTER_API_KEY=") || strings.Contains(text, "JARVIS_MODEL=") {
		t.Fatalf("expected env file keys to be removed, got: %s", text)
	}
}
