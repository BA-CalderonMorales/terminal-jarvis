package repl

import "testing"

func TestFindToolAliasIncludesCode(t *testing.T) {
	tool, ok := findToolAlias(normalizeIntent("launch code"))
	if !ok {
		t.Fatal("expected launch alias for code")
	}
	if tool != "code" {
		t.Fatalf("expected code, got %q", tool)
	}
}

func TestFindToolAliasIncludesKiloSynonym(t *testing.T) {
	tool, ok := findToolAlias(normalizeIntent("launch kilo"))
	if !ok {
		t.Fatal("expected launch alias for kilo")
	}
	if tool != "kilocode" {
		t.Fatalf("expected kilocode, got %q", tool)
	}
}

func TestExtractLaunchTarget(t *testing.T) {
	target := extractLaunchTarget(normalizeIntent("please launch kimi k2"))
	if target != "kimi k2" {
		t.Fatalf("expected kimi k2, got %q", target)
	}
}

func TestSuggestToolFromPartialAlias(t *testing.T) {
	tool, ok := suggestTool("kilo")
	if !ok {
		t.Fatal("expected suggestion for kilo")
	}
	if tool != "kilocode" {
		t.Fatalf("expected kilocode, got %q", tool)
	}
}
