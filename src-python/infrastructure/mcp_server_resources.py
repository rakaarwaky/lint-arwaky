"""MCP Server Resources — Resource handling for auto-linter."""

from __future__ import annotations

from pathlib import Path
from pydantic import AnyUrl
from mcp.types import Resource


def build_resources(project_root: Path) -> list[Resource]:
    """Build MCP Resource URIs for rule definitions and config."""
    resources: list[Resource] = []

    # Rule documentation resources
    docs_dir = project_root / "docs"
    if docs_dir.exists():
        for md_file in sorted(docs_dir.glob("*.md")):
            resources.append(
                Resource(
                    uri=AnyUrl(f"auto-linter://rules/{md_file.name}"),
                    name=f"Auto-Linter Rules: {md_file.stem}",
                    description=f"Rule definitions from {md_file.name}",
                    mimeType="text/markdown",
                )
            )

    # Config resource
    for config_name in [
        "auto_linter.config.python.yaml",
        "auto_linter.config.javascript.yaml",
        "auto_linter.config.rust.yaml",
        "auto_linter.config.json",
        "pyproject.toml",
    ]:
        config_path = project_root / config_name
        if config_path.exists():
            resources.append(
                Resource(
                    uri=AnyUrl(f"auto-linter://config/{config_name}"),
                    name=f"Auto-Linter Config: {config_name}",
                    description=f"Configuration from {config_name}",
                    mimeType="application/x-yaml"
                    if config_name.endswith(".yaml")
                    else "application/toml",
                )
            )

    return resources


async def read_resource(uri: str, project_root: Path) -> str:
    """Read a resource by URI."""
    if not uri.startswith("auto-linter://"):
        raise ValueError(f"Unknown resource URI scheme: {uri}")

    # Parse: auto-linter://rules/file.md or auto-linter://config/file.yaml
    parts = uri.removeprefix("auto-linter://").split("/", 1)
    if len(parts) != 2:
        raise ValueError(f"Invalid resource URI: {uri}")

    resource_type, filename = parts

    if resource_type == "rules":
        file_path = project_root / "docs" / filename
    elif resource_type == "config":
        file_path = project_root / filename
    else:
        raise ValueError(f"Unknown resource type: {resource_type}")

    # Security: ensure resolved path is under project root
    resolved = file_path.resolve()
    if not resolved.is_relative_to(project_root.resolve()):
        raise ValueError(f"Resource path escapes project root: {filename}")

    if not file_path.exists():
        raise FileNotFoundError(f"Resource not found: {filename}")

    return file_path.read_text(encoding="utf-8")
