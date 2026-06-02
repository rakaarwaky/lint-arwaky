"""Agent lifecycle — startup, shutdown, health management."""

from __future__ import annotations

from ..contract import (
    PipelineOutputAggregate,
    AgentLifecycleAggregate,
    ICommandExecutorPort,
    IJobRegistryPort,
    ServiceContainerAggregate,
)
from ..taxonomy import Duration, ResponseData
from typing import Type, Any
import logging
import time
import os
import platform
import sys

logger = logging.getLogger("agent.lifecycle")


class AgentState(AgentLifecycleAggregate):
    """Track agent lifecycle state and aggregate system health."""

    container: ServiceContainerAggregate
    start_time: float | None = None

    def __init__(self, container: ServiceContainerAggregate) -> None:
        super().__init__(container=container)

    @property
    def _INTERFACE_COMPLIANCE(self) -> Type[PipelineOutputAggregate]:
        return PipelineOutputAggregate

    @property
    def uptime(self) -> Duration:
        if self.start_time is None:
            return Duration(value=0.0)
        return Duration(value=float(time.time() - self.start_time))

    def mark_started(self) -> None:
        self.started = True
        self.start_time = time.time()
        self.status = "running"
        logger.info("Agent started (uptime tracking enabled)")

    async def get_health(self) -> ResponseData:
        """AGGREGATOR: Agent presses the buttons of multiple ports to gather health."""
        result: dict[str, Any] = {
            "lifecycle": {
                "status": self.status,
                "uptime_seconds": round(float(self.uptime), 1),
                "started": self.started,
            },
            "system": {
                "os": platform.system(),
                "python": sys.version.split()[0],
                "cwd": os.getcwd(),
            },
            "components": {},
        }

        # Agent uses ports to gather more info
        try:
            self.container.get(ICommandExecutorPort)
            result["components"]["executor"] = "healthy"
        except Exception:
            result["components"]["executor"] = "unavailable"

        try:
            registry = self.container.get(IJobRegistryPort)
            jobs = await registry.list_jobs()
            result["components"]["jobs"] = {
                "total": len(jobs),
                "running": sum(
                    1 for j in jobs.values() if j.get("status") == "running"
                ),
            }
        except Exception:
            result["components"]["jobs"] = "unavailable"

        return ResponseData(value=result)

    def mark_stopped(self) -> None:
        self.status = "stopped"
        logger.info("Agent stopped")

    def mark_degraded(self) -> None:
        self.status = "degraded"


def get_state(container: ServiceContainerAggregate) -> AgentState:
    """Factory to create a container-aware agent state."""
    return AgentState(container=container)
