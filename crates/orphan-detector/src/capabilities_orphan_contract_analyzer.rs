use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::{
    IContractOrphanProtocol, IOrphanFileCachePort, IOrphanFilenameExtractorProtocol,
};
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::collections::HashMap;
use std::sync::Arc;

use shared::orphan_detector::taxonomy_contract_detection_utility::{
    extract_contract_trait_name, has_py_call, has_py_impl, has_py_wire, has_rust_call,
    has_rust_impl, has_rust_wire, has_ts_call, has_ts_impl, has_ts_wire,
};
use shared::orphan_detector::taxonomy_contract_regex_utility::word_boundary_re;

// ═══════════════════════════════════════════════════════════════════════════════
// CONSTANTS
// ═══════════════════════════════════════════════════════════════════════════════

const SUFFIX_PORT: &str = "port";
const SUFFIX_PROTOCOL: &str = "protocol";
const SUFFIX_AGGREGATE: &str = "aggregate";
const LAYER_INFRASTRUCTURE: &str = "infrastructure";
const LAYER_CAPABILITIES: &str = "capabilities";
const LAYER_AGENT: &str = "agent";

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN STRUCT
// ═══════════════════════════════════════════════════════════════════════════════

// ─── Block 1: Struct Definition ───────────────────────────
pub struct ContractOrphanAnalyzer {
    extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
    cache: Arc<dyn IOrphanFileCachePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl IContractOrphanProtocol for ContractOrphanAnalyzer {
    fn is_contract_orphan(
        &self,
        f: &FilePath,
        root_dir: &FilePath,
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        let search_files = build_search_files(all_files, root_dir, self.cache.as_ref());
        let mut contents: HashMap<String, String> = HashMap::new();
        let mut basenames: HashMap<String, String> = HashMap::new();

        for path in &search_files {
            if contents.contains_key(path) {
                continue;
            }
            let fp = FilePath {
                value: path.clone(),
            };
            let content = self.cache.read_cached(&fp).value;
            if !content.is_empty() {
                contents.insert(path.clone(), content.clone());
                let basename = self.extractor.file_basename(&fp).value;
                basenames.insert(path.clone(), basename);
            }
        }

        is_contract_orphan(f, &contents, &basenames, self.extractor.as_ref())
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for ContractOrphanAnalyzer {
    fn default() -> Self {
        Self {
            extractor: Arc::new(
                crate::capabilities_orphan_filename_extractor::OrphanFilenameExtractor::new(),
            ),
            cache: Arc::new(crate::infrastructure_file_cache::OrphanFileCache::new()),
        }
    }
}

impl ContractOrphanAnalyzer {
    pub fn new(
        extractor: Arc<dyn IOrphanFilenameExtractorProtocol>,
        cache: Arc<dyn IOrphanFileCachePort>,
    ) -> Self {
        Self { extractor, cache }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ORCHESTRATOR
// ═══════════════════════════════════════════════════════════════════════════════

pub fn is_contract_orphan(
    f: &FilePath,
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    extractor: &dyn IOrphanFilenameExtractorProtocol,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

    let content = match contents.get(fp) {
        Some(c) => c.as_str(),
        None => return not_orphan(),
    };

    let trait_name = match extract_contract_trait_name(content, fp) {
        Some(t) => t,
        None => return not_orphan(),
    };

    let target_prefix = match suffix.as_str() {
        SUFFIX_PORT => LAYER_INFRASTRUCTURE,
        SUFFIX_PROTOCOL => LAYER_CAPABILITIES,
        SUFFIX_AGGREGATE => LAYER_AGENT,
        _ => return not_orphan(),
    };

    if !check_implemented(contents, basenames, &trait_name, target_prefix) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not implemented by any {} file.",
                suffix, trait_name, target_prefix
            ),
        );
    }

    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL)
        && !check_called(contents, basenames, &trait_name)
    {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not called by any orchestrator, container, capabilities, or surface file.",
                suffix, trait_name
            ),
        );
    }

    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL)
        && !check_wired(contents, basenames, &trait_name)
    {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract {} '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                suffix, trait_name
            ),
        );
    }

    if suffix == SUFFIX_AGGREGATE && !check_called(contents, basenames, &trait_name) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract aggregate '{}' not called by any surface or container file.",
                trait_name
            ),
        );
    }

    if suffix == SUFFIX_AGGREGATE && !check_wired(contents, basenames, &trait_name) {
        return orphan_result(
            &suffix,
            &trait_name,
            target_prefix,
            &format!(
                "Contract aggregate '{}' not wired in any DI container (no Arc::new, Box::new, or constructor injection).",
                trait_name
            ),
        );
    }

    not_orphan()
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULAR DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

fn check_implemented(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
    target_prefix: &str,
) -> bool {
    let rust_impl_pattern = format!("impl {} for", trait_name);
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_target_layer = bn.starts_with(target_prefix);
        let is_container_impl = bn.starts_with("root_")
            && (bn.ends_with("_container.rs")
                || bn.ends_with("_container.py")
                || bn.ends_with("_container.ts")
                || bn.ends_with("_container.js"));
        if !is_target_layer && !is_container_impl {
            continue;
        }
        if has_rust_impl(content, &rust_impl_pattern, &re_trait)
            || has_py_impl(content, trait_name)
            || has_ts_impl(content, trait_name)
        {
            return true;
        }
    }
    false
}

fn check_called(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_relevant = bn.starts_with("agent_")
            || bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js")
            || bn.starts_with("capabilities_")
            || bn.starts_with("surface_");

        if !is_relevant {
            continue;
        }
        if has_rust_call(content, &re_trait)
            || has_py_call(content, &re_trait)
            || has_ts_call(content, &re_trait)
        {
            return true;
        }
    }
    false
}

fn check_wired(
    contents: &HashMap<String, String>,
    basenames: &HashMap<String, String>,
    trait_name: &str,
) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in contents {
        let bn = match basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_relevant = bn.starts_with("agent_")
            || bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js")
            || bn.starts_with("capabilities_")
            || bn.starts_with("surface_");

        if !is_relevant {
            continue;
        }
        if has_rust_wire(content, &re_trait)
            || has_py_wire(content, &re_trait)
            || has_ts_wire(content, &re_trait)
        {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn not_orphan() -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

fn orphan_result(
    suffix: &str,
    trait_name: &str,
    target_prefix: &str,
    reason: &str,
) -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(
        true,
        AesOrphanViolation::ContractOrphan {
            suffix: suffix.to_string(),
            trait_name: trait_name.to_string(),
            target_layer: target_prefix.to_string(),
            reason: Some(reason.to_string().into()),
        }
        .to_string(),
        Severity::LOW,
    )
}

fn build_search_files(
    all_files: &[FilePath],
    root_dir: &FilePath,
    cache: &dyn IOrphanFileCachePort,
) -> Vec<String> {
    let mut search_files: Vec<String> = all_files.iter().map(|fp| fp.value().to_string()).collect();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_source_files(&ws_path, &mut search_files, cache);
        }
    }
    search_files
}

fn collect_source_files(
    dir: &std::path::Path,
    files: &mut Vec<String>,
    cache: &dyn IOrphanFileCachePort,
) {
    let dir_str = dir.to_str().unwrap_or("");
    if cache.is_symlink(dir_str) {
        return;
    }

    let entries = cache.read_dir(dir_str);
    for entry_path in &entries {
        let path = std::path::Path::new(entry_path);
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            collect_source_files(path, files, cache);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx") {
                files.push(entry_path.clone());
            }
        }
    }
}
