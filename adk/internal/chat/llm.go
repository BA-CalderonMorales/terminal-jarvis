package chat

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/providers"
	"github.com/BA-CalderonMorales/terminal-jarvis/adk/internal/tools"
)

// maxToolLoops caps the number of tool-call rounds to prevent infinite loops.
const maxToolLoops = 5

// Send adds the user message to the session, calls the provider, handles any
// tool calls in a loop, then returns the final text reply.
func Send(ctx context.Context, session *Session, provider providers.Provider, userText string) (string, error) {
	session.AddUser(userText)

	toolSpecs := tools.SpecList()

	for i := 0; i < maxToolLoops; i++ {
		resp, err := provider.Chat(ctx, session.Messages, toolSpecs)
		if err != nil {
			return "", err
		}

		// Plain text reply -- done.
		if resp.ToolCall == nil {
			session.AddAssistant(resp.Text)
			return resp.Text, nil
		}

		// Tool call: execute and add result to history.
		tc := resp.ToolCall
		strArgs := decodeArgs(tc.Args)
		result := tools.Dispatch(tc.Name, strArgs)

		// Record the assistant's tool-call message.
		session.Messages = append(session.Messages, providers.Message{
			Role:    "assistant",
			Content: fmt.Sprintf("[tool_call: %s]", tc.Name),
		})
		session.AddToolResult(tc.ID, tc.Name, result)
	}

	return "", fmt.Errorf("tool call loop exceeded %d iterations", maxToolLoops)
}

// decodeArgs converts json.RawMessage values to plain strings for tool dispatch.
func decodeArgs(raw map[string]json.RawMessage) map[string]string {
	out := make(map[string]string, len(raw))
	for k, v := range raw {
		var s string
		if err := json.Unmarshal(v, &s); err == nil {
			out[k] = s
		} else {
			out[k] = string(v)
		}
	}
	return out
}
