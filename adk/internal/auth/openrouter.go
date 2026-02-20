package auth

import (
	"bufio"
	"bytes"
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"strings"
	"time"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/ui"
)

const (
	orAuthURL       = "https://openrouter.ai/auth"
	orKeysExchange  = "https://openrouter.ai/api/v1/auth/keys"
	orKeysPage      = "https://openrouter.ai/keys"
	callbackTimeout = 120 * time.Second
)

// SetupOpenRouter runs a PKCE OAuth flow for OpenRouter.
// Returns the API key if obtained, or "" on failure/cancel.
func SetupOpenRouter(envPath string) string {
	return setupOpenRouterWithPrompt(envPath, nil)
}

func setupOpenRouterWithPrompt(envPath string, promptFn TextPrompt) string {
	fmt.Println()
	fmt.Printf("   %s► %sOpenRouter Setup%s\n", ui.Cyan, ui.BoldW, ui.Reset)
	fmt.Printf("   %sAccess 100+ cloud models with a single key.%s\n", ui.Dim, ui.Reset)
	fmt.Printf("   %sWe'll save your key to adk/.env automatically.%s\n", ui.Dim, ui.Reset)
	fmt.Println()

	// OpenRouter OAuth has been unstable (409 from provider side), so use
	// manual key entry by default. OAuth remains opt-in.
	if strings.ToLower(os.Getenv("JARVIS_OPENROUTER_OAUTH")) == "1" ||
		strings.ToLower(os.Getenv("JARVIS_OPENROUTER_OAUTH")) == "true" {
		return runOpenRouterOAuth(envPath, promptFn)
	}
	return promptOpenRouterKeyFallback(envPath, promptFn)
}

func runOpenRouterOAuth(envPath string, promptFn TextPrompt) string {
	verifier, challenge, err := pkcePair()
	if err != nil {
		fmt.Printf("   %sFailed to generate PKCE pair: %v%s\n", ui.Red, err, ui.Reset)
		return ""
	}

	port, err := freePort()
	if err != nil {
		fmt.Printf("   %sCould not find a free port: %v%s\n", ui.Red, err, ui.Reset)
		return ""
	}

	callbackURL := fmt.Sprintf("http://localhost:%d/callback", port)
	authURL := fmt.Sprintf("%s?callback_url=%s&code_challenge=%s&code_challenge_method=S256",
		orAuthURL,
		url.QueryEscape(callbackURL),
		challenge,
	)

	codeCh := make(chan string, 1)
	srv := startCallbackServer(port, codeCh)
	defer srv.Close()

	fmt.Printf("   %sOpening OpenRouter in your browser to sign in...%s\n", ui.LightB, ui.Reset)
	fmt.Println()
	if !openBrowser(authURL) {
		fmt.Printf("   %sCould not open browser. Open this URL to continue:%s\n\n", ui.LightB, ui.Reset)
		fmt.Printf("   %s%s%s\n\n", ui.Cyan, authURL, ui.Reset)
	}
	fmt.Printf("   %sWaiting for callback on localhost:%d (timeout %ds)%s\n", ui.Dim, port, int(callbackTimeout.Seconds()), ui.Reset)
	fmt.Printf("   %sPress Ctrl-C to cancel.%s\n\n", ui.Dim, ui.Reset)

	ctx, cancel := context.WithTimeout(context.Background(), callbackTimeout)
	defer cancel()

	var code string
	select {
	case code = <-codeCh:
	case <-ctx.Done():
		fmt.Printf("   %sTimed out waiting for OAuth callback.%s\n", ui.Red, ui.Reset)
		return promptOpenRouterKeyFallback(envPath, promptFn)
	}

	fmt.Printf("   %sCallback received. Exchanging code for API key...%s\n", ui.LightB, ui.Reset)
	apiKey, err := exchangeCode(code, verifier)
	if err != nil {
		fmt.Printf("   %sCould not exchange code: %v%s\n", ui.Red, err, ui.Reset)
		return promptOpenRouterKeyFallback(envPath, promptFn)
	}

	writeEnvKey(envPath, "OPENROUTER_API_KEY", apiKey)
	return apiKey
}

func pkcePair() (verifier, challenge string, err error) {
	b := make([]byte, 48)
	if _, err = rand.Read(b); err != nil {
		return
	}
	verifier = base64.RawURLEncoding.EncodeToString(b)
	sum := sha256.Sum256([]byte(verifier))
	challenge = base64.RawURLEncoding.EncodeToString(sum[:])
	return
}

func freePort() (int, error) {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return 0, err
	}
	defer l.Close()
	return l.Addr().(*net.TCPAddr).Port, nil
}

func startCallbackServer(port int, codeCh chan<- string) *http.Server {
	mux := http.NewServeMux()
	mux.HandleFunc("/callback", func(w http.ResponseWriter, r *http.Request) {
		code := r.URL.Query().Get("code")
		if code == "" {
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		body := []byte(`<html><body style="font-family:monospace;background:#0d1117;color:#c9d1d9;padding:2em">` +
			`<h2 style="color:#58a6ff">Authentication successful</h2>` +
			`<p>Your OpenRouter API key has been saved.</p>` +
			`<p>You can close this tab and return to the terminal.</p></body></html>`)
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		w.WriteHeader(http.StatusOK)
		w.Write(body)
		select {
		case codeCh <- code:
		default:
		}
	})

	srv := &http.Server{
		Addr:    fmt.Sprintf("127.0.0.1:%d", port),
		Handler: mux,
	}
	go srv.ListenAndServe()
	return srv
}

func exchangeCode(code, verifier string) (string, error) {
	payload, _ := json.Marshal(map[string]string{
		"code":                  code,
		"code_verifier":         verifier,
		"code_challenge_method": "S256",
	})
	resp, err := http.Post(orKeysExchange, "application/json", bytes.NewReader(payload))
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		msg, code := parseOpenRouterError(body)
		if msg != "" {
			return "", fmt.Errorf("openrouter auth error (%s): %s", code, msg)
		}
		return "", fmt.Errorf("openrouter auth error (status %d): %s", resp.StatusCode, strings.TrimSpace(string(body)))
	}
	var data map[string]interface{}
	if err := json.Unmarshal(body, &data); err != nil {
		return "", err
	}
	key, _ := data["key"].(string)
	if key == "" {
		return "", fmt.Errorf("no key in response: %s", string(body))
	}
	return key, nil
}

// writeEnvKey writes or replaces KEY=value in the .env file.
func writeEnvKey(envPath, key, value string) {
	writeEnvKeyToFile(envPath, key, value)
}

func parseOpenRouterError(body []byte) (message string, code string) {
	var data map[string]interface{}
	if err := json.Unmarshal(body, &data); err != nil {
		return "", ""
	}
	errObj, _ := data["error"].(map[string]interface{})
	if errObj == nil {
		return "", ""
	}
	if m, ok := errObj["message"].(string); ok {
		message = m
	}
	if c, ok := errObj["code"]; ok {
		code = fmt.Sprintf("%v", c)
	}
	return message, code
}

func promptOpenRouterKeyFallback(envPath string, promptFn TextPrompt) string {
	fmt.Printf("   %sManual API key setup.%s\n", ui.LightB, ui.Reset)
	fmt.Printf("   %sOpen this page and create/copy a key:%s %s%s%s\n", ui.LightB, ui.Reset, ui.Cyan, orKeysPage, ui.Reset)
	_ = openBrowser(orKeysPage)
	reader := bufio.NewReader(os.Stdin)
	raw, _ := readInput(reader, promptFn, "   Paste OPENROUTER_API_KEY (Enter to skip): ")
	if raw == "" {
		return ""
	}
	if !strings.HasPrefix(raw, "sk-or-v1-") {
		ans, _ := readInput(reader, promptFn, "   Key format looks unusual. Save anyway? [y/N] ")
		ans = strings.ToLower(strings.TrimSpace(ans))
		if ans != "y" && ans != "yes" {
			return ""
		}
	}
	writeEnvKey(envPath, "OPENROUTER_API_KEY", raw)
	return raw
}
