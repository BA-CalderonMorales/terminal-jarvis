# Anti-Patterns

This directory is the ledger of anti-patterns observed in this repository's
docs/ and scripts/ domains. When you notice a recurring mistake a coding agent
(or human) made in committed work, record it while the shape of it is fresh.
The record states the pattern, why it is wrong, and what to do instead, so the
next pass does not repeat it from memory.

The ledger is organized as a domain of categories, mirroring the `index.md`
pattern used for scripts: every category lives at
`docs/anti-patterns/{type}/index.md`, newest entries first. Categories are
created when a pattern does not fit an existing one -- never force a pattern
into the wrong type.

## Rules

- One anti-pattern per entry, newest first, inside the matching category.
- Write the pattern as it appeared in real committed work, not as a
  hypothetical.
- State the fix plainly; prefer lines that can move verbatim into `AGENTS.md`.
- When a rule is lifted into `AGENTS.md`, keep it mirrored here so this ledger
  stays the source.

## Format

```markdown
### <Name of the anti-pattern> (<first observed in>)

**Pattern:** What the agent (or human) actually did.

**Why it is wrong:** The durable cost of the pattern.

**Fix:** What to do instead.
```

## Categories

| Type | Records |
|---|---|
| [data/](data/index.md) | Capability metadata that does not match the real CLI |
| [naming/](naming/index.md) | File names that encode bookkeeping instead of purpose |
| [structure/](structure/index.md) | Layout that hides what a file is or how it is called |
| [tooling/](tooling/index.md) | Reach of standard tools instead of installed ones |

Lost? The active count is `rg -c '^### ' docs/anti-patterns/*/index.md` short
of the categories listed above.
