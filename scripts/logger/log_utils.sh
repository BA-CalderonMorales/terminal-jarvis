#!/bin/bash

# Log Utility Functions for Terminal Jarvis Scripts
# Provides standardized logging with timestamps and colored output
# Following SOLID principles: Single Responsibility for logging

# Color definitions (ANSI escape codes)
readonly LOG_COLOR_RED='\033[0;31m'
readonly LOG_COLOR_GREEN='\033[0;32m'
readonly LOG_COLOR_YELLOW='\033[0;33m'
readonly LOG_COLOR_BLUE='\033[0;34m'
readonly LOG_COLOR_PURPLE='\033[0;35m'
readonly LOG_COLOR_CYAN='\033[0;36m'
readonly LOG_COLOR_WHITE='\033[0;37m'
readonly LOG_COLOR_RESET='\033[0m'

# Log level definitions
readonly LOG_LEVEL_ERROR="ERROR"
readonly LOG_LEVEL_WARN="WARN"
readonly LOG_LEVEL_INFO="INFO"
readonly LOG_LEVEL_SUCCESS="SUCCESS"
readonly LOG_LEVEL_DEBUG="DEBUG"

# Get current timestamp in log4net style format
get_log_timestamp() {
    date '+%Y-%m-%d %H:%M:%S.%3N'
}

# Base logging function
# Usage: log_message "LEVEL" "COLOR" "MESSAGE"
log_message() {
    local level="$1"
    local color="$2"
    local message="$3"
    local timestamp
    timestamp=$(get_log_timestamp)
    
    printf "%b[%s] %-7s %s%b\n" \
        "$color" \
        "$timestamp" \
        "$level" \
        "$message" \
        "$LOG_COLOR_RESET"
}

# Specific log level functions
log_error() {
    log_message "$LOG_LEVEL_ERROR" "$LOG_COLOR_RED" "$1"
}

log_warn() {
    log_message "$LOG_LEVEL_WARN" "$LOG_COLOR_YELLOW" "$1"
}

log_info() {
    log_message "$LOG_LEVEL_INFO" "$LOG_COLOR_BLUE" "$1"
}

log_success() {
    log_message "$LOG_LEVEL_SUCCESS" "$LOG_COLOR_GREEN" "$1"
}

log_debug() {
    log_message "$LOG_LEVEL_DEBUG" "$LOG_COLOR_PURPLE" "$1"
}

# Special formatting functions
log_header() {
    local header="$1"
    printf "\n%b=== %s ===%b\n\n" \
        "$LOG_COLOR_CYAN" \
        "$header" \
        "$LOG_COLOR_RESET"
}

log_separator() {
    printf "%b%s%b\n" \
        "$LOG_COLOR_WHITE" \
        "$(printf '%*s' 50 '' | tr ' ' '-')" \
        "$LOG_COLOR_RESET"
}

# Progress indicator (for long running operations)
log_progress() {
    local message="$1"
    printf "%b[%s] %-7s %s...%b" \
        "$LOG_COLOR_CYAN" \
        "$(get_log_timestamp)" \
        "PROGRESS" \
        "$message" \
        "$LOG_COLOR_RESET"
}

log_progress_done() {
    printf "%b DONE%b\n" "$LOG_COLOR_GREEN" "$LOG_COLOR_RESET"
}

log_progress_failed() {
    printf "%b FAILED%b\n" "$LOG_COLOR_RED" "$LOG_COLOR_RESET"
}

# Section header for deployment steps
log_section() {
    local section_name="$1"
    printf "\n%b[%s] %-7s %s%b\n" \
        "$LOG_COLOR_CYAN" \
        "$(get_log_timestamp)" \
        "SECTION" \
        "$section_name" \
        "$LOG_COLOR_RESET"
}
