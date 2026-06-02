"""async_bridge_aggregate - Aggregate contract for async bridge."""

from __future__ import annotations
import asyncio
from abc import ABC


class AsyncBridgeAggregate(ABC):
    """AGGREGATE: Contract for safely running async code from sync contexts."""
    pass


def run_async(coro):
    """Safely run a coroutine from a sync context.

    Handles both:
    - No event loop: uses asyncio.run()
    - Existing event loop: uses loop.run_until_complete()
    """
    try:
        return asyncio.run(coro)
    except RuntimeError:
        loop = asyncio.get_running_loop()
        return loop.run_until_complete(coro)
