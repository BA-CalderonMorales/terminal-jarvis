pub fn text() -> &'static str {
    "terminal-jarvis\n\n\
     usage:\n\
       terminal-jarvis [harness] [args...]\n\
       terminal-jarvis run [harness] [capability] [args...]\n\
       terminal-jarvis version [--verbose]\n\
       terminal-jarvis list\n\
       terminal-jarvis check\n\
       terminal-jarvis use <harness>\n\
       terminal-jarvis current\n\
       terminal-jarvis show <harness>\n\
       terminal-jarvis plan [harness] <capability>\n\
       terminal-jarvis install <harness>\n\
       terminal-jarvis update [harness]\n\
       terminal-jarvis auth help <harness>\n\
       terminal-jarvis config show\n\
       terminal-jarvis cache status\n\
       terminal-jarvis security [status|audit|harness]\n\n\
     capabilities:\n\
       download update headless version stats models security yolo ui\n\n\
     examples:\n\
       terminal-jarvis use opencode\n\
       terminal-jarvis run opencode\n\
       terminal-jarvis run opencode fix failing tests\n\
       terminal-jarvis run headless summarize this repository\n\
       terminal-jarvis opencode --help\n\n\
     legacy aliases:\n\
       tools -> list, status -> check, info <harness> -> show <harness>\n\
       install <harness> -> run <harness> download\n\
       update <harness> -> run <harness> update\n"
}
