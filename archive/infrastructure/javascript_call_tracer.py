"""JSTracer — Semantic analysis adapter for JavaScript/TypeScript files."""

from __future__ import annotations

import logging
import os
import re
import glob


from ..taxonomy import (
    CallChainList,
    Count,
    DataFlowList,
    DirectoryPath,
    FilePath,
    LineNumber,
    ResponseData,
    ResponseDataList,
    ScopeRef,
    SymbolName,
    SymbolNameList,
)
from ..contract import ISemanticTracerPort


logger = logging.getLogger("infrastructure.javascript_tracer")


# --- Inlined from naming_variant_generator (avoid cross-layer dependency
# capabilities→infrastructure) ---
class JSTracer(ISemanticTracerPort):
    """Regex-based semantic tracer for JavaScript/TypeScript files."""

    @staticmethod
    def _get_variant_dict(name):
        """Produce common naming variants for a symbol name."""
        n = name
        words = re.findall(r"[A-Za-z][a-z0-9]*|[A-Z]+(?=[A-Z][a-z0-9]|\b)|[0-9]+", n)
        words = [w.lower() for w in words]
        if not words:
            return {
                "snake_case": n,
                "camel_case": n,
                "pascal_case": n,
                "screaming_snake": n,
            }
        snake_case = "_".join(words)
        first = words[0]
        rest = "".join(w.capitalize() for w in words[1:])
        return {
            "snake_case": snake_case,
            "camel_case": first + rest,
            "pascal_case": "".join(w.capitalize() for w in words),
            "screaming_snake": snake_case.upper(),
        }

    @staticmethod
    def _build_variants_raw(name):
        """Build list of naming variants."""
        n = name
        d = JSTracer._get_variant_dict(n)
        kebab = d["snake_case"].replace("_", "-")
        return list(
            {
                n,
                d["snake_case"],
                d["camel_case"],
                d["pascal_case"],
                d["screaming_snake"],
                kebab,
            }
        )

    @staticmethod
    def _show_enclosing_scope(file_path, line):
        """Stub: return enclosing scope name for a given line in a JS/TS file."""
        return None

    @staticmethod
    def _find_flow(file_path: FilePath, var_name: SymbolName, start: LineNumber):
        """Stub: return data-flow entries for a variable in a JS/TS file."""
        return []

    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        return ResponseData(value=self._get_variant_dict(str(name)))

    def build_variants(self, name: SymbolName) -> SymbolNameList:
        variants = self._build_variants_raw(str(name))
        return SymbolNameList(values=[SymbolName(value=v) for v in variants])

    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        result = self._show_enclosing_scope(str(file_path), int(line))
        if result:
            return ScopeRef(name=result)
        return None

    def get_symbol_locations(
        self, file_path: FilePath, symbol_name: SymbolName
    ) -> ResponseDataList:
        """Return empty list — not yet implemented for JS tracer."""
        return ResponseDataList(values=[])

    def find_flow(
        self,
        file_path: FilePath,
        var_name: SymbolName,
        start_line: LineNumber | None = None,
    ) -> DataFlowList:
        start = start_line if start_line is not None else LineNumber(value=0)
        flows = self._find_flow(
            FilePath(value=str(file_path)), SymbolName(value=str(var_name)), start
        )
        return DataFlowList(values=flows)

    def trace_call_chain(
        self, root_dir: DirectoryPath, target_name: SymbolName
    ) -> CallChainList:
        callers = []
        name = str(target_name)
        root = str(root_dir)
        call_pattern = re.compile(rf"\b{re.escape(name)}\s*\(")
        def_pattern = re.compile(rf"(?:function|class)\s+{re.escape(name)}\b")
        js_files = []
        for ext in ("*.js", "*.jsx", "*.ts", "*.tsx", "*.mjs"):
            js_files.extend(glob.glob(os.path.join(root, "**", ext), recursive=True))
        for filepath in js_files:
            try:
                with open(filepath, "r", encoding="utf-8") as f:
                    file_lines = f.readlines()
            except OSError:
                continue
            for i, line in enumerate(file_lines):
                if call_pattern.search(line) and not def_pattern.search(line):
                    rel_path = os.path.relpath(filepath, root)
                    callers.append(f"{rel_path}:{i + 1} -> {line.strip()}")
        return CallChainList(values=callers)

    def project_wide_rename(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        root = str(root_dir)
        old = str(old_name)
        new = str(new_name)
        pattern = re.compile(
            rf"""
            (
                `(?:\\.|[^`\\])*`             |
                \"(?:\\.|[^\"\\])*\"          |
                '(?:\\.|[^'\\])*'             |
                //[^\n]*                      |
                /\*(?:.|\n)*?\*/
            )
            |
            \b({re.escape(old)})\b
            """,
            re.VERBOSE | re.DOTALL,
        )

        def replacer(match: re.Match):
            if match.group(1) is not None:
                return match.group(1)
            return new

        js_files = []
        for ext in ("*.js", "*.jsx", "*.ts", "*.tsx", "*.mjs"):
            js_files.extend(glob.glob(os.path.join(root, "**", ext), recursive=True))

        modified_count = 0
        for filepath in js_files:
            try:
                with open(filepath, "r", encoding="utf-8") as f:
                    source = f.read()
            except OSError:
                continue
            if old in source:
                new_source = pattern.sub(replacer, source)
                if new_source != source:
                    try:
                        with open(filepath, "w", encoding="utf-8") as f:
                            f.write(new_source)
                        modified_count += 1
                    except OSError:
                        pass
        return Count(value=modified_count)
