package providers

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"
)

// OllamaProvider calls a local Ollama instance via its HTTP API.
type OllamaProvider struct {
	host      string
	modelName string
}

// NewOllama creates an Ollama provider.
func NewOllama(host, modelName string) *OllamaProvider {
	if host == "" {
		host = "http://localhost:11434"
	}
	if modelName == "" {
		modelName = "llama3.2"
	}
	modelName = strings.TrimPrefix(modelName, "ollama/")
	return &OllamaProvider{host: host, modelName: modelName}
}

func (o *OllamaProvider) Label() string {
	return "ollama/" + o.modelName + " (local)"
}

// ollamaMessage is the Ollama /api/chat message wire type.
type ollamaMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type ollamaChatRequest struct {
	Model    string          `json:"model"`
	Messages []ollamaMessage `json:"messages"`
	Stream   bool            `json:"stream"`
}

type ollamaChatResponse struct {
	Message struct {
		Role    string `json:"role"`
		Content string `json:"content"`
	} `json:"message"`
	Error string `json:"error"`
}

// OllamaReachable returns true if the Ollama server is listening.
func OllamaReachable(host string) bool {
	if host == "" {
		host = "http://localhost:11434"
	}
	client := &http.Client{Timeout: 500 * time.Millisecond}
	resp, err := client.Get(host + "/api/tags")
	if err != nil {
		return false
	}
	resp.Body.Close()
	return resp.StatusCode == 200
}

func (o *OllamaProvider) Chat(ctx context.Context, messages []Message, _ []ToolDef) (Response, error) {
	wireMessages := make([]ollamaMessage, 0, len(messages))
	for _, m := range messages {
		role := m.Role
		if role == "tool" {
			role = "user" // Ollama doesn't support tool messages; fold into user
		}
		wireMessages = append(wireMessages, ollamaMessage{Role: role, Content: m.Content})
	}

	req := ollamaChatRequest{
		Model:    o.modelName,
		Messages: wireMessages,
		Stream:   false,
	}
	body, err := json.Marshal(req)
	if err != nil {
		return Response{}, err
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", o.host+"/api/chat", bytes.NewReader(body))
	if err != nil {
		return Response{}, err
	}
	httpReq.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(httpReq)
	if err != nil {
		return Response{}, fmt.Errorf("ollama request: %w", err)
	}
	defer resp.Body.Close()

	rawBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return Response{}, fmt.Errorf("ollama read: %w", err)
	}

	var ollamaResp ollamaChatResponse
	if err := json.Unmarshal(rawBody, &ollamaResp); err != nil {
		return Response{}, fmt.Errorf("ollama parse: %w", err)
	}
	if ollamaResp.Error != "" {
		return Response{}, fmt.Errorf("ollama error: %s", ollamaResp.Error)
	}

	return Response{Text: strings.TrimSpace(ollamaResp.Message.Content)}, nil
}
