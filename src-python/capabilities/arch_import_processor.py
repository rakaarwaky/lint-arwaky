"""arch_import_util — Logic for evaluating architectural import rules."""

import re
import os
from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    LayerDefinition,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    Severity,
    AdapterName,
    ContentString,
    LayerNameVO,
    LAYER_CONTRACT,
    SymbolName,
)
from ..contract import IArchImportProcessorProtocol


class ArchImportProcessor(IArchImportProcessorProtocol):
    """Helper for complex architectural import analysis."""

    def process_file_imports(
        self, analyzer, file_path: FilePath, root_dir: FilePath, results: LintResultList
    ) -> None:
        layer_vo = analyzer._detect_layer(file_path, root_dir)
        if not layer_vo:
            return
        definition = analyzer.layer_map[layer_vo]

        # Respect exceptions
        basename = os.path.basename(str(file_path))
        if definition.exceptions.values and basename in definition.exceptions.values:
            return

        if (
            not definition.forbidden_import.values
            and not definition.allowed_import.values
        ):
            return
        imports = analyzer.parser.extract_imports(file_path)
        for imp in imports:
            self._evaluate_import(
                analyzer, imp, file_path, layer_vo, definition, results
            )

    def _evaluate_import(
        self,
        analyzer,
        imp,
        file_path: FilePath,
        file_layer: LayerNameVO,
        definition: LayerDefinition,
        results: LintResultList,
    ) -> None:
        target_layer = analyzer._detect_module_layer(imp.module)
        if not target_layer:
            return

        if definition.allowed_import.values:
            is_same = self._is_same_domain_layer(target_layer, file_layer)
            allowed = any(
                self._is_layer_match(target_layer, p)
                for p in definition.allowed_import.values
            )
            if not is_same and not allowed:
                self._add_forbidden_violation(
                    results,
                    file_path,
                    imp,
                    file_layer,
                    target_layer,
                    definition.forbidden_import_violation_message
                    or ContentString(value="Forbidden layer import detected."),
                )
                return

        if any(
            self._is_layer_match(target_layer, p)
            for p in definition.forbidden_import.values
        ):
            self._add_forbidden_violation(
                results,
                file_path,
                imp,
                file_layer,
                target_layer,
                definition.forbidden_import_violation_message
                or ContentString(value="Forbidden layer import detected."),
            )

    def _is_layer_match(self, layer_vo: LayerNameVO, pattern: str):
        layer_name = str(layer_vo)
        if layer_name == pattern:
            return True
        if "(" in layer_name:
            base = layer_name.split("(")[0]
            if pattern == base:
                return True
        if "(" in pattern and "(" in layer_name:
            p_base, p_subs_raw = pattern.split("(", 1)
            l_base, l_sub_raw = layer_name.split("(", 1)
            if p_base != l_base:
                return False
            p_subs = [
                s.strip() for s in p_subs_raw.rstrip(")").replace(",", "|").split("|")
            ]
            l_sub = l_sub_raw.rstrip(")")
            return l_sub in p_subs
        return False

    def _is_same_domain_layer(self, layer_a: LayerNameVO, layer_b: LayerNameVO):
        if layer_a == layer_b:
            return True
        return str(layer_a).split("(")[0] == str(layer_b).split("(")[0]

    def _add_forbidden_violation(
        self,
        results: LintResultList,
        file_path: FilePath,
        imp,
        layer_name: LayerNameVO,
        target_layer_name: LayerNameVO,
        message: ContentString,
    ) -> None:
        results.append(
            LintResult(
                file=file_path,
                line=imp.line,
                column=getattr(imp, "column", ColumnNumber(value=0)),
                code=ErrorCode(code="AES001"),
                message=LintMessage(value=str(message)),
                source=AdapterName(value="architecture"),
                severity=Severity.CRITICAL,
            )
        )

    def validate_imports_present(
        self,
        analyzer,
        file_path: FilePath,
        root_dir: FilePath,
        required_layers,
        results: LintResultList,
        message_template: ContentString,
        layer_vo: LayerNameVO,
        layers_display,
    ) -> None:
        symbols_data = analyzer.parser.get_raw_symbols(file_path).value
        imported_aliases = symbols_data.get("aliases", {})
        used_symbols = set(symbols_data.get("used", []))
        class_bases = symbols_data.get("class_bases", {})
        real_usages = {n for n in used_symbols if not self._is_bypass_marker(n)}
        found_layers = set()

        for req_layer in required_layers.values:
            if req_layer.startswith(str(LAYER_CONTRACT)):
                satisfied = self._check_contract_layer(
                    analyzer,
                    root_dir,
                    req_layer,
                    imported_aliases,
                    real_usages,
                    class_bases,
                    file_path,
                    layer_vo,
                    results,
                )
            else:
                satisfied = self._check_general_layer(
                    analyzer, req_layer, imported_aliases, real_usages
                )
            if satisfied:
                found_layers.add(req_layer)

        missing = [r for r in required_layers.values if r not in found_layers]
        if missing:
            self._report_missing_imports(
                results, file_path, layer_vo, layers_display, missing, message_template
            )

    def _report_missing_imports(
        self,
        results: LintResultList,
        file_path: FilePath,
        layer_vo: LayerNameVO,
        layers_display,
        missing,
        template: ContentString,
    ):
        contract_missing = any(m.startswith(str(LAYER_CONTRACT)) for m in missing)
        suffix = (
            f" [STRICT] '{LAYER_CONTRACT}' imports must be from a specific _port, _protocol, or _aggregate module and used as a base class. Bare 'import {LAYER_CONTRACT} as {LAYER_CONTRACT}' or bypass markers are forbidden."
            if contract_missing
            else ""
        )
        message = (
            str(template).format(
                layer=str(layer_vo), layers=layers_display, missing=missing
            )
            + suffix
        )
        results.append(
            LintResult(
                file=file_path,
                line=LineNumber(value=0),
                column=ColumnNumber(value=0),
                code=ErrorCode(code="AES002"),
                message=LintMessage(value=message),
                source=AdapterName(value="architecture"),
                severity=Severity.HIGH,
            )
        )

    def _is_bypass_marker(self, name: str | SymbolName):
        # Original patterns: _arch_*_marker and standalone _
        raw_name = name.value if hasattr(name, "value") else str(name)
        if (
            raw_name.startswith("_arch_") and raw_name.endswith("_marker")
        ) or raw_name == "_":
            return True
        # Extended: internal names (starting with _) containing bypass-related keywords
        if raw_name.startswith("_"):
            lower = raw_name.lower()
            bypass_keywords = [
                "marker",
                "stub",
                "compliance",
                "dummy",
                "fake",
                "bypass",
                "placeholder",
                "sentinel",
                "shim",
            ]
            if any(kw in lower for kw in bypass_keywords):
                return True
        return False

    def _check_import_stem_matches(self, aliases, imported_aliases, class_bases, file_path: FilePath):
        """Determine which contract barrel aliases are used as base classes (stem matching)."""
        all_bases = {b for bases in class_bases.values() for b in bases}
        used_as_base = [
            a for a in aliases 
            if a in all_bases or imported_aliases.get(a) in all_bases
            or any(b.startswith(f"{a}.") for b in all_bases)
        ]
        is_utility = any(
            file_path.value.endswith(s) for s in ["_util.py", "_visitor.py"]
        )
        if not used_as_base:
            if not class_bases or is_utility:
                return aliases
            return []
        return used_as_base

    def _check_contract_layer(
        self,
        analyzer,
        root_dir,
        req_layer_str,
        imported_aliases,
        real_usages,
        class_bases,
        file_path,
        layer_vo,
        results,
    ):
        aliases = self._get_contract_barrel_aliases(
            imported_aliases, real_usages, file_path, layer_vo, results
        )
        if not aliases:
            return False
        used_as_base = self._check_import_stem_matches(
            aliases, imported_aliases, class_bases, file_path
        )
        if not used_as_base:
            return False

        match = re.match(r"contract\((.+)\)", req_layer_str)
        if not match:
            return True

        contract_init_path = self._find_contract_barrel(analyzer, root_dir, file_path)
        if not contract_init_path:
            return False
        contract_symbols = analyzer.parser.get_raw_symbols(contract_init_path).value
        barrel_map = contract_symbols.get("aliases", {})
        return self._validate_contract_suffix(
            used_as_base, barrel_map, match.group(1), file_path, layer_vo, results
        )

    def _find_contract_barrel(self, analyzer, root_dir, file_path) -> FilePath | None:
        barrel_names = ["__init__.py", "mod.rs", "index.ts", "index.js"]
        for subdir in ["", "src", "lib", "app"]:
            for name in barrel_names:
                candidate = (
                    os.path.join(root_dir.value, subdir, "contract", name)
                    if subdir
                    else os.path.join(root_dir.value, "contract", name)
                )
                if os.path.exists(candidate):
                    return FilePath(value=candidate)
        parts = str(file_path).replace("\\", "/").split("/")
        if len(parts) >= 2:
            src_dir = "/".join(parts[:-2])
            for name in barrel_names:
                candidate = os.path.join(src_dir, "contract", name)
                if os.path.exists(candidate):
                    return FilePath(value=candidate)
        return None

    def _get_contract_barrel_aliases(
        self, imported_aliases, real_usages, file_path, layer_vo, results
    ):
        aliases = []
        for alias, fullname in imported_aliases.items():
            # Support both absolute (contract.X) and relative (..contract.X) imports
            parts = fullname.split(".")
            if str(LAYER_CONTRACT) not in parts:
                continue
            is_barrel = len(parts) >= 2 and parts[-2] == str(LAYER_CONTRACT)
            if is_barrel:
                if alias != str(LAYER_CONTRACT):
                    aliases.append(alias)
            elif alias in real_usages:
                results.append(
                    LintResult(
                        file=file_path,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES007"),
                        message=LintMessage(
                            value=f"{str(LAYER_CONTRACT).capitalize()} import must be from barrel."
                        ),
                        source=AdapterName(value="architecture"),
                        severity=Severity.MEDIUM,
                    )
                )
        return aliases

    def _validate_contract_suffix(
        self, aliases, barrel_map, pattern, file_path, layer_vo, results
    ):
        for alias in aliases:
            origin_fullname = barrel_map.get(alias)
            if not origin_fullname:
                continue
            origin_module = origin_fullname.rsplit(".", 1)[0]
            if re.search(rf"_({pattern})$", origin_module):
                return True
            results.append(
                LintResult(
                    file=file_path,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES008"),
                    message=LintMessage(value="Contract suffix mismatch detected."),
                    source=AdapterName(value="architecture"),
                    severity=Severity.HIGH,
                )
            )
        return False

    def _check_general_layer(self, analyzer, req_layer, imported_aliases, real_usages):
        for alias, fullname in imported_aliases.items():
            detected = analyzer._detect_module_layer(fullname)
            if (detected and self._is_layer_match(detected, req_layer)) or (
                req_layer in fullname.split(".") and alias in real_usages
            ):
                if alias in real_usages:
                    return True
        return False
