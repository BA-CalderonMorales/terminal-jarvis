# Put the decision at the cursor

Branch `release/0.1.21` -> `develop`, 2026-10-19. Linked from `README.md`.

**Decision:** A pending consent owns the prompt row. `tui::input::ask` publishes
the question, the viewport renders it where the cursor already is, and the body
bottom-anchors so the plan sits directly above it. The clean plan line becomes
one sentence per mechanism: `install droid · remote script · app.factory.ai`.

**Context:** The plan and the question rendered in the vertical centre of the
frame while the cursor parked at the bottom prompt row. The user read the
middle, then found the prompt to answer, and nothing at the cursor said a
decision was pending -- so the frame read as broken even when every line of it
was correct. The lead was also raw argv (`sh -c 'curl … | sh'`), which names
neither the mechanism nor where the bytes come from.

**Considered:** Moving the cursor to the centred question keeps the eye
travelling to the middle. Defaulting the question to yes is still a prompt in
the wrong place. Bottom-anchoring the body always is a bigger aesthetic pivot
than the pain asked for, so the primer and the one-shot cards (`status`,
`help`, `show`, `list`) stay centred and only a decision or a growing log
anchors low.

**Consequence:** The decision, its context, and the cursor are one glance, and
the prompt row degrades to the question so the modeline hint stands down for
it. `show` and `plan` keep their own `plan_with_extra` format; only the clean
consent lead changed. A narrow frame falls back to `Continue? [Y/n]` rather
than wrapping.
