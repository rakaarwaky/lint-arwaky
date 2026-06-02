"""ast_rust_scanner — Specialized parser for Rust source code using regex analysis."""

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


class RegexRustParser:
    """A highly robust and lightweight regex-based symbol collector for Rust."""

    @staticmethod
    def _parse_impl(
        stripped: str,
        impl_pattern: re.Pattern,
        class_bases: dict,
    ) -> tuple[str | None, int]:
        impl_match = impl_pattern.match(stripped)
        if impl_match:
            trait_name = impl_match.group(1)
            struct_name = impl_match.group(2)
            if trait_name:
                clean_trait = trait_name.split("::")[-1]
                if struct_name not in class_bases:
                    class_bases[struct_name] = []
                class_bases[struct_name].append(clean_trait)
            return struct_name, 0
        return None, 0

    @staticmethod
    def _parse_use_imports(
        stripped: str,
        idx: int,
        use_pattern: re.Pattern,
        imported_aliases: dict,
        imports_list: list,
    ) -> None:
        use_match = use_pattern.match(stripped)
        if use_match:
            raw_path = use_match.group(1).strip()
            clean_path = raw_path
            for prefix in ["crate::", "self::", "super::"]:
                if clean_path.startswith(prefix):
                    clean_path = clean_path[len(prefix):]

            # Expand groupings: a::b::{C, D}
            if "::{" in clean_path:
                parts = clean_path.split("::{", 1)
                prefix = parts[0]
                sub_parts = parts[1].rstrip("}").split(",")
                expanded = [f"{prefix}::{p.strip()}" for p in sub_parts if p.strip()]
            else:
                expanded = [clean_path]

            for item in expanded:
                dotted = item.replace("::", ".")
                alias = dotted.split(".")[-1]
                if alias != "*":
                    imported_aliases[alias] = dotted
                    imports_list.append(ImportInfo(
                        line=LineNumber(value=idx),
                        module=ModuleName(value=dotted)
                    ))

    @staticmethod
    def _parse_struct_enum_trait(
        line: str,
        stripped: str,
        idx: int,
        lines: list[str],
        struct_pattern: re.Pattern,
        class_defs: list,
        defined: set,
    ) -> None:
        struct_match = struct_pattern.match(stripped)
        if struct_match:
            name = struct_match.group(1)
            defined.add(name)
            is_dead = ";" in stripped or "{}" in stripped
            if not is_dead and idx < len(lines):
                next_line = lines[idx].strip()
                if next_line == "}" or next_line == ";":
                    is_dead = True

            class_defs.append({
                "name": name,
                "line": idx,
                "column": line.find(name),
                "is_dead": is_dead,
                "bases": [],
                "resolved_bases": []
            })

    @staticmethod
    def _parse_fn_method(
        line: str,
        stripped: str,
        idx: int,
        fn_pattern: re.Pattern,
        current_impl: str | None,
        class_methods: dict,
        func_defs: list,
        defined: set,
    ) -> None:
        fn_match = fn_pattern.match(stripped)
        if fn_match:
            name = fn_match.group(1)
            defined.add(name)
            if current_impl:
                if current_impl not in class_methods:
                    class_methods[current_impl] = []
                class_methods[current_impl].append(name)
            else:
                func_defs.append({
                    "name": name,
                    "line": idx,
                    "column": line.find(name)
                })

    @staticmethod
    def _parse_assignments(
        line: str,
        stripped: str,
        idx: int,
        assignments: list,
    ) -> None:
        if stripped.startswith("let "):
            assign_match = re.match(r'^let\s+(?:mut\s+)?([a-zA-Z0-9_]+)', stripped)
            if assign_match:
                assignments.append({
                    "name": assign_match.group(1),
                    "type": "Assign",
                    "line": idx,
                    "column": line.find(assign_match.group(1))
                })

    @staticmethod
    def _parse_exported(
        lines: list[str],
        exported: set,
    ) -> None:
        for idx, line in enumerate(lines, 1):
            stripped = line.strip()
            if stripped.startswith("pub "):
                # pub struct/enum/trait/fn/const X
                match = re.search(r'\b(?:struct|enum|trait|fn|const)\s+([a-zA-Z0-9_]+)', stripped)
                if match:
                    exported.add(match.group(1))
                # pub mod X
                mod_match = re.search(r'\bmod\s+([a-zA-Z0-9_]+)', stripped)
                if mod_match:
                    exported.add(mod_match.group(1))
                # pub use X or pub use path::to::X
                use_match = re.search(r'\buse\s+(?:.*::)?([a-zA-Z0-9_]+)\s*(?:::\{|;|$)', stripped)
                if use_match:
                    exported.add(use_match.group(1))
                # pub use path::{A, B, C} — capture each name in the group
                use_group = re.search(r'\buse\s+.*::\{([^}]+)\}', stripped)
                if use_group:
                    for name in use_group.group(1).split(','):
                        clean = name.strip()
                        if clean:
                            exported.add(clean)

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

        # Regex patterns
        use_pattern = re.compile(r'^(?:pub\s+)?use\s+([^;]+);')
        struct_pattern = re.compile(r'^(?:pub\s+)?(?:pub\s*\([^)]*\)\s+)?(?:struct|enum|trait)\s+([a-zA-Z0-9_]+)')
        fn_pattern = re.compile(r'^(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)')
        impl_pattern = re.compile(r'^impl\s+(?:([a-zA-Z0-9_:]+)\s+for\s+)?([a-zA-Z0-9_]+)')
        cf_pattern = re.compile(r'\b(if|for|while|match|loop)\b')

        current_impl = None
        brace_count = 0

        for idx, line in enumerate(lines, 1):
            stripped = line.strip()
            if not stripped or stripped.startswith("//") or stripped.startswith("/*") or stripped.startswith("*"):
                continue

            open_braces = stripped.count("{")
            close_braces = stripped.count("}")

            # Check for impl block
            cimpl, _ = RegexRustParser._parse_impl(
                stripped, impl_pattern, class_bases
            )
            if cimpl:
                current_impl = cimpl
                brace_count = 0

            brace_count += open_braces - close_braces
            if brace_count < 0:
                brace_count = 0
                current_impl = None

            # 1. Imports
            RegexRustParser._parse_use_imports(
                stripped, idx, use_pattern, imported_aliases, imports_list
            )

            # 2. Struct, Enum, Trait Definitions
            RegexRustParser._parse_struct_enum_trait(
                line, stripped, idx, lines, struct_pattern, class_defs, defined
            )

            # 3. Functions / Methods
            RegexRustParser._parse_fn_method(
                line, stripped, idx, fn_pattern, current_impl, class_methods, func_defs, defined
            )

            # 4. Assignments
            RegexRustParser._parse_assignments(
                line, stripped, idx, assignments
            )

            # 5. Control Flow
            cf_matches = cf_pattern.findall(stripped)
            control_flow_count += len(cf_matches)

            # 6. Used symbols
            for word in re.findall(r'\b[a-zA-Z_][a-zA-Z0-9_]*\b', stripped):
                used.add(word)

        # Resolve bases for each class definition
        for cdef in class_defs:
            cname = cdef["name"]
            if cname in class_bases:
                cdef["bases"] = class_bases[cname]
                cdef["resolved_bases"] = [imported_aliases.get(b, b) for b in class_bases[cname]]

        # Exported: treat pub items as exported
        RegexRustParser._parse_exported(lines, exported)

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


class ASTRustParserAdapter(ISourceParserPort):
    """Adapter that implements ISourceParserPort for Rust using dynamic delegation to PyO3 native scanner when available."""

    def __init__(self) -> None:
        self._cache: dict[FilePath, dict] = {}
        try:
            import auto_linter_rust
            self._native = auto_linter_rust.NativeASTRustParser()
        except ImportError:
            self._native = None

    def _get_data(self, path: FilePath) -> dict:
        if path in self._cache:
            return self._cache[path]
        with open(str(path), "r", encoding="utf-8") as f:
            content = f.read()
        data = RegexRustParser.parse(content, path)
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
                message=ErrorMessage(value=f"Failed to parse Rust imports: {e}")
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
                message=ErrorMessage(value=f"Failed to extract Rust symbols: {e}")
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
        if not path_str.endswith("mod.rs"):
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
            if stripped.startswith("use ") or stripped.startswith("//") or stripped.startswith("/*"):
                continue
            for prim in prim_words:
                if re.search(r'\b(?:struct|enum|trait|fn|impl|pub)\b', stripped):
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

        return str(path).replace("\\", "/").endswith("/mod.rs") or str(path).replace("\\", "/").endswith("/lib.rs")

    def get_stem(self, path: FilePath) -> SymbolName:
        if self._native is not None:
            try:
                val = self._native.get_stem(str(path))
                return SymbolName(value=val)
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        return SymbolName(value=basename.replace(".rs", ""))

    def is_entry_point(self, path: FilePath) -> bool:
        if self._native is not None:
            try:
                return self._native.is_entry_point(str(path))
            except Exception:
                pass

        basename = str(path).replace("\\", "/").split("/")[-1]
        return basename in ["main.rs", "lib.rs", "mod.rs"]

    def get_supported_extensions(self) -> PatternList:
        return PatternList(values=[".rs"])
