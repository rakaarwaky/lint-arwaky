"""scope_boundary_resolver — Capability for resolving code scope boundaries (JS/TS)."""

from ..taxonomy import (
    FilePath,
    LineNumber,
    ScopeRef,
    SymbolName,
    Count,
    LineContentVO,
    FileContentVO,
)

import re


from ..contract import IFileSystemPort, IScopeBoundaryResolverProtocol


class ScopeBoundaryResolver(IScopeBoundaryResolverProtocol):
    """Business logic for detecting and resolving function/class boundaries."""

    def __init__(self, fs_scanner: IFileSystemPort):
        self._fs = fs_scanner

        # Function patterns for JS/TS - use internal types for logic
        self._FUNCTION_PATTERNS = [
            re.compile(r"(?:async\s+)?function\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*\("),
            re.compile(
                r"(?:const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:async\s*)?(?:\([^)]*\)|[A-Za-z_$][A-Za-z0-9_$]*)\s*=>"
            ),
            re.compile(
                r"^\s+(?:async\s+|static\s+|private\s+|protected\s+|public\s+)*([A-Za-z_$][A-Za-z0-9_$]*)\s*\("
            ),
        ]
        self._CLASS_PATTERN = re.compile(
            r"class\s+([A-Za-z_$][A-Za-z0-9_$]*)(?:\s+extends\s+[A-Za-z_$][A-Za-z0-9_$]*)?"
        )

    def resolve_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        """Identifies the hierarchy of scopes enclosing a specific line."""
        content: FileContentVO = self._fs.read_text(file_path)
        if not content or not content.value:
            return None

        # Process lines as VOs immediately
        raw_lines = content.value.splitlines()
        target_line_val = int(line)

        # Use domain types for internal stack tracking
        # tuple[SymbolName, Count] -> (ScopeName, DepthAtWhichScopeStarted)
        scope_stack: list[tuple[SymbolName, Count]] = []
        brace_depth = Count(value=0)

        for i, raw_line_str in enumerate(raw_lines):
            current_line_no = LineNumber(value=i + 1)
            raw_line_vo = LineContentVO(value=raw_line_str)
            stripped_vo = LineContentVO(value=raw_line_str.strip())

            # Update scope stack based on this line
            scope_stack, brace_depth = self._update_scope_stack(
                scope_stack,
                brace_depth,
                stripped_vo,
                raw_line_vo,
            )

            if int(current_line_no) == target_line_val:
                if scope_stack:
                    hierarchy_name = " -> ".join(str(s[0]) for s in scope_stack)
                    return ScopeRef(
                        name=SymbolName(value=hierarchy_name),
                        kind=SymbolName(value="hierarchy"),
                    )
                return None

        return None

    def _update_scope_stack(
        self,
        stack: list[tuple[SymbolName, Count]],
        depth: Count,
        stripped: LineContentVO,
        raw_line: LineContentVO,
    ) -> tuple[list[tuple[SymbolName, Count]], Count]:
        """Updates brace depth and scope stack for a single line."""
        depth_val = int(depth)
        raw_line_str = str(raw_line)

        # Remove expired scopes (exited before this line's braces)
        while stack and depth_val <= int(stack[-1][1]):
            stack.pop()
            stack.pop()

        # Detect and add new scope
        detected = self._detect_js_scope(stripped)
        if detected and "{" in raw_line_str:
            stack.append((detected, Count(value=depth_val)))

        # Apply brace count
        new_depth_val = depth_val + raw_line_str.count("{") - raw_line_str.count("}")

        # Remove any scopes that closed on this line
        while stack and new_depth_val <= int(stack[-1][1]):
            stack.pop()

        return stack, Count(value=new_depth_val)

    def _detect_js_scope(self, stripped_line: LineContentVO) -> SymbolName | None:
        """Helper to detect if a line starts a new scope."""
        line_str = str(stripped_line)
        match = self._CLASS_PATTERN.search(line_str)
        if match:
            return SymbolName(value=f"class {match.group(1)}")

        for pattern in self._FUNCTION_PATTERNS:
            match = pattern.search(line_str)
            if match:
                name = match.group(1)
                # Ensure it's not a keyword masquerading as a function name (e.g., if
                # (condition))
                if name not in {"if", "for", "while", "switch", "catch", "else"}:
                    return SymbolName(value=f"function {name}")
        return None
