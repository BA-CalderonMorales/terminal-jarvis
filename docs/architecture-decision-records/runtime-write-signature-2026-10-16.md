# Match the runtime `write` signature instead of silencing the lint

Branch `release/0.1.20` -> `develop`, 2026-10-16. Linked from `README.md`.

**Decision:** Declare `write` in `src/tui/sigint.rs` with the signature the
platform actually provides (`*const c_void`) and cast at the two call sites,
rather than `#[allow]`-ing `suspicious-runtime-symbol-definitions` or pinning
the CI toolchain in this merge.

**Context:** CI floats on `dtolnay/rust-toolchain@stable`. Stable moved to
1.99.1 on 2026-10-13 and its `suspicious-runtime-symbol-definitions` lint began
covering `write`; the last Linux run of `develop` was 2026-09-09 on 1.98, which
does not check it. A branch carrying no relevant change therefore went red:
gate [2] lint (`cargo clippy --all-targets -- -D warnings`) failed on a
declaration that predates the branch. The block is `#[cfg(unix)]`, so no Windows
run could have surfaced it -- and the Windows suite is untouched by the fix,
because the declaration never compiles there.

**Considered:** `#[allow(suspicious_runtime_symbol_definitions)]` is the
smallest change, and the two signatures are ABI-identical, but it makes the code
disagree with its platform on purpose and would hide a real divergence later.
Pinning the toolchain removes the whole class of surprise, but that is a CI
policy change belonging to a maintainer's decision rather than to a release
branch. Matching the signature is the only option that removes the warning by
being correct.

**Consequence:** `write` declares `*const c_void`, so any new call site casts.
`clippy -D warnings` on the Linux target stays the enforcement point, and a
future divergence fails there instead of silently linking. The floating
toolchain hazard itself remains: a stable release can still redden an untouched
branch, and that is raised to maintainers rather than fixed here.
