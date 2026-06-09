use once_cell::sync::Lazy;
use regex::Regex;

use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::naming_rules::taxonomy_symbol_vo::SymbolName;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_names_vo::layer_infrastructure;
use crate::source_parsing::taxonomy_path_vo::FilePath;

fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}

pub struct AgentRoleChecker {}
impl AgentRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_container(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_orchestrator(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_coordinator(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_registry(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_manager(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_mixin(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_state(&self) -> Vec<LintResult> {
        vec![]
    }

    // ---- moved from capabilities_role_checker.rs ----

    fn _check_must_implement_contract_lazy(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let contract_name = SymbolName::new("ServiceContainerAggregate");
        let violation_msg = definition
            .must_implement_service_container_aggregate_violation_message
            .value
            .clone();
        self._check_must_implement_contract(
            f,
            &contract_name,
            &violation_msg,
            analyzer,
            results,
            "AES021",
        );
    }

    fn _check_stateless_execution(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let metadata_assigns = analyzer.parser().get_assignment_targets(f);
        let assignments = metadata_assigns
            .value
            .get("assignments")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let metadata_methods = analyzer.parser().get_class_methods(f);

        for assign in &assignments {
            let line_val = assign.get("line").and_then(|v| v.as_i64()).unwrap_or(0);
            let line_vo = LineNumber::new(line_val);
            let method_name = self._find_method_name_for_line(
                &serde_json::Value::Object(serde_json::Map::from_iter(
                    metadata_methods.value.clone().into_iter(),
                )),
                line_val,
            );
            if let Some(ref name) = method_name {
                if name.value != "__init__" {
                    let msg = definition
                        .stateless_execution_violation_message
                        .value
                        .clone();
                    let message = if msg.is_empty() {
                        "Non-stateless behavior detected: state assignment found outside __init__."
                            .to_string()
                    } else {
                        msg
                    };
                    results.push(LintResult {
                        file: f.clone(),
                        line: line_vo,
                        column: ColumnNumber::new(0),
                        code: ErrorCode::raw("AES021"),
                        message: LintMessage::new(message),
                        source: make_adapter("architecture"),
                        severity: Severity::HIGH,
                        enclosing_scope: None,
                        related_locations:
                            crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                    });
                }
            }
        }
    }

    fn _check_high_level_policy_only(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let imports = match analyzer.parser().extract_imports(f) {
            Ok(imp) => imp,
            Err(_) => return,
        };

        for imp in imports.values {
            if imp.module.contains(&layer_infrastructure().value) {
                let msg = definition
                    .high_level_policy_only_violation_message
                    .value
                    .clone();
                let message = if msg.is_empty() {
                    "Low-level implementation details found (infrastructure import).".to_string()
                } else {
                    msg
                };
                results.push(LintResult {
                    file: f.clone(),
                    line: imp.line.clone(),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw("AES021"),
                    message: LintMessage::new(message),
                    source: make_adapter("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: None,
                    related_locations:
                        crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                });
            }
        }
    }

    fn _check_coordinates_multiple_orchestrators(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let metadata = analyzer.parser().get_class_methods(f);
        for (_, class_methods) in &metadata.value {
            let init_method = self._find_init_method(class_methods);
            if let Some(ref init_m) = init_method {
                if self._count_orchestrator_args(init_m) < 2 {
                    let line_val = init_m.get("line").and_then(|v| v.as_i64()).unwrap_or(0);
                    let msg = definition
                        .coordinates_multiple_orchestrators_violation_message
                        .value
                        .clone();
                    let message = if msg.is_empty() {
                        "Coordinator must manage multiple orchestrators.".to_string()
                    } else {
                        msg
                    };
                    results.push(LintResult {
                        file: f.clone(),
                        line: LineNumber::new(line_val),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::raw("AES021"),
                        message: LintMessage::new(message),
                        source: make_adapter("architecture"),
                        severity: Severity::MEDIUM,
                        enclosing_scope: None,
                        related_locations:
                            crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                    });
                }
            }
        }
    }

    fn _find_init_method(&self, class_methods: &serde_json::Value) -> Option<serde_json::Value> {
        if let Some(arr) = class_methods.as_array() {
            for m in arr {
                if let Some(obj) = m.as_object() {
                    if obj.get("name").and_then(|v| v.as_str()) == Some("__init__") {
                        return Some(m.clone());
                    }
                } else if let Some(s) = m.as_str() {
                    if s == "__init__" {
                        let mut obj = serde_json::Map::new();
                        obj.insert(
                            "name".to_string(),
                            serde_json::Value::String("__init__".to_string()),
                        );
                        obj.insert("line".to_string(), serde_json::Value::Number(0.into()));
                        obj.insert("args".to_string(), serde_json::Value::Array(vec![]));
                        return Some(serde_json::Value::Object(obj));
                    }
                }
            }
        }
        None
    }

    fn _count_orchestrator_args(&self, method: &serde_json::Value) -> usize {
        method
            .get("args")
            .and_then(|v| v.as_array())
            .map(|args| {
                args.iter()
                    .filter(|a| a.to_string().to_lowercase().contains("orchestrator"))
                    .count()
            })
            .unwrap_or(0)
    }

    fn _check_no_domain_logic(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        code: &str,
    ) {
        let control_flow_count = analyzer.parser().get_control_flow_count(f);
        if control_flow_count.value > 3 {
            let default_msg =
                "Complex domain logic detected in a passive layer/role.".to_string();
            let violation_msg = if !definition
                .no_domain_logic_violation_message
                .value
                .is_empty()
            {
                definition.no_domain_logic_violation_message.value.clone()
            } else if !definition
                .no_decision_logic_violation_message
                .value
                .is_empty()
            {
                definition.no_decision_logic_violation_message.value.clone()
            } else {
                default_msg
            };
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(code),
                message: LintMessage::new(violation_msg),
                source: make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations:
                    crate::shared_common::taxonomy_lint_vo::LocationList::new(),
            });
        }
    }

    fn _check_lazy_eager_init_only(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let metadata = analyzer.parser().get_class_methods(f);
        for (_, class_methods) in &metadata.value {
            let init_method = self._find_init_method(class_methods);
            if init_method.is_some() {
                let control_flow_count = analyzer.parser().get_control_flow_count(f);
                if control_flow_count.value > 2 {
                    let line_val = init_method
                        .as_ref()
                        .and_then(|m| m.get("line"))
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);
                    let msg = definition
                        .lazy_eager_initialization_only_violation_message
                        .value
                        .clone();
                    let message = if msg.is_empty() {
                        "Complex initialization logic found in Container.".to_string()
                    } else {
                        msg
                    };
                    results.push(LintResult {
                        file: f.clone(),
                        line: LineNumber::new(line_val),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::raw("AES021"),
                        message: LintMessage::new(message),
                        source: make_adapter("architecture"),
                        severity: Severity::HIGH,
                        enclosing_scope: None,
                        related_locations:
                            crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                    });
                }
            }
        }
    }

    fn _check_must_implement_contract(
        &self,
        f: &FilePath,
        contract_name: &SymbolName,
        violation_msg: &str,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        code: &str,
    ) {
        let bases_map = analyzer.parser().get_class_bases_map(f);
        for (_, bases) in &bases_map.value {
            let has_contract = bases.as_array().map_or(false, |arr| {
                arr.iter()
                    .any(|b| b.to_string().contains(&contract_name.value))
            });
            if !has_contract {
                let default_msg = format!("Class must implement {}.", contract_name.value);
                let message = if violation_msg.is_empty() {
                    default_msg
                } else {
                    violation_msg.to_string()
                };
                results.push(LintResult {
                    file: f.clone(),
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: make_adapter("architecture"),
                    severity: Severity::HIGH,
                    enclosing_scope: None,
                    related_locations:
                        crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                });
            }
        }
    }

    fn _find_method_name_for_line(
        &self,
        all_methods: &serde_json::Value,
        line: i64,
    ) -> Option<SymbolName> {
        let mut best_method: Option<String> = None;
        let mut best_line: i64 = -1;

        if let Some(obj) = all_methods.as_object() {
            for (_, methods) in obj {
                if let Some(arr) = methods.as_array() {
                    for m in arr {
                        if let Some(m_obj) = m.as_object() {
                            let m_line = m_obj.get("line").and_then(|v| v.as_i64()).unwrap_or(0);
                            let m_name = m_obj.get("name").and_then(|v| v.as_str());
                            if let Some(name) = m_name {
                                if m_line <= line && m_line > best_line {
                                    best_line = m_line;
                                    best_method = Some(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        best_method.map(|s| SymbolName::new(s))
    }

    fn _check_forbid_any_type(
        &self,
        f: &FilePath,
        _definition: &LayerDefinition,
        _analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        let content = match std::fs::read_to_string(f.to_string().as_str()) {
            Ok(c) => c,
            Err(_) => return,
        };

        static ANY_TYPE_RE: Lazy<Option<Regex>> =
            Lazy::new(|| Regex::new(r":\s*[Aa]ny\b|->\s*[Aa]ny\b|\b[Aa]ny\s*\[").ok());

        for (i, line) in content.lines().enumerate() {
            if let Some(ref re) = *ANY_TYPE_RE {
                for mat in re.find_iter(line) {
                    let line_num = (i + 1) as i64;
                    let col = mat.start() as i64;
                    results.push(LintResult {
                        file: f.clone(),
                        line: LineNumber::new(line_num),
                        column: ColumnNumber::new(col),
                        code: ErrorCode::raw("AES024"),
                        message: LintMessage::new(format!(
                            "`Any` type annotation found in agent orchestrator layer: '{}'.",
                            line.trim()
                        )),
                        source: make_adapter("architecture"),
                        severity: Severity::HIGH,
                        enclosing_scope: None,
                        related_locations:
                            crate::shared_common::taxonomy_lint_vo::LocationList::new(),
                    });
                }
            }
        }
    }
}
