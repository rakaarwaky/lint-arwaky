// Capabilities layer barrel — AES allowed suffixes:
// _checker, _analyzer, _processor, _evaluator, _resolver, _validator,
// _formatter, _handler.

pub mod architecture_compliance_analyzer;
pub mod architecture_cycle_analyzer;
pub mod architecture_import_checker;
pub mod architecture_inheritance_checker;
pub mod architecture_internal_checker;
pub mod architecture_lint_handler;
pub mod architecture_metric_checker;
pub mod architecture_naming_checker;
pub mod architecture_orphan_analyzer;
pub mod architecture_quality_checker;
pub mod architecture_role_checker;
pub mod architecture_unused_checker;

pub mod config_rules_validator;
pub mod dispatch_routing_processor;
pub mod domain_type_checker;
pub mod lint_reporting_formatter;
pub mod mcp_schema_checker;
pub mod naming_renamer_processor;
pub mod naming_rule_checker;
pub mod naming_variant_analyzer;
pub mod semantic_boundary_analyzer;
pub mod semantic_boundary_resolver;
pub mod semantic_flow_analyzer;
pub mod semantic_scope_analyzer;
pub mod semantic_tracer_analyzer;
pub mod setup_management_processor;
pub mod surface_hierarchy_checker;
