"""arch_role_checker — Architectural role checks (agent and surface roles)."""

import ast
import os
import logging
from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    Location,
    LocationList,
    Severity,
    AdapterName,
    SymbolName,
    LayerNameVO,
    LAYER_AGENT,
    LAYER_SURFACES,
    LAYER_CONTRACT,
    LAYER_TAXONOMY,
    LAYER_INFRASTRUCTURE,
    CORE_LAYER_NAMES,
)
from ..contract import IRoleCheckerProtocol

logger = logging.getLogger(__name__)


class ArchRoleChecker(IRoleCheckerProtocol):
    """Handles role-specific architectural checks (e.g., stateless execution, coordination)."""

    def check_agent_roles(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check agent-specific role mandates."""
        for f in files:
            self._check_agent_role_on_file(f, analyzer, root_dir, results)

    def _check_agent_role_on_file(
        self, f: FilePath, analyzer, root_dir: FilePath, results: LintResultList
    ) -> None:
        """Check a single file against agent role mandates."""
        layer_vo = analyzer._detect_layer(f, root_dir)
        if not layer_vo or not (
            layer_vo == LAYER_AGENT or str(layer_vo).startswith(f"{str(LAYER_AGENT)}(")
        ):
            return

        definition = analyzer.layer_map.get(layer_vo)
        if not definition:
            return

        # Omit files in the definition's exceptions list
        basename = os.path.basename(str(f))
        if basename in definition.exceptions.values:
            return

        self._apply_agent_role_checks(f, definition, analyzer, results)

    def _apply_agent_role_checks(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        """Apply all role-specific checks based on definition flags."""
        if definition.stateless_execution:
            self._check_stateless_execution(f, definition, analyzer, results)

        if definition.high_level_policy_only:
            self._check_high_level_policy_only(f, definition, analyzer, results)

        if definition.coordinates_multiple_orchestrators:
            self._check_coordinates_multiple_orchestrators(
                f, definition, analyzer, results
            )

        if definition.no_domain_logic:
            self._check_no_domain_logic(
                f, definition, analyzer, results, ErrorCode(code="AES021")
            )

        if definition.must_implement_service_container_aggregate:
            self._check_must_implement_contract_lazy(f, definition, analyzer, results)

        if definition.lazy_eager_initialization_only:
            self._check_lazy_eager_init_only(f, definition, analyzer, results)

        if definition.forbid_any_type:
            self._check_forbid_any_type(f, definition, analyzer, results)

    def _check_must_implement_contract_lazy(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        """Lazy wrapper: must implement ServiceContainerAggregate."""
        contract_name = SymbolName(value="ServiceContainerAggregate")
        violation_msg = LintMessage(
            value=str(
                definition.must_implement_service_container_aggregate_violation_message or ""
            )
        )
        self._check_must_implement_contract(
            f, contract_name, violation_msg, analyzer, results, ErrorCode(code="AES021")
        )

    def check_surface_roles(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check surface-specific role mandates."""
        for f in files:
            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo or not (
                layer_vo == LAYER_SURFACES
                or str(layer_vo).startswith(f"{str(LAYER_SURFACES)}(")
            ):
                continue

            definition = analyzer.layer_map.get(layer_vo)
            if not definition:
                continue

            # Surfaces should not have domain logic
            if definition.no_domain_logic and bool(definition.no_domain_logic):
                self._check_no_domain_logic(
                    f, definition, analyzer, results, ErrorCode(code="AES022")
                )

            # Surface strict dependency: only allowed to import from contract
            self._check_forbidden_mandatory_imports(f, definition, analyzer, results)
            self._check_agent_mandatory_imports(f, definition, analyzer, results)

    def _check_forbidden_mandatory_imports(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        """AES023 — Check that surface files do not import from forbidden layers (infrastructure, capabilities, etc.)."""
        # Omit files listed in exceptions
        basename = os.path.basename(str(f))
        if definition.exceptions.values and basename in definition.exceptions.values:
            return

        imports = analyzer.parser.extract_imports(f)
        for imp in imports.values:
            module_str = str(imp.module.value)
            if self._is_builtin_or_stdlib_import(module_str):
                continue

            target_layer = analyzer._detect_module_layer(imp.module.value)
            if target_layer and target_layer != LAYER_CONTRACT:
                if self._is_smart_surface_allowed_layer(target_layer):
                    continue

                self._report_surface_dependency_violation(f, imp, target_layer, results)

    def _is_builtin_or_stdlib_import(self, module_str):
        """Check whether an import string refers to a Python built-in or standard library module (heuristic)."""
        # Built-ins and standard libs have no dot and aren't known src modules
        known_src_modules = CORE_LAYER_NAMES
        return "." not in module_str and module_str not in known_src_modules

    def _is_smart_surface_allowed_layer(self, layer_vo: LayerNameVO):
        """Smart surfaces are allowed to import from taxonomy, agent, surfaces and their sub-layers."""
        layer_str = str(layer_vo)
        allowed_bases = {str(LAYER_TAXONOMY), str(LAYER_AGENT), str(LAYER_SURFACES)}
        if layer_str in allowed_bases:
            return True
        return any(layer_str.startswith(f"{base}(") for base in allowed_bases)

    def _report_surface_dependency_violation(
        self, f: FilePath, imp, target_layer, results: LintResultList
    ) -> None:
        """Append an AES023 surface dependency violation LintResult."""
        results.append(
            LintResult(
                file=f,
                line=imp.line,
                column=ColumnNumber(value=0),
                code=ErrorCode(code="AES023"),
                message=LintMessage(
                    value=f"SURFACE DEPENDENCY VIOLATION: Surface layer is only allowed to import from '{LAYER_CONTRACT}' and '{LAYER_TAXONOMY}'. Found import from '{target_layer}'."
                ),
                source=AdapterName(value="architecture"),
                severity=Severity.HIGH,
            )
        )

    def _check_agent_mandatory_imports(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        """Check that agent-related files import mandatory dependencies (e.g., from contract)."""
        # Omit files listed in exceptions
        basename = os.path.basename(str(f))
        if definition.exceptions.values and basename in definition.exceptions.values:
            return

        imports = analyzer.parser.extract_imports(f)
        has_contract_import = False
        has_agent_import = False
        for imp in imports.values:
            target_layer = analyzer._detect_module_layer(imp.module.value)
            if target_layer:
                if target_layer == LAYER_CONTRACT:
                    has_contract_import = True
                elif target_layer == LAYER_AGENT or str(target_layer).startswith(f"{str(LAYER_AGENT)}("):
                    has_agent_import = True

        # If this surface file has agent-like characteristics (imports agent), it must import from contract
        if has_agent_import and not has_contract_import:
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES023"),
                    message=LintMessage(
                        value=f"AGENT MANDATORY IMPORT: Agent-related layer must import from '{LAYER_CONTRACT}'."
                    ),
                    source=AdapterName(value="architecture"),
                    severity=Severity.MEDIUM,
                )
            )

    def _check_stateless_execution(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        metadata_assigns = analyzer.parser.get_assignment_targets(f)
        assignments = metadata_assigns.value.get("assignments", [])
        metadata_methods = analyzer.parser.get_class_methods(f)
        methods = metadata_methods.value  # get_class_methods returns the dict directly

        for assign in assignments:
            line_vo = LineNumber(value=int(assign.get("line", 0)))
            method_name = self._find_method_name_for_line(methods, line_vo)
            if method_name and str(method_name) != "__init__":
                default_msg = LintMessage(
                    value="""Non-stateless behavior detected: state assignment found outside
                            __init__. """
                )
                violation_msg = definition.stateless_execution_violation_message
                results.append(
                    LintResult(
                        file=f,
                        line=line_vo,
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES021"),
                        message=violation_msg or default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )

    def _check_high_level_policy_only(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        # Check if it imports infrastructure directly (low-level)
        imports = analyzer.parser.extract_imports(f)
        for imp in imports.values:
            if str(LAYER_INFRASTRUCTURE) in str(imp.module):
                default_msg = LintMessage(
                    value="Low-level implementation details found (infrastructure import)."
                )
                violation_msg = definition.high_level_policy_only_violation_message
                results.append(
                    LintResult(
                        file=f,
                        line=imp.line,
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES021"),
                        message=violation_msg or default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )

    def _check_coordinates_multiple_orchestrators(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        metadata = analyzer.parser.get_class_methods(f)
        for _, class_methods in metadata.value.items():
            init_method = self._find_init_method(class_methods)
            if not init_method:
                continue

            if self._count_orchestrator_args(init_method) < 2:
                self._report_multi_orchestrator_violation(
                    f, init_method, definition, results
                )

    def _find_init_method(self, class_methods: list) -> dict | None:
        """Find the __init__ method in a list of class methods."""
        for m in class_methods:
            if isinstance(m, dict) and m.get("name") == "__init__":
                return m
            if isinstance(m, str) and m == "__init__":
                return {"name": "__init__", "line": 0, "args": []}
        return None

    def _count_orchestrator_args(self, method: dict) -> int:
        """Count arguments containing 'orchestrator' in their name."""
        count = 0
        for arg in method.get("args", []):
            if "orchestrator" in str(arg).lower():
                count += 1
        return count

    def _report_multi_orchestrator_violation(
        self, f: FilePath, init_method: dict, definition, results: LintResultList
    ) -> None:
        """Report a violation for coordinator not managing multiple orchestrators."""
        default_msg = LintMessage(
            value="Coordinator must manage multiple orchestrators."
        )
        violation_msg = definition.coordinates_multiple_orchestrators_violation_message
        results.append(
            LintResult(
                file=f,
                line=LineNumber(value=int(init_method.get("line", 0))),
                column=ColumnNumber(value=0),
                code=ErrorCode(code="AES021"),
                message=violation_msg or default_msg,
                source=AdapterName(value="architecture"),
                severity=Severity.MEDIUM,
            )
        )

    def _check_no_domain_logic(
        self,
        f: FilePath,
        definition,
        analyzer,
        results: LintResultList,
        code: ErrorCode,
    ) -> None:
        # Heuristic: control flow density.
        control_flow_count = analyzer.parser.get_control_flow_count(f)
        if int(control_flow_count) > 3:  # Arbitrary threshold for "logic"
            default_msg = LintMessage(
                value="Complex domain logic detected in a passive layer/role."
            )
            violation_msg = getattr(
                definition, "no_domain_logic_violation_message", None
            )
            if not violation_msg:
                violation_msg = getattr(
                    definition, "no_decision_logic_violation_message", None
                )

            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=code,
                    message=violation_msg or default_msg,
                    source=AdapterName(value="architecture"),
                    severity=Severity.HIGH,
                )
            )

    def _check_lazy_eager_init_only(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        metadata = analyzer.parser.get_class_methods(f)
        methods = metadata.value
        for _, class_methods in methods.items():
            init_method = None
            for m in class_methods:
                if isinstance(m, dict) and m.get("name") == "__init__":
                    init_method = m
                    break
                elif isinstance(m, str) and m == "__init__":
                    init_method = {"name": "__init__", "line": 0}
                    break

            if init_method:
                control_flow_count = analyzer.parser.get_control_flow_count(f)
                if int(control_flow_count) > 2:  # Very strict for containers
                    default_msg = LintMessage(
                        value="Complex initialization logic found in Container."
                    )
                    violation_msg = (
                        definition.lazy_eager_initialization_only_violation_message
                    )
                    results.append(
                        LintResult(
                            file=f,
                            line=LineNumber(value=init_method["line"]),
                            column=ColumnNumber(value=0),
                            code=ErrorCode(code="AES021"),
                            message=violation_msg or default_msg,
                            source=AdapterName(value="architecture"),
                            severity=Severity.HIGH,
                        )
                    )

    def _check_must_implement_contract(
        self,
        f: FilePath,
        contract_name: SymbolName,
        violation_msg: LintMessage,
        analyzer,
        results: LintResultList,
        code: ErrorCode,
    ) -> None:
        bases_map = analyzer.parser.get_class_bases_map(f)
        for _, bases in bases_map.items():
            if not any(str(contract_name) in str(b) for b in bases):
                default_msg = LintMessage(
                    value=f"Class must implement {str(contract_name)}."
                )
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=code,
                        message=violation_msg if str(violation_msg) else default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )

    def _find_method_name_for_line(
        self, all_methods, line: LineNumber
    ) -> SymbolName | None:
        best_method = None
        best_line = -1
        for _, methods in all_methods.items():
            for m in methods:
                if isinstance(m, dict):
                    m_line = int(m.get("line", 0))
                    if m_line <= int(line) and m_line > best_line:
                        best_line = m_line
                        best_method = m.get("name")
        return SymbolName(value=str(best_method)) if best_method else None

    def _check_forbid_any_type(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        """AES024 — Detect `Any` type annotations in agent orchestrator layer."""
        try:
            with open(str(f), "r", encoding="utf-8") as fh:
                content = fh.read()
                tree = ast.parse(content)
        except (SyntaxError, OSError) as e:
            logger.debug(f"AES024: Parse error {e}")
            return

        violations = LocationList()
        self._collect_any_violations(tree, violations)
        for violation in violations.values:
            results.append(
                LintResult(
                    file=f,
                    line=violation.line,
                    column=violation.column,
                    code=ErrorCode(code="AES024"),
                    message=LintMessage(
                        value=f"`Any` type annotation found in agent orchestrator layer: '{violation.description}'."
                    ),
                    source=AdapterName(value="architecture"),
                    severity=Severity.HIGH,
                )
            )

    def _collect_any_violations(self, tree: ast.AST, violations: LocationList):
        """Walk AST and collect Any-type annotation violations."""
        for node in ast.walk(tree):
            self._check_node_any_return(node, violations)
            self._check_node_any_assign(node, violations)
            self._check_node_any_param(node, violations)

    def _check_node_any_return(self, node: ast.AST, violations: LocationList):
        """Check function return type annotation for `Any`."""
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            if node.returns and self._is_any_type(node.returns):
                violations.append(
                    Location(
                        line=LineNumber(value=node.lineno),
                        column=ColumnNumber(
                            value=getattr(node.returns, "col_offset", 0)
                        ),
                        description=node.name,
                    )
                )

    def _check_node_any_assign(self, node: ast.AST, violations: LocationList):
        """Check annotated assignment target type for `Any`."""
        if isinstance(node, ast.AnnAssign):
            if node.annotation and self._is_any_type(node.annotation):
                target = ast.unparse(node.target) if hasattr(node, "target") else "?"
                violations.append(
                    Location(
                        line=LineNumber(value=node.lineno),
                        column=ColumnNumber(value=node.col_offset),
                        description=f"attr:{target}",
                    )
                )

    def _check_node_any_param(self, node: ast.AST, violations: LocationList):
        """Check function parameter type annotation for `Any`."""
        if isinstance(node, ast.arg):
            if node.annotation and self._is_any_type(node.annotation):
                violations.append(
                    Location(
                        line=LineNumber(value=node.lineno),
                        column=ColumnNumber(value=node.col_offset),
                        description=f"param:{node.arg}",
                    )
                )

    def _is_any_type(self, node) -> bool:
        """Check if an AST node represents the `Any` type."""
        if isinstance(node, ast.Name) and node.id == "Any":
            return True
        if isinstance(node, ast.Attribute) and node.attr == "Any":
            return True
        return False
