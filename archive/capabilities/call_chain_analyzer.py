"""call_chain_analyzer — Call chain analysis capability."""

from __future__ import annotations
from ..taxonomy import (
    CallChainList,
    Count,
    DataFlowList,
    DirectoryPath,
    FileContentVO,
    FilePath,
    LineNumber,
    ResponseData,
    ResponseDataList,
    ScopeRef,
    SymbolName,
    SymbolNameList,
)

import re


from ..contract import (
    IDataFlowProtocol,
    IFileSystemPort,
    IScopeBoundaryResolverProtocol,
    ISemanticTracerProtocol,
    INamingVariantProtocol,
)


class CallChainAnalyzer(ISemanticTracerProtocol):
    """Call chain analyzer for JavaScript/TypeScript files."""

    def __init__(
        self,
        fs: IFileSystemPort,
        data_flow: IDataFlowProtocol,
        naming: INamingVariantProtocol,
        scope: IScopeBoundaryResolverProtocol | None = None,
    ):
        self._fs = fs
        self._data_flow = data_flow
        self._naming = naming
        self._scope = scope

    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        return self._naming.get_variant_dict(name)

    def build_variants(self, name: SymbolName) -> SymbolNameList:
        return self._naming.build_variants(name)

    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        """Resolve enclosing scope using the scope resolver capability."""
        if not self._scope:
            return None
        return self._scope.resolve_enclosing_scope(file_path, line)

    def get_symbol_locations(
        self, file_path: FilePath, symbol: SymbolName
    ) -> ResponseDataList:
        """Stub for symbol location retrieval."""
        return ResponseDataList(values=[])

    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber
    ) -> DataFlowList:
        return self._data_flow.find_flow(file_path, var_name, start_line)

    def trace_call_chain(
        self, root_dir: DirectoryPath, target_name: SymbolName
    ) -> CallChainList:
        """Find all call sites for the target name within the project."""
        callers: list[SymbolName] = []
        name_str = str(target_name.value)
        call_pattern = re.compile(rf"\b{re.escape(name_str)}\s*\(")
        def_pattern = re.compile(rf"(?:function|class)\s+{re.escape(name_str)}\b")

        js_files: list[FilePath] = []
        for ext in ("*.js", "*.jsx", "*.ts", "*.tsx", "*.mjs"):
            js_files.extend(list(self._fs.glob(f"{root_dir}/**/{ext}")))

        for filepath in js_files:
            source_vo = self._fs.read_text(filepath)
            if not source_vo or not source_vo.value:
                continue

            file_lines = source_vo.value.splitlines()

            for i, line_str in enumerate(file_lines):
                if call_pattern.search(line_str) and not def_pattern.search(line_str):
                    rel_path = self._fs.get_relative_path(
                        filepath, DirectoryPath(value=str(root_dir))
                    )
                    call_site = f"{rel_path}:{i + 1} -> {line_str.strip()}"
                    callers.append(SymbolName(value=call_site))

        return CallChainList(values=callers)

    def project_wide_rename(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        """Rename a symbol across all files in the project."""
        old_str = str(old_name.value)
        new_str = str(new_name.value)

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
            \b({re.escape(old_str)})\b
            """,
            re.VERBOSE | re.DOTALL,
        )

        def _replacer(match: re.Match) -> str:
            if match.group(1) is not None:
                return match.group(1)
            return new_str

        js_files: list[FilePath] = []
        for ext in ("*.js", "*.jsx", "*.ts", "*.tsx", "*.mjs"):
            js_files.extend(list(self._fs.glob(f"{root_dir}/**/{ext}")))

        modified_count = 0
        for filepath in js_files:
            source_vo = self._fs.read_text(filepath)
            if not source_vo or not source_vo.value:
                continue

            source = source_vo.value

            if old_str in source:
                new_source = pattern.sub(_replacer, source)
                if new_source != source:
                    res = self._fs.write_text(filepath, FileContentVO(value=new_source))
                    if res.value:
                        modified_count += 1

        return Count(value=modified_count)
