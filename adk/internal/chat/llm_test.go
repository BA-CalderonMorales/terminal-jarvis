package chat

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
)

type stubProvider struct {
	calls int
}

func (s *stubProvider) Label() string { return "stub" }

func (s *stubProvider) Chat(_ context.Context, messages []providers.Message, _ []providers.ToolDef) (providers.Response, error) {
	s.calls++
	if s.calls == 1 {
		return providers.Response{
			ToolCall: &providers.ToolCall{
				ID:   "tc_1",
				Name: "unknown_tool_name",
				Args: map[string]json.RawMessage{},
			},
		}, nil
	}

	if len(messages) < 3 {
		return providers.Response{Text: "missing history"}, nil
	}

	assistantToolCall := messages[len(messages)-2]
	toolResult := messages[len(messages)-1]
	if assistantToolCall.Role != "assistant" || len(assistantToolCall.ToolCalls) != 1 {
		return providers.Response{Text: "assistant tool call not recorded"}, nil
	}
	if toolResult.Role != "tool" || toolResult.ToolCallID != "tc_1" {
		return providers.Response{Text: "tool result not recorded"}, nil
	}

	return providers.Response{Text: "ok"}, nil
}

func TestSendRecordsStructuredAssistantToolCallHistory(t *testing.T) {
	session := NewSession("")
	p := &stubProvider{}

	resp, err := Send(context.Background(), session, p, "test")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp != "ok" {
		t.Fatalf("expected ok, got %q", resp)
	}
}
