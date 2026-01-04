# Skill: Token Budget Management

**Name**: token-budget
**Description**: Token efficiency patterns for long AI coding sessions
**Trigger**: Long sessions, complex multi-step tasks, session optimization

---

## Objective

Maximize session duration beyond 12 hours through intelligent orchestration.

## Budget Caps

| Agent Type | Model | Max Tokens | Responsibilities |
|------------|-------|------------|------------------|
| **Lead** | Sonnet 4.5 | 1000 | Planning, architecture, orchestration, integration, validation |
| **Spawned** | Haiku Latest | 750 | File edits, docs, tests, refactoring, specialized tasks |

## Lead Orchestrator Pattern

### Phase 1: Planning (Lead - 1000 tokens)
1. Analyze request scope
2. Break into discrete tasks
3. Identify parallelization opportunities
4. Select appropriate agents
5. Define validation criteria

### Phase 2: Delegation (Agents - 750 tokens each)
- Spawn agents in parallel for independent tasks
- Sequential for dependencies
- Focused context (only what's needed)
- Clear success criteria

### Phase 3: Integration (Lead - 1000 tokens)
- Collect results
- Run quality checks (cargo check, clippy, fmt, test)
- Coordinate commits
- Update documentation

## Token Conservation Techniques

| Technique | Good | Bad |
|-----------|------|-----|
| **Delegation** | Lead plans, 5 agents implement | Lead implements everything |
| **Parallelization** | 3 agents simultaneously | Sequential when independent |
| **File reads** | Read specific file | Read entire directory tree |
| **Validation** | cargo check after each agent | Accumulate, validate at end |
| **Context** | "Update lines 45-60 in file X" | "Read file, understand, update" |

## Target Metrics

- Agent utilization: >80% (most work by agents)
- Parallelization: >40% (many concurrent)
- Session duration: >12 hours

## Efficiency Guidelines

- **DO NOT** create documentation files unless explicitly requested
- **DO** leverage specialized agents proactively
- **DO** use concise responses - skip verbose explanations when action is clear

## Proactive Agent Usage

AI assistants MUST invoke specialized agents immediately without waiting to be asked:

| Scenario | Agent |
|----------|-------|
| Documentation | @documentation-specialist |
| Testing | @qa-automation-engineer |
| Code review | @code-reviewer |
| Security | @security-specialist |
| Infrastructure | @devops-engineer |
| Architecture | @software-architect |
| Performance | @performance-specialist |
