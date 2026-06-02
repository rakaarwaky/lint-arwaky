"""DataFlowAnalyzer — Capability for analyzing data flow patterns."""

from __future__ import annotations
from ..taxonomy import DataFlowList, FilePath, LineNumber, SymbolName

import logging
import re


from ..contract import IDataFlowProtocol


from ..contract import IFileSystemPort

from ..contract import IScopeBoundaryProtocol
from .scope_boundary_analyzer import ScopeBoundaryAnalyzer

logger = logging.getLogger(__name__)


class DataFlowAnalyzer(IDataFlowProtocol):
    """Capability for tracking variable lifecycle in JS/TS files."""

    def __init__(
        self, fs: IFileSystemPort, scope: IScopeBoundaryProtocol | None = None
    ):
        self._fs = fs
        self._scope = scope or ScopeBoundaryAnalyzer(fs)

    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber
    ) -> DataFlowList:
        flows = self._find_flow_raw(
            self._fs, file_path, var_name, int(start_line), self._scope
        )
        return DataFlowList(values=flows)

    def _try_build_entry(
        self,
        raw_line: str,
        line_no: int,
        word_pattern: re.Pattern,
        assign_pattern: re.Pattern,
        mutation_pattern: re.Pattern,
        seen: set[str],
    ) -> str | None:
        """Return a formatted flow entry for this line, or None if not relevant."""
        if not word_pattern.search(raw_line):
            return None
        stripped = raw_line.strip()
        match = mutation_pattern.search(raw_line)
        if match:
            entry = f"Line {line_no} [Mutation '{match.group(1)}']: {stripped}"
        elif assign_pattern.search(raw_line):
            entry = f"Line {line_no} [Assignment]: {stripped}"
        elif word_pattern.search(raw_line):
            entry = f"Line {line_no} [Usage]: {stripped}"
        else:
            entry = None
        if entry and entry in seen:
            return None
        if entry:
            seen.add(entry)
        return entry

    def _walk_calls_in_body(
        self,
        lines: list[str],
        word_pattern: re.Pattern,
        assign_pattern: re.Pattern,
        mutation_pattern: re.Pattern,
        scope_start: int | None,
        scope_end: int | None,
    ) -> list[str]:
        """Walk through lines dispatching each to pattern matchers and collecting flow entries."""
        flows: list[str] = []
        seen: set[str] = set()
        for i, raw_line in enumerate(lines):
            line_no = i + 1
            if scope_start is not None and line_no < scope_start:
                continue
            if scope_end is not None and line_no > scope_end:
                break
            entry = self._try_build_entry(
                raw_line,
                line_no,
                word_pattern,
                assign_pattern,
                mutation_pattern,
                seen,
            )
            if entry:
                flows.append(entry)
        return flows

    def _find_flow_raw(
        self,
        fs: IFileSystemPort,
        file_path: FilePath,
        var_name: SymbolName,
        start_line: int | None = None,
        scope_analyzer: IScopeBoundaryProtocol | None = None,
    ) -> list[str]:
        """Track assignments and usages of a variable in a JS/TS file."""
        if not fs.exists(file_path):
            return []

        try:
            content = fs.read_text(file_path)
            lines = content.value.splitlines()
        except Exception:
            return []

        vn = str(var_name)
        scope_start = None
        scope_end = None
        if scope_analyzer:
            start_line_vo = LineNumber(value=start_line) if start_line else None
            s_start_vo, s_end_vo = scope_analyzer.find_scope_bounds(
                content, start_line_vo
            )
            scope_start = int(s_start_vo) if s_start_vo is not None else None
            scope_end = int(s_end_vo) if s_end_vo is not None else None

        word_pattern = re.compile(rf"\b{re.escape(vn)}\b")
        assign_pattern = re.compile(
            rf"(?:const|let|var)\s+{re.escape(vn)}\s*=|(?<![=!<>]){re.escape(vn)}\s*="
        )
        mutation_pattern = re.compile(
            rf"\b{re.escape(vn)}\.(push|pop|shift|unshift|splice|sort|reverse|"
            rf"set|delete|add|assign|merge|update|append|extend)\b"
        )

        return self._walk_calls_in_body(
            lines,
            word_pattern,
            assign_pattern,
            mutation_pattern,
            scope_start,
            scope_end,
        )
