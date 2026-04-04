# Agent Harness Core Philosophy

> Derived from Andrej Karpathy's LLM Wiki pattern and hardened through production agent harness development.

## The Core Idea

Most agent interactions look like stateless RAG: the agent retrieves context, generates a response, and the knowledge evaporates. Nothing accumulates. Ask a subtle question requiring synthesis across five sessions, and the agent rediscovers everything from scratch.

This philosophy is different. We build and maintain a **persistent, compounding knowledge base** that sits between you and the raw work. When you complete a task, the agent doesn't just respond. It extracts key information, integrates it into the existing corpus, updates cross-references, notes contradictions, and strengthens the evolving synthesis.

**The knowledge base is a persistent, compounding artifact.** Cross-references exist. Contradictions are flagged. Synthesis reflects everything learned. The harness gets richer with every session.

---

## The Three-Layer Architecture

Every agent harness in this workspace follows this structure:

```
Raw Sources          The Wiki           The Schema
─────────────────────────────────────────────────────────
Source code    →   AGENTS.md       →   PHILOSOPHY.md
Documentation      Skills/              (this file)
Session logs       Patterns/
                   index.md
                   log.md
```

### Layer 1: Raw Sources

Immutable source of truth. The agent reads from these but never modifies them directly.

- Source code (src/, internal/, cmd/)
- Documentation (docs/, README.md)
- External references (links, papers, articles)

### Layer 2: The Wiki

LLM-generated and maintained. The agent owns this layer entirely.

- **AGENTS.md** - Per-repo constitution and conventions
- **Skills/** - Reusable capability modules
- **Patterns/** - Recognized problem-solution pairs
- **index.md** - Catalog of all wiki pages
- **log.md** - Chronological activity record

### Layer 3: The Schema

The configuration that tells agents how to operate. This document (PHILOSOPHY.md) is the root schema. Each repo's AGENTS.md extends it with domain specifics.

---

## Core Operations

### Ingest

When new code or documentation arrives:

1. Read and analyze the source
2. Discuss key takeaways
3. Update relevant wiki pages
4. Maintain cross-references
5. Append entry to log.md

A single source might touch 10-15 wiki pages. This is expected and correct.

### Query

When answering questions:

1. Search wiki pages first (index.md)
2. Drill into relevant sources
3. Synthesize with citations
4. File valuable answers back into the wiki

Good answers become new pages. Discoveries compound.

### Lint

Periodically health-check the wiki:

- Find contradictions between pages
- Identify stale claims superseded by new work
- Locate orphan pages with no inbound links
- Discover missing cross-references
- Flag data gaps for investigation

---

## Workspace Conventions

### Repository Types

| Type | Examples | Purpose |
|------|----------|---------|
| **Harness** | terminal-jarvis, agent-harness, lumina-bot | Agent execution environments |
| **Reference** | codex-cheat-sheet, kimi-cheat-sheet | Knowledge repositories |
| **Infrastructure** | homebrew-terminal-jarvis | Distribution channels |

### Directory Structure

```
~/
├── PHILOSOPHY.md              # This file - root schema
├── AGENTS.md                  # Workspace-level agent guide
├── projects/
│   ├── terminal-jarvis/       # Rust harness
│   │   ├── AGENTS.md          # Repo constitution
│   │   ├── README.md          # Human docs
│   │   └── ...
│   ├── agent-harness/         # Go harness
│   │   ├── AGENTS.md
│   │   └── ...
│   └── lumina-bot/            # Go gateway
│       ├── AGENTS.md
│       └── ...
├── skills/                    # Cross-project skills
│   ├── memory-system/
│   └── ...
└── .kimi/skills/              # Environment-specific skills
```

### File Naming

- Root level: `README.md`, `AGENTS.md`, `PHILOSOPHY.md`, `LICENSE`
- Documentation: `lowercase.md` (never UPPERCASE)
- Skills: `skills/{name}/SKILL.md`
- Temp/working: `tmp/` directory (gitignored)

---

## Visual Standards

### Status Indicators

| Symbol | Meaning |
|--------|---------|
| `◆` | Acknowledgment / start |
| `→` | Action in progress |
| `✓` | Success |
| `✗` | Error |
| `⚠` | Warning |
| `?` | Needs input |

### Spinner Animation

```
┌( >_<)┘  Frame 1
└( >_<)┐  Frame 2
```

### Response Structure

```
◆ <brief context>

→ <specific action>:
   <progress indicator>

✓ <result summary>
   <formatted output>
```

### Prohibited

- **NO EMOJIS** in any documentation files
- **NO HORIZONTAL RULES** (`---`) as section separators
- **NO REPETITION** of user input in responses

---

## Commit Standards

Format: `type(scope): description`

Types:
- `feat:` - New capability
- `fix:` - Bug fix
- `docs:` - Documentation only
- `refactor:` - Code restructuring
- `test:` - Test changes
- `chore:` - Maintenance

Rules:
- Lowercase after colon
- Present tense
- No emojis
- Body with bullets when details matter

---

## Quality Gates

Before any commit:

1. **Build** - Must compile
2. **Test** - Must pass
3. **Lint** - Must pass (clippy, fmt, or equivalent)
4. **Verify** - Run verification script if available

Verification script pattern:
```bash
./scripts/verify/verify-change.sh        # Full check
./scripts/verify/verify-change.sh --quick # Skip tests
```

---

## Cross-Repo Operations

### Multi-Repo Status

```bash
ws status      # All repos at a glance
ws dirty       # What needs attention
ws sync        # Pull, commit, push
```

### Branch Strategy

- `main` - Default branch, deployable
- `develop` - Active work integration
- `feat/*`, `fix/*` - Feature branches

**Never push tags without explicit approval.**

### Synchronization Workflow

When updating philosophy across repos:

1. Update `PHILOSOPHY.md` (this file)
2. Run `sync-philosophy` to propagate to all harness repos
3. Review changes in each repo
4. Commit with `docs(philosophy): sync core philosophy`

---

## Session Continuity

### The Problem

Traditional agent sessions evaporate. Auth workflows interrupt flow. Context is lost between invocations.

### The Solution

Each harness implements session persistence:

- Session state auto-saves
- Authentication workflows resume seamlessly
- Working context survives interruptions

### Implementation

Sessions stored in:
- `~/.terminal-jarvis/sessions/`
- `~/.agent-harness/sessions/`
- `~/.lumina/sessions/`

---

## Tool Execution Standards

Tool execution is the core value proposition. These requirements are non-negotiable:

1. **Parsing** - LLM output parsed correctly every time
2. **Validation** - Schema validation before execution
3. **Execution** - Proper timeout and cancellation
4. **Feedback** - User sees what tool runs and its status
5. **Recovery** - Failed tools don't crash the session

### Visual Feedback

```
→ tool-name: <what it's doing>
  ┌( >_<)┘  <spinner while running>
✓ tool-name: <result summary>
```

---

## Knowledge Persistence

### Memory System

Significant debugging, optimization, or architectural work must be captured:

```bash
# After fixing non-obvious bugs or creating reusable patterns
# Reference: ~/Projects/skills/memory-system/SKILL.md
```

Target: `EverMemOS` - the persistent memory repository

### Skills Pattern

Reusable knowledge lives in `skills/{name}/SKILL.md`:

- Project-specific: `projects/X/skills/`
- Cross-project: `~/skills/`
- Environment: `~/.kimi/skills/`

---

## Why This Works

The tedious part of agent development is not the coding or the thinking. It is the bookkeeping. Updating cross-references, keeping documentation current, noting when new code contradicts old patterns, maintaining consistency across dozens of files.

Humans abandon documentation because the maintenance burden grows faster than the value. Agents don't get bored, don't forget to update a cross-reference, and can touch 15 files in one pass.

The human's job is to curate direction, ask good questions, and think about what it all means. The agent's job is everything else.

---

## Related

- Andrej Karpathy's LLM Wiki pattern (~/insights/llmwiki.md)
- Termux Environment Guide (~/AGENTS.md)
- Terminal Jarvis ADK (projects/terminal-jarvis/AGENTS.md)
- Agent Harness (projects/agent-harness/AGENTS.md)
- Lumina Bot (projects/lumina-bot/AGENTS.md)

---

*Last updated: 2026-04-04*
*Philosophy version: 1.0*
