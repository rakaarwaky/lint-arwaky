"""MCP Tools: list_commands and read_skill_context."""

from typing import Any
from pathlib import Path
from ..taxonomy import ContentString, MetadataVO

from ..contract import ServiceContainerAggregate


COMMAND_CATALOG = {
    "check": {
        "description": "Run full architecture compliance analysis",
        "example": "auto-lint check /path",
    },
    "scan": {
        "description": "Deep directory scan (alias for check)",
        "example": "auto-lint scan ./src/",
    },
    "fix": {"description": "Apply safe fixes", "example": "auto-lint fix file.py"},
    "report": {
        "description": "Generate quality reports",
        "example": "auto-lint report ./src --format json",
    },
    "ci": {
        "description": "CI-optimized with exit codes",
        "example": "auto-lint ci /path --exit-zero",
    },
    "batch": {
        "description": "Check multiple paths",
        "example": "auto-lint batch path1.py path2.js",
    },
    "watch": {
        "description": "Watch files for changes",
        "example": "auto-lint watch ./src/",
    },
    "security": {
        "description": "Bandit vulnerability scanning",
        "example": "auto-lint security /path",
    },
    "complexity": {
        "description": "Cyclomatic complexity",
        "example": "auto-lint complexity ./src/",
    },
    "duplicates": {
        "description": "Code duplication detection",
        "example": "auto-lint duplicates /path",
    },
    "trends": {
        "description": "Quality trend over time",
        "example": "auto-lint trends .",
    },
    "dependencies": {
        "description": "Dependency vulnerability scan",
        "example": "auto-lint dependencies .",
    },
    "diff": {
        "description": "Compare two versions",
        "example": "auto-lint diff v1.py v2.py",
    },
    "suggest": {
        "description": "AI-powered suggestions",
        "example": "auto-lint suggest file.py",
    },
    "stats": {
        "description": "Statistics dashboard",
        "example": "auto-lint stats ./src/",
    },
    "init": {"description": "Initialize config", "example": "auto-lint init /path"},
    "config": {
        "description": "Edit configuration",
        "example": "auto-lint config get thresholds",
    },
    "ignore": {
        "description": "Manage ignore rules",
        "example": "auto-lint ignore add E501",
    },
    "import": {
        "description": "Import configurations",
        "example": "auto-lint import config.json",
    },
    "export": {
        "description": "Export reports",
        "example": "auto-lint export --format sarif",
    },
    "clean": {"description": "Cleanup cache", "example": "auto-lint clean"},
    "update": {"description": "Update adapters", "example": "auto-lint update"},
    "doctor": {"description": "Diagnose issues", "example": "auto-lint doctor"},
    "adapters": {
        "description": "List enabled adapters",
        "example": "auto-lint adapters",
    },
    "install-hook": {
        "description": "Install git pre-commit hook",
        "example": "auto-lint install-hook",
    },
    "uninstall-hook": {
        "description": "Remove git pre-commit hook",
        "example": "auto-lint uninstall-hook",
    },
    "cancel": {
        "description": "Cancel a running lint job",
        "example": "auto-lint cancel <job_id>",
    },
    "plugins": {
        "description": "List discovered and registered plugins",
        "example": "auto-lint plugins",
    },
    "multi-project": {
        "description": "Run lint across multiple projects",
        "example": "auto-lint multi-project proj1/ proj2/",
    },
    "version": {"description": "Show version", "example": "auto-lint version"},
}


MCP_TOOLS_SCHEMAS = [
    {
        "name": "auto_linter_exec",
        "description": "Execute an auto-linter command. Use auto_linter_list_commands to see available actions.",
        "input_schema": {
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Command to execute (check, fix, report, scan, etc.)",
                    "minLength": 1,
                    "maxLength": 8192
                },
                "args": {
                    "type": "object",
                    "description": "Command arguments as key-value pairs",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Target path for analysis",
                            "minLength": 1,
                            "maxLength": 1024
                        },
                        "paths": {
                            "type": "array",
                            "description": "Multiple target paths",
                            "items": {"type": "string"},
                            "maxItems": 50
                        },
                        "format": {
                            "type": "string",
                            "enum": ["json", "text", "html", "sarif", "markdown"],
                            "description": "Output format"
                        }
                    }
                }
            },
            "required": ["action"],
            "additionalProperties": False
        }
    },
    {
        "name": "auto_linter_list_commands",
        "description": "List all available auto-linter commands with descriptions and usage examples.",
        "input_schema": {
            "type": "object",
            "properties": {
                "domain": {
                    "type": "string",
                    "description": "Filter commands by domain (e.g. 'python', 'javascript')",
                    "maxLength": 8192
                }
            },
            "additionalProperties": False
        }
    },
    {
        "name": "comands_schema",
        "description": "Retrieve the JSON schemas for the registered tools. Pass an optional tool_name to get a specific schema.",
        "input_schema": {
            "type": "object",
            "properties": {
                "tool_name": {
                    "type": "string",
                    "description": "Optional tool name to get schema for",
                    "maxLength": 8192
                }
            },
            "additionalProperties": False
        }
    },
    {
        "name": "auto_linter_health_check",
        "description": "Check overall system health, component status, and uptime.",
        "input_schema": {
            "type": "object",
            "properties": {},
            "additionalProperties": False
        }
    },
    {
        "name": "auto_linter_read_docs",
        "description": "Read SKILL.md documentation sections for auto-linter usage.",
        "input_schema": {
            "type": "object",
            "properties": {
                "section": {
                    "type": "string",
                    "description": "Section to read (omit for full documentation)",
                    "maxLength": 8192
                }
            },
            "additionalProperties": False
        }
    }
]


async def list_commands_func(domain: ContentString = ContentString(value="")):
    """Standalone function to list commands, used by other surfaces."""
    domain_val = str(domain)
    if domain_val:
        domain_key = domain_val.lower()
        commands = {k: v for k, v in COMMAND_CATALOG.items() if domain_key in k}
        return {"domain": domain_val, "commands": commands}

    result = {}
    for command, info in COMMAND_CATALOG.items():
        result[command] = {
            "description": info["description"],
            "example_usage": info["example"],
        }
    return result


class McpCommandCatalogSurface:
    """Surface for the command catalog MCP tool."""

    def __init__(self, mcp: Any = None) -> None:
        self.mcp = mcp
        self.container: ServiceContainerAggregate | None = None

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register the list_commands, read_skill_context, and schema tools."""
        self.container = container
        if self.mcp:
            self.mcp.tool()(self.list_commands)
            self.mcp.tool()(self.read_skill_context)
            self.mcp.tool(name="comands_schema")(self.comands_schema)

    async def list_commands(self, domain: ContentString = ContentString(value="")):
        """List all available AES CLI commands."""
        return await list_commands_func(domain)

    async def comands_schema(self, tool_name: ContentString = ContentString(value="")):
        """Retrieve the JSON schemas for the registered tools. Pass an optional tool_name to get a specific schema."""
        tool_name_str = str(tool_name).strip()
        if tool_name_str:
            for s in MCP_TOOLS_SCHEMAS:
                if s["name"] == tool_name_str or s["name"].replace("auto_linter_", "") == tool_name_str or s["name"].replace("auto_linter_exec", "execute_command") == tool_name_str:
                    return s
            return {"error": f"Tool not found: {tool_name_str}"}

        return {
            "tools": MCP_TOOLS_SCHEMAS
        }

    async def read_skill_context(
        self, section: ContentString = ContentString(value="")
    ):
        """Read SKILL.md documentation sections or the entire file."""
        root_dir = Path(__file__).resolve().parent.parent.parent
        skill_path = root_dir / "SKILL.md"

        if not skill_path.exists():
            return {"error": "SKILL.md not found", "path": str(skill_path)}

        try:
            content = skill_path.read_text(encoding="utf-8")

            # If no section specified, return the WHOLE file as requested
            section_val = str(section)
            if not section_val or section_val.lower() in [
                "all",
                "full",
                "entire",
                "skill.md",
            ]:
                return {"section": "Full Documentation", "content": content.strip()}

            docs: MetadataVO = self._parse_markdown_sections(
                ContentString(value=content)
            )

            query = section_val.lower()
            for key, data in docs.items():
                if query in key:
                    return {"section": data["title"], "content": data["body"]}

            return {
                "error": f"Section '{section}' not found",
                "available_sections": [d["title"] for d in docs.values()],
            }
        except Exception as e:
            return {"error": f"Failed to read documentation: {str(e)}"}

    def _parse_markdown_sections(self, content: ContentString) -> MetadataVO:
        """Parse markdown into a MetadataVO of section {'title':..., 'body':...}."""
        lines = str(content).splitlines()
        docs_dict = {}
        current_title = "Introduction"
        current_section = "introduction"
        current_content: list[str] = []

        for line in lines:
            if line.startswith("# ") or line.startswith("## "):
                if current_content:
                    docs_dict[current_section] = {
                        "title": current_title,
                        "body": "\n".join(current_content).strip(),
                    }
                current_title = line.lstrip("#").strip()
                current_section = current_title.lower()
                current_content = [line]
            else:
                current_content.append(line)

        if current_content:
            docs_dict[current_section] = {
                "title": current_title,
                "body": "\n".join(current_content).strip(),
            }
        return MetadataVO(value=docs_dict)


def register_catalog_commands(mcp, container: ServiceContainerAggregate) -> None:
    """Factory function for container-aware registration."""
    surface = McpCommandCatalogSurface(mcp)
    surface.register_all(container)


def register_list_commands(mcp, container: ServiceContainerAggregate) -> None:
    """Legacy wrapper updated to be container-aware."""
    register_catalog_commands(mcp, container)


def register_read_skill_context(mcp, container: ServiceContainerAggregate) -> None:
    """Legacy wrapper updated to be container-aware."""
    register_catalog_commands(mcp, container)
