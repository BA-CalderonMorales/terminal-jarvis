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
import concurrent.futures as _futures
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

# Suppress LiteLLM's direct print() calls (e.g. "Give Feedback / Get Help",
# "Missing tool results" validation warnings) that bypass the logging system.
try:
    import litellm as _litellm  # type: ignore
    _litellm.suppress_debug_info = True
    _litellm.set_verbose = False
except Exception:
    pass

# ─── Theme ANSI codes (matches src/theme/theme_config.rs TJarvis theme) ───────
CYAN    = "\033[38;2;0;255;255m"       # border / accent (logo colour)
BOLD_W  = "\033[1;38;2;255;255;255m"  # primary text
LIGHT_B = "\033[38;2;200;230;255m"    # secondary text
DIM     = "\033[2;38;2;120;140;160m"  # muted secondary text
THINK_G = "\033[38;2;130;145;160m"    # reasoning text shade
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


def _split_monologue_sentences(text: str) -> tuple[str, str] | None:
    """Try sentence-level split; return None when no clean split exists."""
    sentences = [s.strip() for s in _SENT_SPLIT.split(text) if s.strip()]
    if not sentences:
        return None

    thinking_sents: list[str] = []
    response_sents: list[str] = []
    found_real_sent = False

    for sent in sentences:
        if not found_real_sent and _MONOLOGUE.match(sent):
            thinking_sents.append(sent)
        else:
            found_real_sent = True
            response_sents.append(sent)

    if thinking_sents and response_sents:
        return " ".join(thinking_sents), " ".join(response_sents)
    return None


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
        if found_real:
            response_paras.append(para)
            continue

        if _MONOLOGUE.match(para):
            # Some outputs start as monologue and then switch to the actual
            # user-facing answer in the same paragraph.
            split = _split_monologue_sentences(para)
            if split:
                thinking_part, response_part = split
                if thinking_part:
                    thinking_paras.append(thinking_part)
                if response_part:
                    response_paras.append(response_part)
                    found_real = True
            else:
                thinking_paras.append(para)
            continue

        found_real = True
        response_paras.append(para)

    # Single blob paragraph. Try sentence-level split.
    if not thinking_paras and len(paragraphs) == 1:
        split = _split_monologue_sentences(paragraphs[0])
        if split:
            return split

    thinking = "\n\n".join(thinking_paras)
    response = "\n\n".join(response_paras)

    # We saw monologue paragraphs, but no response paragraph. This can happen
    # when the model joins "final answer" into the last monologue paragraph.
    # Retry sentence-level split over the full text before giving up.
    if thinking and not response:
        split = _split_monologue_sentences(text.strip())
        if split:
            return split

    if not response:
        return "", text.strip()
    return thinking, response


def _print_response(text: str) -> None:
    """Print the agent reply with thinking in muted text.

    Thinking (internal model reasoning) is rendered in a gray shade so it is
    visually de-emphasised but still readable. The final response is displayed
    in the normal accent colour.
    """
    thinking, response = _separate_thinking(text.strip())

    if thinking:
        print(f"   {THINK_G}**thinking**{RESET}")
        for line in thinking.splitlines():
            print(f"   {THINK_G}{line}{RESET}")
        print()

    if response:
        print(f"   {LIGHT_B}{response}{RESET}\n")


# ─── Auth error detection & onboarding guide ──────────────────────────────────

_AUTH_SIGNALS = (
    "AuthenticationError", "401", "403",
    "User not found", "invalid_api_key",
    "Unauthorized", "authentication failed",
    "No auth credentials", "API key not valid",
)


def _is_auth_error(exc_str: str) -> bool:
    lo = exc_str.lower()
    return any(sig.lower() in lo for sig in _AUTH_SIGNALS)


def _print_auth_guide(failed_label: str | None = None) -> None:
    """Print a clear provider setup guide when authentication fails."""
    env_path = Path(__file__).parent / ".env"

    if failed_label:
        print(
            f"\n   {CYAN}[auth failed]{RESET} "
            f"{LIGHT_B}{failed_label} rejected the API key.{RESET}"
        )
    print()
    print(f"   {BOLD_W}Choose a provider and add it to:{RESET} {LIGHT_B}{env_path}{RESET}")
    print()
    print(f"   {CYAN}►{RESET} {BOLD_W}Option 1 — Google Gemini{RESET}  {DIM}(recommended · free tier available){RESET}")
    print(f"   {LIGHT_B}    GOOGLE_API_KEY=your-key-here{RESET}")
    print(f"   {DIM}    https://aistudio.google.com/app/apikey{RESET}")
    print()
    print(f"   {CYAN}►{RESET} {BOLD_W}Option 2 — OpenRouter{RESET}  {DIM}(100+ cloud models){RESET}")
    print(f"   {LIGHT_B}    OPENROUTER_API_KEY=your-key-here{RESET}")
    print(f"   {DIM}    https://openrouter.ai/keys{RESET}")
    print()
    print(f"   {CYAN}►{RESET} {BOLD_W}Option 3 — Ollama{RESET}  {DIM}(local · no API key required){RESET}")
    print(f"   {DIM}    Install: https://ollama.com/download{RESET}")
    print(f"   {DIM}    Then:    ollama pull llama3.2{RESET}")
    print()
    print(
        f"   {LIGHT_B}Edit {env_path} then restart Terminal Jarvis.{RESET}\n"
    )


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


def _run_collect_in_thread(runner: object, session_id: str, content: object) -> list[str]:
    """Run _collect_response in a fresh asyncio event loop inside an OS thread.

    asyncio.wait_for(timeout=N) cannot interrupt a task when LiteLLM makes
    synchronous HTTP calls that block the event loop — the timeout future
    simply never gets scheduled.  Running the collection in a separate thread
    keeps the *outer* event loop free, so wait_for actually fires on time.

    The thread is intentionally fire-and-forget when cancelled: LiteLLM has
    its own connection timeout and will clean up on its own.
    """
    return asyncio.run(_collect_response(runner, session_id, content))


# ─── Main ─────────────────────────────────────────────────────────────────────

async def main() -> None:
    from google.genai import types as genai_types  # type: ignore

    # Build provider chain (Gemini -> OpenRouter -> Ollama).
    try:
        from providers import get_model_chain  # type: ignore
        chain = get_model_chain()
    except RuntimeError:
        # No provider configured — show the onboarding guide instead of a
        # raw error so new users know exactly what to do next.
        _print_auth_guide()
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
        except EOFError:
            print(f"\n   {CYAN}Goodbye.{RESET}\n")
            return
        except KeyboardInterrupt:
            # Overwrite the echoed ^C on the input line, then confirm.
            print(f"\r{' ' * 60}\r", end="", flush=True)
            try:
                ans = input(
                    f"   {LIGHT_B}Exit Terminal Jarvis? [y/N]{RESET} "
                ).strip().lower()
            except (EOFError, KeyboardInterrupt):
                # Second ^C — exit immediately without asking again.
                print(f"\n   {CYAN}Goodbye.{RESET}\n")
                return
            if ans in ("y", "yes"):
                print(f"\n   {CYAN}Goodbye.{RESET}\n")
                return
            continue

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
                    # Run the ADK collection in a real OS thread so the outer
                    # asyncio event loop stays free.  This allows wait_for to
                    # actually fire even when LiteLLM blocks its inner loop.
                    loop = asyncio.get_running_loop()
                    response_parts = await asyncio.wait_for(
                        loop.run_in_executor(
                            None,
                            _run_collect_in_thread,
                            runner, session.id, content,
                        ),
                        timeout=60,
                    )
                finally:
                    stop.set()
                    spin.join(timeout=1)
                    # Ensure suppression is always cleared even if launch_tool
                    # is still running in an orphaned thread after a timeout.
                    from tools import SUPPRESS_SPINNER, NEEDS_SESSION_REBUILD
                    SUPPRESS_SPINNER.clear()

                reply = "".join(response_parts).strip()
                if reply:
                    print()
                    _print_response(reply)

                # Proactively rebuild the session after launch_tool so the
                # next user message starts with a clean conversation history
                # instead of inheriting a potentially corrupted tool-call state.
                if NEEDS_SESSION_REBUILD.is_set():
                    NEEDS_SESSION_REBUILD.clear()
                    try:
                        agent, runner, session = await _make_runner_session(
                            model, build_agent
                        )
                    except Exception:
                        pass

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
                    if _is_auth_error(exc_str):
                        print(
                            f"\n   {CYAN}[auth]{RESET} "
                            f"{LIGHT_B}{provider_label}: bad key — trying {next_label}...{RESET}\n"
                        )
                    else:
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
                    # All providers exhausted — show actionable setup guide for
                    # auth errors; raw error for everything else.
                    if _is_auth_error(exc_str):
                        _print_auth_guide(provider_label)
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
