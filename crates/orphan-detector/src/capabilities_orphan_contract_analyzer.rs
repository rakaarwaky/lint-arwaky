use regex::Regex;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_analysis_vo::FileDefinitionMap;
use shared::code_analysis::taxonomy_analysis_vo::InheritanceMap;
use shared::code_analysis::taxonomy_analysis_vo::OrphanIndicatorResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::contract_orphan_protocol::IContractOrphanProtocol;
use shared::orphan_detector::contract_orphan_protocol::IOrphanFilenameExtractorProtocol;
use shared::orphan_detector::taxonomy_violation_orphan_vo::AesOrphanViolation;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

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
// STATIC REGEXES (LazyLock)
// ═══════════════════════════════════════════════════════════════════════════════

fn re_contract_rust() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").ok())
        .as_ref()
}

fn re_contract_py() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:class\s+([A-Za-z0-9_]+)\s*\([^)]*ABC[^)]*\)|class\s+([A-Za-z0-9_]+)\s*\([^)]*Protocol[^)]*\))").ok()).as_ref()
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

// ═══════════════════════════════════════════════════════════════════════════════
// FILE CACHE — built ONCE, passed by reference
// ═══════════════════════════════════════════════════════════════════════════════

pub struct FileCache {
    pub contents: HashMap<String, Arc<str>>,
    pub basenames: HashMap<String, String>,
}

impl FileCache {
    pub fn build(
        all_files: &[FilePath],
        root_dir: &FilePath,
        extractor: &dyn IOrphanFilenameExtractorProtocol,
    ) -> Self {
        let search_files = build_search_files(all_files, root_dir);
        let mut contents = HashMap::new();
        let mut basenames = HashMap::new();

        for path in &search_files {
            if contents.contains_key(path) {
                continue;
            }
            match std::fs::read_to_string(path) {
                Ok(c) => {
                    contents.insert(path.clone(), Arc::from(c.as_str()));
                    let basename = extractor
                        .file_basename(&FilePath {
                            value: path.clone(),
                        })
                        .value;
                    basenames.insert(path.clone(), basename);
                }
                Err(e) => {
                    eprintln!("[WARN] Failed to read {}: {}", path, e);
                }
            }
        }
        Self {
            contents,
            basenames,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// STRUCT + TRAIT IMPL
// ═══════════════════════════════════════════════════════════════════════════════

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
        _file_definitions: &FileDefinitionMap,
        _inheritance_map: &InheritanceMap,
        all_files: &[FilePath],
    ) -> OrphanIndicatorResult {
        // #1: Cache built ONCE by caller, not per-file
        let cache = FileCache::build(all_files, root_dir, self.extractor.as_ref());
        is_contract_orphan(f, &cache, self.extractor.as_ref())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ORCHESTRATOR
// ═══════════════════════════════════════════════════════════════════════════════

pub fn is_contract_orphan(
    f: &FilePath,
    cache: &FileCache,
    extractor: &dyn IOrphanFilenameExtractorProtocol,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

    // #6: Use cache instead of reading file again
    let content = match cache.contents.get(fp) {
        Some(c) => c.as_ref(),
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

    // Check 1: trait not implemented
    if !check_implemented(cache, &trait_name, target_prefix) {
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

    // Check 2: port/protocol not called
    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL) && !check_called(cache, &trait_name) {
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

    // Check 3: port/protocol not wired
    if (suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL) && !check_wired(cache, &trait_name) {
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

    // Check 4: aggregate not called
    if suffix == SUFFIX_AGGREGATE && !check_called(cache, &trait_name) {
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

    // Check 5: aggregate not wired
    if suffix == SUFFIX_AGGREGATE && !check_wired(cache, &trait_name) {
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
// MODULAR DETECTION: IMPLEMENT / CALL / WIRE
// ═══════════════════════════════════════════════════════════════════════════════

fn check_implemented(cache: &FileCache, trait_name: &str, target_prefix: &str) -> bool {
    let rust_impl_pattern = format!("impl {} for", trait_name);
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in &cache.contents {
        let bn = match cache.basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_target_layer = bn.starts_with(target_prefix);
        let is_container_impl = bn.starts_with("root_") && bn.ends_with("_container");
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

/// Check if trait is called (imported/used) by any relevant file.
fn check_called(cache: &FileCache, trait_name: &str) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in &cache.contents {
        let bn = match cache.basenames.get(path) {
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

/// Check if trait is wired (DI injected) by any relevant file.
fn check_wired(cache: &FileCache, trait_name: &str) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in &cache.contents {
        let bn = match cache.basenames.get(path) {
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
// RUST DETECTION — per-line, tight patterns
// ═══════════════════════════════════════════════════════════════════════════════

pub fn has_rust_impl(content: &str, rust_impl_pattern: &str, re_trait: &Regex) -> bool {
    content.contains(rust_impl_pattern)
        || content.lines().any(|ln| {
            let t = ln.trim();
            t.starts_with("impl") && re_trait.is_match(t) && t.contains(" for")
        })
}

// #4: Per-line checking — not global content.contains("::")
pub fn has_rust_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
            continue;
        }
        // Specific patterns: use TraitName, TraitName::, <dyn TraitName>, Arc<dyn TraitName>
        if (t.starts_with("use ")
            || t.contains("::")
            || t.contains("<dyn ")
            || t.contains("Arc<dyn "))
            && re_trait.is_match(t)
        {
            return true;
        }
    }
    false
}

// #4: Per-line checking for wire
pub fn has_rust_wire(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') {
            continue;
        }
        if re_trait.is_match(t)
            && (t.contains("Arc::new(") || t.contains("Box::new(") || t.contains("::new("))
        {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// PYTHON DETECTION — per-line, tight patterns
// ═══════════════════════════════════════════════════════════════════════════════

pub fn has_py_impl(content: &str, trait_name: &str) -> bool {
    // #3: Don't compile regex per call — use string matching
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        // class Type(BaseTrait):
        if t.starts_with("class ") && t.contains(trait_name) && t.contains('(') {
            return true;
        }
    }
    false
}

// #5: Unified call detection — not duplicated
pub fn has_py_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        // from module import TraitName OR import TraitName
        if (t.starts_with("from ") || t.starts_with("import ")) && re_trait.is_match(t) {
            return true;
        }
        // instance.method() or Type.method()
        if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
            return true;
        }
    }
    false
}

pub fn has_py_wire(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            continue;
        }
        // TraitName() constructor injection
        if re_trait.is_match(t) && t.contains('(') && !t.starts_with("class ") {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPESCRIPT DETECTION — per-line, tight patterns
// ═══════════════════════════════════════════════════════════════════════════════

// #8: Use word boundary for ts_impl
pub fn has_ts_impl(content: &str, trait_name: &str) -> bool {
    let re = word_boundary_re(trait_name);
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        if (t.contains("implements ") || t.contains("extends ")) && re.is_match(t) {
            return true;
        }
    }
    false
}

// #5: Unified call detection
pub fn has_ts_call(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        // import { TraitName } from '...'
        if t.starts_with("import ") && re_trait.is_match(t) {
            return true;
        }
        // instance.method() or Type.method()
        if re_trait.is_match(t) && (t.contains('.') || t.contains(": ")) {
            return true;
        }
    }
    false
}

pub fn has_ts_wire(content: &str, re_trait: &Regex) -> bool {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("//") || t.starts_with("/*") {
            continue;
        }
        // new TraitName() constructor injection
        if re_trait.is_match(t) && t.contains("new ") && t.contains('(') {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

pub fn word_boundary_re(trait_name: &str) -> Regex {
    Regex::new(&format!(r"\b{}\b", regex::escape(trait_name))).expect("valid regex")
}

pub fn not_orphan() -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

pub fn build_search_files(all_files: &[FilePath], root_dir: &FilePath) -> Vec<String> {
    let mut search_files: Vec<String> = all_files.iter().map(|fp| fp.value().to_string()).collect();
    let root_path = std::path::Path::new(root_dir.value());
    for ws_dir in &["crates", "packages", "modules"] {
        let ws_path = root_path.join(ws_dir);
        if ws_path.exists() {
            collect_source_files(&ws_path, &mut search_files);
        }
    }
    search_files
}

pub fn orphan_result(
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

// ═══════════════════════════════════════════════════════════════════════════════
// TRAIT NAME EXTRACTION
// ═══════════════════════════════════════════════════════════════════════════════

// #3: Route by language — don't strip # for Rust
pub fn strip_comments(content: &str, ext: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if in_block_comment {
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        // Rust: only strip // and /* — NOT #
        if ext == "rs" {
            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                    in_block_comment = true;
                }
                continue;
            }
            let code_line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            result.push_str(code_line);
            result.push('\n');
            continue;
        }

        // Python: strip # and """
        if ext == "py" {
            if trimmed.starts_with('#') {
                continue;
            }
            let code_line = if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            };
            result.push_str(code_line);
            result.push('\n');
            continue;
        }

        // TypeScript/JS: strip // and /* — NOT #
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                in_block_comment = true;
            }
            continue;
        }
        let code_line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else {
            line
        };
        result.push_str(code_line);
        result.push('\n');
    }

    result
}

// #10: Route by file extension
pub fn extract_contract_trait_name(content: &str, file_path: &str) -> Option<String> {
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let code = strip_comments(content, ext);

    match ext {
        "rs" => re_contract_rust()
            .and_then(|re| re.captures(&code))
            .map(|caps| caps[1].to_string()),
        "py" => {
            // #11: Try ABC/Protocol first, fallback to any class
            if let Some(caps) = re_contract_py().and_then(|re| re.captures(&code)) {
                caps.get(1)
                    .or_else(|| caps.get(2))
                    .map(|m| m.as_str().to_string())
            } else {
                // Fallback: first class definition
                let fallback = Regex::new(r"class\s+([A-Za-z0-9_]+)\s*[\(:]").expect("valid regex");
                fallback.captures(&code).map(|caps| caps[1].to_string())
            }
        }
        "ts" | "tsx" | "js" | "jsx" => re_ts_interface_export()
            .and_then(|re| re.captures(&code))
            .or_else(|| re_interface().and_then(|re| re.captures(&code)))
            .map(|caps| caps[1].to_string()),
        _ => None,
    }
}

// #19: Handle symlinks
pub fn collect_source_files(dir: &std::path::Path, files: &mut Vec<String>) {
    let meta = match std::fs::symlink_metadata(dir) {
        Ok(m) => m,
        Err(_) => return,
    };
    if meta.file_type().is_symlink() {
        return;
    }

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
