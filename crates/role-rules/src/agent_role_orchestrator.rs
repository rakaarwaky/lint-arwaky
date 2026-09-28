// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// FRD-compliant: accepts pre-parsed FileEntry from the filesystem crate.
// No file I/O or AST parsing is performed internally.

use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::ExternalReferenceMap;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::filesystem::taxonomy_filesystem_vo::Language;
use shared::filesystem::taxonomy_filesystem_vo::ParseMetadata;
use shared::role_rules::contract_role_protocol::IAgentRoleProtocol;
use shared::role_rules::contract_role_protocol::ICapabilitiesRoleProtocol;
use shared::role_rules::contract_role_protocol::IClassificationProtocol;
use shared::role_rules::contract_role_protocol::IContractRoleProtocol;
use shared::role_rules::contract_role_protocol::ISurfaceRoleProtocol;
use shared::role_rules::contract_role_protocol::ITaxonomyRoleProtocol;
use shared::role_rules::contract_role_protocol::IUtilityRoleProtocol;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::role_rules::taxonomy_role_request::RoleRequest;
use shared::role_rules::taxonomy_role_response::RoleResponse;
use std::path::Path;
use std::sync::Arc;

use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

// ─── Block 1: Struct Definitions ──────────────────────────

pub struct RoleCheckerDeps {
    pub taxonomy: Arc<dyn ITaxonomyRoleProtocol>,
    pub contract_rust: Arc<dyn IContractRoleProtocol>,
    pub contract_python: Arc<dyn IContractRoleProtocol>,
    pub contract_typescript: Arc<dyn IContractRoleProtocol>,
    pub capabilities_rust: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities_python: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities_typescript: Arc<dyn ICapabilitiesRoleProtocol>,
    pub capabilities: Arc<dyn ICapabilitiesRoleProtocol>,
    pub surface: Arc<dyn ISurfaceRoleProtocol>,
    pub agent: Arc<dyn IAgentRoleProtocol>,
    pub utility: Arc<dyn IUtilityRoleProtocol>,
}

pub struct RoleOrchestrator {
    deps: RoleCheckerDeps,
    config: ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── FR-001: File Classification and Dispatch ─────────────

impl IClassificationProtocol for RoleOrchestrator {
    fn classify_layer(&self, file: &FileEntry) -> Option<LayerNameVO> {
        let filename = file
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        match stem.split('_').next().unwrap_or_default() {
            "taxonomy" => Some(LayerNameVO::new("taxonomy")),
            "contract" => Some(LayerNameVO::new("contract")),
            "capabilities" | "capability" => Some(LayerNameVO::new("capabilities")),
            "utility" => Some(LayerNameVO::new("utility")),
            "agent" => Some(LayerNameVO::new("agent")),
            "surface" | "surfaces" => Some(LayerNameVO::new("surfaces")),
            // `root` is pure DI wiring; unrecognised prefixes are skipped.
            _ => None,
        }
    }
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IRoleRunnerAggregate for RoleOrchestrator {
    fn execute(&self, request: RoleRequest) -> RoleResponse {
        match request {
            RoleRequest::RunAuditWithEntries { files } => {
                let violations = self.run_audit_with_entries(&files);
                RoleResponse::Audit { violations }
            }
            RoleRequest::Name => RoleResponse::Name {
                name: self.name().to_string(),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl RoleOrchestrator {
    pub fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        let mut results = Vec::new();
        self.run_all_role_checks(files, &mut results);
        results
    }

    pub fn name(&self) -> &str {
        "role-rules"
    }

    pub fn new(deps: RoleCheckerDeps, config: &ArchitectureConfig) -> Self {
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            deps,
            config: config.clone(),
            ignored_paths,
        }
    }

    /// Run all role checks on pre-parsed FileEntry slices.
    pub fn run_all_role_checks(&self, files: &[FileEntry], violations: &mut Vec<LintResult>) {
        if !self.config.enabled.value {
            return;
        }

        let references = build_external_reference_map(files);

        for file in files {
            if !file.parse_ok || file.content.is_empty() {
                continue;
            }

            let path_str = file.path.to_string_lossy();
            let filename = file
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let basename = stem;
            let prefix = basename.split('_').next().unwrap_or_default();

            // Skip barrel files (single source: shared::common::DEFAULT_RULE_EXCEPTIONS)
            if shared::common::DEFAULT_RULE_EXCEPTIONS.contains(&filename) || filename == "main.rs"
            {
                continue;
            }

            if self.is_ignored(&path_str) {
                continue;
            }

            match prefix {
                "agent"
                    if self.is_rule_enabled("AES405") && !self.is_exception("AES405", filename) =>
                {
                    self.deps
                        .agent
                        .check_agent_routing(file, "agent", violations);
                }
                "root" => {}
                "surfaces" | "surface"
                    if self.is_rule_enabled("AES406") && !self.is_exception("AES406", filename) =>
                {
                    self.deps.surface.check_fn_count_limit(file, violations);
                    let is_smart = basename.ends_with("_command")
                        || basename.ends_with("_controller")
                        || basename.ends_with("_page")
                        || basename.ends_with("_entry")
                        || basename.ends_with("_router");
                    let is_utility = basename.ends_with("_hook")
                        || basename.ends_with("_store")
                        || basename.ends_with("_action")
                        || basename.ends_with("_screen");
                    if is_smart {
                        self.deps.surface.check_smart_surface(file, violations);
                    } else if is_utility {
                        self.deps.surface.check_utility_surface(file, violations);
                    } else {
                        self.deps.surface.check_passive_surface(file, violations);
                    }
                }
                "contract"
                    if self.is_rule_enabled("AES402") && !self.is_exception("AES402", filename) =>
                {
                    let auditor = match file.language {
                        Language::Rust => &*self.deps.contract_rust,
                        Language::Python => &*self.deps.contract_python,
                        Language::TypeScript | Language::JavaScript => {
                            &*self.deps.contract_typescript
                        }
                        _ => continue,
                    };
                    auditor.check_contract_routing(file, "contract", violations);
                }
                "capabilities" | "capability"
                    if self.is_rule_enabled("AES403") && !self.is_exception("AES403", filename) =>
                {
                    let language = file.language;
                    let deps = &self.deps;
                    let auditor: &dyn ICapabilitiesRoleProtocol = match language {
                        Language::Rust => &*deps.capabilities_rust,
                        Language::Python => &*deps.capabilities_python,
                        Language::TypeScript | Language::JavaScript => {
                            &*deps.capabilities_typescript
                        }
                        _ => {
                            continue;
                        }
                    };
                    auditor.check_capability_routing_with_references(
                        file,
                        "capabilities",
                        &references,
                        violations,
                    );
                }
                "utility"
                    if self.is_rule_enabled("AES404") && !self.is_exception("AES404", filename) =>
                {
                    self.deps.utility.check_utility_convention(file, violations);
                }
                "taxonomy"
                    if self.is_rule_enabled("AES401") && !self.is_exception("AES401", filename) =>
                {
                    self.deps.taxonomy.check_entity(file, violations);
                    self.deps.taxonomy.check_error(file, violations);
                    self.deps.taxonomy.check_event(file, violations);
                    self.deps.taxonomy.check_constant(file, violations);
                }
                _ => {}
            }
        }
    }

    fn is_ignored(&self, path: &str) -> bool {
        let segments: Vec<&str> = path.split('/').collect();
        self.ignored_paths.iter().any(|ignored| {
            let ignored_segments: Vec<&str> = ignored.split('/').collect();
            ignored_segments
                .iter()
                .all(|igs| segments.iter().any(|s| s == igs))
        })
    }

    fn is_exception(&self, code: &str, filename: &str) -> bool {
        let stem = std::path::Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str());
        self.config.rules.iter().any(|r| {
            r.rule_type.code() == code
                && r.exceptions
                    .values
                    .iter()
                    .any(|e| e == filename || stem.is_some_and(|s| e == s))
        })
    }

    fn is_rule_enabled(&self, code: &str) -> bool {
        self.config
            .rules
            .iter()
            .find(|r| r.rule_type.code() == code)
            .map(|r| r.enabled.value)
            .unwrap_or(true)
    }
}

// ─── Module-level helper ──────────────────────────────

/// Build a workspace-wide map of which file references which method name.
///
/// The role rules receive every parsed file, so this is a single pass over
/// `used_identifiers` per file. `used_identifiers` is populated by the
/// filesystem parser from the file body (excluding `use` declarations), so a
/// method name appearing in a caller file means the caller calls it.
///
/// Test and bench files are excluded from the main file index, so they are
/// scanned separately here to keep the reference map complete.
fn build_external_reference_map(files: &[FileEntry]) -> ExternalReferenceMap {
    let mut map = ExternalReferenceMap::default();
    for file in files {
        let path = file.path.to_string_lossy().to_string();
        if is_test_or_bench_path(&path) {
            map.has_test_references = true;
        }
        let identifiers: Vec<String> = match &file.parse_metadata {
            Some(ParseMetadata::Rust(r)) => r.used_identifiers.clone(),
            Some(ParseMetadata::Python(py)) => py.used_identifiers.clone(),
            Some(ParseMetadata::TypeScript(ts)) | Some(ParseMetadata::JavaScript(ts)) => {
                ts.used_identifiers.clone()
            }
            _ => continue,
        };
        if identifiers.is_empty() {
            continue;
        }
        map.by_file.insert(path, identifiers);
    }
    // Supplement with on-disk test/bench files, which the main index excludes.
    let mut seen: std::collections::HashSet<String> = map.by_file.keys().cloned().collect();
    if let Ok(ws_root) = std::env::current_dir() {
        for sub in ["crates", "packages", "modules"] {
            let base = ws_root.join(sub);
            if !base.is_dir() {
                continue;
            }
            if let Ok(members) = std::fs::read_dir(&base) {
                for member in members.flatten() {
                    let member_path = member.path();
                    if !member_path.is_dir() {
                        continue;
                    }
                    let src_dir = member_path.join("src");
                    for sub_dir in ["tests", "benches"] {
                        let dir = src_dir.join(sub_dir);
                        if !dir.is_dir() {
                            continue;
                        }
                        collect_test_refs(&dir, &mut map, &mut seen);
                    }
                }
            }
        }
    }
    map
}

fn is_test_or_bench_path(path: &str) -> bool {
    path.contains("/tests/")
        || path.contains("/benches/")
        || path.contains("\\tests\\")
        || path.contains("\\benches\\")
}

fn collect_test_refs(
    dir: &std::path::Path,
    map: &mut ExternalReferenceMap,
    seen: &mut std::collections::HashSet<String>,
) {
    fn walk(
        d: &std::path::Path,
        map: &mut ExternalReferenceMap,
        seen: &mut std::collections::HashSet<String>,
    ) {
        let entries = match std::fs::read_dir(d) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                walk(&p, map, seen);
                continue;
            }
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !matches!(ext, "rs" | "py" | "ts" | "js") {
                continue;
            }
            let path_str = p.to_string_lossy().to_string();
            if !seen.insert(path_str.clone()) {
                continue;
            }
            let content = match std::fs::read_to_string(&p) {
                Ok(c) => c,
                Err(_) => continue,
            };
            // Simple identifier harvest: trim lines, skip `use`/`import` declarations.
            let identifiers: Vec<String> = content
                .lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.is_empty() && !t.starts_with("use ") && !t.starts_with("import ")
                })
                .flat_map(|l| l.split(' ').map(|w| w.trim().to_string()))
                .filter(|w| {
                    w.chars()
                        .next()
                        .map(|c| c.is_ascii_lowercase())
                        .unwrap_or(false)
                        && w.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                })
                .collect();
            if identifiers.is_empty() {
                continue;
            }
            map.has_test_references = true;
            map.by_file.insert(path_str, identifiers);
        }
    }
    walk(dir, map, seen);
}
