"""
import_violation_analyzer — Cross-layer import rule enforcer (Capability).

Scans Python files and detects forbidden cross-layer imports based on
architecture rules defined in auto_linter.config.yaml.
"""

from __future__ import annotations
from ..taxonomy import (
    AdapterName,
    ArchitectureConfig,
    ColumnNumber,
    ComplianceStatus,
    DirectoryPath,
    ErrorCode,
    FilePath,
    LayerNameVO,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Location,
    LocationList,
    Severity,
    SymbolName,
    LAYER_AGENT,
    LayerMapVO,
)

import logging

from ..contract import (
    IImportViolationProtocol,
    IFileSystemPort,
    ISourceParserPort,
    ISemanticTracerProtocol,
)


logger = logging.getLogger(__name__)


class ImportViolationAnalyzer(IImportViolationProtocol):
    """Enforces cross-layer import restrictions (Capability)."""

    def __init__(
        self,
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
        fs: IFileSystemPort,
        parser: ISourceParserPort,
        tracer: ISemanticTracerProtocol | None = None,
    ):
        self.config = config
        self.fs = fs
        self.parser = parser
        self.tracer = tracer
        self.layer_map = layer_map

    def name(self) -> AdapterName:
        return AdapterName(value="architecture")

    async def scan(self, path: FilePath) -> LintResultList:
        """Scan path (file or directory) for cross-layer import violations."""
        if not self.config.enabled or not self.config.governance_rules:
            return LintResultList()

        results: list[LintResult] = []
        root_dir = self._resolve_root(path)

        # Walk all Python files
        for file_path in self.fs.walk(path):
            if not str(file_path).endswith(".py"):
                continue
            file_layer = self._detect_layer(file_path, root_dir)
            if not file_layer or file_layer == LAYER_AGENT:
                continue

            results.extend(self._scan_file(file_path, file_layer, root_dir))

        # Record history for trends analysis
        self._record_history(str(path), results)
        return LintResultList(values=results)

    def _scan_file(
        self, file_path: FilePath, file_layer: LayerNameVO, root_dir: FilePath
    ) -> list[LintResult]:
        """Process a single Python file for import violations."""
        violations: list[LintResult] = []
        imports = self.parser.extract_imports(file_path)

        for imp in imports:
            target_layer = self._detect_module_layer(imp.module)
            if not target_layer:
                continue

            rule = self._find_applicable_rule(file_layer, target_layer)
            if rule:
                violation = self._create_violation(
                    file_path=file_path,
                    line_no=imp.line,
                    module_path=imp.module,
                    description=str(rule.get("description", "Forbidden import")),
                    file_layer=str(file_layer),
                    target_layer=str(target_layer),
                    root_dir=root_dir,
                    imported_name=imp.name,
                )
                violations.append(violation)

        return violations

    def _find_applicable_rule(
        self, file_layer: LayerNameVO, target_layer: LayerNameVO
    ) -> dict[str, object] | None:
        """Return the first governance rule matching file→target layers."""
        for rule in self.config.governance_rules:
            if str(file_layer) == rule.get("from") and str(target_layer) == rule.get(
                "to"
            ):
                return rule
        return None

    def _create_violation(
        self,
        file_path: FilePath,
        line_no: LineNumber,
        module_path: str,
        description: str,
        file_layer: str,
        target_layer: str,
        root_dir: FilePath,
        imported_name: str | None = None,
    ) -> LintResult:
        """Construct a LintResult for a layer violation."""
        base_msg = (
            f"[AES Layer Violation] {description}. "
            f"File in '{file_layer}' imports from '{target_layer}' via '{module_path}'."
        )

        # Optional: extend with tracer information if available
        related_locations: list[str] = []
        if self.tracer and imported_name and imported_name != "*":
            try:
                func_candidate = imported_name
                callers = self.tracer.trace_call_chain(
                    root_dir=DirectoryPath(value=str(root_dir)),
                    target_name=SymbolName(value=func_candidate),
                )
                related_locations = [f"CallSite: {c}" for c in callers.values[:3]]
            except Exception as e:
                logger.debug(f"Tracer error: {e}")

        return LintResult(
            file=file_path,
            line=line_no,
            column=ColumnNumber(value=0),
            code=ErrorCode(code="AES001"),
            message=LintMessage(value=base_msg),
            source=AdapterName(value=str(self.name())),
            severity=Severity.CRITICAL,
            related_locations=LocationList(
                values=[Location(description=loc) for loc in related_locations]
            ),
        )

    def _resolve_root(self, path: FilePath) -> FilePath:
        """Find project root (directory containing 'src')."""
        current = path
        for _ in range(5):
            src_dir = self.fs.path_join(str(current), "src")
            if self.fs.is_directory(src_dir).value:
                return current
            parent = self.fs.get_parent(current)
            if parent == current:
                break
            current = parent
        return path

    def _detect_layer(
        self, file_path: FilePath, root_dir: FilePath
    ) -> LayerNameVO | None:
        rel = str(self.fs.get_relative_path(file_path, root_dir))
        sorted_layers = sorted(
            self.layer_map.values.items(), key=lambda x: len(str(x[1].path)), reverse=True
        )
        for name, definition in sorted_layers:
            path_def = definition.path_str
            if rel.startswith(path_def) or f"/{path_def}/" in f"/{rel}/":
                return LayerNameVO(value=str(name))
        return None

    def _detect_module_layer(self, module_path: str) -> LayerNameVO | None:
        parts = str(module_path).split(".")
        for part in parts:
            for name, definition in self.layer_map.values.items():
                if part == name or part == definition.path_str:
                    return LayerNameVO(value=str(name))
        return None

    def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Import violations require manual architectural refactoring."""
        return ComplianceStatus(value=False)

    def _record_history(self, path: str, results: list[LintResult]) -> None:
        """Save a snapshot of quality for trend analysis."""
        score = 100 - len(results) * 5
        history_file = FilePath(value=".auto_lint_history")
        try:
            import json
            from datetime import datetime

            record = (
                json.dumps(
                    {
                        "timestamp": datetime.now().isoformat(),
                        "path": path,
                        "score": max(0, score),
                        "violations": len(results),
                    }
                )
                + "\n"
            )
            self.fs.write_text(history_file, record, mode="append")
        except Exception as e:
            logger.debug(f"Failed to write history file: {e}")
