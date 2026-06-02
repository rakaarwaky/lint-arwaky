"""
state_any_mess.py — TEST PROJECT ONLY.
Intentionally triggers AES layer violations:
- AES021: agent with mutable class-level state
- AES024: explicit Any type in method signature
- AES015: unused mandatory import
- AES003: single-word filename
- mypy: no-untyped-call (call function without type hints)
"""

from typing import Any


class SharedStateManager:
    """
    AES021: Agent with mutable class-level state that changes across calls.
    This is a violation — agents should be stateless.
    """
    _shared_cache: dict = {}  # class-level mutable state
    _call_count: int = 0
    _active_sessions: list = []

    def __init__(self) -> None:
        self._instance_state: dict = {}

    @classmethod
    def reset_cache(cls) -> None:
        cls._shared_cache.clear()
        cls._call_count = 0

    def execute(self, task_id: str) -> dict:
        """Mutates class-level state across calls."""
        self.__class__._call_count += 1
        self.__class__._active_sessions.append(task_id)
        result = self._shared_cache.get(task_id, {})
        self._shared_cache[task_id] = result
        return result

    def get_call_count(self) -> int:
        return self.__class__._call_count

    # =========================================================================
    # AES024: Explicit Any type in method signature
    # =========================================================================
    def process_data(self, data: Any) -> Any:
        """AES024 violation: using explicit Any type."""
        return self._shared_cache.get(str(data), data)

    def dispatch_event(self, event: Any, payload: Any) -> Any:
        """AES024 violation: multiple Any parameters."""
        if isinstance(event, str):
            self._active_sessions.append(payload)
        return payload

    def transform(self, input_data: Any, config: Any) -> Any:
        """AES024 violation in orchestrator method."""
        return input_data


def untyped_function():
    """
    Mypy no-untyped-call: calling a function without type hints.
    This function has no type annotations at all.
    """
    manager = SharedStateManager()
    # Calling typed methods with no issues:
    manager.execute("task_1")
    # Calling untyped helpers:
    result = helper_without_types(42, "test")
    return result


def helper_without_types(x, y):
    """No type hints — calling this triggers mypy no-untyped-call."""
    return str(x) + str(y)

