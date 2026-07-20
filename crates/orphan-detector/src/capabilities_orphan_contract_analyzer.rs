use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::utility_orphan_filename::{file_basename, file_suffix};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::OnceLock;

// PURPOSE: ContractOrphanAnalyzer — IContractOrphanProtocol for orphan contract detection
use regex::Regex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ContractOrphanAnalyzer {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[String],
    ) -> OrphanIndicatorResult {
        is_contract_orphan(f, root_dir, file_definitions, inheritance_map, all_files)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractOrphanAnalyzer {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[String],
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = file_suffix(fp);

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let trait_name = extract_contract_trait_name(&content);
    let trait_name = match trait_name {
        Some(t) => t,
        None => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    // Build search_files: combine scan-directory files with all workspace .rs files
    let mut search_files: Vec<String> = all_files.to_vec();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_source_files(&ws_path, &mut search_files);
        }
    }

    // Check 1: contract not implemented by expected layer
    let target_prefix = match suffix.as_str() {
        "port" => "infrastructure",
        "protocol" => "capabilities",
        "aggregate" => "agent",
        _ => return OrphanIndicatorResult::new(false, String::new(), Severity::LOW),
    };

    let mut has_impl = false;
    for cf in &search_files {
        let cb = file_basename(cf);
        // Check target layer files (capabilities_ for ports, capabilities_ for protocols, agent_ for aggregates)
        // Also check root_*_container files (DI wiring often implements traits there)
        let is_target_layer = cb.starts_with(target_prefix);
        let is_container_impl = cb.starts_with("root_") && cb.ends_with("_container.rs");
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            if c.contains(&format!("impl {} for", trait_name))
                || c.lines().any(|ln| {
                    let t = ln.trim();
                    t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
                })
                || c.contains(&format!("class {}(\\(", trait_name))
                || c.contains(&format!("class {} ", trait_name))
                || c.contains(&format!("class {}:", trait_name))
            {
                has_impl = true;
                break;
            }
        }
    }

    if !has_impl {
        return OrphanIndicatorResult::new(
            true,
            AesOrphanViolation::ContractOrphan {
                suffix: suffix.clone(),
                trait_name: trait_name.clone(),
                target_layer: target_prefix,
                reason: Some(
                    format!(
                        "Contract {} '{}' not implemented by any {} file.",
                        suffix, trait_name, target_prefix
                    )
                    .into(),
                ),
            }
            .to_string(),
            Severity::LOW,
        );
    }

    // Check 2: port/protocol not called by any orchestrator, container, capabilities, or surface
    if suffix == "port" || suffix == "protocol" {
        let mut called_by_impl_or_user = false;
        for cf in &search_files {
            let cb = file_basename(cf);
            // Check orchestrator files
            let is_orchestrator = cb.starts_with("agent_")
                && (cb.ends_with("_orchestrator.rs")
                    || cb.ends_with("_orchestrator.py")
                    || cb.ends_with("_orchestrator.ts")
                    || cb.ends_with("_orchestrator.js"));
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs")
                || cb.ends_with("_container.py")
                || cb.ends_with("_container.ts")
                || cb.ends_with("_container.js");
            // Check capabilities files (trait implementations)
            let is_capabilities = cb.starts_with("capabilities_");
            // Check surface files (trait usage)
            let is_surface = cb.starts_with("surface_");

            if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_impl_or_user = true;
                    break;
                }
            }
        }
        if !called_by_impl_or_user {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract {} '{}' not called by any orchestrator or container.",
                            suffix, trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }
    }

    // Check 3: aggregate not called by any surface OR container
    if suffix == "aggregate" {
        let mut called_by_surface_or_container = false;
        for cf in &search_files {
            let cb = file_basename(cf);
            // Check surface files
            let is_surface = cb.starts_with("surface_");
            // Check container files (DI wiring)
            let is_container = cb.ends_with("_container.rs")
                || cb.ends_with("_container.py")
                || cb.ends_with("_container.ts")
                || cb.ends_with("_container.js");

            if !is_surface && !is_container {
                continue;
            }
            if let Ok(c) = std::fs::read_to_string(cf) {
                if c.contains(&trait_name) {
                    called_by_surface_or_container = true;
                    break;
                }
            }
        }
        if !called_by_surface_or_container {
            return OrphanIndicatorResult::new(
                true,
                AesOrphanViolation::ContractOrphan {
                    suffix: suffix.clone(),
                    trait_name: trait_name.clone(),
                    target_layer: target_prefix,
                    reason: Some(
                        format!(
                            "Contract aggregate '{}' not called by any surface or container.",
                            trait_name
                        )
                        .into(),
                    ),
                }
                .to_string(),
                Severity::LOW,
            );
        }
    }

    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

fn re_contract_rust() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_contract_py() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"class\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_ts_interface_export() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_interface() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

pub fn extract_contract_trait_name(content: &str) -> Option<String> {
    // Skip comment lines to avoid matching "trait for" in comments
    let code_lines: String = content
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("//") && !t.starts_with("/*") && !t.starts_with("*")
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Some(re) = re_contract_rust() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    if let Some(re) = re_ts_interface_export() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    if let Some(re) = re_interface() {
        if let Some(caps) = re.captures(&code_lines) {
            return Some(caps[1].to_string());
        }
    }
    re_contract_py()
        .and_then(|re| re.captures(&code_lines))
        .map(|caps| caps[1].to_string())
}

fn collect_source_files(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                collect_source_files(&path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                    if let Some(s) = path.to_str() {
                        files.push(s.to_string());
                    }
                }
            }
        }
    }
}

