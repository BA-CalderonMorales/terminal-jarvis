# Execution Anti-Patterns

### Synthetic events that bypass the input protocol (v0.1.24 release review)

**Pattern:** Completion tests constructed a Tab event that the decoder never
emitted. Paging helper tests supplied a full tail that the real reader truncated.

**Why it is wrong:** The handler can pass every test while physical keys fail.

**Fix:** Drive actual bytes through the same decoder and terminal entry path.
Keep helper tests, but add terminal regressions for framing and consumption.

### Mixing bytes, characters and display cells (v0.1.24 release review)

**Pattern:** Completion advanced one byte after Unicode whitespace; picker
budgets omitted cursor/chip cells and placed ellipses beyond the stated limit.

**Why it is wrong:** Valid input can panic, and clipping hides meaningful state.

**Fix:** Advance string offsets at character boundaries. Budget rendered cells,
including every prefix, marker and ellipsis, before optional text. Exercise wide
characters and small supported geometries rather than only ASCII defaults.

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
