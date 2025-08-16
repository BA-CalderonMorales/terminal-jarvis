# Terminal Jarvis Application Flow Diagram

This document provides clean visual flow diagrams showing how Terminal Jarvis executes from startup through tool interaction.

## Application Execution Flow

```
START
  │
  ▼
main.rs ────► Initialize Tokio runtime
  │
  ▼
cli.rs ─────► Parse command line arguments
  │
  ▼
Command specified?
  │
  ├─ YES ──► Execute command directly
  │          │
  │          ▼
  │        cli_logic domain handlers
  │          │
  │          ▼
  │        END
  │
  └─ NO ───► Interactive Mode
             │
             ▼
           Theme System Init
             │
             ▼
        ┌─────────────────┐
        │ Interactive Loop │
        │                 │
        │ 1. Clear screen │
        │ 2. Show welcome │
        │ 3. Main menu    │
        │ 4. Get choice   │
        └─────────┬───────┘
                  │
                  ▼
            User Selection
                  │
        ┌─────────┼─────────┐
        │         │         │
        ▼         ▼         ▼
   AI Tools   Settings    Exit
      │         │         │
      ▼         ▼         ▼
 Tool Menu   Config    Program
      │      Operations   End
      ▼         │
 Tool Launch   ▼
      │    Return to
      ▼     Main Menu
Tool Execution
      │
      ▼
Return to Main Menu
```

## Tool Execution Flow

```
Tool Selected
  │
  ▼
tools Domain ──► Tool Detection
  │              ├─ Check PATH
  │              ├─ Verify version
  │              └─ Cache results
  ▼
Tool Available?
  │
  ├─ YES ──► Continue to Authentication
  │
  └─ NO ───► services Domain
             │
             ▼
           Install Tool
             │
             ▼
         Install Success?
             │
             ├─ YES ──► Continue
             │
             └─ NO ───► Show Error & Exit
                          │
                          ▼
                        Return to Menu

Authentication Check
  │
  ▼
auth_manager Domain ──► Environment Detection
  │                     ├─ Check CI/SSH
  │                     ├─ Detect headless
  │                     └─ Browser prevention
  ▼
Setup Required?
  │
  ├─ YES ──► Show Setup Guidance
  │          │
  │          ▼
  │        Wait for User Setup
  │
  └─ NO ───► Launch Tool
             │
             ▼
        Process Manager
             │
             ▼
        Execute Tool
             │
             ▼
    Session Continuation Monitor
             │
             ▼
        Tool Complete
             │
             ▼
       Return to Menu
```

## Domain Architecture

```
┌─────────────┐
│   cli.rs    │ ──► Routes commands to appropriate handlers
└─────────────┘
      │
      ▼
┌─────────────┐
│ cli_logic   │ ──► Main business logic coordinator
└─────────────┘
      │
      ├──────────────────────────────────────┐
      │                                      │
      ▼                                      ▼
┌─────────────┐                    ┌─────────────┐
│   theme     │ ──► Visual styling │   config    │ ──► Settings/Cache
└─────────────┘     all domains    └─────────────┘     all operations
      │                                      │
      ▼                                      ▼
┌─────────────┐                    ┌─────────────┐
│    tools    │ ◄─────────────────►│ auth_manager│
└─────────────┘                    └─────────────┘
Tool detection    Environment setup    Browser prevention
      │                                      │
      ▼                                      ▼
┌─────────────┐                    ┌─────────────┐
│  services   │ ◄──────────────────┤progress_utils│
└─────────────┘                    └─────────────┘
External APIs     User feedback to all domains
```

## Domain Responsibilities

**Core Flow Control:**
- `cli.rs` → Command parsing and routing
- `cli_logic` → Business logic coordination

**Supporting Systems:**
- `theme` → Visual consistency across all interfaces
- `config` → Settings and version caching for all operations
- `progress_utils` → User feedback and messaging

**Tool Management:**
- `tools` → Detection, execution, session management
- `auth_manager` → Environment checks, browser prevention
- `services` → External integrations (NPM, GitHub, package management)

**Cross-Domain Communication:**
- Theme system provides visual consistency to all domains
- Config system manages settings and cache for all operations
- Progress utils provide user feedback across all domains
- Auth manager coordinates with tools for environment setup
- Services domain supports tools with installation/updates

## Key Flow Characteristics

**Startup Sequence:**
1. `main.rs` initializes Tokio runtime
2. `cli.rs` parses commands and routes to handlers
3. Theme system initializes for visual consistency
4. Interactive mode or direct command execution

**Interactive Mode Features:**
1. Clean screen clearing and themed welcome interface
2. Professional menu styling with consistent colors
3. Seamless navigation between tool selection and settings
4. Graceful return to main menu after operations

**Tool Execution Pipeline:**
1. **Detection** → Verify tool availability and cache results
2. **Installation** → Auto-install missing tools via services domain
3. **Authentication** → Environment checks and browser prevention
4. **Execution** → Process management with session monitoring
5. **Monitoring** → Handle restarts and session continuation

**Domain Coordination Benefits:**
- Focused single responsibilities per domain
- Clean interfaces between domain boundaries
- Consistent theming across all user interactions
- Centralized configuration and caching
- Reliable error handling and user feedback

This architecture ensures reliable tool execution with a professional, consistent user experience.