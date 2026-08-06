# Terminal Jarvis interactive demo -- the switcher, end to end.
# Deterministic and credential-free: every beat renders the same on any
# machine. Agent-launch guards are shown via the decline path (n), and the
# README documents what chatting with a real agent looks like.
#
# Run from the repository root: vhs scripts/demo/tui.tape

Output docs/demo-tui.gif
Set FontSize 18
Set Width 960
Set Height 600
Set Padding 30
Set Shell bash

Type "terminal-jarvis"
Enter
Sleep 4s
# 1. boot: banner, active harness, readiness, modeline

Type "list"
Enter
Sleep 2s
# 2. the numbered picker: every row is one keypress away

Type "5"
Enter
Sleep 2s
# 3. a number switches agents instantly -- codex becomes current

Type "home"
Enter
Sleep 2s
# 4. back to the pristine welcome frame

Type "status"
Enter
Sleep 4s
# 5. readiness report, no version probes -- a responsive dashboard

Type "pi"
Enter
Sleep 2s
# 6. typing a name switches too

Type "list"
Enter
Sleep 2s
# 7. the picker again: pi is current now

Type "/codex"
Enter
Sleep 3s
# 8. a slashed launch shows the guarded plan before anything runs

Type "n"
Enter
Sleep 2s
# 9. declining never touches the agent

Type "exit"
Enter
Sleep 2s
# 10. clean handoff
