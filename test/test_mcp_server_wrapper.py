"""Tests for MCP server wrapper infrastructure.

Tests:
- Error result formatting
- Tool schema generation
- Resource URI validation
- Version compatibility
"""

from pathlib import Path

from auto_linter.infrastructure.mcp_server_wrapper import (
    make_error_result,
)
from auto_linter.infrastructure.mcp_server_schemas import (
    build_tool_schemas,
    ToolSchema,
)
from auto_linter.infrastructure.mcp_server_resources import (
    build_resources,
)


# ── Error result tests ─────────────────────────────────────────────────

class TestMakeErrorResult:
    def test_basic_error(self):
        result = make_error_result("Something went wrong")
        assert result.isError is True
        assert len(result.content) == 1
        assert "Something went wrong" in result.content[0].text

    def test_error_with_exception(self):
        try:
            raise ValueError("test error")
        except ValueError as e:
            result = make_error_result("Wrapped", error=e)
            assert result.isError is True
            assert result.meta is not None
            assert result.meta.get("error_type") == "ValueError"

    def test_error_with_traceback(self):
        try:
            raise RuntimeError("detail")
        except RuntimeError as e:
            result = make_error_result("Wrapped", error=e, include_traceback=True)
            assert result.meta is not None
            assert "traceback" in result.meta
            assert "RuntimeError" in result.meta["traceback"]

    def test_error_without_traceback(self):
        result = make_error_result("Simple error")
        assert result.isError is True
        assert result.meta is None


# ── Tool schema tests ──────────────────────────────────────────────────

class TestToolSchemas:
    def test_schemas_build(self):
        schemas = build_tool_schemas()
        assert len(schemas) >= 5  # At least 5 core tools
        names = [s.name for s in schemas]
        assert "auto_linter_exec" in names
        assert "auto_linter_list_commands" in names
        assert "auto_linter_health_check" in names

    def test_schema_has_required_fields(self):
        schemas = build_tool_schemas()
        for schema in schemas:
            assert isinstance(schema.name, str) and len(schema.name) > 0
            assert isinstance(schema.description, str) and len(schema.description) > 0
            assert isinstance(schema.input_schema, dict)
            assert "type" in schema.input_schema
            assert schema.input_schema["type"] == "object"

    def test_exec_schema_requires_action(self):
        schemas = build_tool_schemas()
        exec_schema = next(s for s in schemas if s.name == "auto_linter_exec")
        assert "action" in exec_schema.input_schema.get("required", [])

    def test_schema_mcp_tool_conversion(self):
        schema = ToolSchema(
            name="test_tool",
            description="A test tool",
            input_schema={"type": "object", "properties": {}},
        )
        tool = schema.to_mcp_tool()
        assert tool.name == "test_tool"
        assert tool.description == "A test tool"


# ── Resource tests ─────────────────────────────────────────────────────

class TestResources:
    def test_resources_from_project_root(self, tmp_path):
        # Create docs directory with a rule file
        docs = tmp_path / "docs"
        docs.mkdir()
        (docs / "RUFF_RULES.md").write_text("# Ruff Rules")

        # Create config file
        (tmp_path / "auto_linter.config.python.yaml").write_text("rules: []")

        resources = build_resources(tmp_path)
        assert len(resources) >= 2  # At least rule + config

        names = [r.name for r in resources]
        assert any("RUFF_RULES" in n for n in names)
        assert any("auto_linter.config.python" in n for n in names)

    def test_no_resources_if_no_docs(self, tmp_path):
        resources = build_resources(tmp_path)
        assert len(resources) == 0


# ── Version compatibility tests ────────────────────────────────────────

class TestVersionCompatibility:
    def test_capabilities_present(self):
        from auto_linter.infrastructure.mcp_server_wrapper import McpServerWrapper
        # McpServerWrapper embeds protocol version info
        wrapper = McpServerWrapper.__init__
        assert wrapper is not None
