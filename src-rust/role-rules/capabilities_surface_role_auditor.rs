// aes: wired-by-dispatch
// PURPOSE: AES0306 — Enforce surface role mandates (no domain logic in passive/utility surfaces).
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_names_vo::{
    layer_surfaces,
};
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_violation_rs_constant::{
    AES0305_NO_DOMAIN_LOGIC,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct SurfaceRoleChecker {}
fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}
impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

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
                let basename = std::path::Path::new(&f.value)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let is_smart = basename.ends_with("_command")
                    || basename.ends_with("_controller")
                    || basename.ends_with("_page")
                    || basename.ends_with("_entry");
                if !is_smart {
                    self._check_no_domain_logic(f, &definition, analyzer, results, "AES0306");
                }
            }

        }
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
                message: LintMessage::new(AES0305_NO_DOMAIN_LOGIC),
                source: make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
            });
        }
    }
}
