pub const PLAIN: &str = "Terminal Jarvis\n\
     Headless command center for coding-agent harnesses\n\n\
     usage:\n\
       terminal-jarvis [harness] [args...] [-- child-args...]\n\
       terminal-jarvis run [harness] [capability] [args...] [-- child-args...]\n\
       terminal-jarvis version [--verbose|--info|-v]\n\
       terminal-jarvis list\n\
       terminal-jarvis check [--verbose]\n\
       terminal-jarvis use <harness>\n\
       terminal-jarvis current\n\
       terminal-jarvis show <harness>\n\
       terminal-jarvis plan [harness] <capability>\n\
       terminal-jarvis install <harness> [lifecycle options]\n\
       terminal-jarvis update [harness] [lifecycle options]\n\
       terminal-jarvis self-update [lifecycle options]\n\
       terminal-jarvis auth help <harness>\n\
       terminal-jarvis config show\n\
       terminal-jarvis cache status\n\
       terminal-jarvis security [status|audit|harness]\n\
       terminal-jarvis gate [status|list|enable [trivy]|disable|run [trivy]]\n\n\
      global flags (anywhere before --):\n\
        --help, -h      show command help\n\
        --version, -v   print the version\n\
        --info          print version with provenance\n\
        --plain         stable line-oriented output\n\
        --json          one schema-version-1 JSON object\n\
        --no-color      disable terminal color\n\
        --verbose       expand check or version output\n\
        --dry-run       preview lifecycle operations without effects\n\
        --no-input      guarantee that no prompt is opened\n\
        --confirm=OP:TARGET\n\
                        bind noninteractive intent to one operation\n\
        --allow-dangerous\n\
                        separately opt in to dangerous execution\n\
        --              forward following flags to run/direct children\n\
        --update        compatibility alias for self-update\n\n\
      capabilities:\n\
       download update headless version stats models security yolo ui\n\n\
     examples:\n\
       terminal-jarvis use opencode\n\
       terminal-jarvis plan codex headless\n\
       terminal-jarvis run opencode headless --dry-run\n\
       terminal-jarvis gate enable trivy\n\n\
     experimental:\n\
       TERMINAL_JARVIS_EXPERIMENTAL_UI=1 terminal-jarvis experimental dashboard\n\n\
     legacy aliases:\n\
       tools -> list, status -> check, info <harness> -> show <harness>\n\
       install <harness> -> run <harness> download\n\
       update <harness> -> run <harness> update\n";
