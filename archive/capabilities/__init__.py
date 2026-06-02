"""Capabilities layer — Business logic and analysis use cases.

Exports:
  - ImportViolationAnalyzer: Cross-layer import rule enforcer
  - ArchitectureComplianceAnalyzer: Full architecture compliance (naming, suffix,
  - SemanticScopeAnalyzer: AST-based semantic scope analysis
  primitives, line counts)
  - CallChainAnalyzer: Function call chain tracing
  - DataFlowAnalyzer: Variable lifecycle tracking
  - NamingVariantAnalyzer: Identifier variant generation
  - ScopeBoundaryAnalyzer: JS/TS scope boundary detection
  - ArchitectureHelpers: Shared utilities for architecture analysis
"""

from .import_violation_analyzer import ImportViolationAnalyzer
from .arch_compliance_analyzer import ArchComplianceAnalyzer
from .arch_import_checker import ArchImportRuleChecker as ArchImportChecker
from .arch_orphan_analyzer import ArchOrphanAnalyzer
from .arch_import_processor import ArchImportProcessor
from .orphan_indicator_evaluator import OrphanIndicatorEvaluator
from .orphan_graph_resolver import OrphanGraphResolver
from .arch_naming_checker import ArchNamingChecker
from .arch_internal_checker import ArchInternalChecker
from .arch_metric_checker import ArchMetricChecker
from .arch_role_checker import ArchRoleChecker
from .architecture_rule_evaluator import ArchitectureRuleEvaluator
from .code_quality_checker import CodeQualityRuleChecker as CodeQualityChecker
from .config_rules_validator import ConfigRulesValidator
from .domain_type_checker import DomainTypeRuleChecker as DomainTypeChecker
from .naming_rule_checker import NamingRuleChecker
from .semantic_scope_analyzer import SemanticScopeAnalyzer
from .call_chain_analyzer import CallChainAnalyzer
from .data_flow_analyzer import DataFlowAnalyzer
from .naming_variant_analyzer import NamingVariantAnalyzer
from .scope_boundary_analyzer import ScopeBoundaryAnalyzer
from .scope_boundary_resolver import ScopeBoundaryResolver
from .report_formatter_processor import ReportFormatterProcessor
from .metric_analyzer_processor import MetricAnalyzerProcessor
from .symbol_renamer_processor import SymbolRenamerProcessor
from .unused_import_checker import UnusedImportRuleChecker as UnusedImportChecker
from .setup_management_processor import SetupManagementProcessor
from .dispatch_routing_checker import DispatchRoutingChecker
from .dispatch_routing_parser import DispatchRoutingParser
from .dependency_cycle_analyzer import CycleAnalyzer

# Re-export module for internal agent use

__all__ = [
    "ImportViolationAnalyzer",
    "ArchComplianceAnalyzer",
    "ArchImportChecker",
    "ArchOrphanAnalyzer",
    "ArchImportProcessor",
    "OrphanIndicatorEvaluator",
    "OrphanGraphResolver",
    "ArchNamingChecker",
    "ArchInternalChecker",
    "ArchMetricChecker",
    "ArchRoleChecker",
    "ArchitectureRuleEvaluator",
    "CodeQualityChecker",
    "ConfigRulesValidator",
    "DomainTypeChecker",
    "NamingRuleChecker",
    "SemanticScopeAnalyzer",
    "CallChainAnalyzer",
    "DataFlowAnalyzer",
    "NamingVariantAnalyzer",
    "ScopeBoundaryAnalyzer",
    "ScopeBoundaryResolver",
    "ReportFormatterProcessor",
    "MetricAnalyzerProcessor",
    "SymbolRenamerProcessor",
    "UnusedImportChecker",
    "SetupManagementProcessor",
    "DispatchRoutingChecker",
    "DispatchRoutingParser",
    "CycleAnalyzer",
]
