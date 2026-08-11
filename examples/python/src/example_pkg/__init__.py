"""Minimal example package for the libraries-ai-starter Python reference path."""

__all__ = ["greet"]


def greet(name: str) -> str:
    """Return a friendly greeting.

    Replace this module with your actual project code once you've promoted
    this example into src/ (see TEMPLATE_SETUP.md at the repository root).
    """
    if not name:
        raise ValueError("name must not be empty")
    return f"Hello, {name}!"
