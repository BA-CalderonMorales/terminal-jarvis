# What is this?

## The catalog truth

A data-driven harness catalog for AI coding agents. It maps **25 coding-agent
CLIs** through a shared **9-capability contract** and fails closed unless a
capability's declared support, evidence, platform, and freshness permit it.
Catalog presence does not mean that a capability is supported; each capability
reports its own evidence-backed state.

## Why it exists

Terminal Jarvis is built to make switching between coding agents a single
keystroke -- a context switcher, not another agent. It deliberately does not
reinvent identity or credentials: every harness keeps its own auth, and the
tool leans on what is baked in.

The agent space moves as fast as the models do. New coding CLIs land
constantly, and this catalog is a way to stay aware of them, and of how they
change over time. The shared capability contract captures which conventions
persist across tools -- `/clear`, `/exit` and `/quit`, `/login` and `/auth`,
and the command sets agents expose in their own TUIs -- so that nobody ends
up vendor-locked to one thing in the long run.

More than likely a few of these agents will remain. Terminal Jarvis is meant
to record how things stand today, and to make moving between them tomorrow
cheap.
