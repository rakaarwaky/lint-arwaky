"""ast_py_scanner — Orchestrator for Python AST analysis."""

from __future__ import annotations
import ast
from .python_primitive_checker import PrimitiveChecker
from .python_symbol_collector import SymbolCollector
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
    BooleanVO,
    Identity,
    SourceParserError,
    SyntaxErrorVO,
    ErrorMessage,
    LineNumber,
    ColumnNumber,
    PatternList,
    ImportInfo,
    ModuleName,
    PrimitiveViolation,
    PrimitiveTypeName,
)
from ..contract import ISourceParserPort


class ASTPythonParserAdapter(ISourceParserPort):
    """
    Adapter that orchestrates specialized Python AST analyzers.
    Strictly follows ISourceParserPort and uses domain-appropriate VOs.
    """

    def __init__(self) -> None:
        self._collector_cache: dict[FilePath, SymbolCollector] = {}
        self._primitive_checker = PrimitiveChecker()
        try:
            from auto_linter import auto_linter_rust
            self._native = auto_linter_rust.NativeASTPythonParser()
        except ImportError:
            self._native = None

    def _parse_file(self, path: FilePath) -> ast.AST:
        with open(str(path), "r", encoding="utf-8") as f:
            return ast.parse(f.read())

    def _get_collector(self, path: FilePath) -> SymbolCollector:
        if path in self._collector_cache:
            return self._collector_cache[path]

        tree = self._parse_file(path)
        collector = SymbolCollector()
        collector.visit(tree)
        self._collector_cache[path] = collector
        return collector

    def extract_imports(self, path: FilePath) -> ImportInfoList | SourceParserError:
        if self._native is not None:
            try:
                res = self._native.extract_imports(str(path))
                if isinstance(res, dict) and "error" in res:
                    return SourceParserError(
                        path=path,
                        message=ErrorMessage(value=res["error"])
                    )
                values = []
                for item in res:
                    values.append(ImportInfo(
                        line=LineNumber(value=item["line"]["value"]),
                        module=ModuleName(value=item["module"]["value"]),
                        name=SymbolName(value=item["name"]) if item.get("name") else None
                    ))
                return ImportInfoList(values=values)
            except Exception:
                pass

        try:
            collector = self._get_collector(path)
            return ImportInfoList(values=collector.imports_list)
        except SyntaxError as e:
            return SyntaxErrorVO(
                path=path,
                message=ErrorMessage(value=str(e)),
                line=LineNumber(value=e.lineno) if e.lineno else None,
                column=ColumnNumber(value=e.offset) if e.offset else None
            )
        except Exception as e:
            return SourceParserError(
                path=path,
                message=ErrorMessage(value=f"Failed to parse source imports: {e}")
            )

    def get_raw_symbols(self, path: FilePath) -> ResponseData | SourceParserError:
        if self._native is not None:
            try:
                res = self._native.get_raw_symbols(str(path))
                if isinstance(res, dict) and "error" in res:
                    return SourceParserError(
                        path=path,
                        message=ErrorMessage(value=res["error"])
                    )
                return ResponseData(value=res)
            except Exception:
                pass

        try:
            collector = self._get_collector(path)
            return ResponseData(
                value={
                    "defined": [s.value for s in collector.defined.values],
                    "used": [s.value for s in collector.used.values],
                    "exported": [s.value for s in collector.exported.values],
                    "aliases": collector.imported_aliases.value,
                    "class_bases": collector.class_bases.mapping,
                }
            )
        except SyntaxError as e:
            return SyntaxErrorVO(
                path=path,
                message=ErrorMessage(value=str(e)),
                line=LineNumber(value=e.lineno) if e.lineno else None,
                column=ColumnNumber(value=e.offset) if e.offset else None
            )
        except Exception as e:
            return SourceParserError(
                path=path,
                message=ErrorMessage(value=f"Failed to extract symbols: {e}")
            )

    def get_class_attributes(self, path: FilePath) -> ResponseData:
        if self._native is not None:
            try:
                res = self._native.get_class_attributes(str(path))
                return ResponseData(value=res)
            except Exception:
                pass
        return ResponseData(value={})

    def has_all_export(self, path: FilePath) -> SuccessStatus:
        if self._native is not None:
            try:
                val = self._native.has_all_export(str(path))
                return SuccessStatus(value=BooleanVO(value=val))
            except Exception:
                pass

        collector = self._get_collector(path)
        return SuccessStatus(value=BooleanVO(value=bool(collector._exported)))

    def find_primitive_violations(
        self, path: FilePath, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        if self._native is not None:
            try:
                prim_words = [p.value if hasattr(p, "value") else str(p) for p in primitive_types.values]
                res = self._native.find_primitive_violations(str(path), prim_words)
                violations = []
                for item in res:
                    violations.append(PrimitiveViolation(
                        line=LineNumber(value=item["line"]["value"]),
                        column=ColumnNumber(value=item["column"]["value"]),
                        type_name=PrimitiveTypeName(value=item["type_name"]["value"])
                    ))
                return PrimitiveViolationList(values=violations)
            except Exception:
                pass

        tree = self._parse_file(path)
        return self._primitive_checker.find_primitive_violations(
            path, tree, primitive_types
        )

    def find_unused_imports(self, path: FilePath) -> ImportInfoList:
        if self._native is not None:
            try:
                res = self._native.find_unused_imports(str(path))
                values = []
                for item in res:
                    values.append(ImportInfo(
                        line=LineNumber(value=item["line"]["value"]),
                        module=ModuleName(value=item["module"]["value"]),
                        name=SymbolName(value=item["name"]) if item.get("name") else None
                    ))
                return ImportInfoList(values=values)
            except Exception:
                pass

        # Unified logic using existing taxonomy VOs
        symbols = self.get_raw_symbols(path).value
        aliases = symbols.get("aliases", {})
        used = set(symbols.get("used", []))
        exported = set(symbols.get("exported", []))

        unused_infos = []
        collector = self._get_collector(path)
        for imp_info in collector.imports_list:
            module_name = str(imp_info.module)
            found_use = False
            if module_name in used or module_name in exported:
                found_use = True

            for alias, fullname in aliases.items():
                if fullname == module_name and (alias in used or alias in exported):
                    found_use = True
                    break

            if not found_use:
                unused_infos.append(imp_info)

        return ImportInfoList(values=unused_infos)

    def get_class_definitions(self, path: FilePath) -> MetadataVO | SourceParserError:
        if self._native is not None:
            try:
                res = self._native.get_class_definitions(str(path))
                if isinstance(res, dict) and "error" in res:
                    return SourceParserError(
                        path=path,
                        message=ErrorMessage(value=res["error"])
                    )
                return MetadataVO(value=res)
            except Exception:
                pass

        try:
            collector = self._get_collector(path)
            return MetadataVO(value={"classes": collector.class_definitions})
        except SyntaxError as e:
            return SyntaxErrorVO(
                path=path,
                message=ErrorMessage(value=str(e)),
                line=LineNumber(value=e.lineno) if e.lineno else None,
                column=ColumnNumber(value=e.offset) if e.offset else None
            )
        except Exception as e:
            return SourceParserError(
                path=path,
                message=ErrorMessage(value=f"Failed to get class definitions: {e}")
            )

    def get_function_definitions(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_function_definitions(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        collector = self._get_collector(path)
        return MetadataVO(value={"functions": collector.function_definitions})

    def is_symbol_exported(
        self, path: FilePath, symbol: SymbolName | Identity
    ) -> SuccessStatus:
        if self._native is not None:
            try:
                sym_val = symbol.value if hasattr(symbol, "value") else str(symbol)
                val = self._native.is_symbol_exported(str(path), sym_val)
                return SuccessStatus(value=BooleanVO(value=val))
            except Exception:
                pass

        collector = self._get_collector(path)
        sym_val = symbol.value if hasattr(symbol, "value") else str(symbol)
        return SuccessStatus(value=BooleanVO(value=sym_val in collector._exported))

    def get_class_methods(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_class_methods(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        collector = self._get_collector(path)
        return MetadataVO(value=collector.class_methods)

    def get_class_bases_map(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_class_bases_map(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        collector = self._get_collector(path)
        return MetadataVO(value=collector.class_bases.mapping)

    def get_assignment_targets(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_assignment_targets(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        collector = self._get_collector(path)
        return MetadataVO(value={"assignments": collector.assignments})

    def get_control_flow_count(self, path: FilePath) -> Count:
        if self._native is not None:
            try:
                val = self._native.get_control_flow_count(str(path))
                return Count(value=val)
            except Exception:
                pass

        collector = self._get_collector(path)
        return Count(value=collector.control_flow_count)

    def is_barrel_file(self, path: FilePath) -> bool:
        if self._native is not None:
            try:
                return self._native.is_barrel_file(str(path))
            except Exception:
                pass

        return str(path).endswith("__init__.py")

    def get_stem(self, path: FilePath) -> SymbolName:
        if self._native is not None:
            try:
                val = self._native.get_stem(str(path))
                return SymbolName(value=val)
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        return SymbolName(value=basename.replace(".py", ""))

    def is_entry_point(self, path: FilePath) -> bool:
        if self._native is not None:
            try:
                return self._native.is_entry_point(str(path))
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        return basename in ["__init__.py", "main.py", "py.typed"]

    def get_supported_extensions(self) -> PatternList:
        return PatternList(values=[".py"])
