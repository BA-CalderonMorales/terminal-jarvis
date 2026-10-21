Output docs/demo-tui.gif
Set FontSize 14
Set Width 1000
Set Height 560
Set Padding 20
Set Shell bash

# Terminal Jarvis interactive demo -- the centered command center.
# Prerequisite: cargo build --release. From the repository root:
#   vhs scripts/demo/tui.tape
# Hermetic: a throwaway session home, so the story always starts from
# "active: none" whatever the recording machine's own state is.
#
# The flow answers the two questions a README gif has to answer: what is
# installed here, and what does picking one actually do. Every step settles
# before the next one types, so no frame is caught mid-keystroke.

Hide
Type "export TERMINAL_JARVIS_HOME=$(mktemp -d)"
Enter
Type "./target/release/terminal-jarvis tui"
Enter
Sleep 1s
Show

# 1. boot: the centered first-boot primer under the merged header
Sleep 1.5s

# 2. the fleet picker -- content owns the space
Type "list"
Enter
Sleep 2.5s

# 3. readiness dashboard
Type "status"
Enter
Sleep 2.5s

# 4. harness details, centered
Type "show codex"
Enter
Sleep 2.5s

# 5. the switch itself -- the header leaves "ACTIVE none"
Type "codex"
Enter
Sleep 2.5s

# 6. a preview, not a run: the plan for a capability
Type "plan codex headless"
Enter
Sleep 2.5s

# 7. the safety spine: an unopted dangerous run is refused, not run
Type "run codex -- --version"
Enter
Sleep 2.5s

# 8. a named theme, so the cycle can also be driven deliberately
Type "/theme solarized"
Enter
Sleep 3s

# 9. the command surface
Type "help"
Enter
Sleep 2.5s

# 10. the vim moment -- Esc for NORMAL, j/k scroll the help table, i returns
Sleep 0.5s
Escape
Sleep 1s
Type "j"
Sleep 0.6s
Type "j"
Sleep 0.6s
Type "k"
Sleep 0.6s
Type "i"
Sleep 0.8s

# 11. home again, and leave the frame settled there -- no shell prompt, no
#     half-typed line as the last thing a visitor sees
Type "home"
Enter
Sleep 3s
