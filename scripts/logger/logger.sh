#!/bin/bash

# Main Logger Interface for Terminal Jarvis Scripts
# Entry point for all logging functionality
# Follows SOLID principles with clear separation of concerns

# Get the directory where this script is located
LOGGER_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source configuration and utilities
# shellcheck source=./log_config.sh
source "$LOGGER_DIR/log_config.sh"

# shellcheck source=./log_utils.sh  
source "$LOGGER_DIR/log_utils.sh"

# Enhanced logging functions that respect configuration
log_error_if_enabled() {
    should_log_level "ERROR" && log_error "$1"
}

log_warn_if_enabled() {
    should_log_level "WARN" && log_warn "$1"
}

log_info_if_enabled() {
    should_log_level "INFO" && log_info "$1"
}

log_success_if_enabled() {
    should_log_level "SUCCESS" && log_success "$1"
}

log_debug_if_enabled() {
    should_log_level "DEBUG" && log_debug "$1"
}

# Convenience function to source this logger from other scripts
# Usage: source_logger "/path/to/scripts/logger"
source_logger() {
    local logger_path="$1"
    if [[ -f "$logger_path/logger.sh" ]]; then
        # shellcheck source=./logger.sh
        source "$logger_path/logger.sh"
    else
        echo "ERROR: Logger not found at $logger_path/logger.sh" >&2
        exit 1
    fi
}

# Export functions for use in other scripts
export -f log_error_if_enabled
export -f log_warn_if_enabled  
export -f log_info_if_enabled
export -f log_success_if_enabled
export -f log_debug_if_enabled
export -f log_header
export -f log_separator
export -f log_progress
export -f log_progress_done
export -f log_progress_failed
