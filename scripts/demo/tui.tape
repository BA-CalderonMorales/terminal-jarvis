# Terminal Jarvis interactive demo -- use it, run it, switch it.
# Boot the tui, survey it, switch agents by verb, launch Codex and hand the
# terminal over, exit cleanly, launch Pi, chat with it, exit Pi, then leave.
#
# Agent beats need your credentials and working codex/pi installs: record
# with your environment loaded. From the repository root:
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
# 1. boot: banner, active harness, readiness, modeline

Type "list"
Enter
Sleep 4s
# 2. the numbered picker -- every row is one keypress away

Type "use codex"
Enter
Sleep 4s
# 3. use <agent> switches the active harness

Type "home"
Enter
Sleep 4s
# 4. back to the pristine welcome frame

Type "status"
Enter
Sleep 4s
# 5. the readiness dashboard: active agent, fleet summary

Type "run codex"
Enter
Sleep 4s
Type "y"
Enter
Sleep 12s
# 6. run <agent> launches it and hands over the terminal

Type "/exit"
Enter
Sleep 6s
# 7. Codex hands the terminal back

Type "use pi"
Enter
Sleep 4s
# 8. switch to Pi

Type "run pi"
Enter
Sleep 4s
Type "y"
Enter
Sleep 10s
# 9. Pi takes the terminal

Type "hi!"
Enter
Sleep 10s
# 10. say hi, wait for Pi's answer

Type "/exit"
Enter
Sleep 6s
# 11. Pi hands the terminal back

Type "exit"
Enter
Sleep 4s
# 12. leave the switcher
