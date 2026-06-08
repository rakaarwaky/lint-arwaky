// arch_role_checker — Architectural role checks (agent and surface roles).
// 1:1 Rust implementation matching capabilities/arch_role_checker.py

use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

use crate::contract::architecture_rule_protocol::{IAnalyzer, IRoleCheckerProtocol};
use crate::taxonomy::layer_names_vo::{
    core_layer_names, layer_agent, layer_contract, layer_infrastructure, layer_surfaces,
    layer_taxonomy,
};
use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, FilePathList, LayerDefinition, LayerNameVO,
    LineNumber, LintMessage, LintResult, LintResultList, Severity, SymbolName,
};

fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}

pub struct ArchRoleChecker {}

impl ArchRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn check_agent_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::taxonomy::FilePathList,
        root_dir: &FilePath,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        for f in &files.values {
            self._check_agent_role_on_file(f, analyzer, root_dir, results)
                .await;
        }
    }

    async fn _check_agent_role_on_file(
        &self,
        f: &FilePath,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        let layer_vo = match analyzer.detect_layer(f, root_dir) {
            Some(l) => l,
            None => return,
        };

        let is_agent = layer_vo == layer_agent()
            || layer_vo
                .value
                .starts_with(&format!("{}(", layer_agent().value));
        if !is_agent {
            return;
        }

        let definition = match analyzer.layer_map().values.get(&layer_vo) {
            Some(d) => d.clone(),
            None => return,
        };

        let file_str = f.to_string();
        let basename = Path::new(file_str.as_str())
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if definition.exceptions.values.iter().any(|e| e == basename) {
            return;
        }

        self._apply_agent_role_checks(f, &definition, analyzer, results)
            .await;
    }

    async fn _apply_agent_role_checks(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        if definition.stateless_execution.value {
            self._check_stateless_execution(f, definition, analyzer, results);
        }

        if definition.high_level_policy_only.value {
            self._check_high_level_policy_only(f, definition, analyzer, results);
        }

        if definition.coordinates_multiple_orchestrators.value {
            self._check_coordinates_multiple_orchestrators(f, definition, analyzer, results);
        }

        if definition.no_domain_logic.value {
            self._check_no_domain_logic(f, definition, analyzer, results, "AES021");
        }

        if definition.must_implement_service_container_aggregate.value {
            self._check_must_implement_contract_lazy(f, definition, analyzer, results);
        }

        if definition.lazy_eager_initialization_only.value {
            self._check_lazy_eager_init_only(f, definition, analyzer, results);
        }

        if definition.forbid_any_type.value {
            self._check_forbid_any_type(f, definition, analyzer, results);
        }
    }

    fn _check_must_implement_contract_lazy(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
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

    pub async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::taxonomy::FilePathList,
        root_dir: &FilePath,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        for f in &files.values {
            let layer_vo = match analyzer.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let is_surface = layer_vo == layer_surfaces()
                || layer_vo
                    .value
                    .starts_with(&format!("{}(", layer_surfaces().value));
            if !is_surface {
                continue;
            }

            let definition = match analyzer.layer_map().values.get(&layer_vo) {
                Some(d) => d.clone(),
                None => continue,
            };

            if definition.no_domain_logic.value {
                self._check_no_domain_logic(f, &definition, analyzer, results, "AES022");
            }

            self._check_forbidden_mandatory_imports(f, &definition, analyzer, results);
        }
    }

    fn _check_forbidden_mandatory_imports(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        let file_str = f.to_string();
        let basename = Path::new(file_str.as_str())
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if definition.exceptions.values.iter().any(|e| e == basename) {
            return;
        }

        let imports = match analyzer.parser().extract_imports(f) {
            Ok(imp) => imp,
            Err(_) => return,
        };

        for imp in imports.values {
            let module_str = &imp.module;
            if self._is_builtin_or_stdlib_import(module_str) {
                continue;
            }

            let module_fp = FilePath::new(module_str.clone())
                .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
            let target_layer = match analyzer.detect_module_layer(&module_fp) {
                Some(l) => l,
                None => continue,
            };

            if target_layer == layer_contract() {
                continue;
            }
            if self._is_smart_surface_allowed_layer(&target_layer) {
                continue;
            }

            self._report_surface_dependency_violation(f, &imp, &target_layer, results);
        }
    }

    fn _is_builtin_or_stdlib_import(&self, module_str: &str) -> bool {
        let known = core_layer_names();
        !module_str.contains('.') && !known.contains(module_str)
    }

    fn _is_smart_surface_allowed_layer(&self, layer_vo: &LayerNameVO) -> bool {
        let layer_str = &layer_vo.value;
        let allowed_bases = [
            layer_taxonomy().value,
            layer_agent().value,
            layer_surfaces().value,
        ];
        if allowed_bases.iter().any(|b| b == layer_str) {
            return true;
        }
        allowed_bases
            .iter()
            .any(|b| layer_str.starts_with(&format!("{}(", b)))
    }

    fn _report_surface_dependency_violation(
        &self,
        f: &FilePath,
        imp: &crate::taxonomy::ImportInfo,
        target_layer: &LayerNameVO,
        results: &mut crate::taxonomy::LintResultList,
    ) {
        results.push(LintResult {
            file: f.clone(),
            line: imp.line.clone(),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES023"),
            message: LintMessage::new(format!(
                "SURFACE DEPENDENCY VIOLATION: Surface layer is only allowed to import from 'contract' and 'taxonomy'. Found import from '{}'.",
                target_layer.value
            )),
            source: make_adapter("architecture"),
            severity: Severity::HIGH,
            enclosing_scope: None,
            related_locations: crate::taxonomy::LocationList::new(),
        });
    }

    fn _check_stateless_execution(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
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
                        related_locations: crate::taxonomy::LocationList::new(),
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
        results: &mut crate::taxonomy::LintResultList,
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
                    related_locations: crate::taxonomy::LocationList::new(),
                });
            }
        }
    }

    fn _check_coordinates_multiple_orchestrators(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
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
                        related_locations: crate::taxonomy::LocationList::new(),
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
        results: &mut crate::taxonomy::LintResultList,
        code: &str,
    ) {
        let control_flow_count = analyzer.parser().get_control_flow_count(f);
        if control_flow_count.value > 3 {
            let default_msg = "Complex domain logic detected in a passive layer/role.".to_string();
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
                related_locations: crate::taxonomy::LocationList::new(),
            });
        }
    }

    fn _check_lazy_eager_init_only(
        &self,
        f: &FilePath,
        definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::taxonomy::LintResultList,
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
                        related_locations: crate::taxonomy::LocationList::new(),
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
        results: &mut crate::taxonomy::LintResultList,
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
                    related_locations: crate::taxonomy::LocationList::new(),
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
        results: &mut crate::taxonomy::LintResultList,
    ) {
        let content = match std::fs::read_to_string(f.to_string().as_str()) {
            Ok(c) => c,
            Err(_) => return,
        };

        static ANY_TYPE_RE: Lazy<Option<Regex>> = Lazy::new(|| {
            Regex::new(r":\s*[Aa]ny\b|->\s*[Aa]ny\b|\b[Aa]ny\s*\[").ok()
        });

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
                        related_locations: crate::taxonomy::LocationList::new(),
                    });
                }
            }
        }
    }

    fn _extract_json_array(value: &Option<serde_json::Value>, key: &str) -> Vec<serde_json::Value> {
        value
            .as_ref()
            .and_then(|v| v.get(key))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default()
    }
}

#[async_trait]
impl IRoleCheckerProtocol for ArchRoleChecker {
    async fn check_agent_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        self.check_agent_roles(analyzer, files, root_dir, results)
            .await;
    }

    async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        self.check_surface_roles(analyzer, files, root_dir, results)
            .await;
    }
}
