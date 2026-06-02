"""source_parser_orchestrator — A language-agnostic dispatcher orchestrator for source parsing."""

from __future__ import annotations
from ..taxonomy import (
    FilePath,
    ImportInfoList,
    MetadataVO,
    ResponseData,
    PrimitiveTypeList,
    PrimitiveViolationList,
    SuccessStatus,
    SymbolName,
    Count,
    Identity,
    SourceParserError,
    PatternList,
)
from ..contract import ISourceParserPort
from .ast_py_scanner import ASTPythonParserAdapter
from .ast_rust_scanner import ASTRustParserAdapter
from .ast_js_scanner import ASTJSParserAdapter


class SourceParserOrchestrator(ISourceParserPort):
    """Orchestrates different parser adapters depending on the file extension."""

    def __init__(self) -> None:
        self._python_parser = ASTPythonParserAdapter()
        self._rust_parser = ASTRustParserAdapter()
        self._js_parser = ASTJSParserAdapter()

    def _select_parser(self, path: FilePath) -> ISourceParserPort:
        path_str = str(path)
        if path_str.endswith(".rs"):
            return self._rust_parser
        elif any(path_str.endswith(ext) for ext in [".ts", ".tsx", ".js", ".jsx"]):
            return self._js_parser
        return self._python_parser

    def extract_imports(self, path: FilePath) -> ImportInfoList | SourceParserError:
        return self._select_parser(path).extract_imports(path)

    def get_raw_symbols(self, path: FilePath) -> ResponseData | SourceParserError:
        return self._select_parser(path).get_raw_symbols(path)

    def get_class_attributes(self, path: FilePath) -> ResponseData:
        return self._select_parser(path).get_class_attributes(path)

    def has_all_export(self, path: FilePath) -> SuccessStatus:
        return self._select_parser(path).has_all_export(path)

    def find_primitive_violations(
        self, path: FilePath, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        return self._select_parser(path).find_primitive_violations(path, primitive_types)

    def find_unused_imports(self, path: FilePath) -> ImportInfoList:
        return self._select_parser(path).find_unused_imports(path)

    def get_class_definitions(self, path: FilePath) -> MetadataVO | SourceParserError:
        return self._select_parser(path).get_class_definitions(path)

    def get_function_definitions(self, path: FilePath) -> MetadataVO:
        return self._select_parser(path).get_function_definitions(path)

    def is_symbol_exported(
        self, path: FilePath, symbol: SymbolName | Identity
    ) -> SuccessStatus:
        return self._select_parser(path).is_symbol_exported(path, symbol)

    def get_class_methods(self, path: FilePath) -> MetadataVO:
        return self._select_parser(path).get_class_methods(path)

    def get_class_bases_map(self, path: FilePath) -> MetadataVO:
        return self._select_parser(path).get_class_bases_map(path)

    def get_assignment_targets(self, path: FilePath) -> MetadataVO:
        return self._select_parser(path).get_assignment_targets(path)

    def get_control_flow_count(self, path: FilePath) -> Count:
        return self._select_parser(path).get_control_flow_count(path)

    def is_barrel_file(self, path: FilePath) -> bool:
        return self._select_parser(path).is_barrel_file(path)

    def get_stem(self, path: FilePath) -> SymbolName:
        return self._select_parser(path).get_stem(path)

    def is_entry_point(self, path: FilePath) -> bool:
        return self._select_parser(path).is_entry_point(path)

    def get_supported_extensions(self) -> PatternList:
        # Return unique combined list of extensions or let caller select
        # A simple general list of all extensions we support:
        return PatternList(values=[".py", ".rs", ".ts", ".tsx", ".js", ".jsx"])
