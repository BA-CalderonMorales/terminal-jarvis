package providers

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

const openRouterBaseURL = "https://openrouter.ai/api/v1/chat/completions"

// OpenRouterProvider calls the OpenRouter API (OpenAI-compatible).
// No litellm required — pure net/http.
type OpenRouterProvider struct {
	apiKey    string
	modelName string // e.g. "minimax/minimax-m2.5"
	label     string
}

// NewOpenRouter creates an OpenRouter provider.
// modelName should be the path after "openrouter/" in the JARVIS_MODEL env var,
// or left empty to default to "google/gemini-flash-1.5".
func NewOpenRouter(apiKey, modelName string) *OpenRouterProvider {
	if modelName == "" {
		modelName = "google/gemini-flash-1.5"
	}
	// Strip "openrouter/" prefix if the caller passed the full env-var form.
	modelName = strings.TrimPrefix(modelName, "openrouter/")
	return &OpenRouterProvider{
		apiKey:    apiKey,
		modelName: modelName,
		label:     "openrouter/" + modelName,
	}
}

func (o *OpenRouterProvider) Label() string { return o.label }

// orMessage is the OpenAI-compatible message wire type.
type orMessage struct {
	Role       string      `json:"role"`
	Content    interface{} `json:"content"`
	ToolCallID string      `json:"tool_call_id,omitempty"`
	Name       string      `json:"name,omitempty"`
	ToolCalls  []orToolCallRef `json:"tool_calls,omitempty"`
}

type orToolCallRef struct {
	ID       string       `json:"id"`
	Type     string       `json:"type"`
	Function orFunctionCall `json:"function"`
}

type orFunctionCall struct {
	Name      string `json:"name"`
	Arguments string `json:"arguments"`
}

type orTool struct {
	Type     string     `json:"type"`
	Function orFunction `json:"function"`
}

type orFunction struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	Parameters  interface{} `json:"parameters"`
}

type orRequest struct {
	Model    string      `json:"model"`
	Messages []orMessage `json:"messages"`
	Tools    []orTool    `json:"tools,omitempty"`
}

type orChoice struct {
	Message struct {
		Role      string         `json:"role"`
		Content   string         `json:"content"`
		ToolCalls []orToolCallRef `json:"tool_calls"`
	} `json:"message"`
}

type orResponse struct {
	Choices []orChoice `json:"choices"`
	Error   *struct {
		Message string `json:"message"`
	} `json:"error"`
}

func (o *OpenRouterProvider) Chat(ctx context.Context, messages []Message, tools []ToolDef) (Response, error) {
	// Build wire messages.
	wireMessages := make([]orMessage, 0, len(messages))
	for _, m := range messages {
		switch m.Role {
		case "user":
			wireMessages = append(wireMessages, orMessage{Role: "user", Content: m.Content})
		case "assistant":
			wireMessages = append(wireMessages, orMessage{Role: "assistant", Content: m.Content})
		case "tool":
			wireMessages = append(wireMessages, orMessage{
				Role:       "tool",
				Content:    m.Content,
				ToolCallID: m.ToolCallID,
				Name:       m.ToolName,
			})
		}
	}

	req := orRequest{
		Model:    o.modelName,
		Messages: wireMessages,
	}

	// Attach tool definitions when provided.
	if len(tools) > 0 {
		for _, t := range tools {
			req.Tools = append(req.Tools, orTool{
				Type: "function",
				Function: orFunction{
					Name:        t.Name,
					Description: t.Description,
					Parameters:  t.Parameters,
				},
			})
		}
	}

	body, err := json.Marshal(req)
	if err != nil {
		return Response{}, err
	}

	httpReq, err := http.NewRequestWithContext(ctx, "POST", openRouterBaseURL, bytes.NewReader(body))
	if err != nil {
		return Response{}, err
	}
	httpReq.Header.Set("Authorization", "Bearer "+o.apiKey)
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("HTTP-Referer", "https://github.com/BA-CalderonMorales/terminal-jarvis")
	httpReq.Header.Set("X-Title", "Terminal Jarvis")

	resp, err := http.DefaultClient.Do(httpReq)
	if err != nil {
		return Response{}, fmt.Errorf("openrouter request: %w", err)
	}
	defer resp.Body.Close()

	rawBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return Response{}, fmt.Errorf("openrouter read: %w", err)
	}

	if resp.StatusCode == 401 || resp.StatusCode == 403 {
		return Response{}, fmt.Errorf("AuthenticationError: %s", string(rawBody))
	}
	if resp.StatusCode != 200 {
		return Response{}, fmt.Errorf("openrouter %d: %s", resp.StatusCode, string(rawBody))
	}

	var orResp orResponse
	if err := json.Unmarshal(rawBody, &orResp); err != nil {
		return Response{}, fmt.Errorf("openrouter parse: %w", err)
	}
	if orResp.Error != nil {
		return Response{}, fmt.Errorf("openrouter API error: %s", orResp.Error.Message)
	}
	if len(orResp.Choices) == 0 {
		return Response{}, fmt.Errorf("openrouter returned no choices")
	}

	choice := orResp.Choices[0].Message

	// Tool call response.
	if len(choice.ToolCalls) > 0 {
		tc := choice.ToolCalls[0]
		args := make(map[string]json.RawMessage)
		if err := json.Unmarshal([]byte(tc.Function.Arguments), &args); err != nil {
			// If arguments aren't JSON object, wrap them.
			args["input"] = json.RawMessage(tc.Function.Arguments)
		}
		return Response{
			ToolCall: &ToolCall{
				ID:   tc.ID,
				Name: tc.Function.Name,
				Args: args,
			},
		}, nil
	}

	return Response{Text: strings.TrimSpace(choice.Content)}, nil
}
