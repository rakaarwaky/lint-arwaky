"""JS/TS regex patterns and scope detection helpers."""

from __future__ import annotations

import re
from typing import Sequence

from ..taxonomy import (
    LineContentVO,
    LineNumber,
    ScopeBounds,
    SymbolName,
    LineContentList,
    SemanticError,
    ErrorMessage,
)
from ..contract import IJSScopeProviderPort


class JSScopeProvider(IJSScopeProviderPort):
    """Patterns and logic for JS/TS scope detection."""

    def detect_js_scope(self, stripped_line: LineContentVO) -> SymbolName | SemanticError | None:
        """Detect if a stripped JS/TS line opens a named scope."""
        # Define patterns locally to avoid AES006 on class attributes
        class_pattern = re.compile(
            r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?"
        )
        function_patterns: Sequence[re.Pattern] = [
            re.compile(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\("),
            re.compile(
                r"(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>"
            ),
            re.compile(
                r"^\s+(?:async\s+|static\s+|private\s+|protected\s+|public\s+)*([A-Za-z_$][A-Za-z0-9_$]*)\s*\("
            ),
        ]

        line_str = str(stripped_line)
        match = class_pattern.search(line_str)
        if match:
            return SymbolName(value=f"class {match.group(1)}")
        for pattern in function_patterns:
            match = pattern.search(line_str)
            if match:
                name = match.group(1)
                if name not in {"if", "for", "while", "switch", "catch", "else"}:
                    return SymbolName(value=f"function {name}")
        return None

    def find_scope_bounds(
        self, lines: LineContentList, scope_line: LineNumber | None
    ) -> ScopeBounds | SemanticError | None:
        """Find start/end line numbers of enclosing function body via brace counting."""
        if scope_line is None:
            return None

        line_idx = int(scope_line)
        if line_idx <= 0 or line_idx > len(lines):
            return SemanticError(
                message=ErrorMessage(value=f"Scope line {line_idx} is out of bounds (1 to {len(lines)})")
            )
        brace_depth = 0
        scope_start: int | None = None
        scope_end: int | None = None

        for i in range(line_idx - 1, len(lines)):
            line = str(lines[i])
            if "{" in line and scope_start is None:
                scope_start = i + 1
                brace_depth = 1
                continue
            if scope_start is not None:
                brace_depth += line.count("{") - line.count("}")
                if brace_depth <= 0:
                    scope_end = i + 1
                    break

        if scope_start is not None and scope_end is not None:
            return ScopeBounds(
                start=LineNumber(value=scope_start), end=LineNumber(value=scope_end)
            )
        return None
