# Let an interactive launch through without a consent keystroke

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** The intent gate returns early for `Interaction::Interactive`
capabilities. A bare `tj`, `run <harness>`, and the tui's run path reach the
harness ui without a `Continue with ui:<harness>? [y/N]` prompt. The dangerous
check still runs first, and state-changing noninteractive runs keep their
bracket and their confirm token.

**Context:** The consent layer treats every non-read-only effect as worth
reviewing, so launching the agent -- the primary action, and the one the user
just asked for -- stopped at a prompt whose default was NO. The harness owns
the terminal in that mode and asks its own questions, so the extra keystroke
reviewed nothing and could not review anything.

**Considered:** Defaulting the prompt to yes for `ui` keeps the friction and
the prompt. Leaving it is what the pty acceptance tests pinned, but pinning a
bad contract is not a reason to keep it. Dropping consent for all state changes
would remove the review that headless installs and uninstalls genuinely need,
so the early return is scoped to the interactive case.

**Consequence:** Launching a tool is one action. The two pty tests that pinned
the `ui` consent now pin its absence, and their scan-count assertions -- the
reason they exist -- are unchanged. A dangerous capability still needs
`--allow-dangerous`, which is why the README demo can show a refusal without
showing an install.
