# Terminal Jarvis interactive demo -- the switcher, end to end.
# Boot the tui, survey it, launch Codex and hand the terminal over, exit it
# cleanly, launch Pi, chat with it, exit Pi, then leave the switcher.
#
# Agent beats need your agent credentials and a working codex/pi install:
# record with your environment loaded. From the repository root:
#   vhs scripts/demo/tui.tape

Output docs/demo-tui.gif
Set FontSize 16
Set Width 1100
Set Height 820
Set Padding 24
Set Shell bash

Type "terminal-jarvis tui"
Enter
Sleep 5s
# 1. boot: banner, active harness, readiness, modeline

Type "list"
Enter
Sleep 3s
# 2. the numbered picker -- every row is one keypress away

Type "home"
Enter
Sleep 3s
# 3. back to the pristine welcome frame

Type "status"
Enter
Sleep 3s
# 4. the readiness dashboard: active agent, fleet summary

Type "/codex"
Enter
Sleep 3s
# 5. the guarded launch plan -- nothing runs unchecked

Type "y"
Enter
Sleep 10s
# 6. Codex takes the terminal

Type "/exit"
Enter
Sleep 5s
# 7. Codex hands the terminal back

Type "/pi"
Enter
Sleep 3s
Type "y"
Enter
Sleep 10s
# 8. Pi takes the terminal

Type "hi!"
Enter
Sleep 10s
# 9. say hi, wait for Pi's answer

Type "/exit"
Enter
Sleep 5s
# 10. Pi hands the terminal back

Type "/exit"
Enter
Sleep 3s
# 11. leave the switcher
