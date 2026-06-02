"""JS Tracer - Enclosing scope detection for JS/TS files."""

from __future__ import annotations

import os

from ..taxonomy import FilePath, LineNumber, ScopeRef, LineContentVO, SemanticError
from ..contract import IJSScopeTracerPort


class JSScopeTracer(IJSScopeTracerPort):
    """Detection logic for enclosing scopes in JavaScript/TypeScript files."""

    def __init__(self):
        from . import JSScopeProvider

        self._scope_provider = JSScopeProvider()

    def _pop_invalid_scopes(
        self,
        scope_stack: list[tuple[str, int]],
        brace_depth: int,
    ) -> None:
        """Remove scopes that have been closed at current brace depth."""
        while scope_stack:
            last_scope = scope_stack[-1]
            if brace_depth <= last_scope[1]:
                scope_stack.pop()
            else:
                break

    def show_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | SemanticError | None:
        """Find the nearest enclosing function or class for a given 1-indexed line."""
        path_str = str(file_path)
        line_int = int(line)

        if not os.path.exists(path_str):
            return None
        try:
            with open(path_str, "r", encoding="utf-8") as f:
                lines = f.readlines()
        except OSError:
            return None

        scope_stack: list[tuple[str, int]] = []
        brace_depth = 0
        best_match: list[str] = []

        for i, raw_line in enumerate(lines):
            current_line_no = i + 1
            stripped = raw_line.strip()
            detected_scope = self._scope_provider.detect_js_scope(
                LineContentVO(value=stripped)
            )
            braces_on_line = raw_line.count("{") - raw_line.count("}")

            self._pop_invalid_scopes(scope_stack, brace_depth)

            if detected_scope and "{" in raw_line:
                scope_stack.append((detected_scope, brace_depth))

            brace_depth += braces_on_line

            self._pop_invalid_scopes(scope_stack, brace_depth)

            if current_line_no == line_int:
                best_match = [s[0] for s in scope_stack]
                break

        if best_match:
            return ScopeRef(name=" -> ".join(best_match))
        return None
