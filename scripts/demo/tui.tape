# Terminal Jarvis interactive demo -- the context command center.
# Boot the tui, survey the fleet, switch agents, preview plans, flip the
# debug view, and leave. Self-contained: no agent credentials or external
# installs needed to record. From the repository root:
#   vhs scripts/demo/tui.tape

Output docs/demo-tui.gif
Set FontSize 16
Set Width 1100
Set Height 820
Set Padding 24
Set Shell bash

Type "terminal-jarvis tui"
Enter
Sleep 6s
# 1. boot: banner, horizontal status row, and the context indicator

Type "list"
Enter
Sleep 4s
# 2. the numbered picker -- every row is one keypress away

Type "10"
Enter
Sleep 3s
# 3. a bare number switches agents instantly

Type "status"
Enter
Sleep 4s
# 4. the readiness dashboard: active agent, fleet summary

Type "plan pi headless"
Enter
Sleep 4s
# 5. plan previews a command without running it

Type "help"
Enter
Sleep 5s
# 6. the tui's own command table -- no headless-only flags

Type "/debug on"
Enter
Sleep 3s
# 7. /debug lifts the clean frame to show the raw view

Type "status"
Enter
Sleep 4s
# 8. the same command, verbose

Type "/debug off"
Enter
Sleep 3s
# 9. and back to the clean frame

Type "home"
Enter
Sleep 4s
# 10. home resets the frame

Type "exit"
Enter
Sleep 4s
# 11. leave the switcher
