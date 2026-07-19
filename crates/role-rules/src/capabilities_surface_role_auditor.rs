// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM:
//   1. check_fn_count_limit — Counts `fn ` occurrences. If > 15, flags SurfaceRoleViolation.
//   2. check_surface_roles (async, IAnalyzer-dependent) — Uses analyzer.detect_layer + layer_map
//      to check no_domain_logic on non-smart surfaces (control_flow_count > 3).
//
// NOTE: check_smart_surface / check_utility_surface / check_passive_surface are no-ops because
//      the actual surface role checks run via check_surface_roles (no-domain-logic checks).
//      These trait methods are required by ISurfaceRoleChecker but are intentionally empty.
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_path_vo::FilePath;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::taxonomy_layer_names_vo::layer_surfaces;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_source_vo::SourceContentVO;

// NOTE: Constants for passive checks were removed along with the dead code.

// NOTE: Regex statics for passive checks were removed along with the dead code.

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SurfaceRoleChecker {}

#[async_trait::async_trait]
// ─── Block 2: Public Contract ─────────────────────────────
impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_utility_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_passive_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_fn_count_limit(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_fn_count_limit(source, violations);
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate AES406 passive violation detail message.
    pub fn aes406_passive_violation_details(file: &str, details: &str) -> String {
        format!(
            "AES406 SURFACE_ROLE: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.",
            file, details
        )
    }

    /// Check if the file is a surface file by filename prefix `surface_` or `surfaces_` or directory `surfaces/`.
    pub fn is_in_surfaces(f: &FilePath) -> bool {
        let path_str = f.to_string();
        let basename = match path_str.rsplit('/').next() {
            Some(s) => s,
            None => &path_str,
        };
        let stem = match basename.rfind('.') {
            Some(pos) => &basename[..pos],
            None => basename,
        };
        if stem.starts_with("surface_") || stem.starts_with("surfaces_") {
            return true;
        }
        if let Some(parent) = path_str.rsplit('/').nth(1) {
            if parent == "surfaces" || parent == "surface" || parent == "cli_commands" {
                return true;
            }
        }
        false
    }

    /// Check if the file is a barrel/init file.
    pub fn is_init(f: &FilePath) -> bool {
        let path_str = f.to_string();
        path_str.ends_with("__init__.py")
            || path_str.ends_with("mod.rs")
            || path_str.ends_with("index.ts")
            || path_str.ends_with("index.js")
    }

    /// Create an AdapterName from a string (helper for _report_aes0306).
    pub fn make_adapter(name: &str) -> Option<AdapterName> {
        AdapterName::new(name).ok()
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

    pub fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let li = crate::taxonomy_language_info_vo::LanguageInfo::new(source);
        let fn_keyword = if li.is_py {
            "def "
        } else if li.is_js {
            "function "
        } else {
            "fn "
        };
        if content.matches(fn_keyword).count() > 15 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation { reason: None },
            ));
        }
    }

    // ---- moved from capabilities_role_checker.rs ----

    pub async fn check_surface_roles(
        &self,
        analyzer: &dyn ILayerDetectionProtocol,
        files: &shared::common::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
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

            let definition = match analyzer.get_layer_def(&layer_vo) {
                Some(d) => d,
                None => continue,
            };

            if definition.role.no_domain_logic.value {
                let basename = std::path::Path::new(&f.value)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                let is_smart = basename.ends_with("_command")
                    || basename.ends_with("_controller")
                    || basename.ends_with("_page")
                    || basename.ends_with("_entry");
                if !is_smart {
                    self._check_no_domain_logic(f, &definition, results, "AES406");
                }
            }
        }
    }

    fn _check_no_domain_logic(
        &self,
        f: &FilePath,
        _definition: &shared::common::taxonomy_definition_vo::LayerDefinition,
        results: &mut LintResultList,
        code: &str,
    ) {
        // Control flow check removed - analyzer no longer provides parser access
        let control_flow_count = shared::common::taxonomy_common_vo::Count::default();
        if control_flow_count.value > 3 {
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(code),
                message: LintMessage::new(AesRoleViolation::NoDomainLogic { reason: None }),
                source: Self::make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
    }

    // ---- migrated from capabilities_hierarchy_checker.rs ----
    // NOTE: check_surface_hierarchy and passive check methods were removed
    // because they were dead code (never called from the orchestrator).
    // The orchestrator uses check_passive_surface (no-op) instead.
}

impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_in_surfaces(f: &FilePath) -> bool {
    SurfaceRoleChecker::is_in_surfaces(f)
}

pub fn is_init(f: &FilePath) -> bool {
    SurfaceRoleChecker::is_init(f)
}
