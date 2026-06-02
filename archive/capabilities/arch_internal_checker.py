"""arch_internal_checker — Internal architectural rule checks (barrels, primitives)."""

from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    PRIMITIVE_TYPE_LIST,
    PrimitiveTypeList,
    Severity,
    AdapterName,
    LayerNameVO,
)
from ..contract import IInternalCheckerProtocol


class ArchInternalChecker(IInternalCheckerProtocol):
    """Handles internal layer rules like barrel completeness and primitive usage."""

    def check_layer_internal_rules(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        for f in files:
            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo:
                continue
            definition = analyzer.layer_map[layer_vo]

            if analyzer.parser.is_barrel_file(f):
                self._check_barrel_completeness(
                    f, definition, layer_vo, analyzer, results
                )
                continue

            self._check_forbid_internal_all(f, definition, analyzer, results)
            self._check_no_primitives(f, definition, analyzer, results)

    def _check_barrel_completeness(
        self,
        f: FilePath,
        definition,
        layer_name: LayerNameVO,
        analyzer,
        results: LintResultList,
    ) -> None:
        if definition.barrel_completeness and not analyzer.parser.has_all_export(f):
            default_msg = LintMessage(value="__init__.py missing __all__ export list.")
            violation_msg = definition.barrel_completeness_violation_message
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES012"),
                    message=violation_msg or default_msg,
                    source=AdapterName(value="architecture"),
                    severity=Severity.MEDIUM,
                )
            )

    def _check_forbid_internal_all(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        if definition.forbid_internal_all and analyzer.parser.has_all_export(f):
            default_msg = LintMessage(value="__all__ is forbidden in non-barrel files.")
            violation_msg = definition.forbid_internal_all_violation_message
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES013"),
                    message=violation_msg or default_msg,
                    source=AdapterName(value="architecture"),
                    severity=Severity.MEDIUM,
                )
            )

    def _check_no_primitives(
        self, f: FilePath, definition, analyzer, results: LintResultList
    ) -> None:
        if not definition.no_primitives:
            return

        target_primitives = PRIMITIVE_TYPE_LIST
        if isinstance(definition.no_primitives, PrimitiveTypeList):
            target_primitives = definition.no_primitives

        code = ErrorCode(code="AES006")

        violations = analyzer.parser.find_primitive_violations(f, target_primitives)
        for violation in violations.values:
            default_msg = LintMessage(
                value="Use Value Objects instead of raw primitives."
            )
            violation_msg = definition.no_primitives_violation_message
            results.append(
                LintResult(
                    file=f,
                    line=violation.line,
                    column=violation.column,
                    code=code,
                    message=violation_msg or default_msg,
                    source=AdapterName(value="architecture"),
                    severity=Severity.MEDIUM,
                )
            )
