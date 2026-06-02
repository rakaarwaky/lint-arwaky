"""MCP Server Lifespan — Lifecycle management for MCP server instances."""

from __future__ import annotations

import logging
from contextlib import asynccontextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import AsyncIterator, Any

from ..taxonomy import DirectoryPath
from .mcp_server_constants import MCP_SERVER_VERSION, AUTO_LINT_VERSION

logger = logging.getLogger("infrastructure.mcp_server_lifespan")


@dataclass
class WrapperContext:
    """Context object yielded by the lifespan manager."""

    container: Any  # Decoupled from ServiceContainerAggregate
    project_root: Path
    server_version: str = MCP_SERVER_VERSION
    auto_lint_version: str = AUTO_LINT_VERSION


@asynccontextmanager
async def mcp_server_lifespan(
    container: Any, project_root: DirectoryPath
) -> AsyncIterator[WrapperContext]:
    """MCP server lifespan:
    - Container provided by surface
    - Version context
    - Cleanup on shutdown
    """
    root = Path(project_root.value).resolve()
    logger.info("MCP server starting up — project_root=%s", root)

    ctx = WrapperContext(container=container, project_root=root)
    logger.info(
        "MCP server lifespan: container initialized, version=%s", MCP_SERVER_VERSION
    )

    try:
        yield ctx
    finally:
        logger.info("MCP server shutting down — cleanup complete")
