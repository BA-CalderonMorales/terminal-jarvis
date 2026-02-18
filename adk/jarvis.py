"""
Terminal Jarvis ADK Home Screen - Entry point.

Architecture:
  User input
      |
      v
  REPL loop
      +-- starts with "/" --> direct subprocess call (no LLM overhead)
      +-- plain English   --> ADK Agent --> tool call --> terminal-jarvis binary

Run:
  cd adk && .venv/bin/python jarvis.py
"""

from __future__ import annotations

import asyncio
import os
import shutil
import subprocess
import sys
import threading
import time
from pathlib import Path

# Load .env from the adk/ directory before importing providers/agent.
_env_path = Path(__file__).parent / ".env"
if _env_path.exists():
    from dotenv import load_dotenv  # type: ignore
    load_dotenv(_env_path)

# Silence ADK / LiteLlm internal warnings so they never bleed into the terminal.
import logging as _logging
for _noisy in ("google.adk", "LiteLLM", "litellm", "httpx", "httpcore"):
    _logging.getLogger(_noisy).setLevel(_logging.CRITICAL)

# ─── Theme ANSI codes (matches src/theme/theme_config.rs TJarvis theme) ───────
CYAN    = "\033[38;2;0;255;255m"       # border / accent (logo colour)
BOLD_W  = "\033[1;38;2;255;255;255m"  # primary text
LIGHT_B = "\033[38;2;200;230;255m"    # secondary text
DIM     = "\033[2;38;2;120;140;160m"  # muted — used for collapsed thinking block
RESET   = "\033[0m"

VERSION = "v0.0.77"

# ─── Response post-processing ─────────────────────────────────────────────────

import re as _re

_MONOLOGUE = _re.compile(
    r"^(the user (is|just|wants|said|asked|'s)|"
    r"i (should|need to|will|can|realize|think|must|'m going|'ve)|"
    r"let me \S|"          # any "let me <word>" — catches "let me be", "let me think", etc.
    r"according to (my|the)|"
    r"actually,?\s|wait,?\s|hmm,?\s|"
    r"looking at (this|the)|"
    r"based on|given that|"
    r"this is (a|an|the)|"
    r"so,?\s|ok(ay)?,?\s)",
    _re.IGNORECASE,
)

# Matches sentence-ending punctuation followed by a space (for inline splits).
_SENT_SPLIT = _re.compile(r'(?<=[.!?])\s+')


def _separate_thinking(text: str) -> tuple[str, str]:
    """Return (thinking_text, response_text) from model output.

    Works at paragraph level first, then falls back to sentence-level
    splitting for cases where the model joins its reasoning and reply
    into a single paragraph with no blank line between them.

    Returns ("", full_text) when no thinking pattern is detected.
    """
    paragraphs = [p.strip() for p in text.split("\n\n") if p.strip()]

    thinking_paras: list[str] = []
    response_paras: list[str] = []
    found_real = False

    for para in paragraphs:
        if not found_real and _MONOLOGUE.match(para):
            thinking_paras.append(para)
        else:
            found_real = True
            response_paras.append(para)

    # Single blob paragraph — try sentence-level split.
    if not thinking_paras and len(paragraphs) == 1:
        sentences = _SENT_SPLIT.split(paragraphs[0])
        thinking_sents: list[str] = []
        response_sents: list[str] = []
        found_real_sent = False
        for sent in sentences:
            if not found_real_sent and _MONOLOGUE.match(sent.strip()):
                thinking_sents.append(sent)
            else:
                found_real_sent = True
                response_sents.append(sent)
        if thinking_sents:
            return " ".join(thinking_sents), " ".join(response_sents)

    thinking = "\n\n".join(thinking_paras)
    response = "\n\n".join(response_paras)
    if not response:
        return "", text.strip()
    return thinking, response


_THINK_BOX_WIDTH = 56  # inner width for the thinking collapse border


def _print_response(text: str) -> None:
    """Print the agent reply with thinking shown as a dim collapsible block.

    Thinking (internal model reasoning) is rendered in muted style so it
    is visually de-emphasised but still readable. The actual response is
    displayed in the normal accent colour.
    """
    thinking, response = _separate_thinking(text.strip())

    if thinking:
        bar = "─" * _THINK_BOX_WIDTH
        print(f"   {DIM}┌ thinking {bar}{RESET}")
        for line in thinking.splitlines():
            print(f"   {DIM}│ {line}{RESET}")
        print(f"   {DIM}└{bar}─{RESET}")
        print()

    if response:
        print(f"   {LIGHT_B}{response}{RESET}\n")


# ─── Helpers ──────────────────────────────────────────────────────────────────

def _tj(*args: str) -> None:
    """Run terminal-jarvis and stream output directly to the terminal."""
    binary = shutil.which("terminal-jarvis") or "terminal-jarvis"
    try:
        subprocess.run([binary, *args])
    except FileNotFoundError:
        print(
            f"   {CYAN}[ERROR]{RESET} terminal-jarvis binary not found.\n"
            "   Install: cargo install terminal-jarvis"
        )


def _clear() -> None:
    print("\033[2J\033[H", end="", flush=True)


def _print_home(provider_label: str) -> None:
    """Render the home screen matching the Rust TUI layout."""
    cwd = str(Path.cwd())

    _clear()

    # T.J logo + info block (mirrors cli_logic_welcome.rs Default theme)
    print(f"{CYAN}   ┌─────┐{RESET}  {BOLD_W}Terminal Jarvis{RESET}")
    print(f"{CYAN}   │ T.J │{RESET}  {LIGHT_B}{VERSION}{RESET}")
    print(f"{CYAN}   │ ═ ═ │{RESET}  {LIGHT_B}Provider: {provider_label}{RESET}")
    print(f"{CYAN}   │     │{RESET}  {LIGHT_B}{cwd}{RESET}")
    print(f"{CYAN}   └─────┘{RESET}  {CYAN}Type /help to see available commands{RESET}")
    print()
    print(f"   {LIGHT_B}Or describe what you want in plain English.{RESET}")
    print()


def _print_help() -> None:
    print()
    print(f"   {CYAN}Commands:{RESET}")
    print(f"   {CYAN}/tools{RESET}               list all AI coding tools")
    print(f"   {CYAN}/install <tool>{RESET}      install a tool")
    print(f"   {CYAN}/status{RESET}              tool health dashboard")
    print(f"   {CYAN}/auth [tool]{RESET}         authentication help")
    print(f"   {CYAN}/config{RESET}              show current config")
    print(f"   {CYAN}/update [tool]{RESET}       update one or all tools")
    print(f"   {CYAN}/help{RESET}                show this help")
    print(f"   {CYAN}/exit{RESET}                exit")
    print()
    print(f"   {LIGHT_B}Tab to autocomplete  |  plain English also works{RESET}")
    print()
    print(f"   {BOLD_W}Examples:{RESET}")
    print(f"   {LIGHT_B}which tools are installed?{RESET}")
    print(f"   {LIGHT_B}launch claude{RESET}")
    print(f"   {LIGHT_B}how do I set up auth for gemini?{RESET}")
    print()


def _handle_slash(line: str) -> None:
    """Handle slash commands without involving the LLM."""
    parts = line.strip().split()
    cmd   = parts[0].lower()
    rest  = parts[1:]

    if cmd in ("/exit", "/quit"):
        print(f"\n   {CYAN}Goodbye.{RESET}\n")
        sys.exit(0)
    elif cmd == "/help":
        _print_help()
    elif cmd == "/tools":
        _tj("list")
    elif cmd == "/status":
        _tj("status")
    elif cmd == "/config":
        _tj("config", "show")
    elif cmd == "/install":
        if rest:
            _tj("install", *rest)
        else:
            print(f"   {LIGHT_B}Usage: /install <tool-name>{RESET}")
    elif cmd == "/update":
        if rest:
            _tj("update", *rest)
        else:
            _tj("update")
    elif cmd == "/auth":
        if rest:
            _tj("auth", "help", *rest)
        else:
            _tj("auth", "manage")
    else:
        print(f"   {LIGHT_B}Unknown command '{cmd}'. Type /help for options.{RESET}")


# ─── ADK runner ───────────────────────────────────────────────────────────────

_APP_NAME = "terminal_jarvis"
_USER_ID  = "local"


async def _make_runner_session(model: object, build_agent_fn: object) -> tuple:
    """Build a fresh Runner + InMemorySessionService for the given model.

    Uses an explicit app_name so run_async session lookup always matches —
    avoids the 'Session not found' mismatch from InMemoryRunner's default.
    """
    from google.adk.runners import Runner  # type: ignore
    from google.adk.sessions import InMemorySessionService  # type: ignore

    agent = build_agent_fn(model)  # type: ignore
    svc   = InMemorySessionService()
    runner = Runner(agent=agent, app_name=_APP_NAME, session_service=svc)
    session = await svc.create_session(app_name=_APP_NAME, user_id=_USER_ID)
    return agent, runner, session


_SPINNER_FRAMES = ["   ┌( >_<)┘", "   └( >_<)┐"]


def _spinner_thread(stop: threading.Event) -> None:
    """Animate the T.J spinner in a real OS thread.

    Runs independently of the asyncio event loop so it keeps ticking
    even when LiteLlm blocks the loop during its HTTP call.

    Pauses (erases + idles) when tools.SUPPRESS_SPINNER is set so a
    launched tool's own terminal output doesn't race with our frames.
    """
    from tools import SUPPRESS_SPINNER  # late import — avoids circular import at module load

    i = 0
    _erased = False
    while not stop.is_set():
        if SUPPRESS_SPINNER.is_set():
            if not _erased:
                # Clear the spinner line once; then stay silent.
                print(f"\r{' ' * 60}\r", end="", flush=True)
                _erased = True
            stop.wait(0.35)
            continue
        _erased = False
        frame = _SPINNER_FRAMES[i % 2]
        print(f"\r{CYAN}{frame}{RESET}  ", end="", flush=True)
        i += 1
        stop.wait(0.35)  # interruptible sleep
    # Erase spinner line.
    print(f"\r{' ' * 60}\r", end="", flush=True)


async def _collect_response(runner: object, session_id: str, content: object) -> list[str]:
    """Stream events from run_async; return only final-response text.

    Intermediate events (model chain-of-thought, tool calls) are skipped so
    the user never sees the model's internal reasoning — only the reply.
    """
    parts: list[str] = []
    async for event in runner.run_async(  # type: ignore
        user_id=_USER_ID,
        session_id=session_id,
        new_message=content,
    ):
        # Skip thinking / intermediate model output; keep only the final reply.
        if not event.is_final_response():
            continue
        if event.content and event.content.parts:
            for part in event.content.parts:
                if hasattr(part, "text") and part.text:
                    parts.append(part.text)
    return parts


# ─── Main ─────────────────────────────────────────────────────────────────────

async def main() -> None:
    from google.genai import types as genai_types  # type: ignore

    # Build provider chain (Gemini -> OpenRouter -> Ollama).
    try:
        from providers import get_model_chain  # type: ignore
        chain = get_model_chain()
    except RuntimeError as exc:
        print(f"\n   {CYAN}[Terminal Jarvis]{RESET} {exc}")
        sys.exit(1)

    from agent import build_agent  # type: ignore

    provider_idx = 0
    model, provider_label = chain[provider_idx]

    try:
        agent, runner, session = await _make_runner_session(model, build_agent)
    except Exception as exc:
        print(f"\n   {CYAN}[Terminal Jarvis]{RESET} Failed to initialise provider: {exc}")
        sys.exit(1)

    _print_home(provider_label)

    # REPL loop.
    while True:
        try:
            line = input(f"   {CYAN}>{RESET} ").strip()
        except (EOFError, KeyboardInterrupt):
            print(f"\n   {CYAN}Goodbye.{RESET}\n")
            return

        if not line:
            continue

        if line.startswith("/"):
            _handle_slash(line)
            continue

        # Plain English → ADK agent, with automatic provider fallback.
        content = genai_types.Content(
            role="user",
            parts=[genai_types.Part(text=line)],
        )

        replied = False
        session_rebuilds = 0  # guard against infinite rebuild loops
        while not replied and provider_idx < len(chain):
            try:
                stop = threading.Event()
                spin = threading.Thread(
                    target=_spinner_thread, args=(stop,), daemon=True
                )
                spin.start()
                try:
                    response_parts = await asyncio.wait_for(
                        _collect_response(runner, session.id, content),
                        timeout=60,
                    )
                finally:
                    stop.set()
                    spin.join(timeout=1)

                reply = "".join(response_parts).strip()
                if reply:
                    print()
                    _print_response(reply)
                replied = True

            except asyncio.TimeoutError:
                next_idx = provider_idx + 1
                if next_idx < len(chain):
                    _, next_label = chain[next_idx]
                    print(
                        f"\n   {CYAN}[timeout]{RESET} "
                        f"{LIGHT_B}{provider_label} took >60s — "
                        f"falling back to {next_label}...{RESET}\n"
                    )
                    provider_idx = next_idx
                    model, provider_label = chain[provider_idx]
                    try:
                        agent, runner, session = await _make_runner_session(
                            model, build_agent
                        )
                    except Exception as init_exc:
                        print(f"\n   {CYAN}[ERROR]{RESET} {init_exc}\n")
                        replied = True
                else:
                    print(
                        f"\n   {CYAN}[timeout]{RESET} "
                        f"{LIGHT_B}No response after 60s. "
                        f"Check your provider or try /exit.{RESET}\n"
                    )
                    replied = True

            except KeyboardInterrupt:
                # Ctrl-C during an agent call returns to the prompt.
                print()
                replied = True

            except Exception as exc:
                exc_str = str(exc)
                # Session corrupts after launch_tool hands off the terminal.
                # Rebuild silently and retry with the same provider (max 2x).
                if "Missing tool results" in exc_str or "tool_call_id" in exc_str:
                    if session_rebuilds < 2:
                        session_rebuilds += 1
                        try:
                            agent, runner, session = await _make_runner_session(
                                model, build_agent
                            )
                        except Exception:
                            pass
                        continue  # retry same provider with fresh session
                    # Exhausted rebuilds — fall through to provider fallback.

                next_idx = provider_idx + 1
                if next_idx < len(chain):
                    _, next_label = chain[next_idx]
                    print(
                        f"\n   {CYAN}[{provider_label} failed]{RESET} "
                        f"{LIGHT_B}Falling back to {next_label}...{RESET}\n"
                    )
                    provider_idx = next_idx
                    model, provider_label = chain[provider_idx]
                    try:
                        agent, runner, session = await _make_runner_session(
                            model, build_agent
                        )
                    except Exception as init_exc:
                        print(f"\n   {CYAN}[ERROR]{RESET} {init_exc}\n")
                        replied = True
                else:
                    print(
                        f"\n   {CYAN}[ERROR]{RESET} "
                        f"All providers failed. Last error: {exc}\n"
                    )
                    replied = True


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except (KeyboardInterrupt, SystemExit):
        pass
