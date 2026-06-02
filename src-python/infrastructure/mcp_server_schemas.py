"""MCP Server Schemas — JSON Schema definitions for auto-linter tools."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any
from mcp.types import Tool

from .mcp_server_constants import (
    MAX_STRING_LENGTH,
    MAX_PATH_LENGTH,
    MAX_BATCH_SIZE,
)


@dataclass
class ToolSchema:
    """Explicit JSON Schema definition for an MCP tool."""

    name: str
    description: str
    input_schema: dict[str, Any]
    output_schema: dict[str, Any] | None = None

    def to_mcp_tool(self) -> Tool:
        """Convert to MCP Tool type."""
        tool_kwargs: dict[str, Any] = {
            "name": self.name,
            "description": self.description,
            "inputSchema": self.input_schema,
        }
        if self.output_schema is not None:
            tool_kwargs["outputSchema"] = self.output_schema
        return Tool(**tool_kwargs)


def build_tool_schemas() -> list[ToolSchema]:
    """Build explicit JSON Schemas for all auto-linter MCP tools."""
    schemas: list[ToolSchema] = []

    # Generic execute_command schema
    schemas.append(
        ToolSchema(
            name="auto_linter_exec",
            description="Execute an auto-linter command. Use auto_linter_list_commands to see available actions.",
            input_schema={
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "description": "Command to execute (check, fix, report, scan, etc.)",
                        "minLength": 1,
                        "maxLength": MAX_STRING_LENGTH,
                    },
                    "args": {
                        "type": "object",
                        "description": "Command arguments as key-value pairs",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "Target path for analysis",
                                "minLength": 1,
                                "maxLength": MAX_PATH_LENGTH,
                            },
                            "paths": {
                                "type": "array",
                                "description": "Multiple target paths",
                                "items": {"type": "string"},
                                "maxItems": MAX_BATCH_SIZE,
                            },
                            "format": {
                                "type": "string",
                                "enum": ["json", "text", "html", "sarif", "markdown"],
                                "description": "Output format",
                            },
                        },
                    },
                },
                "required": ["action"],
                "additionalProperties": False,
            },
        )
    )

    # list_commands schema
    schemas.append(
        ToolSchema(
            name="auto_linter_list_commands",
            description="List all available auto-linter commands with descriptions and usage examples.",
            input_schema={
                "type": "object",
                "properties": {
                    "domain": {
                        "type": "string",
                        "description": "Filter commands by domain (e.g. 'python', 'javascript')",
                        "maxLength": MAX_STRING_LENGTH,
                    },
                },
                "additionalProperties": False,
            },
        )
    )

    # comands_schema schema
    schemas.append(
        ToolSchema(
            name="comands_schema",
            description="Retrieve the JSON schemas for the registered tools. Pass an optional tool_name to get a specific schema.",
            input_schema={
                "type": "object",
                "properties": {
                    "tool_name": {
                        "type": "string",
                        "description": "Optional tool name to get schema for",
                        "maxLength": MAX_STRING_LENGTH,
                    },
                },
                "additionalProperties": False,
            },
        )
     )



    # health_check schema
    schemas.append(
        ToolSchema(
            name="auto_linter_health_check",
            description="Check overall system health, component status, and uptime.",
            input_schema={
                "type": "object",
                "properties": {},
                "additionalProperties": False,
            },
        )
    )

    # read_docs schema
    schemas.append(
        ToolSchema(
            name="auto_linter_read_docs",
            description="Read SKILL.md documentation sections for auto-linter usage.",
            input_schema={
                "type": "object",
                "properties": {
                    "section": {
                        "type": "string",
                        "description": "Section to read (omit for full documentation)",
                        "maxLength": MAX_STRING_LENGTH,
                    },
                },
                "additionalProperties": False,
            },
        )
    )

    return schemas
