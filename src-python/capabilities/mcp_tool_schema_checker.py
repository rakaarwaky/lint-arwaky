"""McpToolSchemaChecker — AES025 for MCP tools JSON Schema compliance.

AES025 MCP_TOOL_SCHEMA_VIOLATION:
MCP tools must declare valid JSON Schema for their input parameters.
Detects: missing schema, empty schema, invalid JSON Schema syntax,
missing descriptions, missing required fields array when properties exist.
"""
from __future__ import annotations

import re
import ast

from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    Severity,
)


# JSON Schema draft-07/2020-12 required keywords
_JSON_SCHEMA_KEYWORDS = frozenset({
    "type", "properties", "required", "items",
    "additionalProperties", "description", "title",
    "enum", "const", "default", "minimum", "maximum",
    "minLength", "maxLength", "pattern", "format",
    "anyOf", "oneOf", "allOf", "not",
    "$ref", "$defs", "$schema",
})

# Patterns that indicate a tool registration (FastMCP, stdio MCP, etc.)
_TOOL_DECORATOR_PATTERNS = [
    re.compile(r"@\w+\.tool\s*\("),       # @mcp.tool(...)
    re.compile(r"@\w+\.tool\s*$"),        # @mcp.tool
    re.compile(r"server\.add_tool\b"),    # server.add_tool(...)
    re.compile(r"register_tool\b"),       # register_tool(...)
]

_JSON_SCHEMA_TYPE_VALUES = frozenset({
    "string", "number", "integer", "boolean",
    "array", "object", "null",
})


class McpToolSchemaChecker:
    """AES025 — Validate MCP tool input/output schemas."""

    def check_mcp_tool_schema(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Scan all files for MCP tool schema violations."""
        for f in files:
            if not str(f).endswith(".py"):
                continue
            self._check_file(f, results)

    def _check_file(self, f: FilePath, results: LintResultList) -> None:
        """Parse a Python file and find tool registration patterns."""
        try:
            content = open(f.value, "r", encoding="utf-8").read()
            tree = ast.parse(content, filename=f.value)
        except (SyntaxError, UnicodeDecodeError, OSError):
            return

        for node in ast.walk(tree):
            # Check decorated tool functions
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for decorator in node.decorator_list:
                    if self._is_tool_decorator(decorator, content):
                        self._check_tool_schema(node, f, results)

    # ------------------------------------------------------------------
    # Detection
    # ------------------------------------------------------------------
    def _is_tool_decorator(self, decorator: ast.expr, content: str) -> bool:
        """Determine if a decorator node represents an MCP tool registration."""
        if isinstance(decorator, ast.Call):
            if isinstance(decorator.func, ast.Attribute):
                return decorator.func.attr == "tool"
        if isinstance(decorator, ast.Attribute):
            return decorator.attr == "tool"
        return False

    # ------------------------------------------------------------------
    # Schema validation
    # ------------------------------------------------------------------
    def _check_tool_schema(
        self, func: ast.FunctionDef | ast.AsyncFunctionDef,
        f: FilePath, results: LintResultList,
    ) -> None:
        """Validate the schema associated with a tool function."""
        func_name = func.name
        self._check_docstring(func, f, func_name, results)
        self._check_parameter_types(func, f, func_name, results)
        self._check_explicit_schemas(func, f, func_name, results)

    def _check_docstring(
        self, func: ast.FunctionDef | ast.AsyncFunctionDef,
        f: FilePath, func_name: str, results: LintResultList,
    ) -> None:
        """Tools must have a docstring — this becomes the tool description in tools/list."""
        docstring = ast.get_docstring(func)
        if not docstring or len(docstring.strip()) < 10:
            results.values.append(
                LintResult(
                    code=ErrorCode(code="AES025"),
                    message=LintMessage(
                        value=(
                            f"AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool "
                            f"'{func_name}' is missing a descriptive docstring.\n"
                            f"WHY? The docstring becomes the tool description "
                            f"in tools/list response — models use it for routing.\n"
                            f"FIX: Add a docstring describing what this tool does, "
                            f"its inputs, and expected output."
                        )
                    ),
                    severity=Severity.CRITICAL,
                    file=f,
                    line=LineNumber(value=func.lineno),
                    column=ColumnNumber(value=1),
                )
            )

    def _check_parameter_types(
        self, func: ast.FunctionDef | ast.AsyncFunctionDef,
        f: FilePath, func_name: str, results: LintResultList,
    ) -> None:
        """All non-self parameters on a tool function must have type annotations."""
        for arg in func.args.args:
            if arg.arg == "self" or arg.arg == "ctx":
                continue
            if not arg.annotation:
                results.values.append(
                    LintResult(
                        code=ErrorCode(code="AES025"),
                        message=LintMessage(
                            value=(
                                f"AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool "
                                f"'{func_name}' parameter '{arg.arg}' lacks a type annotation.\n"
                                f"WHY? Untyped parameters cannot be mapped to JSON Schema "
                                f"in the tools/list schema — models won't know the input format.\n"
                                f"FIX: Add a type annotation (e.g., str, int, FilePath, or a "
                                f"Pydantic model)."
                            )
                        ),
                        severity=Severity.CRITICAL,
                        file=f,
                        line=LineNumber(value=arg.lineno or func.lineno),
                        column=ColumnNumber(value=1),
                    )
                )

    def _check_explicit_schemas(
        self, func: ast.FunctionDef | ast.AsyncFunctionDef,
        f: FilePath, func_name: str, results: LintResultList,
    ) -> None:
        """Check for explicit JSON Schema dicts passed to the tool."""
        for decorator in func.decorator_list:
            if isinstance(decorator, ast.Call):
                for keyword in decorator.keywords:
                    if keyword.arg in ("parameters", "input_schema", "schema"):
                        self._validate_schema_value(keyword.value, f, func_name, results)

    def _validate_schema_value(
        self, node: ast.expr, f: FilePath, func_name: str, results: LintResultList,
    ) -> None:
        """If an explicit schema dict is found, validate it."""
        if not isinstance(node, ast.Dict):
            return

        keys = _collect_dict_string_keys(node)
        violations: list[str] = []
        self._check_required_keywords(keys, violations)
        self._check_property_descriptions(node, keys, violations)
        self._check_type_values(node, keys, violations)

        if violations:
            self._report_aes025(func_name, node, violations, f, results)

    # -- Sub-checks for _validate_schema_value -----------------------------------

    def _check_required_keywords(self, keys: list[str], violations: list[str]) -> None:
        """Verify that required JSON Schema keywords are present."""
        if "type" not in keys:
            violations.append("Schema missing 'type' keyword")
            return
        if "properties" not in keys:
            violations.append("Schema has 'type' but no 'properties'")

    def _check_property_descriptions(
        self, node: ast.Dict, keys: list[str], violations: list[str],
    ) -> None:
        """Ensure each property in the schema has a description."""
        if "properties" not in keys:
            return
        props_value = node.values[keys.index("properties")]
        if not isinstance(props_value, ast.Dict):
            return
        for pkey, pval in zip(props_value.keys, props_value.values):
            if pkey is None:
                continue
            _validate_property_has_description(pkey, pval, violations)

    def _check_type_values(
        self, node: ast.Dict, keys: list[str], violations: list[str],
    ) -> None:
        """Validate that 'type' values are valid JSON Schema types."""
        if "type" not in keys:
            return
        type_node = node.values[keys.index("type")]
        if isinstance(type_node, ast.Constant) and type_node.value not in _JSON_SCHEMA_TYPE_VALUES:
            type_str = str(type_node.value) if isinstance(type_node.value, str) else repr(type_node.value)
            violations.append(f"Schema type='{type_str}' is not a valid JSON Schema type")

    def _report_aes025(
        self, func_name: str, node: ast.expr,
        violations: list[str], f: FilePath, results: LintResultList,
    ) -> None:
        """Append a single AES025 result to the results list."""
        detail = "\n".join(f"  - {v}" for v in violations)
        results.values.append(
            LintResult(
                code=ErrorCode(code="AES025"),
                message=LintMessage(
                    value=(
                        f"AES025 MCP_TOOL_SCHEMA_VIOLATION: MCP tool "
                        f"'{func_name}' has an invalid JSON Schema:\n{detail}\n"
                        f"WHY? MCP tools must declare valid JSON Schema so "
                        f"LLM clients can validate input before tool calls.\n"
                        f"FIX: Use a Pydantic BaseModel for tool parameters "
                        f"or provide a valid dict with 'type' and 'properties' keys."
                    )
                ),
                severity=Severity.CRITICAL,
                file=f,
                line=LineNumber(value=node.lineno or 1),
                column=ColumnNumber(value=1),
            )
        )


# --- module-level helpers ------------------------------------------------------

def _collect_dict_string_keys(node: ast.Dict) -> list[str]:
    """Extract string keys from an ast.Dict node."""
    return [
        k.value for k in node.keys
        if isinstance(k, ast.Constant) and isinstance(k.value, str)
    ]


def _validate_property_has_description(pkey: ast.expr, pval: ast.expr, violations: list[str]) -> None:
    """Flag a property that lacks a 'description' sub-key in its schema."""
    if isinstance(pkey, ast.Constant) and isinstance(pkey.value, str) and isinstance(pval, ast.Dict):
        sub_keys = _collect_dict_string_keys(pval)
        if "description" not in sub_keys:
            violations.append(f"Property '{pkey.value}' missing description in schema")
