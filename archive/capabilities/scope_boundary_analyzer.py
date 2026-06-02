"""scope_boundary_analyzer — JS/TS scope boundary detection."""

from __future__ import annotations
from ..taxonomy import FileContentVO, FilePath, LineContentVO, LineNumber, SymbolName

import re

from ..contract import IScopeBoundaryProtocol
from ..contract import IFileSystemPort


_FUNCTION_PATTERNS: list[re.Pattern] = [
    re.compile(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\("),
    re.compile(
        r"(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>"
    ),
    re.compile(
        r"^(?:async\s+|static\s+|private\s+|protected\s+|public\s+)*([A-Za-z_$][A-Za-z0-9_$]*)\s*\("
    ),
]

_CLASS_PATTERN = re.compile(
    r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?"
)


class ScopeBoundaryAnalyzer(IScopeBoundaryProtocol):
    """Capability for detecting JS/TS scope boundaries."""

    def __init__(self, fs: IFileSystemPort):
        self.fs = fs

    def detect_js_scope(self, stripped_line: LineContentVO) -> SymbolName | None:
        res = self._detect_js_scope_raw(str(stripped_line))
        return SymbolName(value=res) if res else None

    def find_scope_bounds(
        self, content: FileContentVO, scope_line: LineNumber | None
    ) -> tuple[LineNumber | None, LineNumber | None]:
        lines = str(content).splitlines()
        start, end = self._find_scope_bounds_raw(
            lines, int(scope_line) if scope_line is not None else None
        )
        return (
            LineNumber(value=start) if start is not None else None,
            LineNumber(value=end) if end is not None else None,
        )

    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> SymbolName | None:
        res = self._get_enclosing_scope_raw(file_path, line)
        return SymbolName(value=res) if res else None

    def _detect_js_scope_raw(self, stripped_line: str) -> str | None:
        """Detect if a stripped JS/TS line opens a named scope."""
        match = _CLASS_PATTERN.search(stripped_line)
        if match:
            return f"class {match.group(1)}"
        for pattern in _FUNCTION_PATTERNS:
            match = pattern.search(stripped_line)
            if match:
                name = match.group(1)
                if name not in {"if", "for", "while", "switch", "catch", "else"}:
                    return f"function {name}"
        return None

    def _find_scope_bounds_raw(
        self, lines: list[str], scope_line: int | None
    ) -> tuple[int | None, int | None]:
        """Find start/end line numbers of enclosing function body via brace counting."""
        if scope_line is None:
            return None, None
        brace_depth = 0
        scope_start: int | None = None
        scope_end: int | None = None
        for i in range(scope_line - 1, len(lines)):
            line = lines[i]
            if "{" in line and scope_start is None:
                scope_start = i + 1
                brace_depth = 1
                continue
            if scope_start is not None:
                brace_depth += line.count("{") - line.count("}")
                if brace_depth <= 0:
                    scope_end = i + 1
                    break
        return scope_start, scope_end

    def _update_scope_stack(
        self,
        stack: list[tuple[str, int]],
        depth: int,
        detected_scope: str | None,
        raw_line: str,
    ) -> tuple[list[tuple[str, int]], int]:
        """Pop expired scopes, optionally push new, then update depth and pop again."""
        while stack and depth <= stack[-1][1]:
            stack.pop()
        if detected_scope and "{" in raw_line:
            stack.append((detected_scope, depth))
        depth += raw_line.count("{") - raw_line.count("}")
        while stack and depth <= stack[-1][1]:
            stack.pop()
        return stack, depth

    def _get_enclosing_scope_raw(
        self, file_path: FilePath, line: LineNumber
    ) -> str | None:
        """Find the nearest enclosing function or class for a given 1-indexed line."""
        if not self.fs.is_file(file_path).value:
            return None

        try:
            content = self.fs.read_file(file_path).value
            lines = content.splitlines()
        except Exception:
            return None

        scope_stack: list[tuple[str, int]] = []
        brace_depth = 0

        for i, raw_line in enumerate(lines):
            current_line_no = i + 1
            stripped = raw_line.strip()
            detected_scope = self._detect_js_scope_raw(stripped)

            scope_stack, brace_depth = self._update_scope_stack(
                scope_stack, brace_depth, detected_scope, raw_line
            )

            if current_line_no == int(line):
                break

        return " -> ".join(s[0] for s in scope_stack) if scope_stack else None
