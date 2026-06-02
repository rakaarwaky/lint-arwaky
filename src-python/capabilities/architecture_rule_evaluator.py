"""Architecture rule evaluation — delegates to specialized checkers."""

from ..taxonomy import (
    ArchitectureConfig,
    FilePath,
    FilePathList,
    LayerNameVO,
    LintResultList,
)

from .naming_rule_checker import NamingRuleChecker
from .code_quality_checker import CodeQualityRuleChecker
from .arch_naming_checker import ArchNamingChecker
from .arch_internal_checker import ArchInternalChecker
from .arch_metric_checker import ArchMetricChecker
from .arch_role_checker import ArchRoleChecker
from .arch_import_checker import ArchImportRuleChecker
from ..contract import IArchRuleEngineProtocol, IScannerProviderPort, ISourceParserPort


class ArchitectureRuleEvaluator(IArchRuleEngineProtocol):
    """Architecture rule evaluation — delegates to specialized checkers."""

    def __init__(
        self,
        config: ArchitectureConfig,
        fs: IScannerProviderPort,
        parser: ISourceParserPort,
        layer_map: dict,
    ):
        self.config = config
        self.fs = fs
        self.parser = parser
        self.layer_map = layer_map

        # Instantiate checkers
        self.naming_rule_checker = NamingRuleChecker()
        self.quality_checker = CodeQualityRuleChecker()

        self.arch_naming_checker = ArchNamingChecker()
        self.internal_checker = ArchInternalChecker()
        self.metric_checker = ArchMetricChecker()
        self.role_checker = ArchRoleChecker()

        self.import_checker = ArchImportRuleChecker()

    def check_file_naming(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check file naming conventions (underscore count)."""
        # Collect exceptions from all global naming rules
        global_exceptions: list[str] = []
        for rule in self.config.rules:
            if rule.rule_type == "global" and rule.exceptions:
                global_exceptions.extend(rule.exceptions.values)

        self.naming_rule_checker.check_file_naming(
            files,
            root_dir,
            self.layer_map,
            self.config.naming.word_count,
            global_exceptions,
            results,
            self.detect_layer,
        )

    def check_domain_suffixes(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check domain suffix rules per layer."""
        self.arch_naming_checker.check_domain_suffixes(self, files, root_dir, results)

    def check_layer_internal_rules(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check barrel completeness, no-primitives, and no-internal-all rules."""
        self.internal_checker.check_layer_internal_rules(self, files, root_dir, results)

    def check_no_bypass_comments(self, files: FilePathList, results: LintResultList):
        """Ensure no bypass comments exist."""
        for f in files:
            layer = self.detect_layer(f, FilePath(value="."))
            if not layer:
                continue

            layer_def = self.layer_map.get(layer.value)
            if not layer_def:
                continue

            self.quality_checker.check_no_bypass_comments(
                f,
                self.fs,
                results,
                layer_def.forbidden_bypass,
                layer_def.forbidden_bypass_message,
            )

    def check_unused_mandatory_imports(
        self, files: FilePathList, results: LintResultList
    ):
        """Ensure mandatory imports are used."""
        self.quality_checker.check_unused_mandatory_imports(files, self.parser, results)

    def check_mandatory_imports(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check mandatory layer imports."""
        self.import_checker.check_mandatory_imports(self, files, root_dir, results)

    def check_forbidden_imports(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check forbidden layer imports."""
        self.import_checker.check_forbidden_imports(self, files, root_dir, results)

    def check_line_counts(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ) -> None:
        """Check file line count against thresholds."""
        self.metric_checker.check_line_counts(self, files, root_dir, results)

    def check_mandatory_class_definition(
        self, files: FilePathList, root_dir: FilePath, results: LintResultList
    ):
        """Check for mandatory class definitions in files."""
        self.metric_checker.check_mandatory_class_definition(
            self, files, root_dir, results
        )

    def _detect_layer(
        self, file_path: FilePath, root_dir: FilePath
    ) -> LayerNameVO | None:
        """Internal alias for detect_layer to satisfy checker expectations."""
        return self.detect_layer(file_path, root_dir)

    def _detect_module_layer(self, module_path: FilePath) -> LayerNameVO | None:
        """Internal alias for detect_module_layer to satisfy checker expectations."""
        return self.detect_module_layer(module_path)

    def detect_layer(
        self, file_path: FilePath, root_dir: FilePath
    ) -> LayerNameVO | None:
        """Detect which layer a file belongs to."""
        rel_path_vo = self.fs.get_relative_path(file_path, root_dir)
        rel = str(rel_path_vo)

        # Sort layers by path length descending to match most specific first
        sorted_layers = sorted(
            self.layer_map.items(),
            key=lambda x: len(str(x[1].path)),
            reverse=True,
        )
        for name, definition in sorted_layers:
            path_def = definition.path_str
            # Check if relative path starts with layer path or contains it as segment
            if rel.startswith(path_def) or f"/{path_def}/" in f"/{rel}/":
                return name
        return None

    def detect_module_layer(self, module_path: FilePath) -> LayerNameVO | None:
        """Detect layer from a dotted module path."""
        parts = str(module_path).split(".")
        for part in parts:
            for name, definition in self.layer_map.items():
                path_last = definition.path_str.split("/")[-1]
                if part == name or part == path_last:
                    return name
        return None
