use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::sync::Arc;
use std::sync::OnceLock;

pub struct ContractOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
}

impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self {
            extractor: Arc::new(
                crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new(),
            ),
        }
    }
}

impl ContractOrphanAnalyzer {
    pub fn new(extractor: Arc<dyn IOrphanFilenameExtractorProtocol>) -> Self {
        Self { extractor }
    }
}

impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        file_definitions: &FileDefinitionMap,
        inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        is_contract_orphan(
            f,
            root_dir,
            file_definitions,
            inheritance_map,
            all_files,
            &self.extractor,
        )
    }
}

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    _file_definitions: &FileDefinitionMap,
    _inheritance_map: &InheritanceMap,
    all_files: &[FilePath],
    extractor: &Arc<dyn IOrphanFilenameExtractorProtocol>,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

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
    let mut search_files: Vec<String> = all_files.iter().map(|fp| fp.value().to_string()).collect();
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
        let cb = extractor
            .file_basename(&shared::common::taxonomy_path_vo::FilePath { value: cf.clone() })
            .value;
        // Check target layer files (infrastructure_ for ports, capabilities_ for protocols, agent_ for aggregates)
        // Also check root_*_container files (DI wiring often implements traits there)
        let is_target_layer = cb.starts_with(target_prefix);
        let is_container_impl = cb.starts_with("root_") && cb.ends_with("_container.rs");
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(cf) {
            let has_rust_impl = c.contains(&format!("impl {} for", trait_name))
                || c.lines().any(|ln| {
                    let t = ln.trim();
                    t.starts_with("impl") && t.contains(&trait_name) && t.contains(" for")
                });

            // Python: class MyClass(TraitName): or class MyClass(Base, TraitName):
            let py_pattern = format!(
                r"class\s+\w+\([^)]*\b{}\b[^)]*\)",
                regex::escape(&trait_name)
            );
            let has_py_impl = regex::Regex::new(&py_pattern)
                .map(|re| re.is_match(&c))
                .unwrap_or(false);

            // TypeScript: class MyClass implements TraitName or extends TraitName
            let has_ts_impl = c.contains(&format!("implements {}", trait_name))
                || c.contains(&format!("extends {}", trait_name));

            if has_rust_impl || has_py_impl || has_ts_impl {
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
            let cb = extractor
                .file_basename(&shared::common::taxonomy_path_vo::FilePath { value: cf.clone() })
                .value;
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
            let cb = extractor
                .file_basename(&shared::common::taxonomy_path_vo::FilePath { value: cf.clone() })
                .value;
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
