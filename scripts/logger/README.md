# Terminal Jarvis Logging System

Professional logging system for Terminal Jarvis scripts with timestamps and colored output, following SOLID principles.

## Features

- **Professional timestamps**: log4net style format (YYYY-MM-DD HH:MM:SS.mmm)
- **Colored output**: Different colors for different log levels
- **Configurable log levels**: ERROR, WARN, INFO, SUCCESS, DEBUG
- **CI/CD friendly**: Automatically disables colors in CI environments
- **Modular design**: Following SOLID principles for maintainability

## Usage

### Basic Usage

Source the logger in your script:

```bash
#!/bin/bash

# Source the logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

# Use logging functions
log_info_if_enabled "Starting process..."
log_success_if_enabled "Operation completed successfully"
log_error_if_enabled "Something went wrong"
log_warn_if_enabled "This is a warning"
log_debug_if_enabled "Debug information"
```

### Log Levels

- **ERROR** (Red): Critical errors that require immediate attention
- **WARN** (Yellow): Warnings about potential issues  
- **INFO** (Blue): General informational messages
- **SUCCESS** (Green): Successful operations and confirmations
- **DEBUG** (Purple): Detailed debugging information

### Special Formatting

```bash
# Section headers
log_header "Deployment Phase 1"

# Visual separators
log_separator

# Progress indicators
log_progress "Building application"
# ... long running operation ...
log_progress_done  # or log_progress_failed
```

### Configuration

Set environment variables to control logging behavior:

```bash
# Set log level (ERROR, WARN, INFO, SUCCESS, DEBUG)
export LOG_LEVEL="INFO"

# Disable colors (automatically done in CI)
export LOG_ENABLE_COLORS="false"

# Custom timestamp format
export LOG_TIMESTAMP_FORMAT='%Y-%m-%d %H:%M:%S.%3N'
```

## File Structure

- `logger.sh` - Main interface and entry point
- `log_utils.sh` - Core logging functions and utilities
- `log_config.sh` - Configuration and settings

## Example Output

```
[2025-08-12 14:30:15.123] INFO    Starting deployment process...
[2025-08-12 14:30:15.456] SUCCESS Version validation passed
[2025-08-12 14:30:16.789] WARN    Skipping optional step
[2025-08-12 14:30:17.012] ERROR   Deployment failed: missing file
```

## Integration with Existing Scripts

Replace emoji-based output with professional logging:

```bash
# Old approach
echo "✅ Success: Operation completed"
echo "❌ Error: Something failed"

# New approach  
log_success_if_enabled "Operation completed"
log_error_if_enabled "Something failed"
```

This provides consistent, professional output across all Terminal Jarvis scripts while maintaining readability and functionality.
