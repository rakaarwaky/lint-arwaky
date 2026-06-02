from __future__ import annotations
from typing import TYPE_CHECKING

from ..contract import CapabilityContainerAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath
from ..capabilities import (
    ArchComplianceAnalyzer,
    ArchImportChecker,
    ArchOrphanAnalyzer,
    ArchNamingChecker,
    ArchInternalChecker,
    ArchMetricChecker,
    ArchRoleChecker,
    ArchitectureRuleEvaluator,
    DispatchRoutingChecker,
    DispatchRoutingParser,
    CodeQualityChecker,
    ConfigRulesValidator,
    DomainTypeChecker,
    NamingRuleChecker,
    SemanticScopeAnalyzer,
    CallChainAnalyzer,
    DataFlowAnalyzer,
    NamingVariantAnalyzer,
    ScopeBoundaryAnalyzer,
    ScopeBoundaryResolver,
    ReportFormatterProcessor,
    MetricAnalyzerProcessor,
    SymbolRenamerProcessor,
    UnusedImportChecker,
    SetupManagementProcessor,
    ArchImportProcessor,
    OrphanGraphResolver,
    OrphanIndicatorEvaluator,
    CycleAnalyzer,
)
from .arch_compliance_orchestrator import ArchitectureOrchestrator
from ..capabilities import ImportViolationAnalyzer
from .arch_compliance_coordinator import ArchComplianceCoordinator

if TYPE_CHECKING:
    from .dependency_injection_container import Container


class CapabilityMixinContainer(ServiceContainerAggregate, CapabilityContainerAggregate):
    """Logic for initializing analyzers and processors."""

    def _init_capabilities(self: "Container") -> None:
        # Ground mandatory imports
        _cap_container: CapabilityContainerAggregate = self
        _ = _cap_container
        FilePath  # taxonomy import grounding
        config = self.config

        # 1. Semantic Analyzers
        self.naming_variant_analyzer = NamingVariantAnalyzer()
        self.scope_analyzer = SemanticScopeAnalyzer(fs=self.fs_scanner)
        self.scope_boundary_analyzer = ScopeBoundaryAnalyzer(fs=self.fs_scanner)
        self.scope_boundary_resolver = ScopeBoundaryResolver(fs_scanner=self.fs_scanner)
        self.data_flow_analyzer = DataFlowAnalyzer(
            fs=self.fs_scanner, scope=self.scope_boundary_analyzer
        )
        self.call_chain_analyzer = CallChainAnalyzer(
            fs=self.fs_scanner,
            data_flow=self.data_flow_analyzer,
            naming=self.naming_variant_analyzer,
            scope=self.scope_boundary_analyzer,
        )
        self.semantic_analyzers = {
            "scope": self.scope_analyzer,
            "data_flow": self.data_flow_analyzer,
            "call_chain": self.call_chain_analyzer,
            "scope_boundary": self.scope_boundary_analyzer,
            "scope_resolver": self.scope_boundary_resolver,
        }

        # 2. Setup & Utility Processors
        self.setup_processor = SetupManagementProcessor(
            http_provider=self.http_provider
        )
        self.report_formatter = ReportFormatterProcessor()
        self.metric_analyzer = MetricAnalyzerProcessor()
        self.symbol_renamer = SymbolRenamerProcessor(fs_scanner=self.fs_scanner)
        self.unused_import_checker = UnusedImportChecker(parser=self.source_parser)
        self.cycle_analyzer = CycleAnalyzer(
            config=config,
            layer_map={},
            fs=self.fs_scanner,
            parser=self.source_parser,
        )

        # 3. Architecture Compliance
        if config.project.architecture and config.project.architecture.enabled:
            orchestrator = ArchitectureOrchestrator()
            layer_map = orchestrator.resolve_effective_layer_map(
                config.project.architecture
            )

            # Initialize leaf-node capabilities first
            # Initialize modular checkers
            self.arch_naming_checker = ArchNamingChecker()
            self.arch_internal_checker = ArchInternalChecker()
            self.arch_metric_checker = ArchMetricChecker()
            self.arch_role_checker = ArchRoleChecker()
            self.dispatch_parser = DispatchRoutingParser()
            self.dispatch_checker = DispatchRoutingChecker(parser=self.dispatch_parser)

            self.arch_import_checker = ArchImportChecker()
            self.arch_import_processor = ArchImportProcessor()
            self.orphan_resolver = OrphanGraphResolver()
            self.orphan_evaluator = OrphanIndicatorEvaluator()
            self.arch_orphan_detector = ArchOrphanAnalyzer(
                resolver=self.orphan_resolver, evaluator=self.orphan_evaluator
            )
            self.code_quality_checker = CodeQualityChecker()
            self.domain_type_checker = DomainTypeChecker(parser=self.source_parser)
            self.naming_rule_checker = NamingRuleChecker()
            self.config_rules_validator = ConfigRulesValidator(
                project_config=config.project
            )

            # Inject them into the main compliance orchestrator
            self.arch_analyzer = ArchComplianceAnalyzer(
                config=config.project.architecture,
                layer_map=layer_map,
                fs=self.fs_scanner,
                parser=self.source_parser,
                ignored_paths=config.project.ignored_paths,
                naming_checker=self.arch_naming_checker,
                internal_checker=self.arch_internal_checker,
                metric_checker=self.arch_metric_checker,
                role_checker=self.arch_role_checker,
                import_checker=self.arch_import_checker,
                quality_checker=self.code_quality_checker,
                orphan_detector=self.arch_orphan_detector,
                dispatch_checker=self.dispatch_checker,
            )

            self.import_violation_analyzer = ImportViolationAnalyzer(
                config=config.project.architecture,
                layer_map=layer_map,
                fs=self.fs_scanner,
                parser=self.source_parser,
                tracer=self.python_tracer,
            )
            self.arch_rule_evaluator = ArchitectureRuleEvaluator(
                config=config.project.architecture,
                fs=self.fs_scanner,
                parser=self.source_parser,
                layer_map=layer_map,
            )

            # Agent bridge for the adapter
            self.arch_compliance_coordinator = ArchComplianceCoordinator(
                compliance_orchestrator=self.arch_analyzer
            )
