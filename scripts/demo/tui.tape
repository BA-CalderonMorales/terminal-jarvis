Output docs/demo-tui.gif
Set FontSize 14
Set Width 1000
Set Height 620
Set Padding 20
Set Shell bash

# Terminal Jarvis interactive demo -- the centered command center.
# Prerequisite: cargo build --release. From the repository root:
#   vhs scripts/demo/tui.tape
# Hermetic: a throwaway session home, so the story always starts from
# "active: none" whatever the recording machine's own state is.

Hide
Type "export TERMINAL_JARVIS_HOME=$(mktemp -d)"
Enter
Type "./target/release/terminal-jarvis tui"
Enter
Sleep 1s
Show

# 1. boot: the centered first-boot primer under the merged header
Sleep 1s

# 2. the fleet picker -- content owns the space
Type "list"
Enter
Sleep 2s

# 3. readiness dashboard
Type "status"
Enter
Sleep 2s

# 4. harness details, centered
Type "show codex"
Enter
Sleep 2s

# 5. the command surface
Type "help"
Enter
Sleep 2s

# 6. home again, then leave with a clean shell
Type "home"
Enter
Sleep 1.5s
Type "exit"
Enter
Sleep 1s
