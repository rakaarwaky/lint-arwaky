"""JSTracer - Variable flow tracking for JS/TS files."""

from __future__ import annotations

import os
import re

from ..taxonomy import (
    FilePath,
    SymbolName,
    LineNumber,
    DataFlowList,
    LineContentVO,
    SemanticError,
    ErrorMessage,
)
from ..contract import IJSFlowTracerPort, IJSScopeProviderPort


class JSFlowTracer(IJSFlowTracerPort):
    """Variable flow tracking logic for JavaScript/TypeScript files."""

    def __init__(self, scope_provider: IJSScopeProviderPort):
        self._scope_provider = scope_provider

    def find_flow(
        self,
        file_path: FilePath,
        var_name: SymbolName,
        start_line: LineNumber | None = None,
    ) -> DataFlowList | SemanticError:
        """Track assignments and usages of a variable in a JS/TS file."""
        path_str = str(file_path.value)
        var_str = str(var_name.value)

        if not os.path.exists(path_str):
            return SemanticError(
                path=file_path,
                message=ErrorMessage(value=f"JavaScript file does not exist: {path_str}")
            )
        try:
            with open(path_str, "r", encoding="utf-8") as f:
                lines = f.readlines()
        except OSError as e:
            return SemanticError(
                path=file_path,
                message=ErrorMessage(value=f"Failed to read JavaScript file: {e}")
            )

        vo_lines = [LineContentVO(value=line) for line in lines]
        scope_bounds = self._scope_provider.find_scope_bounds(vo_lines, start_line)

        # Ensure we use .value for VOs
        scope_start = int(scope_bounds.start.value) if scope_bounds and scope_bounds.start else None
        scope_end = int(scope_bounds.end.value) if scope_bounds and scope_bounds.end else None

        flows: list[str] = []
        word_pattern = re.compile(rf"\b{re.escape(var_str)}\b")
        assign_pattern = re.compile(
            rf"(?:const|let|var)\s+{re.escape(var_str)}\s*=|(?<![=!<>]){re.escape(var_str)}\s*="
        )
        mutation_pattern = re.compile(
            rf"\b{re.escape(var_str)}\.(push|pop|shift|unshift|splice|sort|reverse|"
            rf"set|delete|add|assign|merge|update|append|extend)\b"
        )
        seen: set[str] = set()

        def _process_line(raw_line: str, lno: int) -> str | None:
            """Return flow entry string if line matches, else None."""
            if self._is_outside_scope(lno, scope_start, scope_end):
                # Signal stop if past scope_end
                if scope_end is not None and lno > scope_end:
                    return "__STOP__"
                return None
            if not word_pattern.search(raw_line):
                return None
            return self._create_flow_entry(
                lno, raw_line, assign_pattern, mutation_pattern, word_pattern
            )

        for i, raw_line in enumerate(lines):
            line_no = i + 1
            result = _process_line(raw_line, line_no)
            if result == "__STOP__":
                break
            if result is None:
                continue
            if result not in seen:
                seen.add(result)
                flows.append(result)

        return DataFlowList(values=flows)

    def _is_outside_scope(
        self, line_no: int, start: int | None, end: int | None
    ) -> bool:
        """Checks if a line is outside the given scope bounds."""
        if start is not None and line_no < start:
            return True
        if end is not None and line_no > end:
            return True
        return False

    def _create_flow_entry(
        self,
        line_no: int,
        line: str,
        assign_re: re.Pattern,
        mutation_re: re.Pattern,
        word_re: re.Pattern,
    ) -> str | None:
        """Creates a flow tracking entry based on the line content."""
        stripped = line.strip()
        match = mutation_re.search(line)
        if match:
            return f"Line {line_no} [Mutation '{match.group(1)}']: {stripped}"
        if assign_re.search(line):
            return f"Line {line_no} [Assignment]: {stripped}"
        if word_re.search(line):
            return f"Line {line_no} [Usage]: {stripped}"
        return None

    def trace_flow(self, path: FilePath) -> DataFlowList | SemanticError:
        """Trace overall data flow in a Javascript file (batch find_flow for all symbols)."""
        # Placeholder implementation for contract fulfillment
        return DataFlowList(values=[])
