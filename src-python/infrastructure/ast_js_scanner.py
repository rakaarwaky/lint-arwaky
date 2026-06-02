"""ast_js_scanner — Specialized parser for Javascript and Typescript using regex analysis."""

from __future__ import annotations
import re
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
    ErrorMessage,
    LineNumber,
    ColumnNumber,
    PrimitiveTypeName,
    ImportInfo,
    ModuleName,
    PrimitiveViolation,
    PatternList,
)
from ..contract import ISourceParserPort


class RegexJSParser:
    """A highly robust and lightweight regex-based symbol collector for JS/TS."""

    @staticmethod
    def _parse_class(
        line: str,
        stripped: str,
        idx: int,
        class_pattern: re.Pattern,
        imported_aliases: dict,
        class_bases: dict,
        class_defs: list,
        defined: set,
    ) -> tuple[str | None, int]:
        class_match = class_pattern.search(stripped)
        if class_match:
            cname = class_match.group(1)
            base = class_match.group(2)
            defined.add(cname)

            resolved_base = imported_aliases.get(base, base) if base else None
            if base:
                class_bases[cname] = [base]

            class_defs.append({
                "name": cname,
                "line": idx,
                "column": line.find(cname),
                "is_dead": False,
                "bases": [base] if base else [],
                "resolved_bases": [resolved_base] if resolved_base else []
            })
            return cname, 0
        return None, 0

    @staticmethod
    def _parse_imports(
        stripped: str,
        idx: int,
        import_pattern: re.Pattern,
        require_pattern: re.Pattern,
        imported_aliases: dict,
        imports_list: list,
    ) -> None:
        imp_match = import_pattern.match(stripped)
        if imp_match:
            imports_raw = imp_match.group(1).strip()
            module_path = imp_match.group(2).strip().replace("/", ".")
            module_path = module_path.lstrip(".")

            if "{" in imports_raw:
                symbols = imports_raw.split("{", 1)[1].rstrip("}").split(",")
                for sym in symbols:
                    sym = sym.strip()
                    if " as " in sym:
                        name, alias = sym.split(" as ", 1)
                        name, alias = name.strip(), alias.strip()
                    else:
                        name = alias = sym
                    if name:
                        imported_aliases[alias] = f"{module_path}.{name}"
                        imports_list.append(ImportInfo(
                            line=LineNumber(value=idx),
                            module=ModuleName(value=f"{module_path}.{name}")
                        ))
            else:
                alias = imports_raw
                imported_aliases[alias] = module_path
                imports_list.append(ImportInfo(
                    line=LineNumber(value=idx),
                    module=ModuleName(value=module_path)
                ))
            return

        req_match = require_pattern.match(stripped)
        if req_match:
            alias = req_match.group(1).strip()
            module_path = req_match.group(2).strip().replace("/", ".").lstrip(".")
            imported_aliases[alias] = module_path
            imports_list.append(ImportInfo(
                line=LineNumber(value=idx),
                module=ModuleName(value=module_path)
            ))

    @staticmethod
    def _parse_functions_and_methods(
        line: str,
        stripped: str,
        idx: int,
        fn_pattern: re.Pattern,
        current_class: str | None,
        class_match_found: bool,
        class_methods: dict,
        func_defs: list,
        defined: set,
    ) -> None:
        fn_match = fn_pattern.search(stripped)
        if fn_match:
            name = fn_match.group(1)
            defined.add(name)
            func_defs.append({
                "name": name,
                "line": idx,
                "column": line.find(name)
            })

        if current_class and not class_match_found:
            method_match = re.match(r'^(?:async\s+)?(\w+)\s*\([^)]*\)\s*\{', stripped)
            if method_match:
                mname = method_match.group(1)
                if mname not in ["if", "for", "while", "switch"]:
                    if current_class not in class_methods:
                        class_methods[current_class] = []
                    class_methods[current_class].append(mname)

    @staticmethod
    def _parse_assignments(
        line: str,
        stripped: str,
        idx: int,
        assignments: list,
    ) -> None:
        if stripped.startswith("const ") or stripped.startswith("let ") or stripped.startswith("var "):
            assign_match = re.match(r'^(?:const|let|var)\s+(\w+)\s*=', stripped)
            if assign_match:
                assignments.append({
                    "name": assign_match.group(1),
                    "type": "Assign",
                    "line": idx,
                    "column": line.find(assign_match.group(1))
                })

    @staticmethod
    def parse(content: str, path: FilePath) -> dict:
        defined = set()
        used = set()
        exported = set()
        imported_aliases = {}
        class_bases = {}
        imports_list = []
        class_defs = []
        func_defs = []
        class_methods = {}
        assignments = []
        control_flow_count = 0

        lines = content.splitlines()

        import_pattern = re.compile(r'^import\s+([^from]+)\s+from\s+[\'"]([^\'"]+)[\'"]')
        require_pattern = re.compile(r'^(?:const|let|var)\s+(\w+)\s*=\s*require\([\'"]([^\'"]+)[\'"]\)')
        class_pattern = re.compile(r'^class\s+(\w+)(?:\s+extends\s+(\w+))?')
        fn_pattern = re.compile(r'^(?:async\s+)?function\s+(\w+)')
        cf_pattern = re.compile(r'\b(if|for|while|switch|catch)\b')

        current_class = None
        brace_count = 0

        for idx, line in enumerate(lines, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("//") or stripped.startswith("/*") or stripped.startswith("*"):
                continue

            open_braces = stripped.count("{")
            close_braces = stripped.count("}")

            cname, _ = RegexJSParser._parse_class(
                line, stripped, idx, class_pattern, imported_aliases, class_bases, class_defs, defined
            )
            class_match_found = False
            if cname:
                current_class = cname
                brace_count = 0
                class_match_found = True

            brace_count += open_braces - close_braces
            if brace_count < 0:
                brace_count = 0
                current_class = None

            # 1. Imports
            RegexJSParser._parse_imports(
                stripped, idx, import_pattern, require_pattern, imported_aliases, imports_list
            )

            # 2. Functions / Methods
            RegexJSParser._parse_functions_and_methods(
                line, stripped, idx, fn_pattern, current_class, class_match_found, class_methods, func_defs, defined
            )

            # 3. Assignments
            RegexJSParser._parse_assignments(
                line, stripped, idx, assignments
            )

            # 4. Control Flow
            cf_matches = cf_pattern.findall(stripped)
            control_flow_count += len(cf_matches)

            # 5. Used symbols
            for word in re.findall(r'\b[a-zA-Z_][a-zA-Z0-9_]*\b', stripped):
                used.add(word)

        return {
            "defined": sorted(list(defined)),
            "used": sorted(list(used)),
            "exported": sorted(list(exported)),
            "aliases": imported_aliases,
            "class_bases": class_bases,
            "imports_list": imports_list,
            "class_definitions": class_defs,
            "function_definitions": func_defs,
            "class_methods": class_methods,
            "assignments": assignments,
            "control_flow_count": control_flow_count
        }


class ASTJSParserAdapter(ISourceParserPort):
    """Adapter that implements ISourceParserPort for Javascript/Typescript using Regex analysis."""

    def __init__(self) -> None:
        self._cache: dict[FilePath, dict] = {}
        try:
            from auto_linter import auto_linter_rust
            self._native = auto_linter_rust.NativeASTJSParser()
        except ImportError:
            self._native = None

    def _get_data(self, path: FilePath) -> dict:
        if path in self._cache:
            return self._cache[path]
        with open(str(path), "r", encoding="utf-8") as f:
            content = f.read()
        data = RegexJSParser.parse(content, path)
        self._cache[path] = data
        return data

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
            data = self._get_data(path)
            return ImportInfoList(values=data["imports_list"])
        except Exception as e:
            return SourceParserError(
                path=path,
                message=ErrorMessage(value=f"Failed to parse JS imports: {e}")
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
            data = self._get_data(path)
            return ResponseData(
                value={
                    "defined": data["defined"],
                    "used": data["used"],
                    "exported": data["exported"],
                    "aliases": data["aliases"],
                    "class_bases": data["class_bases"]
                }
            )
        except Exception as e:
            return SourceParserError(
                path=path,
                message=ErrorMessage(value=f"Failed to extract JS symbols: {e}")
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

        path_str = str(path)
        is_barrel = "index.ts" in path_str or "index.js" in path_str
        if not is_barrel:
            return SuccessStatus(value=BooleanVO(value=False))
        data = self._get_data(path)
        return SuccessStatus(value=BooleanVO(value=bool(data["exported"])))

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

        violations = []
        prim_words = [p.value if hasattr(p, "value") else str(p) for p in primitive_types.values]
        with open(str(path), "r", encoding="utf-8") as f:
            content = f.read()
        lines = content.splitlines()
        for idx, line in enumerate(lines, 1):
            stripped = line.strip()
            if stripped.startswith("import ") or stripped.startswith("//") or stripped.startswith("/*"):
                continue
            for prim in prim_words:
                if re.search(r'\b(?:class|constructor)\b', stripped):
                    pattern = rf'\b{prim}\b'
                    match = re.search(pattern, stripped)
                    if match:
                        violations.append(PrimitiveViolation(
                            line=LineNumber(value=idx),
                            column=ColumnNumber(value=match.start() + 1),
                            type_name=PrimitiveTypeName(value=prim)
                        ))
        return PrimitiveViolationList(values=violations)

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

        data = self._get_data(path)
        symbols = self.get_raw_symbols(path).value
        aliases = symbols.get("aliases", {})
        used = set(symbols.get("used", []))
        exported = set(symbols.get("exported", []))

        unused_infos = []
        for imp_info in data["imports_list"]:
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
            data = self._get_data(path)
            return MetadataVO(value={"classes": data["class_definitions"]})
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

        data = self._get_data(path)
        return MetadataVO(value={"functions": data["function_definitions"]})

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

        sym_val = symbol.value if hasattr(symbol, "value") else str(symbol)
        data = self._get_data(path)
        return SuccessStatus(value=BooleanVO(value=sym_val in data["exported"]))

    def get_class_methods(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_class_methods(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        data = self._get_data(path)
        return MetadataVO(value=data["class_methods"])

    def get_class_bases_map(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_class_bases_map(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        data = self._get_data(path)
        return MetadataVO(value=data["class_bases"])

    def get_assignment_targets(self, path: FilePath) -> MetadataVO:
        if self._native is not None:
            try:
                res = self._native.get_assignment_targets(str(path))
                return MetadataVO(value=res)
            except Exception:
                pass

        data = self._get_data(path)
        return MetadataVO(value={"assignments": data["assignments"]})

    def get_control_flow_count(self, path: FilePath) -> Count:
        if self._native is not None:
            try:
                val = self._native.get_control_flow_count(str(path))
                return Count(value=val)
            except Exception:
                pass

        data = self._get_data(path)
        return Count(value=data["control_flow_count"])

    def is_barrel_file(self, path: FilePath) -> bool:
        if self._native is not None:
            try:
                return self._native.is_barrel_file(str(path))
            except Exception:
                pass

        path_str = str(path).replace("\\", "/")
        return any(path_str.endswith(barrel) for barrel in ["/index.ts", "/index.js", "/index.tsx", "/index.jsx"])

    def get_stem(self, path: FilePath) -> SymbolName:
        if self._native is not None:
            try:
                val = self._native.get_stem(str(path))
                return SymbolName(value=val)
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        stem = basename
        for ext in [".tsx", ".ts", ".jsx", ".js"]:
            if basename.endswith(ext):
                stem = basename[:-len(ext)]
                break
        return SymbolName(value=stem)

    def is_entry_point(self, path: FilePath) -> bool:
        if self._native is not None:
            try:
                return self._native.is_entry_point(str(path))
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        return basename in ["index.ts", "index.js", "index.tsx", "index.jsx", "main.ts", "main.js"]

    def get_supported_extensions(self) -> PatternList:
        return PatternList(values=[".ts", ".tsx", ".js", ".jsx"])
