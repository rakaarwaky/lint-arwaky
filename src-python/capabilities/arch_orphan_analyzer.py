"""arch_orphan_analyzer — Multi-indicator orphan code detection logic."""

import logging
from typing import TYPE_CHECKING
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
    Identity,
    GraphAnalysisContext,
    LayerDefinition,
    LayerNameVO,
    OrphanIndicatorResult,
    ReachabilityResult,
    LAYER_AGENT,
    LAYER_CONTRACT,
    LAYER_SURFACES,
    LAYER_TAXONOMY,
    LAYER_INFRASTRUCTURE,
    LAYER_CAPABILITIES,
)
from ..contract import IArchOrphanProtocol
from .orphan_graph_resolver import OrphanGraphResolver
from .orphan_indicator_evaluator import OrphanIndicatorEvaluator

if TYPE_CHECKING:
    from .arch_compliance_analyzer import ArchComplianceAnalyzer


class ArchOrphanAnalyzer(IArchOrphanProtocol):
    """Detects 'Orphan' files using multiple indicators (Imports, Reachability, Barrel)."""

    def __init__(
        self,
        resolver: OrphanGraphResolver | None = None,
        evaluator: OrphanIndicatorEvaluator | None = None,
    ) -> None:
        self.resolver = resolver or OrphanGraphResolver()
        self.evaluator = evaluator or OrphanIndicatorEvaluator()

    @property
    def rule_name(self) -> Identity:
        return Identity(value="arch_orphan")

    def _filter_project_files(
        self,
        analyzer: "ArchComplianceAnalyzer",
        root_dir: FilePath,
    ) -> FilePathList:
        """Discovers and filters project files by supported extension."""
        all_project_files = list(analyzer.fs.walk(root_dir, analyzer.ignored_paths))
        supported_exts = analyzer.parser.get_supported_extensions()
        ext = ".py"
        for e in supported_exts:
            if any(str(f).endswith(e) for f in all_project_files):
                if e in [".ts", ".tsx", ".js", ".jsx"]:
                    ext = (".ts", ".tsx", ".js", ".jsx")
                else:
                    ext = e
                break
        filtered = [f for f in all_project_files if str(f).endswith(ext)]
        return FilePathList(values=filtered)

    def check_orphans(
        self,
        analyzer: "ArchComplianceAnalyzer",
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Analyzes all files to find those unreachable from entry points."""

        all_project_files_fp = self._filter_project_files(analyzer, root_dir)

        # 2. Build Comprehensive Context (Project-wide)
        context: GraphAnalysisContext = self.resolver.build_graph_context(
            analyzer, all_project_files_fp, root_dir
        )

        # 3. Trace Reachability
        entry_points = self.resolver.identify_entry_points(
            analyzer, all_project_files_fp, root_dir
        )
        alive_files = self.resolver.trace_reachability(
            entry_points, context.import_graph
        )

        # 4. Multi-Indicator Evaluation
        for f in files.values:
            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo:
                logging.getLogger("capabilities.orphan").warning(
                    "Cannot detect layer for %s — skipping orphan check", f.value
                )
                continue

            definition = analyzer.layer_map.get(layer_vo)
            if not definition:
                continue

            basename = str(analyzer.fs.get_basename(f))
            if definition.exceptions.values and basename in definition.exceptions.values:
                continue

            if hasattr(definition, "check_orphan") and not definition.check_orphan:
                continue

            res = self._evaluate_layer(
                analyzer, f, root_dir, definition, context, alive_files, layer_vo
            )

            if res and res.is_orphan:
                results.values.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=1),
                        code=ErrorCode(code="AES017"),
                        message=LintMessage(value=res.reason),
                        severity=res.severity,
                        source=AdapterName(value="architecture"),
                    )
                )

    def _evaluate_layer(
        self,
        analyzer: "ArchComplianceAnalyzer",
        f: FilePath,
        root_dir: FilePath,
        definition: LayerDefinition,
        context: GraphAnalysisContext,
        alive_files: ReachabilityResult,
        layer_vo: LayerNameVO,
    ) -> OrphanIndicatorResult | None:
        """Route to the appropriate layer-specific orphan evaluation."""
        # Barrel files are never orphans — they export types for the system
        if analyzer.parser.is_barrel_file(f):
            return None

        layer_str = str(layer_vo).lower()
        if str(LAYER_TAXONOMY) in layer_str:
            return self.evaluator.is_taxonomy_orphan(
                analyzer, f, root_dir, definition, context.inbound_links
            )
        if str(LAYER_CONTRACT) in layer_str:
            return self._check_contract_orphans(analyzer, f, root_dir, context)
        if any(
            str(name) in layer_str
            for name in [LAYER_INFRASTRUCTURE, LAYER_CAPABILITIES]
        ):
            return self._check_infra_orphans(analyzer, f, root_dir, alive_files)
        if str(LAYER_AGENT) in layer_str:
            return self._check_agent_orphans(analyzer, f, root_dir)
        if str(LAYER_SURFACES) in layer_str:
            return self._check_surface_orphans(f, alive_files, definition)
        return self.evaluator.is_generic_orphan(f, alive_files, context.inbound_links)

    def _check_contract_orphans(
        self,
        analyzer: "ArchComplianceAnalyzer",
        f: FilePath,
        root_dir: FilePath,
        context: GraphAnalysisContext,
    ) -> OrphanIndicatorResult:
        return self.evaluator.is_contract_orphan(
            analyzer, f, root_dir, context.file_definitions, context.inheritance_map
        )

    def _check_infra_orphans(
        self,
        analyzer: "ArchComplianceAnalyzer",
        f: FilePath,
        root_dir: FilePath,
        alive_files: ReachabilityResult,
    ) -> OrphanIndicatorResult:
        return self.evaluator.is_infra_cap_orphan(analyzer, f, root_dir, alive_files)

    def _check_agent_orphans(
        self,
        analyzer: "ArchComplianceAnalyzer",
        f: FilePath,
        root_dir: FilePath,
    ) -> OrphanIndicatorResult:
        return self.evaluator.is_agent_orphan(analyzer, f, root_dir)

    def _check_surface_orphans(
        self,
        f: FilePath,
        alive_files: ReachabilityResult,
        definition: LayerDefinition,
    ) -> OrphanIndicatorResult:
        return self.evaluator.is_surface_orphan(f, alive_files, definition)
