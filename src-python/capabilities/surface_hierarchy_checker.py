"""SurfaceHierarchyChecker — AES018/AES019 for surface hierarchy enforcement.

AES018 SURFACE_HIERARCHY_VIOLATION:
A file that is NOT an __init__.py barrel in the surfaces layer is not
imported from the layer __init__.py — meaning it is unreachable from the
surface entry point.

AES019 PASSIVE_SURFACE_VIOLATION:
A surface file contains complex domain logic (many public methods, deep
control flow) instead of acting as a thin pass-through to the agent layer.
Surfaces must be declarative/passive — I/O parsing + delegation only.
"""
from __future__ import annotations

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


class SurfaceHierarchyChecker:
    """AES018 + AES019 — surface barrel wiring and passivity checks."""

    # Thresholds for AES019
    _MAX_PUBLIC_METHODS = 10
    _MAX_FUNCTION_BODY_LINES = 80
    _MAX_IF_DEPTH = 3

    def check_surface_hierarchy(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Main entry point — run AES018 (barrel wiring) and AES019 (passive surface)."""
        for f in files:
            if not _is_in_surfaces(f):
                continue
            if _is_init(f):
                continue

            basename = str(analyzer.fs.get_basename(f))
            layer_vo = analyzer._detect_layer(f, root_dir)
            if layer_vo:
                definition = analyzer.layer_map.get(layer_vo)
                if definition and definition.exceptions.values and basename in definition.exceptions.values:
                    continue

            # AES018: check if file is wired in barrel
            if not _is_wired(f):
                desc = (
                    f"AES018 SURFACE_HIERARCHY_VIOLATION: Surface file "
                    f"'{f.value}' is not imported from the layer barrel.\n"
                    f"WHY? All surface files must be reachable through __init__.py "
                    f"to maintain a clear entry-point hierarchy.\n"
                    f"FIX: Add 'from .{_stem(f)} import ...' to "
                    f"{_directory(f)}/__init__.py, or delete if unused."
                )
                results.values.append(
                    LintResult(
                        code=ErrorCode(code="AES018"),
                        message=LintMessage(value=desc),
                        severity=Severity.CRITICAL,
                        file=f,
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=1),
                    )
                )

            # AES019: check if file is passive
            self._check_passive(f, results)

    def _check_passive(self, f: FilePath, results: LintResultList) -> None:
        """Check if a surface file is passive (thin I/O boundary)."""
        try:
            content = open(f.value, "r", encoding="utf-8").read()
            tree = ast.parse(content, filename=f.value)
        except (SyntaxError, UnicodeDecodeError, OSError):
            return

        violations: list[str] = []
        for node in ast.walk(tree):
            if not isinstance(node, ast.ClassDef):
                continue
            pub_methods = _collect_public_methods(node)
            self._check_methods_too_public(node, pub_methods, violations)
            self._check_method_lengths(node, pub_methods, violations)
            self._check_method_nesting(node, pub_methods, violations)

        if violations:
            self._report_aes019(f, violations, results)

    # -- AES019 sub-checks (each isolated to reduce CC) -------------------------

    def _check_methods_too_public(
        self, node: ast.ClassDef, pub_methods: list[ast.FunctionDef | ast.AsyncFunctionDef], violations: list[str],
    ) -> None:
        """AES019: too many public methods in a surface class."""
        if len(pub_methods) > self._MAX_PUBLIC_METHODS:
            violations.append(
                f"Class '{node.name}' has {len(pub_methods)} public "
                f"methods (max {self._MAX_PUBLIC_METHODS})"
            )

    def _check_method_lengths(
        self, node: ast.ClassDef, pub_methods: list[ast.FunctionDef | ast.AsyncFunctionDef], violations: list[str],
    ) -> None:
        """AES019: method body exceeds line limit."""
        for method in pub_methods:
            body_len = (method.end_lineno or 0) - method.lineno
            if body_len > self._MAX_FUNCTION_BODY_LINES:
                violations.append(
                    f"Method '{node.name}.{method.name}' is {body_len} lines "
                    f"(max {self._MAX_FUNCTION_BODY_LINES})"
                )

    def _check_method_nesting(
        self, node: ast.ClassDef, pub_methods: list[ast.FunctionDef | ast.AsyncFunctionDef], violations: list[str],
    ) -> None:
        """AES019: method control-flow nesting exceeds limit."""
        for method in pub_methods:
            for child in ast.walk(method):
                if isinstance(child, ast.If) and _if_depth(child) > self._MAX_IF_DEPTH:
                    violations.append(
                        f"Method '{node.name}.{method.name}' has deep "
                        f"control flow (if-nesting > {self._MAX_IF_DEPTH})"
                    )
                    break

    def _report_aes019(
        self, f: FilePath, violations: list[str], results: LintResultList,
    ) -> None:
        """Append a single AES019 result to the results list."""
        detail = "\n".join(f"  - {v}" for v in violations)
        results.values.append(
            LintResult(
                code=ErrorCode(code="AES019"),
                message=LintMessage(
                    value=(
                        f"AES019 PASSIVE_SURFACE_VIOLATION: Surface file "
                        f"'{f.value}' contains active domain logic:\n{detail}\n"
                        f"WHY? Surfaces must be passive I/O boundaries. "
                        f"Business logic belongs in capabilities/agent layers.\n"
                        f"FIX: Move logic to a handler or orchestrator."
                    )
                ),
                severity=Severity.CRITICAL,
                file=f,
                line=LineNumber(value=1),
                column=ColumnNumber(value=1),
            )
        )


# --- helpers -----------------------------------------------------------------

def _collect_public_methods(node: ast.ClassDef) -> list[ast.FunctionDef | ast.AsyncFunctionDef]:
    """Return public (non-underscore) function/method definitions from a class body."""
    return [
        n
        for n in node.body
        if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef))
        and not n.name.startswith("_")
    ]


def _is_in_surfaces(f: FilePath) -> bool:
    return "/surfaces/" in f.value or f.value.endswith("/surfaces")


def _is_init(f: FilePath) -> bool:
    return f.value.endswith("__init__.py") or f.value.endswith("mod.rs") or f.value.endswith("index.ts") or f.value.endswith("index.js")


def _stem(f: FilePath) -> str:
    return f.value.rsplit("/", 1)[-1].rsplit(".", 1)[0]


def _directory(f: FilePath) -> str:
    return f.value.rsplit("/", 1)[0]


def _is_wired(f: FilePath) -> bool:
    """Return True if module stem is imported in its directory barrel."""
    barrel_names = ["__init__.py", "mod.rs", "index.ts", "index.js"]
    for name in barrel_names:
        init_path = _directory(f) + "/" + name
        try:
            content = open(init_path, "r", encoding="utf-8").read()
            stem = _stem(f)
            if (
                f"import {stem}" in content
                or f"from .{stem}" in content
                or f'"{stem}"' in content
                or f"'{stem}'" in content
                or f"mod {stem}" in content
                or f"use {stem}" in content
            ):
                return True
        except (FileNotFoundError, OSError):
            continue
    return False


def _if_depth(node: ast.If, depth: int = 0) -> int:
    max_d = depth
    for child in ast.iter_child_nodes(node):
        if isinstance(child, ast.If):
            max_d = max(max_d, _if_depth(child, depth + 1))
    return max_d
