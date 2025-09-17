#!/bin/bash

# Logging Configuration for Terminal Jarvis Scripts
# Centralized configuration following SOLID principles

# Log level configuration
# Set to control what level of logs are displayed
# Options: ERROR, WARN, INFO, SUCCESS, DEBUG
export LOG_LEVEL="${LOG_LEVEL:-INFO}"

# Timestamp format configuration
# Uses log4net style by default: YYYY-MM-DD HH:MM:SS.mmm
export LOG_TIMESTAMP_FORMAT="${LOG_TIMESTAMP_FORMAT:-'%Y-%m-%d %H:%M:%S.%3N'}"

# Color configuration - can be disabled for CI/non-interactive environments
export LOG_ENABLE_COLORS="${LOG_ENABLE_COLORS:-true}"

# Log output configuration
export LOG_OUTPUT="${LOG_OUTPUT:-/dev/stdout}"

# Special CI/CD environment handling
if [[ "${CI:-}" == "true" ]] || [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
    export LOG_ENABLE_COLORS="false"
fi

# Function to check if a log level should be displayed
should_log_level() {
    local message_level="$1"
    local configured_level="$LOG_LEVEL"
    
    # Define log level hierarchy (higher number = more verbose)
    declare -A level_values=(
        ["ERROR"]=1
        ["WARN"]=2
        ["INFO"]=3
        ["SUCCESS"]=3
        ["DEBUG"]=4
    )
    
    local message_value=${level_values[$message_level]:-0}
    local config_value=${level_values[$configured_level]:-3}
    
    [[ $message_value -le $config_value ]]
}
