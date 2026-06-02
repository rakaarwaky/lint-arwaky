"""arch_metric_checker — Architectural metric checks (line counts, mandatory classes)."""

from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    Severity,
    AdapterName,
    SymbolName,
    BooleanVO,
)
from ..contract import IMetricCheckerProtocol


class ArchMetricChecker(IMetricCheckerProtocol):
    """Handles architectural checks related to file size and structure metrics."""

    def check_line_counts(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        for f in files:
            basename = SymbolName(value=str(analyzer.fs.get_basename(f)))
            if str(basename) == "__init__.py":
                continue

            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo:
                continue

            definition = analyzer.layer_map[layer_vo]
            if definition.exceptions.values and str(basename) in definition.exceptions.values:
                continue
            count = analyzer.fs.get_line_count(f)
            if definition.min_lines and int(count) < definition.min_lines.value:
                default_msg = LintMessage(
                    value=(
                        "AES005 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n"
                        "WHY? Excessively small files clutter the project structure; logic should be merged into a parent module.\n"
                        f"FIX: Expand the component or merge this logic into a related module (min: {definition.min_lines.value})."
                    )
                )
                violation_msg = definition.min_lines_violation_message
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES005"),
                        message=violation_msg or default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )
            if definition.max_lines and int(count) > definition.max_lines.value:
                default_msg = LintMessage(
                    value=(
                        "AES004 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n"
                        "WHY? Large files violate the Single Responsibility Principle and are difficult to maintain or test.\n"
                        f"FIX: Split the module into smaller, more focused files (max: {definition.max_lines.value})."
                    )
                )
                violation_msg = definition.max_lines_violation_message
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES004"),
                        message=violation_msg or default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )

    def check_mandatory_class_definition(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check that every file defines at least one class if mandatory_class_definition is enabled."""
        for f in files:
            layer_vo = analyzer._detect_layer(f, root_dir)
            enabled = BooleanVO(value=False)
            violation_msg = None

            basename = SymbolName(value=str(analyzer.fs.get_basename(f)))
            if layer_vo:
                if layer_vo in analyzer.layer_map:
                    definition = analyzer.layer_map[layer_vo]
                    if definition.exceptions.values and str(basename) in definition.exceptions.values:
                        continue
                    enabled = BooleanVO(
                        value=bool(definition.mandatory_class_definition)
                    )
                    violation_msg = (
                        definition.mandatory_class_definition_violation_message
                    )
            else:
                # Fallback to global rules if not in a specific layer
                enabled = BooleanVO(
                    value=bool(analyzer.config.mandatory_class_definition)
                )
                violation_msg = (
                    analyzer.config.mandatory_class_definition_violation_message
                )

            if not bool(enabled):
                continue

            basename = SymbolName(value=str(analyzer.fs.get_basename(f)))
            # Omit non-code files or entry points that might not need classes
            if str(basename) in ["__init__.py", "main.py", "py.typed"]:
                continue

            metadata = analyzer.parser.get_class_definitions(f)
            classes = metadata.value.get("classes", [])
            if not classes:
                default_msg = LintMessage(
                    value=(
                        "AES009 MANDATORY_CLASS_DEFINITION: File is missing a class definition.\n"
                        "WHY? Encapsulation in classes is required for proper dependency injection and contract adherence.\n"
                        "FIX: Move standalone functions into a class that implements its corresponding domain contract."
                    )
                )
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=0),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code="AES009"),
                        message=violation_msg or default_msg,
                        source=AdapterName(value="architecture"),
                        severity=Severity.HIGH,
                    )
                )
