use std::path::Path;

use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_names_vo::{
    core_layer_names, layer_agent, layer_contract, layer_surfaces, layer_taxonomy,
};
use crate::shared_common::taxonomy_violationrs_constant::{
    AES021_NO_DOMAIN_LOGIC, AES023_SURFACE_DEPENDENCY,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;

fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}

pub struct SurfaceRoleChecker {}
impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
    pub fn check_smart(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_utility(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_passive(&self) -> Vec<LintResult> {
        vec![]
    }

    // ---- moved from capabilities_role_checker.rs ----

    pub async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &crate::source_parsing::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
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
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
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
        imp: &crate::code_analysis::taxonomy_source_vo::ImportInfo,
        target_layer: &LayerNameVO,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
    ) {
        results.push(LintResult {
            file: f.clone(),
            line: imp.line.clone(),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES023"),
            message: LintMessage::new(format!(
                "{} Surface layer is only allowed to import from 'contract' and 'taxonomy'. Found import from '{}'.",
                AES023_SURFACE_DEPENDENCY, target_layer.value
            )),
            source: make_adapter("architecture"),
            severity: Severity::HIGH,
            enclosing_scope: None,
            related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
        });
    }

    fn _check_no_domain_logic(
        &self,
        f: &FilePath,
        _definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut crate::output_report::taxonomy_result_vo::LintResultList,
        code: &str,
    ) {
        let control_flow_count = analyzer.parser().get_control_flow_count(f);
        if control_flow_count.value > 3 {
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(code),
                message: LintMessage::new(AES021_NO_DOMAIN_LOGIC),
                source: make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
            });
        }
    }
}
