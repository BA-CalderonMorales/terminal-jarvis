# Execution Anti-Patterns

### Reconstructing intent from the execution transport (PR #220)

**Pattern:** The streaming guard treated every request as explicit, although
active-agent shorthand and direct commands carry different consent semantics.
Resolving direct lifecycle capabilities also skipped their binary postconditions.

**Why it is wrong:** Choosing a display transport can grant permissions that
blocking execution refuses, or turn installer exit zero into apparent success.

**Fix:** Preserve the original request's intent through planning and authorization.
Apply capability postconditions on every entry path. Keep verifying installation
separate from adopting an active agent. Test refusal before any child or gate is
started, successful explicit consent, missing targets, and selection preservation.
