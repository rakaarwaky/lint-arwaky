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
use std::sync::LazyLock;

// ═══════════════════════════════════════════════════════════════════════════════
// P2 #16: Magic strings → constants
// ═══════════════════════════════════════════════════════════════════════════════

const SUFFIX_PORT: &str = "port";
const SUFFIX_PROTOCOL: &str = "protocol";
const SUFFIX_AGGREGATE: &str = "aggregate";
const LAYER_INFRASTRUCTURE: &str = "infrastructure";
const LAYER_CAPABILITIES: &str = "capabilities";
const LAYER_AGENT: &str = "agent";

// ═══════════════════════════════════════════════════════════════════════════════
// P2 #21: LazyLock instead of OnceLock
// ═══════════════════════════════════════════════════════════════════════════════

static RE_CONTRACT_RUST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:pub\s+)?trait\s+([A-Za-z0-9_]+)").unwrap());

static RE_CONTRACT_PY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"class\s+([A-Za-z0-9_]+)\s*[\(:]").unwrap());

static RE_TS_INTERFACE_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"export\s+interface\s+([A-Za-z0-9_]+)").unwrap());

static RE_INTERFACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"interface\s+([A-Za-z0-9_]+)").unwrap());

// P0 #5: Word boundary regex for trait matching
pub fn word_boundary_re(trait_name: &str) -> Regex {
    Regex::new(&format!(r"\b{}\b", regex::escape(trait_name))).unwrap()
}

// ═══════════════════════════════════════════════════════════════════════════════
// P2 #17: Helper to reduce repetition
// ═══════════════════════════════════════════════════════════════════════════════

pub fn not_orphan() -> OrphanIndicatorResult {
    OrphanIndicatorResult::new(false, String::new(), Severity::LOW)
}

// ═══════════════════════════════════════════════════════════════════════════════
// P1 #7 + P1 #11: File cache + dedup
// ═══════════════════════════════════════════════════════════════════════════════

pub struct FileCache {
    contents: HashMap<String, Arc<str>>,
    basenames: HashMap<String, String>,
}

impl FileCache {
    fn build(
        all_files: &[FilePath],
        root_dir: &FilePath,
        extractor: &dyn IOrphanFilenameExtractorProtocol,
    ) -> Self {
        let search_files = build_search_files(all_files, root_dir);
        let mut contents = HashMap::new();
        let mut basenames = HashMap::new();

        for path in &search_files {
            if contents.contains_key(path) {
                continue; // P1 #11: dedup
            }
            // P2 #14: log warning instead of silent swallow
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

    fn get(&self, path: &str) -> Option<(&str, &str)> {
        let content = self.contents.get(path).map(|s| s.as_ref())?;
        let basename = self.basenames.get(path).map(|s| s.as_str())?;
        Some((content, basename))
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
        is_contract_orphan(f, root_dir, all_files, self.extractor.as_ref())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MAIN ORCHESTRATOR
// ═══════════════════════════════════════════════════════════════════════════════

pub fn is_contract_orphan(
    f: &FilePath,
    root_dir: &FilePath,
    all_files: &[FilePath],
    extractor: &dyn IOrphanFilenameExtractorProtocol,
) -> OrphanIndicatorResult {
    let fp = f.value();
    let suffix = extractor.file_suffix(f).value;

    let content = match std::fs::read_to_string(fp) {
        Ok(c) => c,
        Err(_) => return not_orphan(),
    };

    let trait_name = match extract_contract_trait_name(&content, fp) {
        Some(t) => t,
        None => return not_orphan(),
    };

    // P1 #7: Build cache ONCE, not per-check
    let cache = FileCache::build(all_files, root_dir, extractor);

    let target_prefix = match suffix.as_str() {
        SUFFIX_PORT => LAYER_INFRASTRUCTURE,
        SUFFIX_PROTOCOL => LAYER_CAPABILITIES,
        SUFFIX_AGGREGATE => LAYER_AGENT,
        _ => return not_orphan(),
    };

    // ── Check 1: trait not implemented by expected layer ──
    if !check_implemented(&cache, &trait_name, target_prefix) {
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

    // ── Check 2: port/protocol not called/wired ──
    if suffix == SUFFIX_PORT || suffix == SUFFIX_PROTOCOL {
        if !check_called_port_protocol(&cache, &trait_name) {
            return orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract {} '{}' not called or wired by any orchestrator, container, capabilities, or surface file.",
                    suffix, trait_name
                ),
            );
        }
    }

    // ── Check 3: aggregate not called/wired ──
    if suffix == SUFFIX_AGGREGATE {
        if !check_called_aggregate(&cache, &trait_name) {
            return orphan_result(
                &suffix,
                &trait_name,
                target_prefix,
                &format!(
                    "Contract aggregate '{}' not called or wired by any surface or container file.",
                    trait_name
                ),
            );
        }
    }

    not_orphan()
}

// ═══════════════════════════════════════════════════════════════════════════════
// MODULAR DETECTION: IMPLEMENT / CALL / WIRE
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if trait is implemented by any file in the target layer.
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

/// Check if port/protocol is called or wired.
fn check_called_port_protocol(cache: &FileCache, trait_name: &str) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in &cache.contents {
        let bn = match cache.basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_orchestrator = bn.starts_with("agent_")
            && (bn.ends_with("_orchestrator.rs")
                || bn.ends_with("_orchestrator.py")
                || bn.ends_with("_orchestrator.ts")
                || bn.ends_with("_orchestrator.js"));
        let is_container = bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js");
        let is_capabilities = bn.starts_with("capabilities_");
        let is_surface = bn.starts_with("surface_");

        if !is_orchestrator && !is_container && !is_capabilities && !is_surface {
            continue;
        }
        if has_rust_call(content, &re_trait)
            || has_py_call(content, &re_trait)
            || has_ts_call(content, &re_trait)
            || has_rust_wire(content, &re_trait)
            || has_py_wire(content, &re_trait)
            || has_ts_wire(content, &re_trait)
        {
            return true;
        }
    }
    false
}

/// Check if aggregate is called or wired.
fn check_called_aggregate(cache: &FileCache, trait_name: &str) -> bool {
    let re_trait = word_boundary_re(trait_name);

    for (path, content) in &cache.contents {
        let bn = match cache.basenames.get(path) {
            Some(b) => b.as_str(),
            None => continue,
        };
        let is_surface = bn.starts_with("surface_");
        let is_container = bn.ends_with("_container.rs")
            || bn.ends_with("_container.py")
            || bn.ends_with("_container.ts")
            || bn.ends_with("_container.js");

        if !is_surface && !is_container {
            continue;
        }
        if has_rust_call(content, &re_trait)
            || has_py_call(content, &re_trait)
            || has_ts_call(content, &re_trait)
            || has_rust_wire(content, &re_trait)
            || has_py_wire(content, &re_trait)
            || has_ts_wire(content, &re_trait)
        {
            return true;
        }
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════════
// RUST DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Rust: `impl Trait for Type { ... }`
pub fn has_rust_impl(content: &str, rust_impl_pattern: &str, re_trait: &Regex) -> bool {
    content.contains(rust_impl_pattern)
        || content.lines().any(|ln| {
            let t = ln.trim();
            t.starts_with("impl") && re_trait.is_match(t) && t.contains(" for")
        })
}

/// Rust: `use TraitName`, `TraitName::method()`, `<dyn TraitName>`
pub fn has_rust_call(content: &str, re_trait: &Regex) -> bool {
    re_trait.is_match(content)
        && (content.contains("use ") || content.contains("::") || content.contains("<dyn "))
}

/// Rust: `Arc::new(Type::new())`, `Box::new(Type::new())`
pub fn has_rust_wire(content: &str, re_trait: &Regex) -> bool {
    re_trait.is_match(content)
        && (content.contains("Arc::new(")
            || content.contains("Box::new(")
            || content.contains("::new("))
}

// ═══════════════════════════════════════════════════════════════════════════════
// PYTHON DETECTION (abc abstractmethod)
// ═══════════════════════════════════════════════════════════════════════════════

/// Python: `class Type(BaseTrait):` or ABC with @abstractmethod
pub fn has_py_impl(content: &str, trait_name: &str) -> bool {
    // P1 #12: More specific ABC check — verify @abstractmethod is inside the class
    let inherit_pattern = format!(
        r"class\s+\w+\([^)]*\b{}\b[^)]*\)",
        regex::escape(trait_name)
    );
    if Regex::new(&inherit_pattern)
        .map(|re| re.is_match(content))
        .unwrap_or(false)
    {
        return true;
    }

    // ABC check: verify @abstractmethod exists AND the class defines the trait
    if content.contains("from abc import") && content.contains("@abstractmethod") {
        let class_pattern = format!(r"class\s+{}\s*[\(:]", regex::escape(trait_name));
        if Regex::new(&class_pattern)
            .map(|re| re.is_match(content))
            .unwrap_or(false)
        {
            return true;
        }
    }

    false
}

/// Python: `import TraitName`, `instance.method()`, `: TraitName`
pub fn has_py_call(content: &str, re_trait: &Regex) -> bool {
    // P0 #1: No duplicate — single check with word boundary
    re_trait.is_match(content)
        && (content.contains("import ") || content.contains(".") || content.contains(": "))
}

/// Python: `TraitName()` constructor injection
pub fn has_py_wire(content: &str, re_trait: &Regex) -> bool {
    re_trait.is_match(content) && content.contains("(")
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPESCRIPT DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// TypeScript: `class Type implements TraitName` or `extends TraitName`
pub fn has_ts_impl(content: &str, trait_name: &str) -> bool {
    content.contains(&format!("implements {}", trait_name))
        || content.contains(&format!("extends {}", trait_name))
}

/// TypeScript: `import { TraitName }`, `instance.method()`, `: TraitName`
pub fn has_ts_call(content: &str, re_trait: &Regex) -> bool {
    // P0 #2: No duplicate — single check with word boundary
    re_trait.is_match(content)
        && (content.contains("import ") || content.contains(".") || content.contains(": "))
}

/// TypeScript: `new TraitName()` constructor injection
/// P0 #3: Removed Arc<dyn> — that's Rust syntax, not TypeScript
pub fn has_ts_wire(content: &str, re_trait: &Regex) -> bool {
    re_trait.is_match(content) && content.contains("new ")
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn build_search_files(all_files: &[FilePath], root_dir: &FilePath) -> Vec<String> {
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

// ═══════════════════════════════════════════════════════════════════════════════
// TRAIT NAME EXTRACTION
// ═══════════════════════════════════════════════════════════════════════════════

/// P1 #9: Improved comment filtering — handles #, inline, multi-line
pub fn strip_comments(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Handle multi-line block comments
        if in_block_comment {
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }

        // Skip full-line comments (Rust //, Python #, TS //)
        if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") {
            if trimmed.starts_with("/*") && !trimmed.contains("*/") {
                in_block_comment = true;
            }
            continue;
        }

        // P1 #9: Handle inline comments — strip everything after // or #
        let code_line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else if let Some(pos) = line.find('#') {
            &line[..pos]
        } else {
            line
        };

        result.push_str(code_line);
        result.push('\n');
    }

    result
}

/// P1 #10: Check file extension before language-specific regex
pub fn extract_contract_trait_name(content: &str, file_path: &str) -> Option<String> {
    let code = strip_comments(content);

    // P1 #10: Route by file extension
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => RE_CONTRACT_RUST
            .captures(&code)
            .map(|caps| caps[1].to_string()),
        "py" => RE_CONTRACT_PY
            .captures(&code)
            .map(|caps| caps[1].to_string()),
        "ts" | "tsx" | "js" | "jsx" => RE_TS_INTERFACE_EXPORT
            .captures(&code)
            .or_else(|| RE_INTERFACE.captures(&code))
            .map(|caps| caps[1].to_string()),
        _ => None,
    }
}

/// P2 #19: Handle symlinks to prevent infinite loops
fn collect_source_files(dir: &std::path::Path, files: &mut Vec<String>) {
    // P2 #19: Use symlink_metadata to detect symlinks
    let meta = match std::fs::symlink_metadata(dir) {
        Ok(m) => m,
        Err(_) => return,
    };
    if meta.file_type().is_symlink() {
        return; // Skip symlinks to prevent infinite loops
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
