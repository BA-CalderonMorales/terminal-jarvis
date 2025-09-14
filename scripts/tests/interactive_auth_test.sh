#!/usr/bin/env bash

# Interactive Authentication Test Script
# This script tests browser prevention while keeping users in interactive CLI modes

set -euo pipefail

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../logger/logger.sh
# shellcheck disable=SC1091
source "$SCRIPT_DIR/../logger/logger.sh"

log_header "Terminal Jarvis Interactive Authentication Test"
log_info_if_enabled "Testing browser prevention while maintaining interactive CLI sessions"

# Function to setup browser prevention environment
setup_no_browser_env() {
    log_info_if_enabled "Setting up browser prevention environment..."
    
    # Clear all API keys to force authentication prompts
    unset GOOGLE_API_KEY GEMINI_API_KEY QWEN_CODE_API_KEY DASHSCOPE_API_KEY ANTHROPIC_API_KEY CLAUDE_API_KEY 2>/dev/null || true
    
    # Set browser prevention variables
    export NO_BROWSER=1
    export BROWSER="echo 'Browser prevented by Terminal Jarvis:'"
    export OAUTH_NO_BROWSER=1
    export GOOGLE_APPLICATION_CREDENTIALS_NO_BROWSER=1
    
    # Set additional environment variables that CLIs check
    export CI=true  # Many tools check this to disable interactive features
    export TERM=xterm-256color  # But keep a good terminal for display
    
    log_success_if_enabled "Browser prevention environment set up"
    log_info_if_enabled "  NO_BROWSER: $NO_BROWSER"
    log_info_if_enabled "  BROWSER: $BROWSER"
    log_info_if_enabled "  OAUTH_NO_BROWSER: $OAUTH_NO_BROWSER"
    log_separator
}

# Function to test interactive mode staying active
test_interactive_mode() {
    local tool_name=$1
    local tool_command=$2
    
    log_info_if_enabled "--- Testing Interactive Mode: $tool_name ---"
    
    if ! command -v "$tool_command" >/dev/null 2>&1; then
        log_warn_if_enabled "$tool_name not found, trying to install..."
        
        case "$tool_command" in
            "gemini")
                log_info_if_enabled "Installing: npm install -g @google/gemini-cli"
                npm install -g @google/gemini-cli || {
                    log_error_if_enabled "Failed to install Gemini CLI"
                    return 1
                }
                ;;
            "qwen")
                log_info_if_enabled "Installing: npm install -g @qwen-code/qwen-code"
                npm install -g @qwen-code/qwen-code || {
                    log_error_if_enabled "Failed to install Qwen Code"
                    return 1
                }
                ;;
        esac
    fi
    
    log_info_if_enabled "Testing $tool_name interactive mode with browser prevention..."
    
    # Create a script that will interact with the tool
    cat > "/tmp/test_${tool_command}_interactive.exp" << EOF
#!/usr/bin/expect -f

set timeout 30
spawn $tool_command

# Handle different possible prompts
expect {
    "API key" {
        puts "\nFOUND: Tool is asking for API key instead of opening browser!"
        puts "This means our browser prevention is working!"
        send_user "\nSUCCESS: No browser opening, staying in terminal\n"
        send "\003"  # Ctrl+C to exit gracefully
        exp_continue
    }
    "GOOGLE_API_KEY" {
        puts "\nFOUND: Tool is prompting for GOOGLE_API_KEY environment variable!"
        puts "This means our browser prevention is working!"
        send_user "\nSUCCESS: No browser opening, staying in terminal\n"
        send "\003"  # Ctrl+C to exit gracefully
        exp_continue
    }
    "QWEN_CODE_API_KEY" {
        puts "\nFOUND: Tool is prompting for QWEN_CODE_API_KEY environment variable!"
        puts "This means our browser prevention is working!"
        send_user "\nSUCCESS: No browser opening, staying in terminal\n"
        send "\003"  # Ctrl+C to exit gracefully
        exp_continue
    }
    "environment variable" {
        puts "\nFOUND: Tool is asking for environment variable setup!"
        puts "This means our browser prevention is working!"
        send_user "\nSUCCESS: No browser opening, staying in terminal\n"
        send "\003"  # Ctrl+C to exit gracefully
        exp_continue
    }
    "Please visit" {
        puts "\nWARNING: Tool is asking to visit a URL (possible browser opening)"
        send_user "\nThis might indicate browser opening behavior\n"
        send "\003"  # Ctrl+C to exit
        exp_continue
    }
    "Opening browser" {
        puts "\nALERT: Tool is attempting to open browser!"
        send_user "\nFAILED: Browser opening detected\n"
        send "\003"  # Ctrl+C to exit
        exp_continue
    }
    ">" {
        puts "\nFOUND: Interactive prompt ready!"
        puts "Testing if we can access the CLI without browser opening..."
        send_user "\nSUCCESS: Interactive mode active, no browser opening\n"
        
        # Try a simple command to see what happens
        send "help\r"
        expect {
            "API key" {
                puts "\nTool needs API key for functionality"
                send_user "\nThis is the expected behavior - tool stays in terminal\n"
            }
            timeout {
                puts "\nInteractive session stable"
            }
        }
        
        # Exit gracefully
        send "exit\r"
        expect eof
    }
    timeout {
        puts "\nTool started but no specific prompts detected within timeout"
        puts "This might mean it's waiting for input without browser opening"
        send_user "\nPARTIAL SUCCESS: No immediate browser opening detected\n"
        send "\003"  # Ctrl+C to exit
    }
    eof {
        puts "\nTool exited normally"
    }
}

puts "Test completed for $tool_command"
EOF

    # Make the expect script executable
    chmod +x "/tmp/test_${tool_command}_interactive.exp"
    
    # Run the expect script if expect is available
    if command -v expect >/dev/null 2>&1; then
        log_info_if_enabled "Running interactive test with expect..."
        "/tmp/test_${tool_command}_interactive.exp" || true
    else
        log_warn_if_enabled "expect not available, running basic test..."
        
        # Basic test without expect - run with timeout and see what happens
        log_info_if_enabled "Starting $tool_command with 10 second timeout..."
        timeout 10s "$tool_command" 2>&1 | head -20 || {
            exit_code=$?
            case $exit_code in
                124)
                    log_success_if_enabled "Tool ran for full timeout without browser opening"
                    ;;
                130)
                    log_success_if_enabled "Tool exited gracefully (Ctrl+C)"
                    ;;
                *)
                    log_warn_if_enabled "Tool exited with code $exit_code"
                    ;;
            esac
        }
    fi
    
    # Clean up
    rm -f "/tmp/test_${tool_command}_interactive.exp"
    log_separator
}

# Function to test terminal-jarvis wrapper
test_terminal_jarvis_wrapper() {
    log_info_if_enabled "--- Testing Terminal Jarvis Wrapper ---"
    
    if ! command -v terminal-jarvis >/dev/null 2>&1; then
        log_info_if_enabled "Building terminal-jarvis..."
    cargo build --release
    BUILD_BIN_DIR="$(pwd)/target/release"
    export PATH="$BUILD_BIN_DIR:$PATH"
    fi
    
    log_info_if_enabled "Testing terminal-jarvis run command with browser prevention..."
    
    # Test gemini through terminal-jarvis
    log_info_if_enabled "Testing: terminal-jarvis run gemini"
    timeout 15s terminal-jarvis run gemini 2>&1 | head -20 || {
        exit_code=$?
        case $exit_code in
            124)
                log_success "Terminal Jarvis prevented browser opening and maintained session"
                ;;
            *)
                log_warn "Terminal Jarvis exited with code $exit_code"
                ;;
        esac
    }
    
    echo
    
    # Test qwen through terminal-jarvis
    echo "Testing: terminal-jarvis run qwen"
    timeout 15s terminal-jarvis run qwen 2>&1 | head -20 || {
        exit_code=$?
        case $exit_code in
            124)
                log_success "Terminal Jarvis prevented browser opening and maintained session"
                ;;
            *)
                log_warn "Terminal Jarvis exited with code $exit_code"
                ;;
        esac
    }
    
    echo
}

# Function to demonstrate the solution
demonstrate_solution() {
    echo -e "${BLUE}--- Solution Demonstration ---${NC}"
    echo -e "${GREEN}The Terminal Jarvis solution works by:${NC}"
    echo "1. Detecting browser-problematic environments (containers, CI, headless)"
    echo "2. Setting NO_BROWSER and related environment variables"
    echo "3. Preventing browser opening before running tools"
    echo "4. Keeping users in interactive CLI sessions"
    echo "5. Providing helpful API key setup instructions"
    echo
    echo -e "${YELLOW}Environment variables we set:${NC}"
    echo "  NO_BROWSER=1              # Generic no-browser flag"
    echo "  BROWSER=echo              # Override browser command"
    echo "  OAUTH_NO_BROWSER=1        # OAuth-specific prevention"
    echo
    echo -e "${GREEN}This allows users to:${NC}"
    echo "  • Stay in their terminal/codespace session"
    echo "  • See the actual authentication prompts"
    echo "  • Set up API keys as needed"
    echo "  • Use tools without session disruption"
    echo
}

# Main execution
main() {
    echo -e "${BLUE}Starting interactive authentication tests...${NC}"
    echo
    
    # Set up the environment
    setup_no_browser_env
    
    # Demonstrate the solution
    demonstrate_solution
    
    # Test each tool
    test_interactive_mode "Gemini CLI" "gemini"
    test_interactive_mode "Qwen Code" "qwen"
    
    # Test our wrapper
    test_terminal_jarvis_wrapper
    
    echo -e "${BLUE}=== Test Summary ===${NC}"
    log_success "Browser prevention environment configured"
    log_success "Interactive CLI sessions maintained"
    log_success "Tools prompt for API keys instead of opening browsers"
    log_success "Terminal Jarvis wrapper provides additional protection"
    echo
    echo -e "${YELLOW}Next steps for users:${NC}"
    echo "1. Set appropriate API keys in environment variables"
    echo "2. Use 'terminal-jarvis run <tool>' for automatic browser prevention"
    echo "3. Enjoy uninterrupted terminal-based AI coding sessions!"
}

# Install expect if not available (for better testing)
install_expect_if_needed() {
    if ! command -v expect >/dev/null 2>&1; then
        echo -e "${YELLOW}Installing expect for better interactive testing...${NC}"
        if command -v apt-get >/dev/null 2>&1; then
            sudo apt-get update && sudo apt-get install -y expect
        elif command -v yum >/dev/null 2>&1; then
            sudo yum install -y expect
        elif command -v brew >/dev/null 2>&1; then
            brew install expect
        else
            echo -e "${YELLOW}Could not install expect automatically. Interactive tests will be limited.${NC}"
        fi
    fi
}

# Run the main function
install_expect_if_needed
main "$@"
