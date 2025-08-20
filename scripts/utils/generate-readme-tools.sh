#!/bin/bash

# Generate README tools sections from manifest
# Simple and reliable version using pure bash

set -e

# Source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../logger/logger.sh"

REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MANIFEST_FILE="$REPO_ROOT/tools-manifest.toml"

if [ ! -f "$MANIFEST_FILE" ]; then
    log_error_if_enabled "Error: tools-manifest.toml not found in repo root"
    exit 1
fi

log_info_if_enabled "Generating README tools sections from manifest..."

# Parse TOML and generate bullet list
generate_bullet_list() {
    echo "- **Supported Tools**:"
    
    local in_tool=false
    local tool_id=""
    local description=""
    local status=""
    
    while IFS= read -r line; do
        line=$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        
        if [[ "$line" == "[[tools]]" ]]; then
            # Output previous tool if we have complete info
            if [[ -n "$tool_id" && -n "$description" ]]; then
                local status_text=""
                case "$status" in
                    "testing") status_text=" (Testing)" ;;
                    "new") status_text=" (New)" ;;
                esac
                echo "  - \`$tool_id\` - $description$status_text"
            fi
            
            # Reset for new tool
            in_tool=true
            tool_id=""
            description=""
            status=""
        elif [[ "$in_tool" == true && "$line" =~ ^id\ =\ \"(.+)\"$ ]]; then
            tool_id="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^description\ =\ \"(.+)\"$ ]]; then
            description="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^status\ =\ \"(.+)\"$ ]]; then
            status="${BASH_REMATCH[1]}"
        fi
    done < "$MANIFEST_FILE"
    
    # Output the last tool
    if [[ -n "$tool_id" && -n "$description" ]]; then
        local status_text=""
        case "$status" in
            "testing") status_text=" (Testing)" ;;
            "new") status_text=" (New)" ;;
        esac
        echo "  - \`$tool_id\` - $description$status_text"
    fi
}

# Generate tools table
generate_tools_table() {
    echo "| Tool       | Description                               | Status     | Installation Command                         |"
    echo "| ---------- | ----------------------------------------- | ---------- | -------------------------------------------- |"
    
    local in_tool=false
    local tool_id=""
    local description=""
    local status=""
    local install_cmd=""
    
    while IFS= read -r line; do
        line=$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        
        if [[ "$line" == "[[tools]]" ]]; then
            # Output previous tool if we have complete info
            if [[ -n "$tool_id" && -n "$description" && -n "$status" && -n "$install_cmd" ]]; then
                local status_display=""
                case "$status" in
                    "stable") status_display="Stable" ;;
                    "testing") status_display="Testing" ;;
                    "new") status_display="New" ;;
                esac
                printf "| %-10s | %-41s | %-10s | %-44s |\n" "\`$tool_id\`" "$description" "$status_display" "\`$install_cmd\`"
            fi
            
            # Reset for new tool
            in_tool=true
            tool_id=""
            description=""
            status=""
            install_cmd=""
        elif [[ "$in_tool" == true && "$line" =~ ^id\ =\ \"(.+)\"$ ]]; then
            tool_id="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^description\ =\ \"(.+)\"$ ]]; then
            description="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^status\ =\ \"(.+)\"$ ]]; then
            status="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^installation_command\ =\ \"(.+)\"$ ]]; then
            install_cmd="${BASH_REMATCH[1]}"
        fi
    done < "$MANIFEST_FILE"
    
    # Output the last tool
    if [[ -n "$tool_id" && -n "$description" && -n "$status" && -n "$install_cmd" ]]; then
        local status_display=""
        case "$status" in
            "stable") status_display="Stable" ;;
            "testing") status_display="Testing" ;;
            "new") status_display="New" ;;
        esac
        printf "| %-10s | %-41s | %-10s | %-44s |\n" "\`$tool_id\`" "$description" "$status_display" "\`$install_cmd\`"
    fi
}

# Generate testing phase note
generate_testing_note() {
    local testing_tools=()
    local in_tool=false
    local tool_id=""
    local status=""
    
    while IFS= read -r line; do
        line=$(echo "$line" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        
        if [[ "$line" == "[[tools]]" ]]; then
            # Check previous tool
            if [[ -n "$tool_id" && "$status" == "testing" ]]; then
                testing_tools+=("$tool_id")
            fi
            
            # Reset for new tool
            in_tool=true
            tool_id=""
            status=""
        elif [[ "$in_tool" == true && "$line" =~ ^id\ =\ \"(.+)\"$ ]]; then
            tool_id="${BASH_REMATCH[1]}"
        elif [[ "$in_tool" == true && "$line" =~ ^status\ =\ \"(.+)\"$ ]]; then
            status="${BASH_REMATCH[1]}"
        fi
    done < "$MANIFEST_FILE"
    
    # Check the last tool
    if [[ -n "$tool_id" && "$status" == "testing" ]]; then
        testing_tools+=("$tool_id")
    fi
    
    if [[ ${#testing_tools[@]} -gt 0 ]]; then
        local tools_list=""
        case ${#testing_tools[@]} in
            1) tools_list="${testing_tools[0]}" ;;
            2) tools_list="${testing_tools[0]} and ${testing_tools[1]}" ;;
            *) 
                for ((i=0; i<${#testing_tools[@]}-1; i++)); do
                    if [[ $i -eq 0 ]]; then
                        tools_list="${testing_tools[i]}"
                    else
                        tools_list="$tools_list, ${testing_tools[i]}"
                    fi
                done
                tools_list="$tools_list, and ${testing_tools[-1]}"
                ;;
        esac
        
        echo "**Testing Phase**: Looking for testers! $tools_list are new additions — see the [Testing Guide](docs/TESTING.md)."
    fi
}

# Test the functions by outputting their content
echo ""
echo "=== BULLET LIST ==="
BULLET_LIST=$(generate_bullet_list)
echo "$BULLET_LIST"
echo ""
echo "=== TOOLS TABLE ==="
TOOLS_TABLE=$(generate_tools_table)
echo "$TOOLS_TABLE"
echo ""
echo "=== TESTING NOTE ==="
TESTING_NOTE=$(generate_testing_note)
echo "$TESTING_NOTE"
echo ""

# Now actually update the README
README_FILE="$REPO_ROOT/README.md"
TEMP_FILE="$REPO_ROOT/README.md.tmp"

if [ ! -f "$README_FILE" ]; then
    log_error_if_enabled "Error: README.md not found"
    exit 1
fi

log_info_if_enabled "Updating README.md..."

# Create a backup
cp "$README_FILE" "$TEMP_FILE"

# Replace the bullet list section
awk -v new_content="$BULLET_LIST" '
BEGIN { in_tools_section = 0; skip_bullets = 0 }
/^- \*\*Supported Tools\*\*:/ {
    print new_content
    skip_bullets = 1
    next
}
skip_bullets && /^  - `[a-z]+` -/ {
    next
}
skip_bullets && !/^  - `[a-z]+` -/ {
    skip_bullets = 0
    # Skip empty lines immediately after tools list to prevent double blank lines
    if ($0 != "") {
        print $0
    }
}
!skip_bullets {
    print $0
}
' "$TEMP_FILE" > "$README_FILE"

# Replace the testing phase note
cp "$README_FILE" "$TEMP_FILE"
awk -v new_note="$TESTING_NOTE" '
/^\*\*Testing Phase\*\*:/ {
    print new_note
    next
}
{
    print $0
}
' "$TEMP_FILE" > "$README_FILE"

# Replace the tools table
cp "$README_FILE" "$TEMP_FILE"
awk -v new_table="$TOOLS_TABLE" '
BEGIN { in_table = 0; table_replaced = 0 }
/^\| Tool/ && /Status/ && /Installation Command/ {
    if (!table_replaced) {
        print new_table
        table_replaced = 1
        in_table = 1
    }
    next
}
in_table && /^\| [-\| ]+\|$/ {
    # Skip the separator line after header
    next
}
in_table && /^\| `[a-z]+` / {
    # Skip existing table rows
    next
}
in_table && !/^\| `[a-z]+` / && !/^\| [-\| ]+\|$/ {
    in_table = 0
    # Skip empty lines immediately after table to prevent double blank lines
    if ($0 != "") {
        print $0
    }
}
!in_table && !table_replaced {
    print $0
}
table_replaced && !in_table {
    print $0
}
' "$TEMP_FILE" > "$README_FILE"

# Clean up
rm "$TEMP_FILE"

log_success_if_enabled "README.md updated successfully!"
echo "   • Updated supported tools bullet list"
echo "   • Updated tools table" 
echo "   • Updated testing phase note"
