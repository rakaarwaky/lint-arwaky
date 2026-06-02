"""mandatory_inheritance_checker — AES027: mandatory contract inheritance for agent/capabilities/infrastructure."""

from __future__ import annotations

import logging

from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Severity,
    Identity,
)
# contract imports removed — not used in this module

logger = logging.getLogger(__name__)

# Map layer name → contract suffix to check inheritance against
LAYER_CONTRACT_SUFFIX = {
    "infrastructure": "_port",
    "capabilities": "_protocol",
    "agent": "_aggregate",
}


class MandatoryInheritanceChecker:
    """Check that files in agent/capabilities/infrastructure that import
    at least one contract symbol also have a class inheriting from it.

    Surfaces and contract files import contracts for composition/call —
    they are NOT checked by this rule.
    """

    @property
    def rule_name(self) -> Identity:
        return Identity(value="mandatory_inheritance")

    def check_mandatory_inheritance(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Verify 1:1 mapping: if a file in agent/capabilities/infrastructure
        imports a contract, it must have a class inheriting from at least one of them.
        """
        for f in files:
            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo:
                continue

            layer_str = str(layer_vo).split("(")[0]
            if layer_str not in LAYER_CONTRACT_SUFFIX:
                continue  # only agent/capabilities/infrastructure

            # __init__.py barrel files don't need to inherit
            if f.value.endswith("__init__.py"):
                # But barrel files must have at least one class inheriting
                # from a contract (imported and re-exported)
                continue

            # Get imports
            symbols = analyzer.parser.get_raw_symbols(f)
            imports = symbols.value.get("imports", {})
            contract_imports = self._filter_contract_imports(imports)
            if not contract_imports:
                continue  # no contract imported → skip, surface-like

            # Get class definitions
            metadata = analyzer.parser.get_class_definitions(f)
            classes = metadata.value.get("classes", [])
            resolved_bases = set()
            for cls in classes:
                if not isinstance(cls, dict):
                    continue
                for base in cls.get("resolved_bases") or cls.get("bases") or []:
                    resolved_bases.add(str(base))

            # Check: does any resolved base name match a contract import?
            inherited = False
            for contract_name in contract_imports:
                if contract_name in resolved_bases:
                    inherited = True
                    break

            if not inherited:
                imported_list = ", ".join(sorted(contract_imports))
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES027"),
                        message=LintMessage(
                            value=(
                                f"AES027 MANDATORY_INHERITANCE_VIOLATION: File imports contracts "
                                f"({imported_list}) but no class inherits from any of them. "
                                f"Layer '{layer_str}' must implement its contract via inheritance. "
                                f"FIX: Make at least one class in this file inherit from one of the imported contracts."
                            )
                        ),
                        source=AdapterName(value="architecture"),
                        severity=Severity.CRITICAL,
                    )
                )

    def _filter_contract_imports(self, imports: dict) -> set[str]:
        """Filter import names that refer to contract layer symbols
        (suffixes: _port, _protocol, _aggregate, or names starting with 'I').

        Returns set of symbol names that are contract imports.
        """
        contract_suffixes = ("_port", "_protocol", "_aggregate")
        result = set()
        for imp_name, imp_info in imports.items():
            # Check imported symbol name
            if imp_name.startswith("I") and any(
                imp_name.endswith(s) for s in contract_suffixes
            ):
                result.add(imp_name)
                continue
            # Check module path
            module = str(imp_info.get("module", "")) if isinstance(imp_info, dict) else str(imp_info)
            if "contract" in module.lower() and any(
                imp_name.endswith(s) for s in contract_suffixes
            ):
                result.add(imp_name)
            elif any(imp_name.endswith(s) for s in contract_suffixes):
                result.add(imp_name)
            # Interface names (I-prefixed from contract) — they end up as resolved names
            elif imp_name.startswith("I"):
                result.add(imp_name)

        return result
