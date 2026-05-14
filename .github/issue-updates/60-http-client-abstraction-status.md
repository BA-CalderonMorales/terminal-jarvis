# Issue #60: HTTP Client Abstraction Status

Prepared: 2026-05-13

## Recommendation

Keep #60 blocked until the API-domain decision from #50 is resolved.

Do not implement a new `HttpClient` abstraction yet. The current `src/api`
module is still future-use code with module-level `#![allow(dead_code)]`, and
the #50 audit splits API cleanup as its own prerequisite decision.

## Current Evidence

- `src/api/api_base.rs` is module-level `#![allow(dead_code)]` and documents
  itself as a future-use module for remote service integration.
- `src/api/api_client.rs` is module-level `#![allow(dead_code)]` and already
  contains an unused `ApiClient` wrapper over `reqwest`.
- `src/api/api_tool_operations.rs` is module-level `#![allow(dead_code)]` and
  contains placeholder tool metadata operations.
- `src/api/mod.rs` is module-level `#![allow(dead_code)]` and describes a
  planned API framework roadmap rather than active runtime integration.
- #50 now has an audit PR that treats `src/api/*` as a separate decision:
  either activate it as #60's HTTP foundation or remove the placeholder modules.

## Why This Blocks #60

#60 asks for a broad base HTTP abstraction, specific API clients, and adoption
inside `src/services/`. Implementing that before deciding whether the current
`src/api` module survives would add another abstraction layer to code that may
be removed or reshaped by the #50 API-domain cleanup.

The next useful step is not implementation. It is an API-domain decision:

1. Keep `src/api` and activate a narrow HTTP foundation there.
2. Remove `src/api` placeholder code and defer HTTP abstraction until a real
   service call site exists.
3. Move only reusable patterns into a new ADR or design note, then close this
   issue as premature.

## Suggested Issue Update

```markdown
#60 should stay blocked until the API-domain follow-up from #50 is resolved.

Current evidence:
- `src/api/*` is still future-use code with module-level `#![allow(dead_code)]`.
- The #50 audit splits `src/api` into a prerequisite decision: activate it as the HTTP client foundation or remove the placeholder modules.

Implementing #60 before that decision would add another abstraction on top of unresolved dead code. Recommended next step: decide the `src/api` fate in the #50 API-domain follow-up, then either implement a narrow `HttpClient`/`BaseHttpClient` with real call sites or close #60 as premature.
```
