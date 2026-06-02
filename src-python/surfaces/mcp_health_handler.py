from __future__ import annotations

from typing import Any
import logging

from ..contract import (
    ServiceContainerAggregate,
    AgentLifecycleAggregate,
)

logger = logging.getLogger("surfaces.mcp_health")


class McpHealthCheckSurface:
    """
    PURE CONSUMER: Surface for MCP Health Checks.
    Only interacts with AgentLifecycleAggregate to get orchestrated health data.
    Ensures that Surface layer never touches infrastructure Ports (e.g. JobRegistryPort).
    """

    def __init__(self, container: ServiceContainerAggregate) -> None:
        self.container = container

    async def execute_check(self) -> dict[str, Any]:
        """
        Action: Press the 'get_health' button on the Agent Aggregate contract.
        Surface doesn't care HOW health is gathered, it trusts the Agent orchestrator.
        """
        try:
            # Surface only knows high-level Agent Aggregate
            agent_aggregate = self.container.get(AgentLifecycleAggregate)

            # Press the button: Agent will internaly orchestrate multiple ports
            health_data = await agent_aggregate.get_health()

            logger.info("Health check successfully orchestrated by AgentLifecycleAggregate")
            return {"success": True, "data": health_data}
        except Exception as e:
            logger.error(f"Agent health button failed: {e}")
            return {"success": False, "error": str(e)}

    async def format_health_report(self) -> str:
        """Visual: Format high-level health data for display in CLI/MCP."""
        result = await self.execute_check()
        if not result["success"]:
            return f"SYSTEM CRITICAL: {result['error']}"

        data = result["data"]
        lifecycle = data.get("lifecycle", {})
        system = data.get("system", {})
        components = data.get("components", {})

        report = [
            "=== AUTO-LINTER SYSTEM HEALTH ===",
            f"Status  : {lifecycle.get('status', 'unknown').upper()}",
            f"Uptime  : {lifecycle.get('uptime_seconds', 0)}s",
            f"Platform: {system.get('os')} (Python {system.get('python')})",
            "--- Components ---",
        ]

        for name, status in components.items():
            if isinstance(status, dict):
                # Specific data (e.g. jobs: running/total)
                report.append(
                    f"{name.capitalize():<10}: {status.get('running')}/{status.get('total')} jobs active"
                )
            else:
                report.append(f"{name.capitalize():<10}: {status}")

        return "\n".join(report)


def register_health_commands(mcp, container: ServiceContainerAggregate) -> None:
    """Registration bridge for MCP Server."""
    surface = McpHealthCheckSurface(container)

    @mcp.tool()
    async def health_check() -> object:
        """Check overall system health and component status."""
        return await surface.format_health_report()
