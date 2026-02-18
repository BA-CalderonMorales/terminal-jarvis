"""
ADK tools for Terminal Jarvis.

Each function wraps a terminal-jarvis subcommand via subprocess.
The ADK agent calls these when the user asks in plain English.

launch_tool() is special: it runs without stdout capture so the
tool takes over the terminal interactively. Control returns to
the REPL when the user exits the tool.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import threading
from pathlib import Path

# Set this before subprocess.run in launch_tool so jarvis.py's spinner
# thread knows to erase itself and pause instead of writing over the
# tool's own terminal output.
SUPPRESS_SPINNER = threading.Event()


def _find_binary() -> str:
    """Locate the terminal-jarvis binary.

    Search order:
      1. PATH (installed globally via cargo install / npm / brew)
      2. Project target/release  (optimised dev build)
      3. Project target/debug    (standard dev build)
      4. CARGO_HOME/bin          (cargo install --path .)
    Falls back to the bare name so callers get a clear FileNotFoundError.
    """
    found = shutil.which("terminal-jarvis")
    if found:
        return found

    # Resolve project root relative to this file (adk/ -> project root).
    project_root = Path(__file__).parent.parent

    for candidate in (
        project_root / "target" / "release" / "terminal-jarvis",
        project_root / "target" / "debug"   / "terminal-jarvis",
        Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")) / "bin" / "terminal-jarvis",
    ):
        if candidate.exists():
            return str(candidate)

    return "terminal-jarvis"  # will surface a clear error on FileNotFoundError


def _tj(*args: str) -> str:
    """Run terminal-jarvis with the given args and return captured stdout."""
    binary = _find_binary()
    try:
        result = subprocess.run(
            [binary, *args],
            capture_output=True,
            text=True,
        )
        output = result.stdout
        if result.returncode != 0 and result.stderr:
            output += result.stderr
        return output.strip() or "(no output)"
    except FileNotFoundError:
        return (
            "terminal-jarvis binary not found. "
            "Install it first: cargo install terminal-jarvis"
        )


def list_tools() -> str:
    """List all available AI coding tools and their installation status."""
    return _tj("list")


def get_tool_info(tool_name: str) -> str:
    """
    Get detailed information about a specific AI coding tool.

    Args:
        tool_name: Name of the tool (e.g., claude, gemini, aider, goose).
    """
    return _tj("info", tool_name)


def launch_tool(tool_name: str) -> str:
    """
    Launch an AI coding tool interactively. The tool takes over the terminal.
    Control returns to the Terminal Jarvis home screen when the user exits.

    Args:
        tool_name: Name of the tool to launch (e.g., claude, gemini, aider).
    """
    binary = _find_binary()
    try:
        # Suppress the jarvis.py spinner so it doesn't race with the
        # tool's own terminal output (e.g. its braille availability check).
        SUPPRESS_SPINNER.set()
        try:
            # Run without capture so the tool owns the terminal.
            subprocess.run([binary, "run", tool_name])
        finally:
            SUPPRESS_SPINNER.clear()
    except FileNotFoundError:
        return (
            "terminal-jarvis binary not found. "
            "Install it first: cargo install terminal-jarvis"
        )
    return f"Returned from {tool_name}. Back in Terminal Jarvis home."


def install_tool(tool_name: str) -> str:
    """
    Install an AI coding tool.

    Args:
        tool_name: Name of the tool to install (e.g., aider, goose, llxprt).
    """
    return _tj("install", tool_name)


def update_tool(tool_name: str = "") -> str:
    """
    Update one or all AI coding tools.

    Args:
        tool_name: Name of the tool to update. Leave empty to update all tools.
    """
    if tool_name:
        return _tj("update", tool_name)
    return _tj("update")


def show_status() -> str:
    """Show the health dashboard for all AI coding tools."""
    return _tj("status")


def get_auth_help(tool_name: str) -> str:
    """
    Show authentication setup instructions for a specific tool.

    Args:
        tool_name: Name of the tool to get auth help for (e.g., claude, gemini).
    """
    return _tj("auth", "help", tool_name)


def show_config() -> str:
    """Show the current Terminal Jarvis configuration."""
    return _tj("config", "show")


def clear_cache() -> str:
    """Clear the version cache to force fresh tool detection."""
    return _tj("cache", "clear")


# All tools exposed to the ADK agent.
ALL_TOOLS = [
    list_tools,
    get_tool_info,
    launch_tool,
    install_tool,
    update_tool,
    show_status,
    get_auth_help,
    show_config,
    clear_cache,
]
