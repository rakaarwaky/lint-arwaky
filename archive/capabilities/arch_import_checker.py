import os
from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    AdapterName,
    Severity,
    Identity,
    ContentString,
    LayerNameVO,
    LAYER_AGENT,
)
from ..contract import IArchImportProtocol
from .arch_import_processor import ArchImportProcessor as ArchImportUtil


class ArchImportRuleChecker(IArchImportProtocol):
    """Import-related architectural checks implementation.

    Delegates complex logic to ArchImportUtil to maintain maintainability.
    """

    @property
    def rule_name(self) -> Identity:
        return Identity(value="architecture_import")

    def __init__(self):
        self._processor = ArchImportUtil()

    def _process_mandatory_file(
        self,
        analyzer,
        f: FilePath,
        root_dir: FilePath,
        results: LintResultList,
        global_template: ContentString,
    ) -> None:
        """Process a single file for mandatory import requirements."""
        if "__init__.py" in str(f):
            return
        layer_vo = analyzer._detect_layer(f, root_dir)
        if not layer_vo:
            return

        definition = analyzer.layer_map[layer_vo]
        msg_template = definition.mandatory_import_violation_message or global_template

        # Omit files in exceptions list (relative imports within contract layer, etc.)
        basename = os.path.basename(str(f))
        if definition.exceptions.values and basename in definition.exceptions.values:
            return

        if definition.mandatory_import.values:
            self._processor.validate_imports_present(
                analyzer,
                f,
                root_dir,
                definition.mandatory_import,
                results,
                msg_template,
                layer_vo,
                definition.mandatory_import.values,
            )
        if definition.mandatory_imports:
            basename_str = str(analyzer.fs.get_basename(f)).replace(".py", "")
            for rule_vo in definition.mandatory_imports:
                marker = str(rule_vo.suffix)
                if marker in basename_str:
                    required_layers = rule_vo.imports
                    self._processor.validate_imports_present(
                        analyzer,
                        f,
                        root_dir,
                        required_layers,
                        results,
                        msg_template,
                        layer_vo,
                        required_layers.values,
                    )

    def check_mandatory_imports(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        default_template = ContentString(value="Mandatory layer import is missing.")
        global_template = (
            analyzer.config.mandatory_import_violation_message or default_template
        )
        for f in files:
            self._process_mandatory_file(
                analyzer, f, root_dir, results, global_template
            )

    def check_forbidden_imports(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        for f in files:
            self._processor.process_file_imports(analyzer, f, root_dir, results)

    def _check_import_against_rules(
        self,
        imp,
        file_layer: LayerNameVO,
        analyzer,
        f: FilePath,
        results: LintResultList,
    ) -> None:
        """Resolve target layer for an import and check against all governance rules."""
        target_layer = analyzer._detect_module_layer(imp.module)
        if not target_layer:
            return

        for rule in analyzer.config.governance_rules:
            if (
                file_layer == rule.source_layer
                and target_layer == rule.forbidden_target
            ):
                self._report_legacy_violation(
                    results, f, imp, file_layer, target_layer, rule
                )
                break

    def _report_legacy_violation(
        self,
        results: LintResultList,
        f: FilePath,
        imp,
        file_layer: LayerNameVO,
        target_layer: LayerNameVO,
        rule,
    ) -> None:
        """Report a single legacy governance rule violation."""
        description = rule.description or ContentString(
            value="Forbidden layer import detected."
        )
        base_msg = (
            f"[AES Layer Violation] {description}. "
            f"File in '{file_layer}' imports from '{target_layer}' via '{imp.module}'."
        )
        results.append(
            LintResult(
                file=f,
                line=imp.line,
                column=getattr(imp, "column", ColumnNumber(value=0)),
                code=ErrorCode(code="AES001"),
                message=LintMessage(value=base_msg),
                source=AdapterName(value="architecture"),
                severity=Severity.CRITICAL,
            )
        )

    def check_legacy_import_rules(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Enforces cross-layer import restrictions from top-level governance rules."""
        if not analyzer.config.governance_rules:
            return

        for f in files:
            file_layer = analyzer._detect_layer(f, root_dir)
            if not file_layer or file_layer == LAYER_AGENT:
                continue

            imports = analyzer.parser.extract_imports(f)
            for imp in imports:
                self._check_import_against_rules(imp, file_layer, analyzer, f, results)
