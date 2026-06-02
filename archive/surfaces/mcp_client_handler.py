"""
MCP Desktop Client Surface - Internal helper for DesktopCommander integration.

Provides:
- _get_client: Resolve DesktopCommander client
- _execute_with_retry: Resilience wrapper for CLI calls
- _running_jobs: State tracking for backward compatibility
"""

from typing import Any
from ..contract import ServiceContainerAggregate

# State tracking moved from mcp_execute_command for architectural consistency
_running_jobs: dict = {}


class McpDesktopClientSurface:
    """Internal surface for DesktopCommander integration."""

    def __init__(self, mcp: Any = None):
        self.mcp = mcp
        self.container: ServiceContainerAggregate | None = None

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Initialize the desktop client surface."""
        self.container = container
        # No public tools registered yet, but module is initialized
        pass

    async def _get_client(self):
        """Resolve the primary desktop interaction client."""
        # Implementation placeholder - currently defaults to subprocess in
        # execute_command
        return None

    async def _execute_with_retry(self, operation: Any, max_retries: int = 3):
        """Execute a desktop operation with automatic retry logic."""
        if not self.container:
            return await operation()

        from ..taxonomy import Identity, Count

        job_registry = self.container.get("JobRegistryAggregate")  # Dynamic look up
        if hasattr(job_registry, "run_with_retry"):
            return await job_registry.run_with_retry(
                Identity(value=str(operation)), Count(value=max_retries)
            )
        return await operation()


def register_desktop_client(mcp, container: ServiceContainerAggregate) -> None:
    """Factory function for container-aware registration."""
    surface = McpDesktopClientSurface(mcp)
    surface.register_all(container)
