"""
Provider auto-detection for Terminal Jarvis ADK.

get_model_chain() returns ALL configured providers in priority order
so jarvis.py can fall back automatically at runtime:
  Gemini -> OpenRouter -> Ollama

Priority:
  1. JARVIS_MODEL env var (explicit override, no fallback chain)
  2. GOOGLE_API_KEY / GEMINI_API_KEY  -> gemini-2.0-flash
  3. OPENROUTER_API_KEY               -> openrouter/google/gemini-flash-1.5
  4. Ollama reachable at localhost     -> ollama/llama3.2
"""

from __future__ import annotations

import os
import urllib.request


def _ollama_reachable(host: str = "http://localhost:11434") -> bool:
    try:
        with urllib.request.urlopen(f"{host}/api/tags", timeout=2):
            return True
    except Exception:
        return False


def _litellm(model_name: str) -> object:
    """Wrap a model string in LiteLlm, raising ImportError if not installed."""
    from google.adk.models.lite_llm import LiteLlm  # type: ignore
    return LiteLlm(model_name)


def get_model_chain() -> list[tuple[object, str]]:
    """
    Return an ordered list of (model, label) for all configured providers.
    jarvis.py works through this list and falls back on failure.
    """
    # Explicit override — no fallback, just this model.
    explicit = os.getenv("JARVIS_MODEL")
    if explicit:
        if "/" in explicit and not explicit.startswith("gemini"):
            try:
                return [(_litellm(explicit), explicit)]
            except ImportError:
                raise RuntimeError(
                    f"JARVIS_MODEL={explicit!r} requires 'google-adk[litellm]'.\n"
                    "Run: pip install 'google-adk[litellm]'"
                )
        return [(explicit, explicit)]

    chain: list[tuple[object, str]] = []

    # Google Gemini
    if os.getenv("GOOGLE_API_KEY") or os.getenv("GEMINI_API_KEY"):
        chain.append(("gemini-2.0-flash", "gemini-2.0-flash"))

    # OpenRouter
    if os.getenv("OPENROUTER_API_KEY"):
        try:
            model_name = "openrouter/google/gemini-flash-1.5"
            chain.append((_litellm(model_name), model_name))
        except ImportError:
            pass  # litellm not installed — skip silently

    # Ollama (local)
    ollama_host = os.getenv("OLLAMA_HOST", "http://localhost:11434")
    if _ollama_reachable(ollama_host):
        try:
            model_name = "ollama/llama3.2"
            chain.append((_litellm(model_name), f"{model_name} (local)"))
        except ImportError:
            pass  # litellm not installed — skip silently

    if not chain:
        raise RuntimeError(
            "\n"
            "No provider configured. Set one of the following in adk/.env:\n"
            "\n"
            "  Option 1 - Google Gemini (recommended):\n"
            "    GOOGLE_API_KEY=your-key-here\n"
            "\n"
            "  Option 2 - OpenRouter:\n"
            "    OPENROUTER_API_KEY=your-key-here\n"
            "    (also run: pip install 'google-adk[litellm]')\n"
            "\n"
            "  Option 3 - Ollama (local):\n"
            "    Start Ollama, then: ollama pull llama3.2\n"
            "    (also run: pip install 'google-adk[litellm]')\n"
            "\n"
            "Copy adk/.env.example to adk/.env to get started.\n"
        )

    return chain


def get_model() -> tuple[object, str]:
    """Return (model, label) for the highest-priority available provider."""
    return get_model_chain()[0]
