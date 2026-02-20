package providers

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/google/generative-ai-go/genai"
	"google.golang.org/api/option"
)

// GeminiProvider calls the Gemini API via the official Go SDK.
type GeminiProvider struct {
	client    *genai.Client
	modelName string
	apiKey    string
}

// NewGemini creates a Gemini provider for the given model and API key.
func NewGemini(apiKey, modelName string) (*GeminiProvider, error) {
	if modelName == "" {
		modelName = "gemini-2.0-flash"
	}
	ctx := context.Background()
	client, err := genai.NewClient(ctx, option.WithAPIKey(apiKey))
	if err != nil {
		return nil, fmt.Errorf("gemini client: %w", err)
	}
	return &GeminiProvider{client: client, modelName: modelName, apiKey: apiKey}, nil
}

func (g *GeminiProvider) Label() string {
	return g.modelName
}

func (g *GeminiProvider) Chat(ctx context.Context, messages []Message, tools []ToolDef) (Response, error) {
	model := g.client.GenerativeModel(g.modelName)
	model.SafetySettings = []*genai.SafetySetting{
		{Category: genai.HarmCategoryHarassment, Threshold: genai.HarmBlockNone},
		{Category: genai.HarmCategoryHateSpeech, Threshold: genai.HarmBlockNone},
		{Category: genai.HarmCategoryDangerousContent, Threshold: genai.HarmBlockNone},
		{Category: genai.HarmCategorySexuallyExplicit, Threshold: genai.HarmBlockNone},
	}

	// Register tool declarations when tools are provided.
	if len(tools) > 0 {
		decls := make([]*genai.FunctionDeclaration, 0, len(tools))
		for _, t := range tools {
			schema := toolDefToSchema(t)
			decls = append(decls, &genai.FunctionDeclaration{
				Name:        t.Name,
				Description: t.Description,
				Parameters:  schema,
			})
		}
		model.Tools = []*genai.Tool{{FunctionDeclarations: decls}}
	}

	// Convert message history to genai content.
	var history []*genai.Content
	var lastUserParts []genai.Part

	for i, m := range messages {
		switch m.Role {
		case "user":
			lastUserParts = []genai.Part{genai.Text(m.Content)}
			// If this is NOT the last message, add to history.
			if i < len(messages)-1 {
				history = append(history, &genai.Content{Role: "user", Parts: lastUserParts})
			}
		case "assistant":
			history = append(history, &genai.Content{Role: "model", Parts: []genai.Part{genai.Text(m.Content)}})
		case "tool":
			// Tool results are appended as function responses.
			var result interface{}
			_ = json.Unmarshal([]byte(m.Content), &result)
			if result == nil {
				result = m.Content
			}
			history = append(history, &genai.Content{
				Role: "function",
				Parts: []genai.Part{genai.FunctionResponse{
					Name:     m.ToolName,
					Response: map[string]interface{}{"result": result},
				}},
			})
		}
	}

	chat := model.StartChat()
	chat.History = history

	// The last message must be a user turn.
	if len(lastUserParts) == 0 {
		return Response{}, fmt.Errorf("last message must be from user")
	}

	resp, err := chat.SendMessage(ctx, lastUserParts...)
	if err != nil {
		return Response{}, fmt.Errorf("gemini send: %w", err)
	}

	if len(resp.Candidates) == 0 {
		return Response{}, fmt.Errorf("gemini returned no candidates")
	}

	candidate := resp.Candidates[0]
	if candidate.Content == nil {
		return Response{}, fmt.Errorf("gemini returned empty content")
	}

	for _, part := range candidate.Content.Parts {
		switch v := part.(type) {
		case genai.Text:
			return Response{Text: string(v)}, nil
		case genai.FunctionCall:
			args := make(map[string]json.RawMessage)
			for k, val := range v.Args {
				b, _ := json.Marshal(val)
				args[k] = b
			}
			return Response{
				ToolCall: &ToolCall{
					ID:   v.Name,
					Name: v.Name,
					Args: args,
				},
			}, nil
		}
	}

	return Response{}, fmt.Errorf("gemini returned unrecognized content type")
}

// toolDefToSchema converts a ToolDef's Parameters map to a genai.Schema.
func toolDefToSchema(t ToolDef) *genai.Schema {
	schema := &genai.Schema{Type: genai.TypeObject}

	props, _ := t.Parameters["properties"].(map[string]interface{})
	if props == nil {
		return schema
	}

	schema.Properties = make(map[string]*genai.Schema)
	for name, rawProp := range props {
		propMap, ok := rawProp.(map[string]interface{})
		if !ok {
			continue
		}
		propSchema := &genai.Schema{}
		if desc, ok := propMap["description"].(string); ok {
			propSchema.Description = desc
		}
		if typ, ok := propMap["type"].(string); ok {
			switch typ {
			case "string":
				propSchema.Type = genai.TypeString
			case "integer":
				propSchema.Type = genai.TypeInteger
			case "boolean":
				propSchema.Type = genai.TypeBoolean
			default:
				propSchema.Type = genai.TypeString
			}
		}
		schema.Properties[name] = propSchema
	}

	if req, ok := t.Parameters["required"].([]string); ok {
		schema.Required = req
	}

	return schema
}
