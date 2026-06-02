"""MCP Server Wrapper — Infrastructure adapter providing MCP spec compliance.

Architecture: Infrastructure layer adapter.
Wraps FastMCP with validation, structured errors, and decoupled lifecycle.
"""

from __future__ import annotations

import asyncio
import logging
import sys
import traceback
from pathlib import Path
from typing import Any, Callable, Coroutine, cast

from mcp.server.fastmcp import FastMCP
from mcp.server.fastmcp.exceptions import ToolError
from mcp.types import (
    CallToolResult,
    Resource,
    TextContent,
)

from ..contract import IMcpServerPort
from ..taxonomy import (
    SymbolName,
    DescriptionVO,
)
from .mcp_server_constants import (
    MCP_SERVER_VERSION,
    AUTO_LINT_VERSION,
    MCP_PROTOCOL_MIN,
    MCP_PROTOCOL_MAX,
)

from .mcp_server_schemas import build_tool_schemas
from .mcp_server_resources import build_resources, read_resource

logger = logging.getLogger("infrastructure.mcp_server_wrapper")


def make_error_result(
    message: str,
    error: Exception | None = None,
    include_traceback: bool = False,
) -> CallToolResult:
    """Create a structured MCP error response."""
    content = TextContent(type="text", text=message)

    meta: dict[str, Any] = {}
    if include_traceback and error is not None:
        meta["traceback"] = traceback.format_exc()
        meta["error_type"] = type(error).__name__

    if error is not None:
        meta["error_type"] = type(error).__name__

    return CallToolResult(
        content=[content],
        isError=True,
        _meta=meta if meta else None,
    )


def tool_wrapper(
    func: Callable[..., Coroutine[Any, Any, Any]],
) -> Callable[..., Coroutine[Any, Any, Any]]:
    """Wrap an MCP tool handler with structured error handling."""

    async def wrapped(*args: Any, **kwargs: Any) -> CallToolResult | Any:
        try:
            return await func(*args, **kwargs)
        except FileNotFoundError as e:
            logger.debug("Tool %s: %s", func.__name__, e)
            return make_error_result(f"Path not found: {e}", error=e)
        except PermissionError as e:
            logger.warning("Tool %s: %s", func.__name__, e)
            return make_error_result(f"Permission denied: {e}", error=e)
        except asyncio.TimeoutError as e:
            logger.warning("Tool %s timed out", func.__name__)
            return make_error_result(
                "Operation timed out (300s limit exceeded)", error=e
            )
        except ToolError as e:
            logger.debug("Tool %s validation error: %s", func.__name__, e)
            return make_error_result(str(e), error=e)
        except Exception as e:
            logger.exception("Tool %s crashed: %s", func.__name__, e)
            return make_error_result(
                f"Internal error: {type(e).__name__}",
                error=e,
                include_traceback=True,
            )

    return wrapped


class McpServerWrapper(IMcpServerPort):
    """Infrastructure wrapper that wraps FastMCP with MCP spec compliance."""

    def __init__(
        self,
        project_root: str | Path = ".",
        server_name: str = "auto-linter",
    ) -> None:
        self._project_root = Path(project_root).resolve()
        self._server_name = server_name
        self._schemas = build_tool_schemas()
        self._resources = build_resources(self._project_root)

        # Create FastMCP with lifespan (decoupled from container at this stage)
        # The actual container is passed during setup/lifespan start
        self._server = FastMCP(
            self._server_name,
        )

    @property
    def server(self) -> FastMCP:
        return self._server

    def register_tool(
        self,
        name: SymbolName,
        description: DescriptionVO,
        handler: Callable[..., Coroutine[Any, Any, Any]],
    ) -> None:
        """Register a tool with structured error wrapping."""
        # Use the name/description from Port contract but apply our infra wrapper
        wrapped_handler = tool_wrapper(handler)
        self._server.add_tool(
            fn=wrapped_handler,
            name=name.value,
            description=description.value,
        )

    async def run_server(self) -> None:
        """Start the MCP server (Stdio transport by default)."""
        await self._server.run_stdio_async()

    def stop_server(self) -> None:
        """Stop server logic (FastMCP handles most via lifespan)."""
        pass

    def register_resource_handlers(self) -> None:
        """Register MCP resource handlers for rules and config."""

        @self._server.resource("auto-linter://rules/{filename}")
        async def get_rule(filename: str) -> str:
            return await read_resource(
                f"auto-linter://rules/{filename}", self._project_root
            )

        @self._server.resource("auto-linter://config/{filename}")
        async def get_config(filename: str) -> str:
            return await read_resource(
                f"auto-linter://config/{filename}", self._project_root
            )

        _list_resources = cast(Callable, self._server.list_resources())

        @_list_resources
        async def list_resources_func() -> list[Resource]:
            return self._resources

    def register_tool_schemas(self) -> None:
        """Register explicit JSON schemas for tool introspection."""

        @self._server.tool()
        async def auto_linter_schema(tool_name: str = "") -> dict[str, Any]:
            """Retrieve the JSON schemas for the registered tools.

            Args:
                tool_name: Optional tool name to fetch schema for. If empty, lists all tool schemas.
            """
            if tool_name:
                for schema in self._schemas:
                    if schema.name == tool_name:
                        return {
                            "name": schema.name,
                            "description": schema.description,
                            "input_schema": schema.input_schema,
                        }
                return {"error": f"Tool not found: {tool_name}"}
            return {
                "tools": [
                    {
                        "name": s.name,
                        "description": s.description,
                        "input_schema_keys": list(
                            s.input_schema.get("properties", {}).keys()
                        ),
                    }
                    for s in self._schemas
                ]
            }

    def register_version_info(self) -> None:
        """Register version compatibility tool."""

        @self._server.tool()
        async def auto_linter_version() -> dict[str, Any]:
            """Retrieve the MCP server version info and compatibility boundaries."""
            return {
                "server_version": MCP_SERVER_VERSION,
                "auto_lint_version": AUTO_LINT_VERSION,
                "protocol_min": MCP_PROTOCOL_MIN,
                "protocol_max": MCP_PROTOCOL_MAX,
                "protocol_version": "2025-06-18",
                "python_version": f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
            }

    def setup(self, container: Any) -> None:
        """Complete setup: register all infrastructure features.
        
        Args:
            container: The service container (Any to avoid Agent dependency).
        """
        self.register_resource_handlers()
        self.register_tool_schemas()
        self.register_version_info()
